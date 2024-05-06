use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::*;
pub static APP_HOST: &str = "http://127.0.0.1:8000";
pub fn create_rustacean(client: &Client, email: &str, name: &str) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({"email": email, "name": name}))
        .send()
        .expect("Failed to create rustacean");
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().expect("Failed to parse response")
}
pub fn delete_rustacean(client: &Client, id: u64) {
    let response = client
        .delete(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .send()
        .expect("Failed to delete rustacean");
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
