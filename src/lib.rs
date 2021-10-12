use std::io;
use std::net::TcpListener;

use actix_files::Files;
use actix_web::{
    dev::Server,
    http::header,
    web::{self, Data, Path},
    App, HttpResponse, HttpServer,
};
use uuid::Uuid;

use db::HoursRepo;
use hours::NewHours;

pub mod db;
mod hours;
pub mod configuration;

async fn redirect_to_api_doc() -> HttpResponse {
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, "/openapi/"))
        .finish()
}

async fn list_all_logged_hours<T: HoursRepo>(db: Data<T>) -> HttpResponse {
    let all_hours = db.list();
    HttpResponse::Ok().json(all_hours)
}

async fn get_single_hours_entry<T: HoursRepo>(id: Path<Uuid>, db: Data<T>) -> HttpResponse {
    let id = id.into_inner();
    let result = db.by_id(id);
    match result {
        Some(hours) => HttpResponse::Ok().json(hours),
        None => HttpResponse::NotFound().json(id),
    }
}

async fn log_hours<T: HoursRepo>(db: Data<T>, json: web::Json<NewHours>) -> HttpResponse {
    let new_hours = json.into_inner();
    let hours_entry = db.insert(new_hours);
    HttpResponse::Created().json(hours_entry.id)
}

async fn delete_logged_hours<T: HoursRepo>(id: Path<Uuid>, db: Data<T>) -> HttpResponse {
    let id = id.into_inner();
    match db.delete(id) {
        true => HttpResponse::NoContent().finish(),
        false => HttpResponse::NotFound().json(id),
    }
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run_server<T: HoursRepo + 'static>(hr: T, listener: TcpListener) -> io::Result<Server> {
    let db = Data::new(hr);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(redirect_to_api_doc))
            .service(
                web::scope("/api")
                    .service(web::resource("/health_check").route(web::get().to(health_check)))
                    .service(
                        web::resource("/hours")
                            .route(web::get().to(list_all_logged_hours::<T>))
                            .route(web::post().to(log_hours::<T>)),
                    )
                    .service(
                        web::resource("/hours/{id}")
                            .route(web::get().to(get_single_hours_entry::<T>))
                            .route(web::delete().to(delete_logged_hours::<T>)),
                    ),
            )
            .service(Files::new("/openapi", "./openapi/").index_file("index.html"))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
