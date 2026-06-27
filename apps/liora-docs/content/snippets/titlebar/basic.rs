//! Standalone custom titlebar example.

use gpui::{IntoElement, div, prelude::*, px};
use liora_components::{Button, Card, TitleBar};

pub fn titlebar_basic() -> impl IntoElement {
    Card::new(
        div().overflow_hidden().rounded(px(12.0)).border_1().child(
            TitleBar::new()
                .title("Project Atlas")
                .subtitle("Custom native chrome")
                .window_controls(false)
                .action(Button::new("Share").small())
                .action(Button::new("Deploy").small().primary()),
        ),
    )
    .no_shadow()
}
