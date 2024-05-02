#[allow(dead_code)]
use rocket::routes;
use routes::crates::{create_crate, delete_crate, get_crates, update_crate, view_crate};
use routes::rustaceans::{
    create_rustacean, delete_rustacean, get_rustaceans, update_rustacean, view_rustacean,
};

extern crate rocket;

extern crate diesel;

mod models;
mod repositories;
mod routes;
mod schema;
mod tests;
mod utils;

#[rocket::main]
async fn main() {
    let routes = routes![
        update_rustacean,
        create_rustacean,
        get_rustaceans,
        view_rustacean,
        delete_rustacean,
        update_crate,
        create_crate,
        get_crates,
        view_crate,
        delete_crate
    ];
    let _ = rocket::build()
        .mount("/", routes)
        .attach(routes::DbConn::fairing())
        .launch()
        .await;
}
