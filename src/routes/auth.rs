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
    let tokens = google_oauth::exchange_code_for_tokens(query.code.clone())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let pool = get_pool();

    // store refresh token
    // if let Some(refresh) = tokens.refresh_token.clone() {
    //     sqlx::query!(
    //         "INSERT INTO user_tokens (refresh_token) VALUES ($1)",
    //         refresh
    //     )
    //     .execute(pool)
    //     .await
    //     .unwrap();
    // }

    // decode id_token → get email
   let claims = jwt::decode_jwt_payload(&tokens.id_token);
   let email = claims["email"].as_str().unwrap().to_string();

   let refresh = tokens.refresh_token.clone().unwrap();

   db::user_tokens::insert_token(&email, &refresh).await;
   
   let jwt = jwt::generate_jwt(&email.clone());

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "jwt": jwt,
        "email": email
    })))
}
