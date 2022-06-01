use std::ops::Deref;

use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use rocket::{request::{FromRequest, Outcome}, Request, http::Status};

use super::setup::DBPool;

pub struct DBConn(PooledConnection<ConnectionManager<PgConnection>>);

#[derive(Debug)]
pub struct DBError {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DBConn {
    type Error = DBError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<DBConn, Self::Error> {
         let pool = request.rocket().state::<DBPool>().unwrap();

         match pool.get() {
             Ok(conn) => Outcome::Success(DBConn(conn)),
             Err(_) => Outcome::Failure((Status::ServiceUnavailable, DBError {},))
         }
    }
}

impl Deref for DBConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
