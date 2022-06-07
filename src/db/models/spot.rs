use crate::db::{schema::spots, models::group::Group};
use diesel::{Queryable, Insertable, Identifiable, Associations};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Group)]
pub struct Spot {
    pub id: i32,
    pub title: String,
    pub description: String,

    pub latitude: f64,
    pub longitude: f64,

    pub group_id: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "spots"]
pub struct NewSpot {
    pub title: String,
    pub description: String,

    pub latitude: f64,
    pub longitude: f64,
}
