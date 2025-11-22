use std::env;

pub fn google_client_id() -> String {
    env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID missing")
}

pub fn google_client_secret() -> String {
    env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET missing")
}

pub fn google_redirect_uri() -> String {
    env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI missing")
}

pub fn jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET missing")
}
