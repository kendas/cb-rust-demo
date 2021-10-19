#![cfg(test)]
use std::net::TcpListener;

use reqwest::Client;

use cb_rust_demo::test_utils;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let client = Client::new();

    let response = client
        .get(format!("{}/api/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0))
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Port not open");
    let port = listener.local_addr().unwrap().port();
    let server = cb_rust_demo::run_server(test_utils::get_db_pool().await, listener)
        .expect("Server failed to start");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
