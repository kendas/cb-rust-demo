use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or("8080".into());
    let bind_address = format!("0.0.0.0:{}", port);
    
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/openapi", "./openapi/").show_files_listing())
    })
    .bind(bind_address)?
    .run()
    .await
}
