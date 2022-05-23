use crate::DBPool;

#[get("/")]
pub async fn hello_world(_db: DBPool) -> &'static str {
    "Hello World!"
}
