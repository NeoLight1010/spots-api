use crate::db::{models::{Spot, NewSpot}, schema::spots::dsl::*, setup::DBPool};

use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/")]
pub async fn index(db: DBPool) -> Json<Vec<Spot>> {
    let all_spots: Vec<Spot> = db
        .run(|conn| spots.get_results(conn).expect("Error loading spots."))
        .await;

    Json(all_spots)
}

#[post("/", format = "json", data = "<data>")]
pub async fn create_spot(db: DBPool, data: Json<NewSpot>) -> Json<Spot> {
    let inserted: Spot = db.run(|conn| {
        let new_spot = data.into_inner();

        diesel::insert_into(spots)
            .values(&new_spot)
            .get_result(conn)
            .expect("Error inserting new spot.")
    }).await;

    Json(inserted)
}
