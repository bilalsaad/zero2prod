use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // Create an http client to perform requests.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background and returns address
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod2::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.01:{}", port)
}
