use sqlx::PgPool;
use std::env;
use std::io;
use std::net::TcpListener;

use cb_rust_demo::configuration::{self, Config};

fn init_logger(config: &Config) {
    let logger_environment = env_logger::Env::default().default_filter_or(&config.logging.level);
    env_logger::Builder::from_env(logger_environment).init();
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = configuration::get_configuration().expect("Unable to get configuration");
    init_logger(&config);

    let pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Unable to get a database connection");
    cb_rust_demo::init_db(&pool)
        .await
        .expect("Unable to run migrations");

    let port = env::var("PORT").unwrap_or_else(|_| config.server.port.to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(bind_address)?;
    let server = cb_rust_demo::run_server(pool, listener)?;
    server.await
}
