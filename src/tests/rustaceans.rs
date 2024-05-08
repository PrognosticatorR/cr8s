use reqwest::{blocking::Client, StatusCode};
use serde_json::*;

use super::utility::common;

fn get_rustacean(client: &Client, id: u64) -> Value {
    let response = client
        .get(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .send()
        .expect("Failed to get rustacean");
    assert_eq!(response.status(), StatusCode::OK);
    response.json().expect("Failed to parse response")
}

fn update_rustacean(client: &Client, id: u64, email: &str, name: &str) -> Value {
    let json = json!({"email": email, "name": name});
    let response = client
        .put(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .json(&json)
        .send()
        .expect("Failed to update rustacean");
    assert_eq!(response.status(), StatusCode::OK);
    response.json().expect("Failed to parse response")
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();

    // Create a rustacean
    let json = common::create_rustacean(&client, "dvsg@345", "dev");

    // Test GET one rustacean
    let id = json["id"].as_u64().expect("Failed to get id");
    let _json = get_rustacean(&client, id);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    // Create a rustacean
    let json = common::create_rustacean(&client, "dvsg@345", "dev");
    // Test GET one rustacean
    let id = json["id"].as_u64().expect("Failed to get id");
    // Test update rustacean
    let updated_json = update_rustacean(&client, id, "dvsg@3488", "de33v");
    assert_eq!(
        updated_json,
        json!({"id": updated_json["id"],"email": "dvsg@3488", "name": "de33v", "created_at": updated_json["created_at"]})
    );
}

#[test]
fn test_get_all_rustaceans() {
    let client = Client::new();

    // Create a rustacean
    common::create_rustacean(&client, "dvsg@345", "dev");

    // Test GET all rustaceans
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
