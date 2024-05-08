use reqwest::{blocking::Client, StatusCode};
use serde_json::*;

use crate::tests::utility::{common, common::APP_HOST};

fn create_crate(
    client: &Client,
    rustacean: &Value,
    name: &str,
    code: &str,
    version: &str,
    description: Option<&str>,
) -> Value {
    let json = &json!({
        "rustacean_id": rustacean["id"],
        "name": name,
        "code": code,
        "version": code,
        "description":description,
        "version":version
    });
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(json)
        .send()
        .expect("Failed to create crate");
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().expect("Failed to parse response")
}

fn get_crate(client: &Client, id: u64) -> Value {
    let response = client
        .get(format!("{}/crates/{}", APP_HOST, id))
        .send()
        .expect("Failed to get crate");
    assert_eq!(response.status(), StatusCode::OK);
    response.json().expect("Failed to parse response")
}

fn update_crate(
    client: &Client,
    rustacean_id: i32,
    name: &str,
    code: &str,
    version: &str,
    description: Option<&str>,
    id: u64,
) -> Value {
    let json = &json!({
        "rustacean_id": rustacean_id,
        "name": name,
        "code": code,
        "version": version,
        "description":description,
        "version":version
    });
    let response = client
        .put(format!("{}/crates/{}", APP_HOST, id))
        .json(&json)
        .send()
        .expect("Failed to update crate");
    assert_eq!(response.status(), StatusCode::OK);
    response.json().expect("Failed to parse response")
}

fn delete_crate(client: &Client, id: u64) {
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, id))
        .send()
        .expect("Failed to delete crate");
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_create_crate() {
    let client = Client::new();
    let rustacean = common::create_rustacean(&client, "abc@gmail.com", "ABC@D");
    // Create a crate
    let json = create_crate(&client, &rustacean, "dev", "foo Boo", "0.1.1", None);

    // Test GET one crate
    let id = json["id"].as_u64().expect("Failed to get id");
    let _json = get_crate(&client, id);

    // Test delete crate
    delete_crate(&client, id);
    common::delete_rustacean(&client, id);
}

#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean = common::create_rustacean(&client, "dev@chin2", "devrawat");
    // Create a crate
    let json = create_crate(&client, &rustacean, "dev", "foo Boo", "0.1.1", None);

    // Test GET one crate
    let id = json["id"].as_u64().expect("Failed to get id");
    let rustacean_id = rustacean["id"]
        .as_u64()
        .expect("Failed to get rustacean_id!");
    // Test update crate
    let updated_json = update_crate(
        &client,
        rustacean_id.try_into().unwrap(),
        "de33v",
        "foo Boo",
        "0.1.1",
        Some("description on crate"),
        id,
    );
    assert_eq!(
        updated_json,
        json!({"id": updated_json["id"], "name": "de33v", "created_at": updated_json["created_at"], "rustacean_id": rustacean["id"],"code":"foo Boo","version":"0.1.1", "description":Some("description on crate")})
    );

    // Test delete crate
    delete_crate(&client, id);
    common::delete_rustacean(&client, id);
}

#[test]
fn test_get_all_crates() {
    let client = Client::new();
    let rustacean = common::create_rustacean(&client, "dev@chin2", "devrawat");
    // Create a crate
    create_crate(&client, &rustacean, "dev", "foo Boo", "0.1.1", None);

    // Test GET all crates
    let response = client.get("http://127.0.0.1:8000/crates").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
