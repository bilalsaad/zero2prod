// Configurations
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    /// Creates a postgres connection String
    ///
    /// Example:
    /// ```rust
    ///  use zero2prod2::configuration::DatabaseSettings;
    ///  use secrecy::{Secret, ExposeSecret};
    ///
    ///  let settings = DatabaseSettings {
    ///    username: "user1".into(),
    ///    password: Secret::new("pwd".to_string()),
    ///    port: 5432,
    ///    host: "localhost".into(),
    ///    database_name: "db".into(),
    ///   };
    ///
    ///  assert_eq!(*settings.connection_string().expose_secret(),
    ///            format!("postgres://user1:pwd@localhost:5432/db"));
    /// ```
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    /// Creates a db connection string but omits the database name.
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

/// Reads settings from file named "configuration.yaml".
/// expects file to be a YAML file with the Config struct.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_deserialize::<Settings>()
}
