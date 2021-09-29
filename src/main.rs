use std::sync::Mutex;

use actix_files::Files;
use actix_web::{http::header, web, web::Data, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct NewHours {
    employee: String,
    project: String,
    story_id: Option<String>,
    description: String,
    hours: i16,
}

#[derive(Debug, Serialize)]
struct Hours {
    id: Uuid,
    employee: String,
    project: String,
    story_id: Option<String>,
    description: String,
    hours: i16,
}

impl Hours {
    fn new(new_hours: NewHours) -> Hours {
        return Hours {
            id: Uuid::new_v4(),
            employee: new_hours.employee,
            project: new_hours.project,
            story_id: new_hours.story_id,
            description: new_hours.description,
            hours: new_hours.hours,
        };
    }
}

async fn db_test(db: Data<Db>) -> HttpResponse {
    let mut guard = db.lock().unwrap();
    guard.push(Hours {
        id: Uuid::new_v4(),
        employee: "".into(),
        project: "".into(),
        story_id: None,
        description: "".into(),
        hours: 0,
    });
    let response_body = format!("Welcome, the database contains {:?}", guard);
    return HttpResponse::Ok().body(response_body);
}

async fn list_all_logged_hours(db: Data<Db>) -> HttpResponse {
    let guard = db.lock().unwrap();
    let all_hours = &*guard;
    return HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .json(all_hours);
}

async fn get_single_hours_entry(
    web::Path(id): web::Path<uuid::Uuid>,
    db: Data<Db>,
) -> HttpResponse {
    let guard = db.lock().unwrap();
    let result = guard.iter().find(|&h| h.id == id);
    if let Some(hours) = result {
        return HttpResponse::Ok().json(hours);
    }
    return HttpResponse::NotFound().body(id.to_string());
}

async fn log_hours(db: Data<Db>, json: web::Json<NewHours>) -> HttpResponse {
    let mut guard = db.lock().unwrap();
    let new_hours = json.into_inner();
    let hours_entry = Hours::new(new_hours);
    let id = hours_entry.id;
    guard.push(hours_entry);
    return HttpResponse::Created().body(id.to_string());
}

type Db = Mutex<Vec<Hours>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Data<Db> = Data::new(Default::default());

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(db_test))
            .service(
                web::scope("/api")
                    .route("/hours", web::get().to(list_all_logged_hours))
                    .route("/hours", web::post().to(log_hours))
                    .route("/hours/{id}", web::get().to(get_single_hours_entry)),
            )
            .service(Files::new("/openapi", "./openapi/").show_files_listing())
    })
    .bind(bind_address)?
    .run()
    .await
}
