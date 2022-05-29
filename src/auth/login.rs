use diesel::prelude::*;
use diesel::result::Error;
use rocket::serde::Deserialize;

use crate::db::{connection::DBConn, models::user::User, schema::users::dsl::*};

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

pub enum LoginError {
    UsernameDoesNotExist,
    WrongPassword,
    DBError(Error),
}

pub fn validate_login(login_data: &LoginData, conn: DBConn) -> Result<User, LoginError> {
    let user_result: Result<Option<User>, _> = users
        .filter(username.eq(&login_data.username))
        .limit(1)
        .first(&*conn)
        .optional();

    if let Err(error) = user_result {
        return Err(LoginError::DBError(error));
    }

    let option_user = user_result.ok().unwrap();

    if option_user.is_none() {
        return Err(LoginError::UsernameDoesNotExist);
    }

    let user = option_user.unwrap();

    if !validate_password(login_data.password.as_str(), user.password.as_str()) {
        return Err(LoginError::WrongPassword);
    }

    Ok(user)
}

fn validate_password(raw_password: &str, hashed_password: &str) -> bool {
    argon2::verify_encoded(hashed_password, raw_password.as_bytes()).unwrap()
}
