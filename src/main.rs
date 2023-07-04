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
        html!(
            h1 { "This page has a special title!" }
            p { "It also has some body text. Lorem ipsum dolor sit amet whatever whatever." }
            h3 { "And this is a Heading 3." }
            p { "To be fair, you have to have a very high IQ to understand Rick and Morty. The humour is extremely subtle, and without a solid grasp of theoretical physics most of the jokes will go over a typical viewer’s head. There’s also Rick’s nihilistic outlook, which is deftly woven into his characterisation- his personal philosophy draws heavily from Narodnaya Volya literature, for instance. The fans understand this stuff; they have the intellectual capacity to truly appreciate the depths of these jokes, to realise that they’re not just funny—they say something deep about LIFE. As a consequence people who dislike Rick & Morty truly ARE idiots- of course they wouldn’t appreciate, for instance, the humour in Rick’s existential catchphrase “Wubba Lubba Dub Dub,” which itself is a cryptic reference to Turgenev’s Russian epic Fathers and Sons. I’m smirking right now just imagining one of those addlepated simpletons scratching their heads in confusion as Dan Harmon’s genius wit unfolds itself on their television screens." }
            li { "You gotta remember to buy some milk" }
            li { "You gotta remember to buy bread too" }
        ),
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

    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(health)
            .service(title)
            .service(stylesheet)
    })
    .bind(("127.0.0.1", 8000))?
    .run();
    info!("listening");
    server.await
}
