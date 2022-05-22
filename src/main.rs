use std::env;

use db::{
    models::{NewPost, Post},
    schema::{posts::dsl::*, self},
};
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

#[get("/add")]
async fn add_post(db: DBPool) -> &'static str {
    let _: Post = db
        .run(|conn| {
            let new_post = NewPost {
                title: "My post",
                body: "This is some text.",
            };

            diesel::insert_into(schema::posts::table)
                .values(&new_post)
                .get_result(conn)
                .expect("Error saving new post.")
        })
        .await;

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
