use super::configs;
use super::services;

use std::borrow::Cow;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use actix_web::body::Body;
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

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => {
      let body: Body = match content {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
      };
      HttpResponse::Ok().content_type(from_path(path).first_or_octet_stream().as_ref()).body(body)
    }
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

pub fn index() -> HttpResponse {
  handle_embedded_file("index.html")
}

pub fn dist(path: web::Path<String>) -> HttpResponse {
  handle_embedded_file(&path.0)
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