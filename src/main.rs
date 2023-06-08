use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    tracing::info!("Responding to GET at /");
    HttpResponse::Ok().body("<h1>Hello BrushHeads!</h1>")
}

#[get("/health")]
async fn health() -> impl Responder {
    tracing::info!("Responding to GET at /health");
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
    tracing::info!("Listening...");
    server.await
}
