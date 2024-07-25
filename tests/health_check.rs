//! tests/health_check.rs
// `cargo expand --test health_check` (<- name of the test file)

use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health-check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
//

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Faild to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = rust_server::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
