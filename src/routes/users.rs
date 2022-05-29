use crate::db::{
    connection::DBConn,
    models::user::{NewUserNotEncrypted, User, NewUserEncrypted},
    schema::users::dsl::*,
};
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/")]
pub fn get_users(conn: DBConn) -> Json<Vec<User>> {
    let all_users: Vec<User> = users.get_results(&*conn).expect("Error loading users");

    Json(all_users)
}

#[post("/register", format = "json", data = "<new_user_not_encrypted_json>")]
pub fn register_user(
    conn: DBConn,
    new_user_not_encrypted_json: Json<NewUserNotEncrypted>,
) -> Json<User> {
    let new_user_not_encrypted = new_user_not_encrypted_json.into_inner();

    let new_user_encrypted = NewUserEncrypted::new(&new_user_not_encrypted);

    let new_user = diesel::insert_into(users)
        .values(&new_user_encrypted)
        .get_result(&*conn)
        .expect("Error inserting new user.");

    Json(new_user)
}
