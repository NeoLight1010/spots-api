pub mod groups;
pub mod spot;
pub mod users;

#[get("/")]
pub async fn index() -> &'static str {
    "Spots: collaborative spots for awesome groups."
}
