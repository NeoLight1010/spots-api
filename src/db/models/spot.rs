use crate::db::schema::spots;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Spot {
    pub id: i32,
    pub title: String,
    pub description: String,

    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Insertable, Deserialize)]
#[table_name = "spots"]
pub struct NewSpot {
    pub title: String,
    pub description: String,

    pub latitude: f64,
    pub longitude: f64,
}