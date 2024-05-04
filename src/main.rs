use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
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

    //-------------- Setup TCPListener
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind to port");

    // Finally run server
    run(listener, connection)?.await
}
