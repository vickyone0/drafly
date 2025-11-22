use reqwest::Client;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use crate::services::google_oauth;
use crate::db;
use serde_json::json;

pub async fn send_reply(
    user_email: &str,
    to: &str,
    subject: &str,
    thread_id: &str,
    message_body: &str,
) -> Result<String, String> {
    let access_token = google_oauth::refresh_access_token_for_user(user_email).await?;

    // Build MIME message
    let mime = format!(
        "From: {}\r\nTo: {}\r\nSubject: Re: {}\r\nIn-Reply-To: {}\r\nReferences: {}\r\nContent-Type: text/plain; charset=\"UTF-8\"\r\n\r\n{}",
        user_email,
        to,
        subject,
        thread_id,
        thread_id,
        message_body
    );

    // Gmail API requires base64url encoding
    let encoded = URL_SAFE_NO_PAD.encode(mime.as_bytes());

    let client = Client::new();

    let url = "https://gmail.googleapis.com/gmail/v1/users/me/messages/send";

    let payload = json!({
        "raw": encoded,
        "threadId": thread_id
    });

    let resp = client
        .post(url)
        .bearer_auth(access_token)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {:?}", e))?;
let status = resp.status();
let txt = resp.text().await.unwrap();
println!("\n📨 GMAIL SEND RESPONSE:\n{}\n", txt);

if !status.is_success() {
    return Err(format!("Send failed: {}", txt));
}

    let json: serde_json::Value =
        serde_json::from_str(&txt).map_err(|e| format!("JSON decode error: {:?}", e))?;

    let sent_gmail_id = json["id"].as_str().unwrap_or("").to_string();

    Ok(sent_gmail_id)
}
