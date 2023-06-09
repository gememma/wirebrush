use maud::{html, Markup, DOCTYPE};

pub fn page(title: &str, contents: Markup) -> Markup {
    html! {
        (header(title))
        (contents)
        (footer())
    }
}

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta viewport="width=device-width, initial-scale=1";
        meta description="Personal website of Gemma Tipper, build in Rust";
        meta author="Gemma Tipper";
        title { (page_title) }
    }
}

fn footer() -> Markup {
    html! {
        footer {
        }
    }
}
