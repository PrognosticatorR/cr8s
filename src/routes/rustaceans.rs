use crate::{
    models::rustaceans::{FormRustacean, NewRustacean},
    routes::DbConn,
};

use super::super::repositories::rustacean_repository::RustacenRepository;
extern crate diesel;
use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};
#[get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|conn| {
        RustacenRepository::find(conn, 100)
            .map(|data| json!(data))
            .map_err(|_| Custom(Status::InternalServerError, json!("Something went wrong!")))
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|conn| {
        RustacenRepository::create(conn, new_rustacean.into_inner())
            .map(|res| Custom(Status::Created, json!(res)))
            .map_err(|_| Custom(Status::InternalServerError, json!("something went wrong!")))
    })
    .await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustacean(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        RustacenRepository::find_one(conn, id)
            .map(|res| json!(res))
            .map_err(|_| Custom(Status::NotFound, json!("not found!")))
    })
    .await
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
    db.run(move |conn| {
        RustacenRepository::delete(conn, id)
            .map(|_| NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<update_rustacean>")]
pub async fn update_rustacean(
    id: i32,
    update_rustacean: Json<FormRustacean>,
    db: DbConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        RustacenRepository::update(conn, update_rustacean.into_inner(), id)
            .map(|res| json!(res))
            .map_err(|_| Custom(Status::InternalServerError, json!("something went wrong!")))
    })
    .await
}
