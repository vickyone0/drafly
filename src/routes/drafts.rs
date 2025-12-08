use actix_web::{post, get, patch, web, HttpResponse};
use serde::Deserialize;
use crate::db;
use crate::middleware::AuthenticatedUser;
//use crate::services::ai_service;
use crate::services::groq_ai;

#[derive(Deserialize)]
pub struct DraftRequest {
    email_id: i32,
    tone: Option<String>,
}

#[post("/drafts/generate")]
async fn generate_draft(req: web::Json<DraftRequest>, user: AuthenticatedUser) -> HttpResponse {
    let pool = db::get_pool();
    let user_email = user.email.clone();

    // fetch email content with sender and subject
    let email_row = sqlx::query!(
        "SELECT body_text, sender, subject, user_email FROM emails WHERE id = $1 AND user_email = $2",
        req.email_id,
        user_email
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    if email_row.is_none() {
        return HttpResponse::NotFound().body("Email not found");
    }

    let email_data = email_row.unwrap();
    let email_body = email_data.body_text.clone().unwrap_or_default();
    let sender = email_data.sender.clone().unwrap_or_default();
    let subject = email_data.subject.clone().unwrap_or_default();

    let tone = req.tone.clone().unwrap_or("friendly".into());

    // generate draft using AI
    let generated = match groq_ai::generate_reply(&email_body, &sender, &subject, &tone).await {
        Ok(text) => text,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    // save draft
    let row = sqlx::query!(
        r#"
        INSERT INTO drafts (email_id, user_email, content, tone)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        req.email_id,
        user_email,
        generated,
        tone
    )
    .fetch_one(pool)
    .await
    .unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "draft_id": row.id,
        "content": generated
    }))
}

#[get("/drafts/{id}")]
async fn get_draft(path: web::Path<i32>, user: AuthenticatedUser) -> HttpResponse {
    let id = path.into_inner();
    let pool = db::get_pool();

    let row = sqlx::query!(
        "SELECT * FROM drafts WHERE id = $1 AND user_email = $2",
        id,
        user.email
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    if let Some(r) = row {
        return HttpResponse::Ok().json(serde_json::json!({
            "id": r.id,
            "email_id": r.email_id,
            "content": r.content,
            "tone": r.tone,
            "status": r.status,
            "created_at": r.created_at
        }));
    }

    HttpResponse::NotFound().body("Draft not found")
}

#[derive(Deserialize)]
pub struct DraftUpdate {
    content: String,
}

#[patch("/drafts/{id}")]
async fn update_draft(path: web::Path<i32>, req: web::Json<DraftUpdate>, user: AuthenticatedUser) -> HttpResponse {
    let id = path.into_inner();
    let pool = db::get_pool();

    sqlx::query!(
        "UPDATE drafts SET content = $1, updated_at = NOW() WHERE id = $2 AND user_email = $3",
        req.content,
        id,
        user.email
    )
    .execute(pool)
    .await
    .unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "updated": true
    }))
}

#[post("/drafts/{id}/approve")]
async fn approve_draft(path: web::Path<i32>, user: AuthenticatedUser) -> HttpResponse {
    let id = path.into_inner();
    let pool = db::get_pool();

    sqlx::query!(
        "UPDATE drafts SET status = 'approved', updated_at = NOW() WHERE id = $1 AND user_email = $2",
        id,
        user.email
    )
    .execute(pool)
    .await
    .unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "approved": true
    }))
}

#[post("/drafts/{id}/send")]
async fn send_draft(path: web::Path<i32>, user: AuthenticatedUser) -> HttpResponse {
    let draft_id = path.into_inner();
    let pool = db::get_pool();

    // fetch draft
    let draft = sqlx::query!(
        "SELECT id, email_id, user_email, content, status 
         FROM drafts WHERE id = $1 AND user_email = $2",
        draft_id,
        user.email
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    if draft.is_none() {
        return HttpResponse::NotFound().body("Draft not found");
    }

    let d = draft.unwrap();

    if d.status.as_deref() != Some("approved") {
    return HttpResponse::BadRequest().body("Draft must be approved before sending");
}

    // fetch parent email info
    let email = sqlx::query!(
        "SELECT sender, subject, thread_id FROM emails WHERE id = $1",
        d.email_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    let sender_email = email.sender.unwrap_or_default();
    let subject = email.subject.unwrap_or("No subject".to_string());
    let thread_id = email.thread_id.unwrap_or_default();

    // send email via Gmail API
   let result = crate::services::gmail_sender::send_reply(
    d.user_email.as_deref().unwrap_or(""),
    &sender_email,
    &subject,
    &thread_id,
    d.content.as_deref().unwrap_or(""),
)
.await;

    match result {
        Ok(sent_gmail_id) => {
            // update draft status to "sent"
            sqlx::query!(
                "UPDATE drafts SET status = 'sent', sent = TRUE, sent_gmail_id = $1, updated_at = NOW() WHERE id = $2",
                sent_gmail_id,
                d.id
            )
            .execute(pool)
            .await
            .unwrap();

            HttpResponse::Ok().json(serde_json::json!({
                "sent": true,
                "sent_gmail_id": sent_gmail_id
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}


#[get("/drafts")]
async fn list_drafts(user: AuthenticatedUser) -> HttpResponse {
    let pool = db::get_pool();

    let rows = sqlx::query!(
        "SELECT id, email_id, user_email, content, tone, status, created_at, updated_at 
         FROM drafts 
         WHERE user_email = $1
         ORDER BY created_at DESC",
        user.email
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let drafts: Vec<_> = rows.into_iter().map(|r| {
        serde_json::json!({
            "id": r.id,
            "email_id": r.email_id,
            "content": r.content,
            "tone": r.tone,
            "status": r.status,
            "created_at": r.created_at
        })
    }).collect();

    HttpResponse::Ok().json(drafts)
}

#[get("/drafts/{id}")]
async fn list_drafts(path: web::Path<i32>,user: AuthenticatedUser) -> HttpResponse {
    let draft_id = path.into_inner();
    let pool = db::get_pool();

    let rows = sqlx::query!(
        "SELECT id, email_id, user_email, content, tone, status, created_at, updated_at 
         FROM drafts 
         WHERE user_email = $1,id = $2
         ORDER BY created_at DESC",
        user.email
,draft_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    let drafts: Vec<_> = rows.into_iter().map(|r| {
        serde_json::json!({
            "id": r.id,
            "email_id": r.email_id,
            "content": r.content,
            "tone": r.tone,
            "status": r.status,
            "created_at": r.created_at
        })
    }).collect();

    HttpResponse::Ok().json(drafts)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_drafts)
        .service(generate_draft)
        .service(get_draft)
        .service(update_draft)
        .service(approve_draft)
        .service(send_draft);
}
