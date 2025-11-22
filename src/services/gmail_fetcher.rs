use crate::db;
use reqwest::Client;
use serde_json::Value;
use crate::services::google_oauth;
use sqlx::Row;
use chrono::Utc;

pub async fn fetch_and_store_message(user_email: &str, gmail_id: &str) -> Result<(), String> {
    let access_token = google_oauth::refresh_access_token_for_user(user_email).await?;
    let client = Client::new();

    // fetch full message
    let url = format!("https://gmail.googleapis.com/gmail/v1/users/me/messages/{}?format=full", gmail_id);
    let resp = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("http error: {:?}", e))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| format!("text err: {:?}", e))?;
    if !status.is_success() {
        return Err(format!("gmail api error {} : {}", status, text));
    }

    let json: Value = serde_json::from_str(&text).map_err(|e| format!("json parse: {:?}", e))?;

    // parse headers and body
    let headers = json["payload"]["headers"].as_array().cloned().unwrap_or_default();
    let mut subject = None;
    let mut from = None;
    let mut to = None;
    let mut thread_id = json["threadId"].as_str().map(|s| s.to_string());

    for h in headers {
        if let (Some(name), Some(val)) = (h["name"].as_str(), h["value"].as_str()) {
            match name.to_lowercase().as_str() {
                "subject" => subject = Some(val.to_string()),
                "from" => from = Some(val.to_string()),
                "to" => to = Some(val.to_string()),
                _ => {}
            }
        }
    }

    // snippet
    let snippet = json["snippet"].as_str().map(|s| s.to_string());

    // body: there are parts; prefer "text/plain" else extract html and convert
    let mut body_text: Option<String> = None;
    let mut body_html: Option<String> = None;

    fn extract_parts(part: &Value, bt: &mut Option<String>, bh: &mut Option<String>) {
        if let Some(mime) = part["mimeType"].as_str() {
            if mime == "text/plain" {
                if let Some(data) = part["body"]["data"].as_str() {
                    let decoded = base64_engine_decode(data);
                    *bt = Some(decoded);
                }
            } else if mime == "text/html" {
                if let Some(data) = part["body"]["data"].as_str() {
                    let decoded = base64_engine_decode(data);
                    *bh = Some(decoded);
                }
            } else if mime.starts_with("multipart/") {
                if let Some(parts) = part["parts"].as_array() {
                    for p in parts {
                        extract_parts(p, bt, bh);
                    }
                }
            }
        }
    }

    extract_parts(&json["payload"], &mut body_text, &mut body_html);

    // If no plain text but have html, convert
    if body_text.is_none() && body_html.is_some() {
        body_text = Some(html_to_text(body_html.clone().unwrap().as_str()));
    }

    // collect labels
    let labels: Vec<String> = json["labelIds"].as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    // upsert into emails table
    let pool = db::get_pool();
    sqlx::query!(
        r#"
        INSERT INTO emails (gmail_id, thread_id, user_email, sender, to_recipients, subject, snippet, body_text, body_html, labels, fetched_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
        ON CONFLICT (gmail_id) DO UPDATE SET
          thread_id = EXCLUDED.thread_id,
          sender = EXCLUDED.sender,
          subject = EXCLUDED.subject,
          snippet = EXCLUDED.snippet,
          body_text = EXCLUDED.body_text,
          body_html = EXCLUDED.body_html,
          labels = EXCLUDED.labels,
          fetched_at = EXCLUDED.fetched_at
        "#,
        gmail_id,
        thread_id,
        user_email,
        from,
        to,
        subject,
        snippet,
        body_text,
        body_html,
        &labels[..],
        Utc::now().naive_utc()
    )
    .execute(pool)
    .await
    .map_err(|e| format!("db insert error: {:?}", e))?;

    Ok(())
}

fn base64_engine_decode(s: &str) -> String {
    // Gmail uses URL_SAFE base64 with - and _ and no padding
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    let bytes = URL_SAFE_NO_PAD.decode(s).unwrap_or_else(|_| {
        // fallback: try standard engine (some bodies contain normal base64)
        base64::engine::general_purpose::STANDARD.decode(s).unwrap_or_default()
    });
    String::from_utf8(bytes).unwrap_or_default()
}

fn html_to_text(html: &str) -> String {
    // sanitize and convert to plain text
    let cleaned = ammonia::Builder::new().clean(html).to_string();
    html2text::from_read(cleaned.as_bytes(), 1024)
}
