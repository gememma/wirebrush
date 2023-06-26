pub mod templates;

use crate::templates::page;
use actix_web::http::header::ContentType;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result as AwResult};
use maud::{html, Markup};
use tracing::info;

#[get("/")]
async fn hello() -> AwResult<Markup> {
    info!("responding to GET at /");
    Ok(page(None, html!(h1 { "Hello BrushHeads!" })))
}

#[get("/title")]
async fn title() -> AwResult<Markup> {
    info!("responding to GET at /title");
    Ok(page(
        Some("This is a title"),
        html!(h1 { "This page has a title!" }),
    ))
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging first
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let server = HttpServer::new(|| App::new().service(hello).service(title).service(health))
        .bind(("127.0.0.1", 8000))?
        .run();
    info!("listening");
    server.await
}
