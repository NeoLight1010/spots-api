use crate::db::{
    connection::DBConn,
    models::user::{NewUserEncrypted, NewUserNotEncrypted, User},
    schema::users::dsl::*,
};
use diesel::result::Error;
use diesel::{prelude::*, result::DatabaseErrorKind::UniqueViolation};
use rocket::serde::json::Json;

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
