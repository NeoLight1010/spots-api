use crate::{
    auth::login::{validate_login, LoginError},
    db::{
        connection::DBConn,
        models::user::{NewUserEncrypted, NewUserNotEncrypted, User},
        schema::users::dsl::*,
    },
};
use diesel::result::Error;
use diesel::{prelude::*, result::DatabaseErrorKind::UniqueViolation};
use rocket::{http::{CookieJar, Cookie}, serde::json::Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

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

    let user_result: Result<Option<User>, _> = users
        .filter(username.eq(&login_data.username))
        .limit(1)
        .first(&*conn)
        .optional();

    if let Err(error) = user_result {
        return Err(error.to_string());
    }

    let optional_user = user_result.ok().unwrap();

    let login_result = validate_login(optional_user, &login_data.password);

    if let Err(error) = login_result {
        return match error {
            LoginError::UsernameDoesNotExist => Err("Username does not exist.".into()),
            LoginError::WrongPassword => Err("Incorrect password.".into()),
        };
    }

    let user = login_result.ok().unwrap();

    let user_id_cookie = Cookie::new("user_id", user.id.to_string());

    jar.add(user_id_cookie);

    Ok(Json(user))
}

#[get("/me")]
pub fn me(user: User) -> Json<User> {
    Json(user)
}

#[get("/logout")]
pub fn logout(jar: &CookieJar<'_>) {
    jar.remove(Cookie::named("user_id"));
}
