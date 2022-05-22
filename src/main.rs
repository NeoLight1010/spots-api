use db::setup::establish_connection;

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
    let db_conn = establish_connection();

    rocket::build().mount("/", routes![hello_world])
}
