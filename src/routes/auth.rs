use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;
use base64::{engine::general_purpose, Engine};
use crate::db;


use crate::db::get_pool;
use crate::services::{google_oauth, jwt};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(start_google_auth)
       .service(google_callback);
}

#[get("/auth/google/start")]
async fn start_google_auth() -> HttpResponse {
    let state = Uuid::new_v4().to_string();
    let auth_url = google_oauth::build_auth_url(state.clone());

    HttpResponse::Ok().json(serde_json::json!({
        "auth_url": auth_url,
        "state": state
    }))
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
    state: String,
}

#[get("/auth/google/callback")]
async fn google_callback(query: web::Query<CallbackQuery>) -> Result<HttpResponse, actix_web::Error> {
    // exchange code → tokens
    let tokens = match google_oauth::exchange_code_for_tokens(query.code.clone()).await {
        Ok(tokens) => tokens,
        Err(e) => {
            log::error!("Failed to exchange code for tokens: {}", e);
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Failed to exchange authorization code",
                "message": e
            })));
        }
    };

    // Check if we got an id_token
    if tokens.id_token.is_empty() {
        log::error!("No id_token in response");
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No id_token received from Google"
        })));
    }

    // decode id_token → get email
    let claims = jwt::decode_jwt_payload(&tokens.id_token);
    let email = match claims["email"].as_str() {
        Some(email) => email.to_string(),
        None => {
            log::error!("No email in id_token claims: {:?}", claims);
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "No email found in token"
            })));
        }
    };

    // Store refresh token if available
    if let Some(refresh) = tokens.refresh_token.clone() {
        db::user_tokens::insert_token(&email, &refresh).await;
    } else {
        log::warn!("No refresh token received for user: {}", email);
    }
   
    let jwt = jwt::generate_jwt(&email);

    // Redirect to frontend with token in query params
    // Frontend will extract token from URL and store it
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "https://drafly.onrender.com".to_string());
    let redirect_url = format!("{}/login?token={}&email={}", frontend_url, jwt, urlencoding::encode(&email));
    
    Ok(HttpResponse::Found()
        .insert_header(("Location", redirect_url))
        .finish())
}
