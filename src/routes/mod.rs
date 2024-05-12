use crate::models::users::User;
use crate::repositories::user_repository::UserRepository;
use diesel::PgConnection;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::serde::json::{json, serde_json::Value};
use rocket::Request;
use rocket::{http::Status, response::status::Custom};
use rocket_db_pools::Connection;
use rocket_db_pools::{deadpool_redis, deadpool_redis::redis::AsyncCommands, Database};
use rocket_sync_db_pools::database;

pub mod auth;
pub mod crates;
pub mod rustaceans;
#[database("postgres")]
pub struct DbConn(PgConnection);

pub fn serve_error(err: &Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", err);
    Custom(Status::InternalServerError, json!("something went wrong!"))
}

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session_header = request
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|vec| vec.len() == 2 && vec[0] == "Bearer");
        println!("session, {:?}", session_header);
        if let Some(header_value) = session_header {
            let mut cache = request
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Can't commect to redis in request Guard");
            let db = request
                .guard::<DbConn>()
                .await
                .expect("Can't commect to database in request Guard");
            let result = cache
                .get::<_, i32>(format!("sessions/{}", header_value[1]))
                .await;
            if let Ok(user_id) = result {
                return match db
                    .run(move |conn| UserRepository::find_one(conn, user_id))
                    .await
                {
                    Ok(user) => Outcome::Success(user),
                    _ => Outcome::Error((Status::Unauthorized, ())),
                };
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
