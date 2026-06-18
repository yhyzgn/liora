//! Link underline control.

use gpui::IntoElement;
use liora_components::{Link, Space};

pub fn underline_links() -> impl IntoElement {
    Space::new().wrap().gap_lg().children(vec![
        Link::new("With underline").href("https://github.com"),
        Link::new("No underline")
            .underline(false)
            .href("https://github.com"),
    ])
}
