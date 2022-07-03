use db::setup::init_db_pool;
use dotenv::dotenv;
use routes::{groups, index, spot, users};

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
        .mount("/", routes![index])
        .mount("/spot", routes![spot::index, spot::create_spot])
        .mount(
            "/users",
            routes![
                users::get_users,
                users::register_user,
                users::login,
                users::logout,
                users::me,
            ],
        )
        .mount("/groups", routes![groups::list_groups])
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
