use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use validator::ValidationErrors;

pub enum AuthError {
    InvalidEmail,
    UserNotFound,
    PasswordNotStrongEnough,
    UserAlreadyExists,
    DatabaseError,
}

impl AuthError {
    pub fn code(&self) -> &str {
        match self {
            AuthError::InvalidEmail => "invalid_email",
            AuthError::UserNotFound => "user_not_found",
            AuthError::PasswordNotStrongEnough => "password_not_strong_enough",
            AuthError::UserAlreadyExists => "user_already_exists",
            AuthError::DatabaseError => "database_error",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            AuthError::InvalidEmail => "L'email fourni n'est pas valide.",
            AuthError::UserNotFound => "L'email ou le mot de passe est incorrect.",
            AuthError::PasswordNotStrongEnough => {
                "Le mot de passe doit contenir au moins 8 caractères, une lettre et un chiffre."
            }
            AuthError::UserAlreadyExists => "Un utilisateur avec cet email existe déjà.",
            AuthError::DatabaseError => {
                "Une erreur est survenue lors de l'accès à la base de données."
            }
        }
    }

    pub fn http_status(&self) -> StatusCode {
        match self {
            AuthError::InvalidEmail | AuthError::PasswordNotStrongEnough => StatusCode::BAD_REQUEST,
            AuthError::UserAlreadyExists => StatusCode::CONFLICT,
            AuthError::UserNotFound => StatusCode::UNAUTHORIZED,
            AuthError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, json) = self.to_error_response();
        (status, json).into_response()
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub field: String,
    pub code: String,
    pub message: String,
}

pub trait ToErrorResponse {
    fn to_error_response(&self) -> (StatusCode, Json<ErrorResponse>);
}

impl ToErrorResponse for AuthError {
    fn to_error_response(&self) -> (StatusCode, Json<ErrorResponse>) {
        let field = match self {
            AuthError::InvalidEmail => "email",
            AuthError::PasswordNotStrongEnough => "password",
            _ => "unknown",
        };

        let resp = ErrorResponse {
            field: field.into(),
            code: self.code().into(),
            message: self.message().into(),
        };

        (self.http_status(), Json(resp))
    }
}

pub fn map_validation_errors(errors: ValidationErrors) -> Option<AuthError> {
    errors
        .field_errors()
        .get("email")
        .map(|_| AuthError::InvalidEmail)
        .or_else(|| {
            errors
                .field_errors()
                .get("password")
                .map(|_| AuthError::PasswordNotStrongEnough)
        })
}
