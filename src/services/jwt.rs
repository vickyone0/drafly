use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use base64::{engine::general_purpose, Engine as _};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
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
    if parts.len() < 2 {
        panic!("Invalid JWT token format");
    }
    let payload = parts[1];

    // Fix missing padding
    let rem = payload.len() % 4;
    let padded = if rem > 0 {
        format!("{}{}", payload, "=".repeat(4 - rem))
    } else {
        payload.to_string()
    };

    // Try URL_SAFE_NO_PAD first (for our own tokens), then fall back to STANDARD (for Google's tokens)
    use base64::engine::general_purpose::STANDARD;
    let decoded_bytes = URL_SAFE_NO_PAD.decode(&padded)
        .or_else(|_| {
            // Try with standard base64 decoding (Google uses standard base64)
            STANDARD.decode(&padded)
        })
        .unwrap_or_else(|e| {
            log::error!("Failed to decode JWT payload: {:?}, payload: {}", e, payload);
            panic!("Failed to decode JWT payload: {:?}", e);
        });
    
    serde_json::from_slice(&decoded_bytes).unwrap_or_else(|e| {
        log::error!("Failed to parse JWT payload as JSON: {:?}", e);
        panic!("Failed to parse JWT payload as JSON: {:?}", e);
    })
}

/// Validates and decodes a JWT token, returning the claims if valid
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| jsonwebtoken::errors::ErrorKind::InvalidToken)?;
    
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();
    
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
