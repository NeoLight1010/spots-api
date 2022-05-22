use std::env;

use diesel::PgConnection;
use rocket::figment::{value::{Value, Map}, util::map};
use rocket_sync_db_pools::database;

#[database("spots")]
pub struct DBPool(PgConnection);

pub fn get_db_config() -> Map<&'static str, Value> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let db: Map<_, Value> = map! {
        "url"=> db_url.into(),
    };

    return db;
}
