use db::{models::Post, schema::posts::dsl::*, setup::establish_connection};
use diesel::prelude::*;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

pub mod db;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello World!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let connection = establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts:", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------");
        println!("{}", post.body);
    }

    rocket::build().mount("/", routes![hello_world])
}
