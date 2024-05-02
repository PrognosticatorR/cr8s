use crate::{
    models::crates::{FormCrate, NewCrate},
    routes::DbConn,
};

use super::super::repositories::crate_repository::CrateRepository;
extern crate diesel;
use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

#[get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|conn| {
        CrateRepository::find(conn, 100)
            .map(|data| json!(data))
            .map_err(|_| Custom(Status::InternalServerError, json!("Something went wrong!")))
    })
    .await
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(db: DbConn, new_crate: Json<NewCrate>) -> Result<Value, Custom<Value>> {
    db.run(|conn| {
        CrateRepository::create(conn, new_crate.into_inner())
            .map(|res| json!(res))
            .map_err(|_| Custom(Status::InternalServerError, json!("something went wrong!")))
    })
    .await
}

#[get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::find_one(conn, id)
            .map(|res| json!(res))
            .map_err(|_| Custom(Status::NotFound, json!("not found!")))
    })
    .await
}

#[delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::delete(conn, id)
            .map(|_| NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Something went wrong")))
    })
    .await
}

#[put("/crates/<id>", format = "json", data = "<update_crate>")]
pub async fn update_crate(
    id: i32,
    update_crate: Json<FormCrate>,
    db: DbConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::update(conn, update_crate.into_inner(), id)
            .map(|res| json!(res))
            .map_err(|_| Custom(Status::InternalServerError, json!("something went wrong!")))
    })
    .await
}
