use super::schema::{spots, users};
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

#[derive(Queryable)]
pub struct User {
    pub id: i32,

    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewUserNotEncrypted {
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUserEncrypted {
    pub username: String,
    pub password: String,
}
