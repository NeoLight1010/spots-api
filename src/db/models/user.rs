use crate::db::schema::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
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
