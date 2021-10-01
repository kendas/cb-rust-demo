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

#[derive(Debug, Serialize, Clone)]
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

async fn redirect_to_api_doc() -> HttpResponse {
    return HttpResponse::TemporaryRedirect()
        .header(header::LOCATION, "/openapi/index.html")
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
    return HttpResponse::Created().body(id.to_string());
}

async fn delete_logged_hours<T: HoursRepo>(
    web::Path(id): web::Path<uuid::Uuid>,
    db: Data<T>,
) -> HttpResponse {
    let ok = db.delete(id);
    if !ok {
        return HttpResponse::NotFound().body(id.to_string());
    }
    return HttpResponse::NoContent().finish();
}

trait HoursRepo {
    fn by_id(&self, id: Uuid) -> Option<Hours>;
    fn delete(&self, id: Uuid) -> bool;
    fn list(&self) -> Vec<Hours>;
    fn insert(&self, h: NewHours) -> Hours;
}

type MemDb = Mutex<Vec<Hours>>;

impl HoursRepo for MemDb {
    fn by_id(&self, id: uuid::Uuid) -> std::option::Option<Hours> {
        let guard = self.lock().unwrap();
        let result = guard.iter().find(|&h| h.id == id).map(|h| h.clone());
        return result;
    }
    fn delete(&self, id: uuid::Uuid) -> bool {
        let mut guard = self.lock().unwrap();
        let result = guard.iter().position(|h| h.id == id);
        match result {
            Some(hours_index) => {
                guard.remove(hours_index);
                return true;
            }
            None => false,
        }
    }
    fn list(&self) -> std::vec::Vec<Hours> {
        let guard = self.lock().unwrap();
        let all_hours = &*guard;
        return all_hours.to_vec();
    }
    fn insert(&self, h: NewHours) -> Hours {
        let mut guard = self.lock().unwrap();
        let hours_entry = Hours::new(h);
        guard.push(hours_entry.clone());
        return hours_entry;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Data<MemDb> = Data::new(Default::default());

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(redirect_to_api_doc))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/hours")
                            .route(web::get().to(list_all_logged_hours::<MemDb>))
                            .route(web::post().to(log_hours::<MemDb>)),
                    )
                    .service(
                        web::resource("/hours/{id}")
                            .route(web::get().to(get_single_hours_entry::<MemDb>))
                            .route(web::delete().to(delete_logged_hours::<MemDb>)),
                    ),
            )
            .service(Files::new("/openapi", "./openapi/").index_file("index.html"))
    })
    .bind(bind_address)?
    .run()
    .await
}
