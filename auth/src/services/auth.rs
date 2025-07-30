use super::hash::Hasher;
use crate::helpers::errors::AuthError;
use crate::repositories::users::{add_user, get_user_by_email, NewUser, User};
use sqlx::PgPool;

pub async fn register_user(
    email: String,
    password: String,
    db: &PgPool,
) -> Result<User, AuthError> {
    if let Some(_existing_user) = get_user_by_email(email.as_str(), db).await {
        return Err(AuthError::UserAlreadyExists);
    }

    let hasher = Hasher::new();
    let hashed_password = hasher
        .hash(&password)
        .map_err(|_| AuthError::HashingError)?;

    let user = NewUser {
        email,
        password: hashed_password,
    };
    match add_user(user, db).await {
        Ok(u) => Ok(u),
        Err(_) => Err(AuthError::DatabaseError),
    }
}

pub async fn login_user(email: String, password: String, db: &PgPool) -> Result<User, AuthError> {
    let user = get_user_by_email(email.as_str(), db).await;
    if user.is_none() {
        return Err(AuthError::UserNotFound);
    }
    let user = user.unwrap();

    let hasher = Hasher::new();
    if !hasher
        .verify(&password, user.password.as_str())
        .map_err(|_| AuthError::HashingError)?
    {
        return Err(AuthError::UserNotFound);
    }

    Ok(user)
}
