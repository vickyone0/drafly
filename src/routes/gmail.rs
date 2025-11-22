use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
// Remove unused imports
// use crate::models::EmailRow;
// use sqlx::Row;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_emails)
       .service(get_email)
       .service(fetch_unread)
       .service(fetch_one);
}

#[get("/emails")]
async fn list_emails() -> Result<HttpResponse, actix_web::Error> {
    let pool = crate::db::get_pool();
    let rows = sqlx::query!(
        r#"
        SELECT id, gmail_id, thread_id, user_email, sender, to_recipients, subject, snippet, body_text, body_html, labels, fetched_at
        FROM emails
        ORDER BY fetched_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        log::error!("db error: {:?}", e);
        actix_web::error::ErrorInternalServerError("db error")
    })?;

    let mapped: Vec<_> = rows.into_iter().map(|r| {
        serde_json::json!({
            "id": r.id,
            "gmail_id": r.gmail_id,
            "thread_id": r.thread_id,
            "user_email": r.user_email,
            "sender": r.sender,
            "to_recipients": r.to_recipients,
            "subject": r.subject,
            "snippet": r.snippet,
            "has_body": r.body_text.is_some() || r.body_html.is_some(),
            "fetched_at": r.fetched_at,
        })
    }).collect();

    Ok(HttpResponse::Ok().json(mapped))
}

#[get("/emails/{id}")]
async fn get_email(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    let pool = crate::db::get_pool();
    let row = sqlx::query!(
        r#"
        SELECT id, gmail_id, thread_id, user_email, sender, to_recipients, subject, snippet, body_text, body_html, labels, fetched_at
        FROM emails WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        log::error!("db error: {:?}", e);
        actix_web::error::ErrorInternalServerError("db error")
    })?;

    if let Some(r) = row {
        let out = serde_json::json!({
            "id": r.id,
            "gmail_id": r.gmail_id,
            "thread_id": r.thread_id,
            "user_email": r.user_email,
            "sender": r.sender,
            "to_recipients": r.to_recipients,
            "subject": r.subject,
            "snippet": r.snippet,
            "body_text": r.body_text,
            "body_html": r.body_html,
            "labels": r.labels,
            "fetched_at": r.fetched_at
        });
        Ok(HttpResponse::Ok().json(out))
    } else {
        Ok(HttpResponse::NotFound().body("email not found"))
    }
}

#[post("/internal/fetch-unread")]
async fn fetch_unread() -> Result<HttpResponse, actix_web::Error> {
    let pool = crate::db::get_pool();
    let rec = sqlx::query!("SELECT email FROM user_tokens LIMIT 1")
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("db err: {:?}", e);
            actix_web::error::ErrorInternalServerError("db error")
        })?;

    let user_email = match rec {
        Some(r) => match r.email {
            Some(ref email) => email.clone(),
            None => return Ok(HttpResponse::BadRequest().body("no user token")),
        },
        None => return Ok(HttpResponse::BadRequest().body("no user tokenss")),
    };

    let access_token = match crate::services::google_oauth::refresh_access_token_for_user(&user_email).await {
        Ok(at) => at,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("token refresh failed: {}", e))),
    };

    let client = reqwest::Client::new();
    let list_url = "https://gmail.googleapis.com/gmail/v1/users/me/messages?q=is:unread label:inbox";
    let resp = client.get(list_url).bearer_auth(access_token)
        .send().await
        .map_err(|e| {
            log::error!("http: {:?}", e);
            actix_web::error::ErrorInternalServerError("http error")
        })?;

    let status = resp.status();
    let text = resp.text().await
        .map_err(|e| {
            log::error!("text: {:?}", e);
            actix_web::error::ErrorInternalServerError("text error")
        })?;
    if !status.is_success() {
        return Ok(HttpResponse::InternalServerError().body(format!("gmail list err: {}", text)));
    }

    let v: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| {
            log::error!("json: {:?}", e);
            actix_web::error::ErrorInternalServerError("json error")
        })?;
    let messages = v["messages"].as_array().cloned().unwrap_or_default();

    for m in messages.into_iter().take(20) {
        if let Some(gid) = m["id"].as_str() {
            if let Err(e) = crate::services::gmail_fetcher::fetch_and_store_message(&user_email, gid).await {
                log::error!("fetch store failed for {}: {:?}", gid, e);
            }
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({"fetched": true})))
}

#[derive(Deserialize)]
pub struct FetchOnePath {
    gmail_id: String,
}

#[post("/internal/fetch/{gmail_id}")]
async fn fetch_one(path: web::Path<FetchOnePath>) -> Result<HttpResponse, actix_web::Error> {
    let gmail_id = path.into_inner().gmail_id;
    let pool = crate::db::get_pool();
    let rec = sqlx::query!("SELECT email FROM user_tokens LIMIT 1")
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("db err: {:?}", e);
            actix_web::error::ErrorInternalServerError("db error")
        })?;

    let user_email = match rec {
        Some(r) => match r.email {
            Some(ref email) => email.clone(),
            None => return Ok(HttpResponse::BadRequest().body("no user tokens")),
        },
        None => return Ok(HttpResponse::BadRequest().body("no user tokens")),
    };

    match crate::services::gmail_fetcher::fetch_and_store_message(&user_email, &gmail_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"ok": true}))),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("fetch failed: {}", e))),
    }
}