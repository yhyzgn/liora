use gpui::IntoElement;
use liora_components::{Link, Space};

pub fn keyboard_links() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_lg()
        .child(
            Link::new("First link")
                .id("link-keyboard-first")
                .href("https://example.com/first"),
        )
        .child(Link::new("No destination"))
        .child(
            Link::new("Disabled link")
                .disabled(true)
                .href("https://example.com/disabled"),
        )
        .child(
            Link::new("Last link")
                .id("link-keyboard-last")
                .href("https://example.com/last"),
        )
}
