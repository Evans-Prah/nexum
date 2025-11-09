use std::net::TcpListener;
use nexum::run;

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
    .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String  {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind random app");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to start server");
    let _ = actix_web::rt::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
