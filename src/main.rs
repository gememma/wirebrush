use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result as AwResult};
use maud::{html, Markup};
use tracing::info;

#[get("/")]
async fn hello() -> AwResult<Markup> {
    info!("responding to GET at /");
    Ok(html! { html {
        body {
            h1 { "Hello BrushHeads!" }
        }
    }})
}

#[get("/health")]
async fn health() -> impl Responder {
    info!("responding to GET at /health");
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging first
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let server = HttpServer::new(|| App::new().service(hello).service(health))
        .bind(("127.0.0.1", 8000))?
        .run();
    info!("listening");
    server.await
}
