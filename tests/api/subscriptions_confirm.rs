use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::spawn_app::spawn_app;

#[tokio::test]
async fn confirmation_without_token_are_rejected_with_400() {
    let app = spawn_app().await;

    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    let app = spawn_app().await;
    // This should really be the default when creating the app :(
    Mock::given(path("/v3/mail/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Subscribe
    app.post_subscriptions("name=stanley&email=s%40s.com".into())
        .await;

    // Get the email confirmation link from the email sent out.
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);

    // Act - visit the confirmation link.
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}
