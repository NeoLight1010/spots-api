use std::env;

use db::{models::Post, schema::posts::dsl::*, setup::establish_connection};
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

#[get("/")]
async fn hello_world(conn: DBPool) -> &'static str {
    let results = conn
        .run(|c| {
            posts
                .filter(published.eq(true))
                .limit(5)
                .load::<Post>(c)
                .expect("Error loading posts.")
        })
        .await;

    println!("Displaying {} posts:", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------");
        println!("{}", post.body);
    }

    "Hello World!"
}

#[database("spots")]
pub struct DBPool(PgConnection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let db: Map<_, Value> = map! {
        "url"=> db_url.into(),
        "pool_size" => 10.into(),
    };

    let figment = rocket::Config::figment().merge(("databases", map!["spots" => db]));

    rocket::custom(figment)
        .mount("/", routes![hello_world])
        .attach(DBPool::fairing())
}
