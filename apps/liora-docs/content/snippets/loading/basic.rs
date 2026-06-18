//! Basic Loading indicators.

use gpui::IntoElement;
use liora_components::{Loading, Space};

pub fn basic_loading() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_lg()
        .child(Loading::new())
        .child(Loading::new().text("Loading..."))
}
