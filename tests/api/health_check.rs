use crate::helpers::{spawn_app, teardown};

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute health check request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    teardown(app).await;
}
