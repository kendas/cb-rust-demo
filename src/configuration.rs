use std::path::Path;

use regex::Regex;

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
    pub name: String,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
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
        .set_default("database.name", "postgres")?
        .set_default("logging.level", "info")?;
    let config_file = match std::env::var("APP_ENVIRONMENT") {
        Ok(run_environment) => format!("config.{}.toml", run_environment),
        Err(_) => "config.toml".to_owned(),
    };
    if Path::new(&config_file).exists() {
        configuration.merge(config::File::with_name(&config_file))?;
    }
    set_database_vars_from_heroku_url();
    configuration.merge(config::Environment::with_prefix("APP").separator("_"))?;
    configuration.try_into()
}

fn set_database_vars_from_heroku_url() {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        let pattern = Regex::new(r"^postgres://(?P<username>[^:]+):(?P<password>[^@]+)@(?P<hostname>[^:]+)(:(?P<port>\d+))?/(?P<database>.*)$").unwrap();
        if let Some(captures) = pattern.captures(&url) {
            std::env::set_var(
                "APP_DATABASE_USERNAME",
                captures.name("username").unwrap().as_str(),
            );
            std::env::set_var(
                "APP_DATABASE_PASSWORD",
                captures.name("password").unwrap().as_str(),
            );
            std::env::set_var(
                "APP_DATABASE_HOST",
                captures.name("hostname").unwrap().as_str(),
            );
            if let Some(m) = captures.name("port") {
                std::env::set_var("APP_DATABASE_PORT", m.as_str());
            }
            std::env::set_var(
                "APP_DATABASE_NAME",
                captures.name("database").unwrap().as_str(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn set_database_vars_from_heroku_url_when_empty() {
        std::env::remove_var("DATABASE_URL");

        set_database_vars_from_heroku_url();

        let result = std::env::var("APP_DATABASE_USERNAME");

        assert!(result.is_err());

        clean_env();
    }

    #[test]
    #[ignore]
    fn set_database_vars_from_heroku_url_when_exists() {
        std::env::set_var("DATABASE_URL", "postgres://user:pass@host:5432/database");

        set_database_vars_from_heroku_url();

        assert_eq!(std::env::var("APP_DATABASE_USERNAME").unwrap(), "user");
        assert_eq!(std::env::var("APP_DATABASE_PASSWORD").unwrap(), "pass");
        assert_eq!(std::env::var("APP_DATABASE_HOST").unwrap(), "host");
        assert_eq!(std::env::var("APP_DATABASE_PORT").unwrap(), "5432");
        assert_eq!(std::env::var("APP_DATABASE_NAME").unwrap(), "database");

        clean_env();
    }

    #[test]
    #[ignore]
    fn set_database_vars_from_heroku_url_when_no_port() {
        std::env::set_var("DATABASE_URL", "postgres://user:pass@host/database");

        set_database_vars_from_heroku_url();

        assert!(std::env::var("APP_DATABASE_PORT").is_err());

        clean_env();
    }

    #[test]
    #[ignore]
    fn get_configuration_gets_from_heroku_url() {
        std::env::set_var("DATABASE_URL", "postgres://user:pass@host:5432/database");

        let config = get_configuration().unwrap();

        assert_eq!(config.database.username, "user");
        assert_eq!(config.database.password, "pass");
        assert_eq!(config.database.host, "host");
        assert_eq!(config.database.port, 5432);
        assert_eq!(config.database.name, "database");

        clean_env();
    }

    fn clean_env() {
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("APP_DATABASE_USERNAME");
        std::env::remove_var("APP_DATABASE_PASSWORD");
        std::env::remove_var("APP_DATABASE_HOST");
        std::env::remove_var("APP_DATABASE_PORT");
        std::env::remove_var("APP_DATABASE_NAME");
    }
}
