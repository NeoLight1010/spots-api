use crate::db::connection::DBConn;
use crate::db::models::group::NewGroup;
use crate::db::models::user::User;
use crate::db::{models::group::Group, schema::groups};
use diesel::prelude::*;
use rocket::{serde::json::Json, Route};
use serde::Deserialize;

pub fn routes() -> Vec<Route> {
    routes![list_groups, create_group]
}

#[get("/")]
pub fn list_groups(conn: DBConn) -> Json<Vec<Group>> {
    let all_groups: Vec<Group> = groups::table
        .get_results(&*conn)
        .expect("Error loading groups.");

    Json(all_groups)
}

#[derive(Deserialize)]
pub struct CreateGroupData {
    name: String,
    description: String,
}

#[post("/", format = "json", data = "<group_data_json>")]
pub fn create_group(conn: DBConn, user: User, group_data_json: Json<CreateGroupData>) -> Json<Group> {
    let group_data = group_data_json.into_inner();

    let new_group = NewGroup {
        owner_id: user.id,
        name: group_data.name,
        description: group_data.description,
    };

    let group: Group = diesel::insert_into(groups::table)
        .values(new_group)
        .get_result(&*conn)
        .expect("Error inserting new group.");

    Json(group)
}
