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
        Ok(templates::page(None, html! {(PreEscaped(content))}))
    } else {
        Err(ErrorNotFound(""))
    }
}
