use crate::tests::utility::common;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use rocket::response;
use serde_json::*;
use std::process::Command;

#[test]
fn test_login() {
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

    let correct_response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({"username": "test_admin", "password": "123456"}))
        .send()
        .unwrap();

    assert_eq!(correct_response.status(), StatusCode::OK);
    let json: Value = correct_response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);
    let incorrect_response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({"username": "test_admin", "password": "1234956"}))
        .send()
        .unwrap();

    assert_eq!(incorrect_response.status(), StatusCode::UNAUTHORIZED);
}
