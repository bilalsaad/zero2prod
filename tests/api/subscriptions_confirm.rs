use claims::assert_gt;
use reqwest::Url;
use wiremock::{matchers::{method, path}, Mock, ResponseTemplate};

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
    app.post_subscriptions("name=stanley&email=s%40s.com".into()).await;

    // Get the email confirmation link from the email sent out.
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

    // Extract the link from one of the request fields.
    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_gt!(links.len(), 0);
        links[0].as_str().to_owned()
    };

    let confirmation_link = {
        let link = get_link(&body["content"][0]["value"].as_str().unwrap());
        let mut l = Url::parse(&link).unwrap();
        l.set_port(Some(app.port)).unwrap();
        l
    };
    
    // Make sure no network calls from test.
    assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");

    dbg!("Confirmation link is: {:?}, app address is : {:?}", &confirmation_link, &app.address);


    // Act - visit the confirmation link.
    let response = reqwest::get(confirmation_link)
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);
}
