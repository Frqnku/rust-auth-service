use regex::Regex;
use serde::Deserialize;
use validator::ValidationError;
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
