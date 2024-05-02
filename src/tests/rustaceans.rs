use reqwest::{blocking::Client, StatusCode};
use serde_json::*;

fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({"email": "dvsg@345", "name": "dev"}))
        .send()
        .expect("Failed to create test rustacean");
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().expect("Failed to parse response")
}

#[test]
fn test_crud_rustaceans() {
    let client = Client::new();

    // Create a rustacean
    let mut json = create_test_rustacean(&client);

    // Test GET all rustaceans
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Test GET one rustacean
    let id = json["id"].as_u64().expect("Failed to get id");
    let response = client
        .get(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .send()
        .expect("Failed to get rustacean");
    assert_eq!(response.status(), StatusCode::OK);
    // Test update rustacean
    json["email"] = json!("dvsg@3488");
    json["name"] = json!("de33v");
    let response = client
        .put(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .json(&json)
        .send()
        .expect("Failed to update rustacean");
    assert_eq!(response.status(), StatusCode::OK);
    let updated_json: Value = response.json().expect("Failed to parse response");
    assert_eq!(
        updated_json,
        json!({"id": updated_json["id"],"email": "dvsg@3488", "name": "de33v", "created_at": updated_json["created_at"]})
    );

    // Test delete rustacean
    let response = client
        .delete(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .send()
        .expect("Failed to delete rustacean");
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
