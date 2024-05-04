use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod2::email_client::EmailClient;
use zero2prod2::telemetry::{get_subscriber, init_subscriber};
use zero2prod2::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Setup tracing and logging.
    let subscriber = get_subscriber("zero2prod2".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    //-------------- Setup database
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    // ------------- Setup EmailClient
    let sender_email = configuration
        .email_client
        .sender()
        .expect("invalid sender email address");
    let email_client = EmailClient::new(configuration.email_client.base_url, sender_email);

    //-------------- Setup TCPListener
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind to port");

    // Finally run server
    run(listener, connection, email_client)?.await
}
