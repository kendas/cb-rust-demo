use actix_files::Files;
use actix_web::{
    web::{self, Data},
    App, HttpServer, Responder,
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Hours {
    id: String,
    employee: String,
    project: String,
    story_id: Option<String>,
    description: String,
    hours: i16,
}

async fn db_test(db: Data<Db>) -> impl Responder {
    let mut guard = db.lock().unwrap();
    guard.push(Hours {
        id: "".into(),
        employee: "".into(),
        project: "".into(),
        story_id: None,
        description: "".into(),
        hours: 0,
    });
    println!("{:?}", guard);
    format!("Welcome {}!", "here")
}

type Db = Mutex<Vec<Hours>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Data<Db> = Data::new(Default::default());

    let port = std::env::var("PORT").unwrap_or("8080".into());
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
