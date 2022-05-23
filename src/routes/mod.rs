#[get("/")]
pub async fn index() -> &'static str {
    "Spots: collaborative spots for awesome groups."
}
