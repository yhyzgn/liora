//! Standalone custom titlebar example.

use gpui::IntoElement;
use liora_components::{Button, Card, Flex, TitleBar};

pub fn titlebar_basic() -> impl IntoElement {
    Card::new(
        Flex::new()
            .overflow_hidden()
            .rounded_units(12.0)
            .border()
            .child(
                TitleBar::new()
                    .title("Project Atlas")
                    .subtitle("Custom native chrome")
                    .height_units(58.0)
                    .padding_x_units(20.0)
                    .gap_units(12.0)
                    .actions_gap_units(6.0)
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
