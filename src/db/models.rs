use super::schema::spots;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Spot {
    pub id: i32,
    pub title: String,
    pub description: String,

    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Insertable)]
#[table_name = "spots"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub description: &'a str,

    pub latitude: f64,
    pub longitude: f64,
}
