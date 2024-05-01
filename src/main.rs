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

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                update_rustacean,
                create_rustacean,
                get_rustaceans,
                view_rustacean,
                delete_rustacean,
                hello_world
            ],
        )
        .launch()
        .await;
}
