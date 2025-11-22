use reqwest::Client;
use serde_json::json;

pub async fn generate_reply(email_body: &str, sender: &str, subject: &str, tone: &str) -> Result<String, String> {
    let api_key = std::env::var("GROQ_API_KEY")
        .map_err(|_| "Missing GROQ_API_KEY".to_string())?;

    // Extract sender name from email address if possible
    let sender_name = if let Some(at_pos) = sender.find('@') {
        sender[..at_pos].split('.').next().unwrap_or(&sender[..at_pos])
    } else {
        sender
    };

    let prompt = format!(
        r#"You are writing a professional email reply. Write a complete, ready-to-send email reply in a {} tone.

IMPORTANT INSTRUCTIONS:
- Write a complete email reply - do NOT use placeholders like [Name], [topic], [Your Name], etc.
- Use the actual sender's name or email address from the context
- Reference the original email subject naturally
- Keep it concise (2-4 sentences typically)
- Be professional, polite, and {} in tone
- Write as if you are directly replying to the sender
- Do not include email headers (To, From, Subject) - just the reply body text

Original Email:
From: {}
Subject: {}
Body: {}

Write your complete email reply (body text only, no placeholders):"#,
        tone,
        tone,
        sender,
        if subject.is_empty() { "No subject" } else { subject },
        email_body
    );

    let body = json!({
        "model": "llama-3.3-70b-versatile",
        "messages": [
            { "role": "system", "content": "You are a professional email assistant. Write complete, ready-to-send email replies without any placeholders or variables." },
            { "role": "user", "content": prompt }
        ],
        "max_tokens": 500,
        "temperature": 0.7
    });

    let client = Client::new();
    let resp = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request error: {:?}", e))?;

    let txt = resp.text().await.unwrap();
    println!("🔍 GROQ RAW RESPONSE:\n{}\n", txt);

    let json_resp: serde_json::Value =
        serde_json::from_str(&txt).map_err(|e| format!("JSON error: {:?}", e))?;

    let reply = json_resp["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Unable to generate draft.")
        .to_string();

    Ok(reply)
}
