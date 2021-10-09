use std::env;
use std::io;
use std::net::TcpListener;

use cb_rust_demo::db::MemDb;

fn init_logger() {
    let logger_environment = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(logger_environment).init();
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    init_logger();

    let db: MemDb = Default::default();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(bind_address)?;
    let server = cb_rust_demo::run_server(db, listener)?;
    server.await
}
