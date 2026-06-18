//! 24-column grid built with Row and Col.

use gpui::{App, Hsla, IntoElement};
use liora_components::{Col, Flex, Row, Space};
use liora_core::Config;

pub fn grid_examples(cx: &mut App) -> impl IntoElement {
    let theme = &cx.global::<Config>().theme;
    Space::new()
        .vertical()
        .gap_sm()
        .child(Row::new().column(Col::new(24).child(grid_box(theme, "span 24", gpui::blue()))))
        .child(
            Row::new()
                .column(Col::new(12).child(grid_box(theme, "span 12", gpui::red())))
                .column(Col::new(12).child(grid_box(theme, "span 12", gpui::green()))),
        )
        .child(
            Row::new()
                .column(Col::new(8).child(grid_box(theme, "span 8", gpui::blue())))
                .column(Col::new(8).child(grid_box(theme, "span 8", gpui::red())))
                .column(Col::new(8).child(grid_box(theme, "span 8", gpui::green()))),
        )
}

fn grid_box(theme: &liora_theme::Theme, text: &str, color: Hsla) -> impl IntoElement {
    Flex::new()
        .row()
        .bg(color.opacity(0.5))
        .height_units(36.0)
        .rounded_units(4.0)
        .center()
        .text_color(theme.neutral.text_1)
        .text_xs()
        .child(text.to_string())
}
