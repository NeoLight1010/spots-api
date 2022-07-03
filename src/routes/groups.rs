use crate::db::connection::DBConn;
use crate::db::{models::group::Group, schema::groups};
use diesel::prelude::*;
use rocket::{serde::json::Json, Route};

pub fn routes() -> Vec<Route> {
    routes![list_groups]
}

#[get("/")]
pub fn list_groups(conn: DBConn) -> Json<Vec<Group>> {
    let all_groups: Vec<Group> = groups::table
        .get_results(&*conn)
        .expect("Error loading groups.");

    Json(all_groups)
}
