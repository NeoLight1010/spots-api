use db::setup::{get_db_config, DBPool};
use dotenv::dotenv;
use rocket::figment::util::map;
use routes::{index, spot};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod db;
mod routes;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let db_config = get_db_config();

    let figment = rocket::Config::figment().merge(("databases", map!["spots" => db_config]));

    rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/spot", routes![spot::index])
        .attach(DBPool::fairing())
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
