use std::io;
use std::net::TcpListener;

use actix_files::Files;
use actix_web::{
    dev::Server,
    http::header,
    web::{self, Data, Path},
    App, HttpResponse, HttpServer,
};
use sqlx::{migrate::MigrateError, PgPool};
use uuid::Uuid;

use db::HoursRepo;
use hours::NewHours;

pub mod configuration;
pub mod db;
mod hours;
pub mod test_utils;

async fn redirect_to_api_doc() -> HttpResponse {
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, "/openapi/"))
        .finish()
}

async fn list_all_logged_hours(db: Data<PgPool>) -> HttpResponse {
    let mut connection = db.acquire().await.unwrap();
    let all_hours = connection.list().await;
    HttpResponse::Ok().json(all_hours)
}

async fn get_single_hours_entry(id: Path<Uuid>, db: Data<PgPool>) -> HttpResponse {
    let mut connection = db.acquire().await.unwrap();
    let id = id.into_inner();
    match connection.by_id(id).await {
        Some(hours) => HttpResponse::Ok().json(hours),
        None => HttpResponse::NotFound().json(id),
    }
}

async fn log_hours(db: Data<PgPool>, json: web::Json<NewHours>) -> HttpResponse {
    let mut connection = db.acquire().await.unwrap();
    let new_hours = json.into_inner();
    let hours_entry = connection.insert(new_hours).await;
    HttpResponse::Created().json(hours_entry)
}

async fn delete_logged_hours(id: Path<Uuid>, db: Data<PgPool>) -> HttpResponse {
    let mut connection = db.acquire().await.unwrap();
    let id = id.into_inner();
    match connection.delete(id).await {
        true => HttpResponse::NoContent().finish(),
        false => HttpResponse::NotFound().json(id),
    }
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
                            .route(web::get().to(list_all_logged_hours))
                            .route(web::post().to(log_hours)),
                    )
                    .service(
                        web::resource("/hours/{id}")
                            .route(web::get().to(get_single_hours_entry))
                            .route(web::delete().to(delete_logged_hours)),
                    ),
            )
            .service(Files::new("/openapi", "./openapi/").index_file("index.html"))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
