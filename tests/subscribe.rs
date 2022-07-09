mod common;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let addr = common::spawn_app().await.expect("failed to spawn app");
    let client = reqwest::Client::new();

    let response = client
        .post(format!("http://{addr}/subscribe"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_422_for_invalid_form_data() {
    let addr = common::spawn_app().await.expect("failed to spawn app");
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

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Unprocessable Entity  when the payload was {}.",
            case.1
        );
    }
}
