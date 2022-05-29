use crate::db::models::user::User;

use super::utils::validate_password;

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

#[cfg(test)]
mod test {
    use crate::{auth::utils::encrypt_password, db::models::user::User};

    use super::{validate_login, LoginError};

    #[test]
    fn validate_login_username_does_not_exist() {
        let result = validate_login(None, "");
        let error = result.err().unwrap();

        match error {
            LoginError::UsernameDoesNotExist => (),
            _ => panic!("Incorrect error."),
        };
    }

    #[test]
    fn validate_login_incorrect_password() {
        let correct_password = "my-test-password";
        let incorrect_password = "this-password-is-not-correct";

        let user = User {
            id: 1,
            username: "test".into(),
            password: correct_password.into(),
        };

        let result = validate_login(Some(user), incorrect_password);
        let error = result.err().unwrap();

        match error {
            LoginError::WrongPassword => (),
            _ => panic!("Incorrect error."),
        };
    }

    #[test]
    fn validate_login_successful() {
        let correct_password_raw = "my-test-password";
        let correct_password_encrypted = encrypt_password(correct_password_raw);

        let user = User {
            id: 1,
            username: "test".into(),
            password: correct_password_encrypted,
        };

        let result = validate_login(Some(user), correct_password_raw);

        result.ok().unwrap();
    }
}
