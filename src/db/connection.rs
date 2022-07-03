use std::ops::Deref;

use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, Connection,
};
use dotenv::dotenv;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use super::setup::{DBPool, get_database_url};

pub struct DBConn(PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DBConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<DBConn, Self::Error> {
        let pool = request.rocket().state::<DBPool>().unwrap();

        match pool.get() {
            Ok(conn) => Outcome::Success(DBConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DBConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Creates a db connection and runs the given closure inside a transaction.
#[allow(dead_code)]
pub fn run_test_transaction<F, R>(func: F) -> R
where F: FnOnce(PgConnection) -> R {
    dotenv().ok();

    let conn = PgConnection::establish(&get_database_url()) .expect("Error loading DB.");

    conn.begin_test_transaction().unwrap();

    func(conn)
}
