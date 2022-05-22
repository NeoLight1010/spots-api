use crate::DBPool;

#[get("/")]
pub async fn hello_world(db: DBPool) -> &'static str {
    "Hello World!"
}

#[get("/add")]
pub async fn add_post(db: DBPool) -> &'static str {
    "Added a new post!"
}
