use std::io;
use std::net::TcpListener;

use actix_files::Files;
use actix_web::{
    dev::Server,
    http::header,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use sqlx::{migrate::MigrateError, PgPool};

pub mod configuration;
pub mod db;
pub mod error;
mod hours;
pub mod test_utils;

async fn redirect_to_api_doc() -> HttpResponse {
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, "/openapi/"))
        .finish()
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn init_db(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}

pub fn run_server(pool: PgPool, listener: TcpListener) -> io::Result<Server> {
    let db = Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(redirect_to_api_doc))
            .service(
                web::scope("/api")
                    .service(web::resource("/health_check").route(web::get().to(health_check)))
                    .service(
                        web::resource("/hours")
                            .route(web::get().to(hours::list_all_logged_hours))
                            .route(web::post().to(hours::log_hours)),
                    )
                    .service(
                        web::resource("/hours/{id}")
                            .route(web::get().to(hours::get_single_hours_entry))
                            .route(web::delete().to(hours::delete_logged_hours)),
                    ),
            )
            .service(Files::new("/openapi", "./openapi/").index_file("index.html"))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
