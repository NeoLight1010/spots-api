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
