use crate::{
    auth::login::{validate_login, LoginData, LoginError},
    db::{
        connection::DBConn,
        models::user::{NewUserEncrypted, NewUserNotEncrypted, User},
        schema::users::dsl::*,
    },
};
use diesel::result::Error;
use diesel::{prelude::*, result::DatabaseErrorKind::UniqueViolation};
use rocket::{http::{CookieJar, Cookie}, serde::json::Json};

#[get("/")]
pub fn get_users(conn: DBConn) -> Result<Json<Vec<User>>, String> {
    let all_users_result: Result<Vec<User>, _> = users.get_results(&*conn);

    match all_users_result {
        Ok(all_users) => Ok(Json(all_users)),
        Err(error) => Err(error.to_string()),
    }
}

#[post("/register", format = "json", data = "<new_user_not_encrypted_json>")]
pub fn register_user(
    conn: DBConn,
    new_user_not_encrypted_json: Json<NewUserNotEncrypted>,
) -> Result<Json<User>, String> {
    let new_user_not_encrypted = new_user_not_encrypted_json.into_inner();

    let new_user_encrypted = NewUserEncrypted::new(&new_user_not_encrypted);

    let new_user_result: Result<User, _> = diesel::insert_into(users)
        .values(&new_user_encrypted)
        .get_result(&*conn);

    match new_user_result {
        Ok(new_user) => Ok(Json(new_user)),
        Err(error) => match error {
            Error::DatabaseError(db_error, _) => match db_error {
                UniqueViolation => Err("Username already exists.".into()),
                _ => Err(error.to_string()),
            },

            _ => Err(error.to_string()),
        },
    }
}

#[post("/login", format = "json", data = "<login_data_json>")]
pub fn login(
    conn: DBConn,
    jar: &CookieJar<'_>,
    login_data_json: Json<LoginData>,
) -> Result<Json<User>, String> {
    let login_data = login_data_json.into_inner();

    let login_result = validate_login(&login_data, conn);

    if let Err(error) = login_result {
        return match error {
            LoginError::UsernameDoesNotExist => Err("Username does not exist.".into()),
            LoginError::WrongPassword => Err("Incorrect password.".into()),
            LoginError::DBError(db_error) => Err(db_error.to_string()),
        };
    }

    let user = login_result.ok().unwrap();

    let user_id_cookie = Cookie::new("user_id", user.id.to_string());

    jar.add(user_id_cookie);

    Ok(Json(user))
}
