use super::super::repositories::rustacean_repository::RustacenRepository;
extern crate diesel;
use super::super::utils::database;
use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status::Custom,
    serde::json::{serde_json::json, Value},
};
#[get("/rustaceans")]
pub fn get_rustaceans() -> Result<Value, Custom<Value>> {
    let mut conn = database::establish_connection().unwrap();
    RustacenRepository::find(&mut conn, 100)
        .map(|data| json!(data))
        .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
}

#[post("/rustaceans")]
pub fn create_rustacean() {}

#[get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32) {}

#[delete("/rustacean/<id>")]
pub fn delete_rustacean(id: i32) {}

#[put("/rustaceans/<id>")]
pub fn update_rustacean(id: i32) {}

#[get("/")]
pub fn hello_world() -> Value {
    json!("Hello, world!")
}
