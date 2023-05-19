#[tokio::test]
async fn health_check_works() {
    // Arrange
    // Start up the gateway binary
    spawn_app();
    // Use a web client to make API calls to it
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/check-health")
        .send()
        .await
        .expect("Failed to call the health check.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = gateway4exp::run().expect("Failed to spawn the server for testing.");

    let _ = tokio::spawn(server);
}

// todo!("Start polishing! Bookmark for section 3.5.1 as a stopping point.")
