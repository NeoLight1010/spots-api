use diesel::QueryDsl;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
    outcome::try_outcome,
};

use crate::db::{
    connection::DBConn,
    models::user::User, schema::users
};

use diesel::prelude::*;

pub struct AuthenticatedUser(User);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = match request.cookies().get("user_id") {
            Some(cookie) => cookie,
            None => {
                return Outcome::Failure((Status::Unauthorized, ()))
            }
        };

        let user_id_str = cookie.value();

        let user_id = match user_id_str.parse::<i32>() {
            Ok(user_id) => user_id,
            Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
        };

        let db: DBConn = try_outcome!(request.guard::<DBConn>().await);

        let user: Result<User, _> = users::table.find(user_id).first(&*db);

        match user {
            Ok(user) => Outcome::Success(AuthenticatedUser(user)),
            _ => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
