use crate::controllers::helpers::{AuthError, AuthResponse};
use axum::{response::IntoResponse, Json};
use validator::Validate;

use super::helpers::{map_validation_errors, AuthRequest};

pub async fn login(Json(payload): Json<AuthRequest>) -> impl IntoResponse {
    if let Err(error) = payload.validate() {
        if let Some(auth_error) = map_validation_errors(error) {
            if let AuthError::InvalidEmail = auth_error {
                return AuthResponse::AuthError(auth_error).into_response();
            }
        }
    }
    AuthResponse::LoginSuccess.into_response()
}

pub async fn register(Json(payload): Json<AuthRequest>) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        if let Some(auth_error) = map_validation_errors(errors) {
            return AuthResponse::AuthError(auth_error).into_response();
        }
    }
    AuthResponse::RegisterSuccess.into_response()
}
