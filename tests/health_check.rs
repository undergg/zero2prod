
use std::net::{TcpListener};

// spawn_app spawns our app and returns the url of the application.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    
    // We can retrieve the port assigned to us by the OS.
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");

    // Launch the server as a background task.
    // tokio::spawn returns a handle to spawned future,
    // but we have no use for it here, hence the non-binding let.
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    // Spawn our web service.
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert response code is 200.
    assert!(response.status().is_success());
    // Assert content is empty.
    assert_eq!(Some(0), response.content_length());
}
