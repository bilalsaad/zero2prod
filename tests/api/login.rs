use crate::spawn_app::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password",
    });
    let response = app.post_login(&login_body).await;
    let flash_cookie = response.cookies().find(|c| c.name() == "_flash").unwrap();

    // Assert
    eprintln!("wtffff {:?}", flash_cookie);
    assert_eq!(flash_cookie.value(), "Authentication failed");
    assert_is_redirect_to(&response, "/login");

    // Act - Part 2, fetch the login page from the redirect.
    let html_page = app.get_login_html().await;
    eprintln!("got html page {html_page}");
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));
}
