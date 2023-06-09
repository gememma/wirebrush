use maud::{html, Markup, DOCTYPE};

pub fn page(title: &str, contents: Markup) -> Markup {
    html! {
        (header(title))
        h1 { (title) }
        (contents)
        (footer())
    }
}

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
    }
}

fn footer() -> Markup {
    html! {
        footer {
        }
    }
}
