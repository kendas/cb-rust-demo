use actix_web::{
    web::{self, Data, Path},
    HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::HoursRepo;
use crate::error::{ErrorResponse, Validated};

mod model;

pub use model::{Hours, NewHours};

pub async fn list_all_logged_hours(db: Data<PgPool>) -> HttpResponse {
    let mut connection = db.acquire().await.unwrap();
    let all_hours = connection.list().await;
    HttpResponse::Ok().json(all_hours)
}

pub async fn get_single_hours_entry(id: Path<Uuid>, db: Data<PgPool>) -> HttpResponse {
    let mut connection = db.acquire().await.unwrap();
    let id = id.into_inner();
    match connection.by_id(id).await {
        Some(hours) => HttpResponse::Ok().json(hours),
        None => HttpResponse::NotFound().json(id),
    }
}

pub async fn log_hours(db: Data<PgPool>, json: web::Json<NewHours>) -> HttpResponse {
    let new_hours = json.into_inner();
    match new_hours.validate() {
        Err(errors) => HttpResponse::BadRequest().json(ErrorResponse::with_validation_errors(
            "Validation errors".into(),
            errors,
        )),
        Ok(_) => {
            let mut connection = db.acquire().await.unwrap();
            let hours_entry = connection.insert(new_hours).await;
            HttpResponse::Created().json(hours_entry)
        }
    }
}

pub async fn delete_logged_hours(id: Path<Uuid>, db: Data<PgPool>) -> HttpResponse {
    let mut connection = db.acquire().await.unwrap();
    let id = id.into_inner();
    match connection.delete(id).await {
        true => HttpResponse::NoContent().finish(),
        false => HttpResponse::NotFound().json(id),
    }
}
