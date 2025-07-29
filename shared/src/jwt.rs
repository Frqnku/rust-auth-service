use dotenvy::dotenv;
use jsonwebtoken::{decode, errors::Error, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn get_jwt_secret() -> String {
    dotenv().ok();
    env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env")
}

pub fn verify_jwt(token: &str) -> Result<Claims, Error> {
    let secret = get_jwt_secret();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
