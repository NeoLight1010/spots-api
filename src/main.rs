use std::env;

use db::{models::Post, schema::posts::dsl::*};
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
    let results = db
        .run(|conn| {
            posts
                .filter(published.eq(true))
                .limit(5)
                .load::<Post>(conn)
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

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let db: Map<_, Value> = map! {
        "url"=> db_url.into(),
    };

    let figment = rocket::Config::figment().merge(("databases", map!["spots" => db]));

    rocket::custom(figment)
        .mount("/", routes![hello_world])
        .attach(DBPool::fairing())
}
