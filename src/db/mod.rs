use sqlx::{Pool, Postgres};
use once_cell::sync::OnceCell;

pub mod user_tokens;

static DB: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn init() -> Result<(), sqlx::Error> {
    let url = std::env::var("DATABASE_URL").unwrap();
    let pool = Pool::<Postgres>::connect(&url).await?;
    DB.set(pool).map_err(|_| sqlx::Error::Configuration("DB pool already initialized".into()))?;
    Ok(())
}

pub fn get_pool() -> &'static Pool<Postgres> {
    DB.get().expect("DB pool not initialized")
}
