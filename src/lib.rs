use std::io;
use std::net::TcpListener;

use actix_files::Files;
use actix_web::{dev::Server, http::header, web, web::Data, App, HttpResponse, HttpServer};

use db::HoursRepo;
pub use db::MemDb;
use hours::NewHours;

mod db;
mod hours;

async fn redirect_to_api_doc() -> HttpResponse {
    return HttpResponse::TemporaryRedirect()
        .header(header::LOCATION, "/openapi/")
        .finish();
}

async fn list_all_logged_hours<T: HoursRepo>(db: Data<T>) -> HttpResponse {
    let all_hours = db.list();
    return HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .json(all_hours);
}

async fn get_single_hours_entry<T: HoursRepo>(
    web::Path(id): web::Path<uuid::Uuid>,
    db: Data<T>,
) -> HttpResponse {
    let result = db.by_id(id);
    match result {
        Some(hours) => HttpResponse::Ok().json(hours),
        None => HttpResponse::NotFound().body(id.to_string()),
    }
}

async fn log_hours<T: HoursRepo>(db: Data<T>, json: web::Json<NewHours>) -> HttpResponse {
    let new_hours = json.into_inner();
    let hours_entry = db.insert(new_hours);
    let id = hours_entry.id;
    HttpResponse::Created().body(id.to_string())
}

async fn delete_logged_hours<T: HoursRepo>(
    web::Path(id): web::Path<uuid::Uuid>,
    db: Data<T>,
) -> HttpResponse {
    let ok = db.delete(id);
    if !ok {
        return HttpResponse::NotFound().body(id.to_string());
    }
    HttpResponse::NoContent().finish()
}

pub fn run_server<T: HoursRepo + 'static>(hr: T, listener: TcpListener) -> io::Result<Server> {
    let db = Data::new(hr);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(redirect_to_api_doc))
            .service(
                web::scope("/api")
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
