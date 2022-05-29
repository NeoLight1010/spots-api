use crate::{db::schema::users, auth::utils::encrypt_password};

use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,

    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewUserNotEncrypted {
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUserEncrypted {
    pub username: String,
    pub password: String,
}

impl NewUserEncrypted {
    pub fn new(new_user_not_encrypted: &NewUserNotEncrypted) -> NewUserEncrypted {
        let hashed_password = 
            encrypt_password(new_user_not_encrypted.password.as_str());

        NewUserEncrypted {
            username: new_user_not_encrypted.username.clone(),
            password: hashed_password,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{NewUserEncrypted, NewUserNotEncrypted};

    #[test]
    fn new_user_encrypted_should_encrypt_password() {
        let non_encrypted_password = "my_password";

        let new_user_not_encrypted = NewUserNotEncrypted {
            username: "test".into(),
            password: non_encrypted_password.into(),
        };

        let new_user_encrypted = NewUserEncrypted::new(&new_user_not_encrypted);

        let matches = argon2::verify_encoded(
            &new_user_encrypted.password,
            non_encrypted_password.as_bytes(),
        )
        .unwrap();

        assert!(matches);
    }
}
