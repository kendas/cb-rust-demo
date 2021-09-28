use std::sync::Mutex;

use actix_files::Files;
use actix_web::{
    web::{self, Data},
    App, HttpServer, Responder,
};
use uuid::Uuid;

#[derive(Debug)]
struct Hours {
    id: Uuid,
    employee: String,
    project: String,
    story_id: Option<String>,
    description: String,
    hours: i16,
}

async fn db_test(db: Data<Db>) -> impl Responder {
    let mut guard = db.lock().unwrap();
    guard.push(Hours {
        id: Uuid::new_v4(),
        employee: "".into(),
        project: "".into(),
        story_id: None,
        description: "".into(),
        hours: 0,
    });
    format!("Welcome, the database contains {:?}", guard)
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
            .service(Files::new("/openapi", "./openapi/").show_files_listing())
    })
    .bind(bind_address)?
    .run()
    .await
}
