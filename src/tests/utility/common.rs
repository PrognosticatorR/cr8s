use std::process::Command;

use reqwest::blocking::Client;
use reqwest::header;
use reqwest::{blocking::ClientBuilder, StatusCode};
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

pub fn get_client_with_logged_in_admin() -> Client {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("123456")
        .arg("admin, superadmin")
        .output();
    println!("{:?}", output);
    let client = Client::new();

    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({"username": "test_admin", "password": "123456"}))
        .send()
        .unwrap();
    println!("Response: {:?}", response);
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}
