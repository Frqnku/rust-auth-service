use axum::{http::StatusCode, response::IntoResponse, Json};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{ValidationError, ValidationErrors};
use validator_derive::Validate;

fn contains_digit(password: &str) -> Result<(), ValidationError> {
    let digit_re = Regex::new(r"\d").unwrap();
    if digit_re.is_match(password) {
        Ok(())
    } else {
        Err(ValidationError::new("password_not_strong_enough"))
    }
}

fn contains_letter(password: &str) -> Result<(), ValidationError> {
    let letter_re = Regex::new(r"[A-Za-z]").unwrap();
    if letter_re.is_match(password) {
        Ok(())
    } else {
        Err(ValidationError::new("password_not_strong_enough"))
    }
}

#[derive(Deserialize, Validate)]
pub struct AuthRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 64))]
    #[validate(custom(function = "contains_digit"))]
    #[validate(custom(function = "contains_letter"))]
    pub password: String,
}

pub enum AuthError {
    InvalidEmail,
    PasswordNotStrongEnough,
    _UserAlreadyExists,
}

impl AuthError {
    fn code(&self) -> &str {
        match self {
            AuthError::InvalidEmail => "invalid_email",
            AuthError::PasswordNotStrongEnough => "password_not_strong_enough",
            AuthError::_UserAlreadyExists => "user_already_exists",
        }
    }

    fn message(&self) -> &str {
        match self {
            AuthError::InvalidEmail => "L'email fourni n'est pas valide.",
            AuthError::PasswordNotStrongEnough => {
                "Le mot de passe doit contenir au moins 8 caractères, une lettre et un chiffre."
            }
            AuthError::_UserAlreadyExists => "Un utilisateur avec cet email existe déjà.",
        }
    }

    fn http_status(&self) -> StatusCode {
        match self {
            AuthError::InvalidEmail | AuthError::PasswordNotStrongEnough => StatusCode::BAD_REQUEST,
            AuthError::_UserAlreadyExists => StatusCode::CONFLICT,
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

static REGISTER_SUCCESS: Lazy<&'static str> = Lazy::new(|| "Inscription réussie");
static LOGIN_SUCCESS: Lazy<&'static str> = Lazy::new(|| "Connexion réussie");

pub enum AuthResponse {
    RegisterSuccess,
    LoginSuccess,
    AuthError(AuthError),
}

impl IntoResponse for AuthResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthResponse::RegisterSuccess => {
                (StatusCode::CREATED, Json(*REGISTER_SUCCESS)).into_response()
            }
            AuthResponse::LoginSuccess => (StatusCode::OK, Json(*LOGIN_SUCCESS)).into_response(),
            AuthResponse::AuthError(auth_error) => {
                (auth_error.http_status(), Json(auth_error.message())).into_response()
            }
        }
    }
}
