use aura_components::{Paragraph, Space, Title};
use gpui::IntoElement;

pub fn page(
    title: &'static str,
    description: &'static str,
    body: impl IntoElement,
) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_xl()
        .child(header(title, description))
        .child(body)
}

pub fn section(
    title: &'static str,
    description: &'static str,
    body: impl IntoElement,
) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(header(title, description))
        .child(body)
}

pub fn header(title: &'static str, description: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_xs()
        .child(Title::new(title).h3())
        .child(Paragraph::with_text(description))
}

pub fn row(children: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().wrap().gap_lg().children(children)
}

pub fn row_md(children: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().wrap().gap_md().children(children)
}
