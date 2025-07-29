use chrono::{Duration, Utc};
use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use shared::jwt::get_jwt_secret;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt_token(user_id: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let secret = get_jwt_secret();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
