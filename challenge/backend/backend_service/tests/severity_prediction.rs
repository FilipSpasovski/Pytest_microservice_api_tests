use serde_json::json;
mod common;
use assert2ify::assert2ify;
#[actix_rt::test]
#[test]
#[assert2ify(check)]
async fn predict_return_400_wrong_request() {
    // Arrange
    let test_app = common::spawn_app().await;

    let client = reqwest::Client::new();
    let request_rubish_json = json!({
        "whatever".to_string() : "whatever".to_string()
    });

    // Act
    let response = client
        .post(&format!("{}/predictions/severity", &test_app.address))
        .json(&request_rubish_json)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(400, response.status().as_u16());
    let content = response.text().await.unwrap();
    assert!(content.contains("title"));
    assert!(content.contains("details"));
    assert!(content.contains("status"));
}

#[actix_rt::test]
#[test]
#[assert2ify(check)]
async fn predict_return_ok() {
    let test_app = common::spawn_app().await;

    let client = reqwest::Client::new();
    let request_json = json!({
        "vehicle_sit": 1,
        "weather" : 8,
        "sex" : 2,
        "year" : 16,
        "birth_year" : 1983,
        "security_used" : 1,
        "hour": 14.75,
        "luminosity": 1,
        "department": 590,
        "in_agglomeration": 2,
        "collision_type": 3,
        "road_type": 3,
        "pathways_width": 0,
        "vehicle_type": 2,
        "obstacle_type": 7.0,
        "shock_location": 7,
        "maneuver_type": 15,

    });

    // Act
    let response = client
        .post(&format!("{}/predictions/severity", &test_app.address))
        .json(&request_json)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let content = response.text().await.unwrap();
    assert!(content.contains("severity"));
}
