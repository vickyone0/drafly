use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;
use crate::db;
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
    // 1. Exchange the auth code for tokens
    let tokens = match google_oauth::exchange_code_for_tokens(query.code.clone()).await {
        Ok(t) => t,
        Err(e) => {
            log::error!("Token exchange failed: {}", e);
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "token_exchange_failed",
                "details": e.to_string()
            })));
        }
    };

    if tokens.id_token.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "missing_id_token"
        })));
    }

    // 2. Extract email from id_token
    let claims = jwt::decode_jwt_payload(&tokens.id_token);

    let email = match claims.get("email").and_then(|e| e.as_str()) {
        Some(e) => e.to_string(),
        None => {
            log::error!("No email in id_token claims: {:?}", claims);
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "email_missing_in_token"
            })));
        }
    };

    // 3. Store refresh_token (if provided)
    if let Some(refresh) = tokens.refresh_token.clone() {
        if let Err(e) = db::user_tokens::insert_token(&email, &refresh).await {
            log::error!("Failed to store refresh token for {}: {}", email, e);
        }
    }

    // 4. Backend-generated JWT for frontend sessions
    let jwt = jwt::generate_jwt(&email);

    // 5. Redirect the user safely to frontend
    let frontend = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:3000".into());

    let redirect_url = format!(
        "{}/login?token={}&email={}",
        frontend,
        jwt,
        urlencoding::encode(&email)
    );
    println!("\n🔍 REDIRECTING TO: {}\n", redirect_url);

    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .finish())

}
