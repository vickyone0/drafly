use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use dotenv::dotenv;
mod config;
mod db;
mod models;
mod routes;
mod services;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    db::init().await.expect("DB init failed");

    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    println!("CORS ALLOWED ORIGIN = {}", frontend_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_origin("https://courageous-pastelito-af3c6f.netlify.app")
            .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .configure(routes::auth::init)
            .configure(routes::gmail::init)
            .configure(routes::drafts::init)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
