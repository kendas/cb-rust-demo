use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let mut configuration = config::Config::default();
    configuration
        .set_default("server.port", 8080)?
        .set_default("database.username", "postgres")?
        .set_default("database.password", "postgres")?
        .set_default("database.port", 5432)?
        .set_default("database.host", "localhost")?
        .set_default("database.database_name", "postgres")?
        .set_default("logging.level", "info")?;
    let config_file = match std::env::var("APP_ENVIRONMENT") {
        Ok(run_environment) => format!("config.{}.toml", run_environment),
        Err(_) => "config.toml".to_owned(),
    };
    if Path::new(&config_file).exists() {
        configuration.merge(config::File::with_name(&config_file))?;
    }
    configuration.merge(config::Environment::with_prefix("APP_"))?;
    configuration.try_into()
}
