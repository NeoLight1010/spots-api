use crate::db::{models::{Spot, NewSpot}, schema::spots::dsl::*, connection::DBConn};

use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/")]
pub fn index(conn: DBConn) -> Json<Vec<Spot>> {
    let all_spots: Vec<Spot> = 
        spots.get_results(&*conn).expect("Error loading spots.");

    Json(all_spots)
}

#[post("/", format = "json", data = "<data>")]
pub async fn create_spot(conn: DBConn, data: Json<NewSpot>) -> Json<Spot> {
    let new_spot = data.into_inner();

    let inserted: Spot = 
        diesel::insert_into(spots)
            .values(&new_spot)
            .get_result(&*conn)
            .expect("Error inserting new spot.");

    Json(inserted)
}
