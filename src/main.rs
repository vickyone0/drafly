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

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

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
