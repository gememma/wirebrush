pub mod handler;
pub mod templates;

use crate::handler::{health, home, page, stylesheet};
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use pulldown_cmark::{html, Options, Parser};
use std::fs::{read_dir, read_to_string};
use std::io::Error;
use std::path::Path;
use std::sync::Arc;
use std::{collections::HashMap, env};
use tracing::{info, warn};

#[derive(Clone)]
struct AppState {
    pages: Arc<HashMap<String, String>>,
}

fn parse_md_to_html(markdown_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn read_content(path: &Path) -> Result<HashMap<String, String>, Error> {
    let mut pages = HashMap::new();
    match read_dir(path) {
        Ok(files) => {
            for file in files {
                let file_path = file?.path();
                let markdown_input = read_to_string(&file_path)?;

                pages.insert(
                    file_path
                        .file_stem()
                        .expect("Error getting filename")
                        .to_str()
                        .expect("Error converting filename to string")
                        .to_string(),
                    parse_md_to_html(&markdown_input),
                );
            }
        }
        Err(err) => {
            warn!(%err, "failed to read pages from content folder");
            return Err(err);
        }
    }
    Ok(pages)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging first
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let content_path = if let Ok(path) = env::var("WIREBRUSH_CONTENT") {
        path
    } else {
        "content".to_owned()
    };

    let static_path = if let Ok(path) = env::var("WIREBRUSH_STATIC") {
        path
    } else {
        "static".to_owned()
    };

    // Read files from content directory
    let pages = match read_content(Path::new(&content_path)) {
        Ok(files) => files,
        Err(err) => {
            warn!(%err, "failed to read pages from content folder");
            HashMap::new()
        }
    };
    let pages = Arc::new(pages);
    let app_state = AppState { pages };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(home)
            .service(health)
            .service(stylesheet)
            .service(Files::new("/static", static_path.clone()).prefer_utf8(true))
            .service(page)
    })
    .bind(("127.0.0.1", 8000))?
    .run();
    info!("listening");
    server.await
}
