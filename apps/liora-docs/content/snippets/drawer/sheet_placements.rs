//! Lightweight Drawer sheet-style placement examples.

use gpui::IntoElement;
use liora_components::{Button, Drawer, Space, Text};

pub fn drawer_sheet_buttons() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("Right").on_click(|_, _, cx| {
            Drawer::sheet()
                .title("Inspector")
                .right()
                .content_view(|_| sheet_body("Right inspector"))
                .show(cx);
        }))
        .child(Button::new("Left").on_click(|_, _, cx| {
            Drawer::sheet()
                .title("Navigator")
                .left()
                .content_view(|_| sheet_body("Left navigator"))
                .show(cx);
        }))
        .child(Button::new("Top").on_click(|_, _, cx| {
            Drawer::sheet()
                .title("Command")
                .top()
                .height_sm()
                .content_view(|_| sheet_body("Top command"))
                .show(cx);
        }))
        .child(Button::new("Bottom").on_click(|_, _, cx| {
            Drawer::sheet()
                .title("Actions")
                .bottom()
                .height_sm()
                .content_view(|_| sheet_body("Bottom actions"))
                .show(cx);
        }))
}

fn sheet_body(title: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Text::new(title).bold())
}
