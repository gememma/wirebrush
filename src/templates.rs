use maud::{html, Markup, DOCTYPE};

const AUTHOR: &str = "Gemma Tipper";
const DESC: &str = "This is the personal website of Gemma Tipper, built in Rust.";

pub fn page(title: Option<&str>, contents: Markup) -> Markup {
    html! {
        (header(title))
        body {
            (contents)
            (footer())
        }
    }
}

fn header(page_title: Option<&str>) -> Markup {
    let title = match page_title {
        None => AUTHOR.to_string(),
        Some(_) => {
            format!("{} | {}", page_title.unwrap(), AUTHOR)
        }
    };
    html! {
        head {
            (DOCTYPE)
            meta charset="utf-8";
            meta viewport="width=device-width, initial-scale=1";
            meta description=(DESC);
            meta author=(AUTHOR);
            link rel="stylesheet" href="/style.css" type="text/css";
            title { (title) }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            description { (DESC) };
        }
    }
}
