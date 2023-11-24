use crate::{templates, AppState};
use actix_web::error::ErrorNotFound;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Responder, Result as AwResult};
use maud::{html, Markup, PreEscaped};
use tracing::info;

#[get("/")]
async fn home(data: web::Data<AppState>) -> AwResult<Markup> {
    info!("responding to GET at /");
    if let Some(content) = data.into_inner().pages.get("home") {
        Ok(templates::page(None, html! {(PreEscaped(content))}))
    } else {
        Err(ErrorNotFound(""))
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    info!("responding to GET at /health");
    HttpResponse::Ok().body("OK")
}

const STYLESHEET: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

#[get("/style.css")]
async fn stylesheet() -> impl Responder {
    info!("responding to GET at /style.css");
    HttpResponse::Ok()
        .content_type(ContentType(mime::TEXT_CSS))
        .body(STYLESHEET)
}

#[get("/{page}")]
async fn page(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let path_name = path.into_inner();
    info!("responding to GET at /{path_name}");
    if let Some(content) = data.into_inner().pages.get(&path_name) {
        Ok(templates::page(None, html! {(PreEscaped(content))}))
    } else {
        Err(ErrorNotFound(""))
    }
}
