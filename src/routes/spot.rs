#[get("/")]
pub async fn index() -> &'static str {
    "Spots list."
}
