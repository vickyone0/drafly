use reqwest::Client;
use serde_json::json;

pub async fn generate_reply(email_body: &str, tone: &str) -> Result<String, String> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "Missing OPENAI_API_KEY".to_string())?;

    let prompt = format!(
        "Write a {} reply to this email:\n\n{}\n\nYour reply:",
        tone, email_body
    );

    let body = json!({
        "model": "gpt-4o-mini",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 300
    });

    let client = Client::new();

    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request error: {:?}", e))?;

    let text = resp.text().await.map_err(|e| format!("Read error: {:?}", e))?;

    println!("\n🔍 OPENAI RAW RESPONSE:\n{}\n", text);
    
    let json_resp: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("JSON error: {:?}", e))?;

    let reply = json_resp["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Unable to generate drafts.")
        .to_string();

    Ok(reply)
}
