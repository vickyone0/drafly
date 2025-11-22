use crate::db::get_pool;

pub async fn insert_token(email: &str, token: &str) {
    sqlx::query!(
        "INSERT INTO user_tokens (email, refresh_token)
         VALUES ($1, $2)",
        email,
        token
    )
    .execute(get_pool())
    .await
    .unwrap();
}

pub async fn get_refresh_token(email: &str) -> Option<String> {
    let row = sqlx::query!(
        "SELECT refresh_token FROM user_tokens
         WHERE email = $1 LIMIT 1",
        email
    )
    .fetch_optional(get_pool())
    .await
    .unwrap();

    row.map(|r| r.refresh_token)
}
