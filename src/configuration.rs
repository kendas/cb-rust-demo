use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let mut configuration = config::Config::default();
    configuration
        .set_default("server.port", 8080)?
        .set_default("logging.level", "info")?;
    if Path::new("config.toml").exists() {
        configuration.merge(config::File::with_name("config.toml"))?;
    }
    configuration.merge(config::Environment::with_prefix("APP_"))?;
    configuration.try_into()
}
