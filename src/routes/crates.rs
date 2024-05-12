use crate::{
    models::{
        crates::{FormCrate, NewCrate},
        users::User,
    },
    routes::DbConn,
};

use super::{super::repositories::crate_repository::CrateRepository, serve_error};
extern crate diesel;
use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

#[get("/crates")]
pub async fn get_crates(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|conn| {
        CrateRepository::find(conn, 100)
            .map(|data| json!(data))
            .map_err(|err| serve_error(&err.into()))
    })
    .await
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    db: DbConn,
    new_crate: Json<NewCrate>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|conn| {
        CrateRepository::create(conn, new_crate.into_inner())
            .map(|res| Custom(Status::Created, json!(res)))
            .map_err(|err| serve_error(&err.into()))
    })
    .await
}

#[get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::find_one(conn, id)
            .map(|res| json!(res))
            .map_err(|err| serve_error(&err.into()))
    })
    .await
}

#[delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConn, _user: User) -> Result<NoContent, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::delete(conn, id)
            .map(|_| NoContent)
            .map_err(|err| serve_error(&err.into()))
    })
    .await
}

#[put("/crates/<id>", format = "json", data = "<update_crate>")]
pub async fn update_crate(
    id: i32,
    update_crate: Json<FormCrate>,
    db: DbConn,
    _user: User,
) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        CrateRepository::update(conn, update_crate.into_inner(), id)
            .map(|res| json!(res))
            .map_err(|err| serve_error(&err.into()))
    })
    .await
}
