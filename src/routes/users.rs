use crate::db::{connection::DBConn, models::user::User, schema::users::dsl::*};
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/")]
pub fn get_users(conn: DBConn) -> Json<Vec<User>> {
    let all_users: Vec<User> = users.get_results(&*conn).expect("Error loading users");

    Json(all_users)
}
