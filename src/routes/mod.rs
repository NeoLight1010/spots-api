use rocket::Route;

pub mod groups;
pub mod spot;
pub mod users;

pub fn routes() -> Vec<Route> {
    routes![index]
}

#[get("/")]
pub async fn index() -> &'static str {
    "Spots: collaborative spots for awesome groups."
}
