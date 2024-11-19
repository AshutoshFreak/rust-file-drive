pub mod schema;
mod models;
mod handlers;
mod db;

use actix_web::{web, App, HttpServer};
use handlers::{upload_file, download_file};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let pool = db::establish_connection_pool();

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(
            web::scope("/api")
            .route("/upload", web::post().to(upload_file))
            .route("/files/{id}", web::get().to(download_file))
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
