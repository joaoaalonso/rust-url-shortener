mod configs;
mod handlers;
mod services;
mod repositories;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(handlers::create))
            .route("/{id}", web::get().to(handlers::redirect))
            .service(web::resource("/").route(web::get().to(handlers::index)))
            .service(web::resource("/dist/{_:.*}").route(web::get().to(handlers::dist)))
    })
    .bind(format!("127.0.0.1:{}", configs::PORT))?
    .run()
    .await
}