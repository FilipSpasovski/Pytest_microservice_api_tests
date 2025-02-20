mod common;
use assert2ify::assert2ify;

#[actix_rt::test]
#[test]
#[assert2ify(check)]
async fn status_return_ok() {
    // Arrange
    let test_app = common::spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/status", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let content = response.text().await.unwrap();
    assert!(content.contains("UP"));
}
