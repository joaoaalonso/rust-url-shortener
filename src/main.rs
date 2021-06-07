mod configs;
mod handlers;
mod services;
mod repositories;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::index))
            .route("/", web::post().to(handlers::create))
            .route("/{id}", web::get().to(handlers::redirect))
    })
    .bind(format!("127.0.0.1:{}", configs::PORT))?
    .run()
    .await
}