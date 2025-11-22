use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize,Serialize};
use crate::config;
use reqwest::Client;
use crate::db::get_pool;


#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i64>,
    pub id_token: String,
    pub token_type: Option<String>,
    pub scope: Option<String>,
}


#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt(email: String) -> String {
    let exp = (Utc::now() + chrono::Duration::days(7)).timestamp() as usize;

    let claims = Claims { sub: email, exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(crate::config::jwt_secret().as_bytes()),
    )
    .unwrap()
}

pub fn build_auth_url(state: String) -> String {
    let client_id = config::google_client_id();
    let binding = config::google_redirect_uri();
    let redirect_uri = urlencoding::encode(&binding);

    let scope = urlencoding::encode(
        "openid email profile \
        https://www.googleapis.com/auth/gmail.readonly \
        https://www.googleapis.com/auth/gmail.send \
        https://www.googleapis.com/auth/gmail.modify"
    );

    // USE THE STATE PASSED FROM THE ROUTE
    format!(
        "https://accounts.google.com/o/oauth2/v2/auth?\
        client_id={}&redirect_uri={}&response_type=code&\
        scope={}&access_type=offline&prompt=consent&state={}",
        client_id, redirect_uri, scope, state
    )
}

pub async fn exchange_code_for_tokens(code: String) -> Result<TokenResponse, String> {
    let client = Client::new();

    let resp = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code),
            ("client_id", config::google_client_id()),
            ("client_secret", config::google_client_secret()),
            ("redirect_uri", config::google_redirect_uri()),
            ("grant_type", "authorization_code".to_string()),
        ])
        .send()
        .await
        .map_err(|e| format!("Request failed: {:?}", e))?;

    let status = resp.status();
    let text = resp.text().await.unwrap();

    println!("\n🔍 GOOGLE TOKEN RESPONSE:\n{}\n", text);

    if !status.is_success() {
        return Err(format!("Google returned error: {}", text));
    }

    let parsed: TokenResponse =
        serde_json::from_str(&text).map_err(|e| format!("JSON decode error: {:?}", e))?;

    Ok(parsed)
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub id_token: String,
}


pub async fn refresh_access_token_for_user(user_email: &str) -> Result<String, String> {
    // lookup stored refresh token
    let pool = get_pool();
    let rec = sqlx::query!("SELECT refresh_token FROM user_tokens WHERE email = $1 LIMIT 1", user_email)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("db fetch error: {:?}", e))?;

    let refresh_token = match rec {
        Some(r) => r.refresh_token,
        None => return Err("no refresh token stored for user".into()),
    };

    let client_id = crate::config::google_client_id();
    let client_secret = crate::config::google_client_secret();

    let client = Client::new();
    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("refresh_token", refresh_token.as_str()),
        ("grant_type", "refresh_token"),
    ];

    let resp = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("reqwest error: {:?}", e))?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| format!("read text err: {:?}", e))?;
    if !status.is_success() {
        return Err(format!("token refresh failed: {} => {}", status, text));
    }

    let tok: TokenResponse = serde_json::from_str(&text).map_err(|e| format!("json decode: {:?}", e))?;
    tok.access_token.ok_or_else(|| "no access token in response".to_string())
}