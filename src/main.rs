use std::env;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::figment::{
    util::map,
    value::{Map, Value},
};
use rocket_sync_db_pools::database;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

pub mod db;

#[database("spots")]
pub struct DBPool(PgConnection);

#[get("/")]
async fn hello_world(db: DBPool) -> &'static str {
    "Hello World!"
}

#[get("/add")]
async fn add_post(db: DBPool) -> &'static str {
    "Added a new post!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let db: Map<_, Value> = map! {
        "url"=> db_url.into(),
    };

    let figment = rocket::Config::figment().merge(("databases", map!["spots" => db]));

    rocket::custom(figment)
        .mount("/", routes![hello_world, add_post])
        .attach(DBPool::fairing())
}
