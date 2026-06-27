//! Standalone custom titlebar example.

use gpui::{IntoElement, div, prelude::*, px};
use liora_components::{Button, Card, TitleBar};

pub fn titlebar_basic() -> impl IntoElement {
    Card::new(
        div().overflow_hidden().rounded(px(12.0)).border_1().child(
            TitleBar::new()
                .title("Project Atlas")
                .subtitle("Custom native chrome")
                .height(px(58.0))
                .padding_x(px(20.0))
                .gap(px(12.0))
                .actions_gap(px(6.0))
                .background(gpui::transparent_black())
                .border(false)
                .content_align(liora_components::TitleBarContentAlign::Start)
                .window_controls_position(liora_components::WindowControlsPosition::Right)
                .window_controls(false)
                .action(Button::new("Share").small())
                .action(Button::new("Deploy").small().primary()),
        ),
    )
    .no_shadow()
}
