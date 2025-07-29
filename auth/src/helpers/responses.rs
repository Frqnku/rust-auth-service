use crate::helpers::errors::AuthError;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum AuthResponse {
    RegisterSuccess(String),
    LoginSuccess(String),
    AuthError(AuthError),
}

impl IntoResponse for AuthResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthResponse::RegisterSuccess(jwt) => {
                (StatusCode::CREATED, Json(json!({ "token": jwt }))).into_response()
            }
            AuthResponse::LoginSuccess(jwt) => {
                (StatusCode::OK, Json(json!({ "token": jwt }))).into_response()
            }
            AuthResponse::AuthError(auth_error) => (
                auth_error.http_status(),
                Json(json!({ "error": auth_error.message() })),
            )
                .into_response(),
        }
    }
}
