extern crate cr8s;
use cr8s::routes;
use cr8s::routes::auth::{login, logout};
use cr8s::routes::crates::{create_crate, delete_crate, get_crates, update_crate, view_crate};
use cr8s::routes::rustaceans::{
    create_rustacean, delete_rustacean, get_rustaceans, update_rustacean, view_rustacean,
};
use rocket::routes;
use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let routes = routes![
        login,
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
        .attach(routes::CacheConn::init())
        .launch()
        .await;
}
