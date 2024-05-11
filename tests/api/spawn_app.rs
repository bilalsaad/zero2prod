use once_cell::sync::Lazy;
use reqwest::Url;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;
use zero2prod2::configuration::get_configuration;
use zero2prod2::startup::{get_connection_pool, Application};
use zero2prod2::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    /// Address the application is serving HTTP requests from. e.g., localhost:8080.
    pub address: String,
    /// The applications underlying DB.
    pub db_pool: PgPool,
    /// Mock email client
    pub email_server: MockServer,
    /// Application port
    pub port: u16,
}

/// Confirmation links embedded inthe email API.
pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

impl TestApp {
    /// Sends a POST /subscriptions with the given body.
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Sends a POST /newsletters with the given body.
    pub async fn post_newsletters(&self, body: &serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/newsletters", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Extract  the confirmation links embedded in the request to the email API.
    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

        // Extract the link from one of the request fields.
        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            assert_eq!(links.len(), 1);
            let raw_link = links[0].as_str().to_owned();
            let mut link = Url::parse(&raw_link).unwrap();
            link.set_port(Some(self.port)).unwrap();
            assert_eq!(link.host_str().unwrap(), "127.0.0.1");
            link
        };

        let html = get_link(&body["content"][0]["value"].as_str().unwrap());
        let plain_text = get_link(&body["content"][1]["value"].as_str().unwrap());
        ConfirmationLinks { html, plain_text }
    }
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

/// Launch our application in the background and returns address
pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    // Launch a fake email server to stand in for SendGrid.
    let email_server = MockServer::start().await;

    // Create a fresh DB for this test run.
    let configuration = {
        let mut configuration = get_configuration().expect("failed to get configuration");
        // Use a random name to make tests hermetic.
        configuration.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port.
        configuration.application.port = 0;
        // Use fake mail server
        configuration.email_client.base_url = email_server.uri();
        configuration
    };

    // Configure DB pool connection.
    let _ = {
        let config = &configuration.database;
        // Create the database
        let mut connection = PgConnection::connect_with(&config.without_db())
            .await
            .expect("failed to connect to postgres");

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
            .await
            .expect("Failed to create ephemeral database.");

        // migrate the database
        let connection_pool = PgPool::connect_with(config.with_db())
            .await
            .expect("Failed to connect to postgres after creating ephemeral db");
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("failed to run DB migration ./migrations");
        connection_pool
    };

    let application = Application::build(configuration.clone())
        .await
        .expect("failed to build application.");
    let application_port = application.port();
    let address = format!("http://localhost:{}", application_port);
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
        port: application_port,
    }
}
