use crate::spawn_app::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_a_valid_form_data() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let body = "name=stanley%20the%20third&email=stan%40gmail.com";
    let response = app.post_subscriptions(body.into()).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name from subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "stan@gmail.com");
    assert_eq!(saved.name, "stanley the third");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_missing_data() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=x", "missing email"),
        ("email=y@y.com", "missing the name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_subscriptions(invalid_body.into()).await;

        // Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {} {}",
            invalid_body,
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_empty() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=stanley%40catfood.com", "empty name"),
        ("name=Stanely&email=", "empty email"),
        ("name=Stanely&email=not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        // Act
        let response = app.post_subscriptions(body.into()).await;

        // Assert
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not return a 400 BAD REQUEST when the payload was {} {}",
            body,
            description
        );
    }
}
