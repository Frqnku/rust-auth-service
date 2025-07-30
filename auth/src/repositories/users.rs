use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

pub async fn add_user(user: NewUser, db: &PgPool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password) 
        VALUES ($1, $2)
        RETURNING id, email, password
        "#,
        user.email,
        user.password
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn get_user_by_email(email: &str, db: &PgPool) -> Option<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password FROM users WHERE email = $1",
        email
    )
    .fetch_optional(db)
    .await
    .ok()?;
    user
}
