use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod2::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let connection = PgPool::connect(&connection_string)
        .await
        .expect("failed to connect to postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind to port 8000");
    run(listener, connection)?.await
}
