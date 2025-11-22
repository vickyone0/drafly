use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(email: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes()),
    )
    .unwrap()
}

pub use create_jwt as generate_jwt;

pub fn decode_jwt_payload(token: &str) -> serde_json::Value {
    let parts: Vec<&str> = token.split('.').collect();
    let payload = parts[1];

    // Fix missing padding
    let rem = payload.len() % 4;
    let padded = if rem > 0 {
        format!("{}{}", payload, "=".repeat(4 - rem))
    } else {
        payload.to_string()
    };

    let decoded_bytes = URL_SAFE_NO_PAD.decode(payload).unwrap();
    serde_json::from_slice(&decoded_bytes).unwrap()
}
