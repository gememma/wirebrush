pub mod templates;

use actix_web::error::ErrorNotFound;
use actix_web::http::header::ContentType;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result as AwResult};
use maud::{html, Markup};
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::path::Path;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Clone)]
struct AppState {
    pages: Arc<HashMap<String, String>>,
}

#[get("/")]
async fn hello() -> AwResult<Markup> {
    info!("responding to GET at /");
    Ok(templates::page(None, html!(h1 { "Hello BrushHeads!" })))
}

#[get("/title")]
async fn title() -> AwResult<Markup> {
    info!("responding to GET at /title");
    Ok(templates::page(
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

#[get("/{page}")]
async fn page(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let path_name = path.into_inner();
    info!("responding to GET at /{path_name}");
    if let Some(content) = data.into_inner().pages.get(&path_name) {
        Ok(templates::page(None, html! {(content)}))
    } else {
        Err(ErrorNotFound(""))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging first
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Read files from content directory
    let mut pages = HashMap::new();
    if let Ok(files) = read_dir(Path::new("content/")) {
        for file in files {
            let file_path = file.expect("IO error").path();
            pages.insert(
                file_path
                    .file_stem()
                    .expect("Path error")
                    .to_str()
                    .expect("String error")
                    .to_string(),
                read_to_string(file_path).expect("Error reading file contents"),
            );
        }
    } else {
        warn!("No pages found, content folder is missing or empty")
    }
    let pages = Arc::new(pages);
    let app_state = AppState { pages };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(hello)
            .service(health)
            .service(title)
            .service(stylesheet)
            .service(page)
    })
    .bind(("127.0.0.1", 8000))?
    .run();
    info!("listening");
    server.await
}
