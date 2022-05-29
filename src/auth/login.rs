use crate::db::models::user::User;

pub enum LoginError {
    UsernameDoesNotExist,
    WrongPassword,
}

pub fn validate_login(optional_user: Option<User>, raw_password: &str) -> Result<User, LoginError> {
    if optional_user.is_none() {
        return Err(LoginError::UsernameDoesNotExist);
    }

    let user = optional_user.unwrap();

    if !validate_password(raw_password, user.password.as_str()) {
        return Err(LoginError::WrongPassword);
    }

    Ok(user)
}

fn validate_password(raw_password: &str, hashed_password: &str) -> bool {
    argon2::verify_encoded(hashed_password, raw_password.as_bytes()).unwrap()
}
