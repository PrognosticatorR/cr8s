use diesel::PgConnection;
use rocket::serde::json::{json, serde_json::Value};
use rocket::{http::Status, response::status::Custom};
use rocket_sync_db_pools::database;

pub mod crates;
pub mod rustaceans;
#[database("postgres")]
pub struct DbConn(PgConnection);

pub fn serve_error(err: &Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", err);
    Custom(Status::InternalServerError, json!("something went wrong!"))
}
