use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let body = "name=ryuma%20taguchi&email=ryuma017%40gmail.com";
    let response = app.post_subscriptions(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ryuma017@gmail.com");
    assert_eq!(saved.name, "ryuma taguchi")
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=ryuma%20taguchi", "missing the email"),
        ("email=ryuma017%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_subscriptions(invalid_body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=ryuma017%40gmail.com", "empty_name"),
        ("name=Ryuma&email=", "empty email"),
        ("name=Ryuma&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        // Act
        let response = app.post_subscriptions(body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}
