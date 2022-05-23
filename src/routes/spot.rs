use crate::db::{models::Spot, schema::spots::dsl::*, setup::DBPool};

use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/")]
pub async fn index(db: DBPool) -> Json<Vec<Spot>> {
    let all_spots: Vec<Spot> = db
        .run(|conn| {
            spots
                .get_results(conn)
                .expect("Error loading spots.")
        })
        .await;

    Json(all_spots)
}
