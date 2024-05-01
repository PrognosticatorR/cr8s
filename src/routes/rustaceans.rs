use rocket::{
    delete, get, post, put,
    serde::json::{serde_json::json, Value},
};

#[get("/rustaceans")]
pub fn get_rustaceans() -> Value {
    json!["Hello, there!"]
}

#[post("/rustaceans")]
pub fn create_rustacean() {}

#[get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32) {}

#[delete("/rustacean/<id>")]
pub fn delete_rustacean(id: i32) {}

#[put("/rustaceans/<id>")]
pub fn update_rustacean(id: i32) {}
