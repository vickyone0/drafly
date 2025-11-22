use reqwest::Client;
use serde_json::json;

pub async fn generate_reply(email_body: &str, tone: &str) -> Result<String, String> {
    let api_key = std::env::var("GROQ_API_KEY")
        .map_err(|_| "Missing GROQ_API_KEY".to_string())?;

    let prompt = format!(
        "Write a {} professional email reply. Keep it concise and polite.\n\nEmail:\n{}\n\nReply:",
        tone,
        email_body
    );

    let body = json!({
        "model": "llama-3.3-70b-versatile",
        "messages": [
            { "role": "user", "content": prompt }
        ],
        "max_tokens": 300
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
