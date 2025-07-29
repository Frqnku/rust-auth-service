use crate::helpers::errors::{map_validation_errors, AuthError};
use crate::helpers::responses::AuthResponse;
use crate::helpers::validators::AuthRequest;
use crate::services::{auth, jwt};
use axum::{response::IntoResponse, Json};
use validator::Validate;

pub async fn login(Json(payload): Json<AuthRequest>) -> impl IntoResponse {
    if let Err(error) = payload.validate() {
        if let Some(auth_error) = map_validation_errors(error) {
            if let AuthError::InvalidEmail = auth_error {
                return AuthResponse::AuthError(auth_error).into_response();
            }
        }
    }

    let user = match auth::login_user(payload.email, payload.password) {
        Err(error) => return AuthResponse::AuthError(error).into_response(),
        Ok(u) => u,
    };

    let jwt = jwt::generate_jwt_token(&user.id.to_string()).map_err(|_| AuthError::UserNotFound);

    match jwt {
        Err(error) => AuthResponse::AuthError(error).into_response(),
        Ok(token) => AuthResponse::LoginSuccess(token).into_response(),
    }
}

pub async fn register(Json(payload): Json<AuthRequest>) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        if let Some(auth_error) = map_validation_errors(errors) {
            return AuthResponse::AuthError(auth_error).into_response();
        }
    }

    let user = match auth::register_user(payload.email, payload.password) {
        Err(error) => return AuthResponse::AuthError(error).into_response(),
        Ok(u) => u,
    };

    let jwt = jwt::generate_jwt_token(&user.id.to_string()).map_err(|_| AuthError::UserNotFound);

    match jwt {
        Err(error) => AuthResponse::AuthError(error).into_response(),
        Ok(token) => AuthResponse::RegisterSuccess(token).into_response(),
    }
}
