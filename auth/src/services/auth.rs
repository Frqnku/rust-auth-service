use crate::helpers::errors::AuthError;
use crate::repositories::users::{add_record, get_user_by_email, show_db, User};

pub fn register_user(email: String, password: String) -> Result<User, AuthError> {
    if let Some(_existing_user) = get_user_by_email(email.as_str()) {
        return Err(AuthError::UserAlreadyExists);
    }
    let user = User {
        id: show_db().len() as i32 + 1,
        email,
        password,
    };
    if add_record(user.clone()).is_err() {
        return Err(AuthError::DatabaseError);
    }
    Ok(user)
}

pub fn login_user(email: String, password: String) -> Result<User, AuthError> {
    let user = get_user_by_email(email.as_str());
    if user.is_none() || user.as_ref().unwrap().password != password {
        return Err(AuthError::UserNotFound);
    }
    Ok(user.unwrap())
}
