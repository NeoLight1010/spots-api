pub fn encrypt_password(password: &str) -> String {
    let salt = b"random_salt";

    let config = argon2::Config::default();

    let encrypted =
        argon2::hash_encoded(password.as_bytes(), salt, &config).expect("Error hashing password.");

    encrypted
}
