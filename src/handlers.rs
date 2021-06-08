use super::configs;
use super::services;

use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, Responder, web, http};

#[derive(Deserialize)]
pub struct InputURL {
    url: String
}

#[derive(Serialize)]
struct ResponseMessage {
    message: &'static str
}

#[derive(Serialize)]
struct ResponseUrl {
    url: String
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(ResponseMessage {
        message: "Server is up and running!"
    })
}

pub async fn redirect(web::Path(id): web::Path<String>) -> impl Responder {
    let url: String;
    
    match services::get_url_by_id(&id) {
        Ok(u) => url = u,
        Err(_m) => url = String::from("/")
    }

    HttpResponse::Found()
        .set_header(http::header::LOCATION, url)
        .finish()
}

pub async fn create(input: web::Json<InputURL>) -> impl Responder {
    let id = services::shorten_url(&input.url);

    HttpResponse::Ok().json(ResponseUrl {
        url: format!("{}/{}", configs::PUBLIC_URL, id)
    })
}