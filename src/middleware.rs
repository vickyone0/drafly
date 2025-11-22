use crate::services::jwt;

/// Extractor to get the authenticated user's email from the JWT token
#[derive(Clone)]
pub struct AuthenticatedUser {
    pub email: String,
}

impl actix_web::FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        // Extract token from Authorization header
        let auth_header = req.headers().get("Authorization");
        
        let token = match auth_header {
            Some(header_value) => {
                let header_str = header_value.to_str().unwrap_or("");
                if header_str.starts_with("Bearer ") {
                    Some(header_str[7..].to_string())
                } else {
                    None
                }
            }
            None => None,
        };

        let token = match token {
            Some(t) => t,
            None => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized("Missing Authorization header"))
                });
            }
        };

        // Validate token
        match jwt::validate_jwt(&token) {
            Ok(claims) => {
                let email = claims.sub;
                Box::pin(async move { Ok(AuthenticatedUser { email }) })
            }
            Err(_) => Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized("Invalid or expired token"))
            }),
        }
    }
}

