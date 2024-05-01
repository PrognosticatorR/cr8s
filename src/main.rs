#[allow(dead_code)]
use rocket::routes;
use routes::rustaceans::{
    create_rustacean, delete_rustacean, get_rustaceans, hello_world, update_rustacean,
    view_rustacean,
};
extern crate rocket;

extern crate diesel;

mod models;
mod repositories;
mod routes;
mod schema;
mod utils;

#[rocket::main]
async fn main() {
    let routes = routes![
        update_rustacean,
        create_rustacean,
        get_rustaceans,
        view_rustacean,
        delete_rustacean,
        hello_world
    ];
    let _ = rocket::build().mount("/", routes).launch().await;
}
