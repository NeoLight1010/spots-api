pub fn encrypt_password(password: &str) -> String {
    let salt = b"random_salt";

    let config = argon2::Config::default();

    let encrypted =
        argon2::hash_encoded(password.as_bytes(), salt, &config).expect("Error hashing password.");

    encrypted
}

pub fn validate_password(raw_password: &str, hashed_password: &str) -> bool {
    argon2::verify_encoded(hashed_password, raw_password.as_bytes()).unwrap_or(false)
}

#[cfg(test)]
mod test {
    use super::{encrypt_password, validate_password};

    #[test]
    fn encrypt_should_not_return_same_password() {
        let raw_password = "my_raw_password";
        let encrypted_password = encrypt_password(raw_password);

        assert_ne!(raw_password, encrypted_password);
    }

    #[test]
    fn validate_correct_password() {
        let raw_password = "my_raw_password";
        let encrypted_password = encrypt_password(raw_password);

        let matches = validate_password(raw_password, &encrypted_password);

        assert!(matches);
    }

    #[test]
    fn validate_incorrect_password() {
        let encrypted_correct_password = encrypt_password("my_correct_password");

        let raw_incorrect_password = "my_incorrect_password";

        let matches = validate_password(raw_incorrect_password, &encrypted_correct_password);

        assert!(!matches);
    }
}
