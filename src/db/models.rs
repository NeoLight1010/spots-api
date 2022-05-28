use super::schema::spots;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Spot {
    pub id: i32,
    pub title: String,
    pub description: String,

    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "spots"]
pub struct NewSpot {
    pub title: String,
    pub description: String,

    pub latitude: f64,
    pub longitude: f64,
}
