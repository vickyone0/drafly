use crate::db::get_pool;

pub async fn insert_token(email: &str, token: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user_tokens (email, refresh_token)
        VALUES ($1, $2)
        ON CONFLICT (email)
        DO UPDATE SET refresh_token = EXCLUDED.refresh_token
        "#,
        email,
        token
    )
    .execute(get_pool())
    .await?;

    Ok(())
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
