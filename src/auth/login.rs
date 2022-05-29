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
    argon2::verify_encoded(hashed_password, raw_password.as_bytes()).unwrap_or(false)
}

#[cfg(test)]
mod test {
    use crate::db::models::user::User;

    use super::{validate_login, LoginError};

    #[test]
    fn validate_login_username_does_not_exist() {
        let result = validate_login(None, "");
        let error = result.err().unwrap();

        match error {
            LoginError::UsernameDoesNotExist => (),
            _ => panic!("Incorrect error.")
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
            _ => panic!("Incorrect error.")
        };
    }

    #[test]
    fn validate_login_successful() {
        let correct_password = "my-test-password";

        let user = User {
            id: 1,
            username: "test".into(),
            password: correct_password.into(),
        };

        let result = validate_login(Some(user), correct_password);

        result.ok().unwrap();
    }
}
