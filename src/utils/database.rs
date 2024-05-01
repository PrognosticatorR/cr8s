use diesel::{Connection, PgConnection};
use rocket::{figment::value::Value, http::Status, response::status::Custom, serde::json::json};
pub fn establish_connection() -> Result<PgConnection, Custom<Value>> {
    let database_url = "postgres://postgres:postgres@postgres/app_db";
    PgConnection::establish(&database_url)
        .map(|conn| Ok(conn))
        .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
        .unwrap()
}
