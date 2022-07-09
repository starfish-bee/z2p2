mod common;

#[tokio::test]
async fn health_check_returns_200() {
    let addr = common::spawn_app().await.expect("failed to spawn app");
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(0), response.content_length());
}
