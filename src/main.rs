use db::setup::init_db_pool;
use dotenv::dotenv;
use routes::{groups, spot, users};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod auth;
mod db;
mod routes;

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .manage(init_db_pool())
        .mount("/", routes::routes())
        .mount("/spot", spot::routes())
        .mount("/users", users::routes())
        .mount("/groups", groups::routes())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;

    #[test]
    fn rocket_instance() {
        Client::tracked(rocket()).expect("Invalid rocket instance.");
    }
}
