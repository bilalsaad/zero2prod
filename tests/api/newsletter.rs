// e2e test for newsletter.
use wiremock::{
    matchers::{any, method, path},
    Mock, ResponseTemplate,
};

use crate::spawn_app::{assert_is_redirect_to, spawn_app, ConfirmationLinks, TestApp};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    // Expect
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        // Assert that no request is fired.
        .expect(0)
        .mount(&app.email_server)
        .await;

    // Act - Send newsletter out

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain_text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response = app.post_newsletters(&newsletter_request_body).await;

    // Assert
    assert_is_redirect_to(&response, "/admin/newsletter")
    // Mock verifies on drop.
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    // arrange
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    // expect
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // act - send newsletter out

    let newsletter_request_body = serde_json::json!({
        "title": "newsletter title",
        "text_content": "newsletter body as plain_text",
        "html_content": "<p>newsletter body as html</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response = app.post_newsletters(&newsletter_request_body).await;
    dbg!(&response);

    // assert
    assert_is_redirect_to(&response, "/admin/newsletter")
    // mock verifies on drop.
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    // arrange
    let app = spawn_app().await;
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    let test_cases = vec![
        (
            serde_json::json!({
                "text_content": "text", "html_content": "html",
                "idempotency_key": uuid::Uuid::new_v4().to_string()

            }),
            "missing_title",
        ),
        (
            serde_json::json!({
                "title": "title",
                "idempotency_key": uuid::Uuid::new_v4().to_string()
            }),
            "missing_content",
        ),
        (
            serde_json::json!({
                "title": "title",
                "text_content": "t",
                "html_content": "tt",
            }),
            "missing_idempotency_ley",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_newsletters(&invalid_body).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with a 400 bad request for {}. \n{:?}",
            error_message,
            invalid_body
        );
    }
}

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=stan&email=bigcat%40gmail.com";
    let _mock_guard = Mock::given(path("/v3/mail/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;
    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(&email_request)
}

#[tokio::test]
async fn post_newsletters_redirects_to_login_if_not_logged_in() {
    // arrange
    let app = spawn_app().await;

    let request_body = serde_json::json!({
        "title": "newsletter title",
        "text_content": "newsletter body as plain_text",
        "html_content": "<p>newsletter body as html</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()

    });

    let response = app.post_newsletters(&request_body).await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn newsletter_creation_is_idempotent() {
    // arrange
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    Mock::given(path("/v3/mail/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Submit newsletter
    let request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        // We expect the idempotency key as part of the
        // form data, not as an header
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });

    let response = app.post_newsletters(&request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletter");

    // Act - Part 2 - Follow the redirect
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!"));

    // Act - Part 3 - Submit the same requst again
    let response = app.post_newsletters(&request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletter");
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!"));

    // Mock verifies on Drop that we have sent the email one.
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;
    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}
