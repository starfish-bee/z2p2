//use std::iter::repeat;

use std::iter::repeat;

mod common;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let (addr, db_pool) = common::spawn_app().await.expect("failed to spawn app");
    let client = reqwest::Client::new();

    let response = client
        .post(format!("http://{addr}/subscribe"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_422_for_missing_form_fields() {
    let (addr, _) = common::spawn_app().await.expect("failed to spawn app");
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for case in test_cases {
        let response = client
            .post(format!("http://{addr}/subscribe"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(case.0)
            .send()
            .await
            .expect("Failed to execute request.");
        println!("response: {response:?}");

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Unprocessable Entity when the payload was {}.",
            case.1
        );
    }
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_fields() {
    let (addr, _) = common::spawn_app().await.expect("failed to spawn app");
    let client = reqwest::Client::new();

    let long_name = String::from_iter(repeat("a").take(257));
    let long_name = format!("name={long_name}&email=ursula_le_guin%40gmail.com");

    let test_cases = vec![
        (
            "email=ursula_le_guin%40gmail.com&name=".to_string(),
            "name field is empty",
        ),
        (long_name, "name too long"),
        (
            "email=ursula_le_guin%40gmail.com&name=asdfjlk/".to_string(),
            "name asdfjlk/ contains invalid characters",
        ),
    ];

    for case in test_cases {
        let response = client
            .post(format!("http://{addr}/subscribe"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(case.0)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            case.1
        );
    }
}
