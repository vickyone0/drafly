use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
mod config;
mod db;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    db::init().await.expect("DB init failed");

    HttpServer::new(|| {
        App::new()
            .configure(routes::auth::init)   
            .configure(routes::gmail::init)
            .configure(routes::drafts::init)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
