use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod2::configuration::get_configuration;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    // Create an http client to perform requests.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_a_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=stanley%20the%20third&email=stan%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

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
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=x", "missing email"),
        ("email=y@y.com", "missing the name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

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

pub struct TestApp {
    /// Address the application is serving HTTP requests from. e.g., localhost:8080.
    pub address: String,
    /// The applications underlying DB.
    pub db_pool: PgPool,
}

/// Launch our application in the background and returns address
async fn spawn_app() -> TestApp {
    // Run the app on an ephemral port on the localhost.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.01:{}", port);

    // Create a fresh DB for this test run.
    let configuration = {
        let mut configuration = get_configuration().expect("failed to get configuration");
        // Use a random name to make tests hermetic.
        configuration.database.database_name = Uuid::new_v4().to_string();
        configuration
    };

    let db_pool = {
        let config = &configuration.database;
        // Create the database
        let mut connection = PgConnection::connect(&config.connection_string_without_db())
            .await
            .expect("failed to connect to postgres");

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
            .await
            .expect("Failed to create ephemeral database.");

        // migrate the database
        let connection_pool = PgPool::connect(&config.connection_string())
            .await
            .expect("Failed to connect to postgres after creating ephemeral db");
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("failed to run DB migration ./migrations");
        connection_pool
    };

    let server =
        zero2prod2::startup::run(listener, db_pool.clone()).expect("failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}
