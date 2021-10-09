#![cfg(test)]
use std::net::TcpListener;

use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

use cb_rust_demo::db;

const HOURS: &str = "{\
    \"employee\": \"employee\",
    \"date\": \"2021-10-09\",
    \"project\": \"project\",
    \"story_id\": null,
    \"description\": \"description\",
    \"hours\": 1
}";

#[derive(Deserialize)]
#[allow(dead_code)]
struct Hours {
    id: Uuid,
    employee: String,
    project: String,
    story_id: Option<String>,
    description: String,
    hours: u8,
}

#[actix_rt::test]
async fn hours_from_empty_db() {
    let address = spawn_app();

    let client = Client::new();

    let response = client
        .get(format!("{}/api/hours", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "[]");
}

#[actix_rt::test]
async fn hours_insert_and_retrieve() {
    let address = spawn_app();

    let client = Client::new();

    let response = client
        .post(format!("{}/api/hours", address))
        .body(HOURS)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let id: Uuid = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    let response = client
        .get(format!("{}/api/hours/{}", address, id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let result: Hours = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(result.id, id);
}

#[actix_rt::test]
async fn hours_insert_and_retrieve_list() {
    let address = spawn_app();

    let client = Client::new();

    let response = client
        .post(format!("{}/api/hours", address))
        .body(HOURS)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let id: Uuid = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    let response = client
        .get(format!("{}/api/hours", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let result: Vec<Hours> = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result.get(0).unwrap().id, id);
}

#[actix_rt::test]
async fn hours_insert_and_delete() {
    let address = spawn_app();

    let client = Client::new();
    let response = client
        .post(format!("{}/api/hours", address))
        .body(HOURS)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let id: Uuid = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    let response = client
        .delete(format!("{}/api/hours/{}", address, id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let response = client
        .get(format!("{}/api/hours", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let result: Vec<Hours> = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert!(result.is_empty());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Port not open");
    let port = listener.local_addr().unwrap().port();
    let server =
        cb_rust_demo::run_server(db::MemDb::default(), listener).expect("Server failed to start");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
