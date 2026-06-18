//! Open Drawer from each window edge.

use gpui::IntoElement;
use liora_components::{Button, Drawer, DrawerPlacement, Space, Text};

pub fn drawer_placement_buttons() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("Right Drawer").primary().on_click(|_, _, cx| {
            drawer("Right Drawer", DrawerPlacement::Right).show(cx);
        }))
        .child(Button::new("Left Drawer").on_click(|_, _, cx| {
            drawer("Left Drawer", DrawerPlacement::Left).show(cx);
        }))
        .child(Button::new("Top Drawer").on_click(|_, _, cx| {
            drawer("Top Drawer", DrawerPlacement::Top)
                .height_sm()
                .show(cx);
        }))
        .child(Button::new("Bottom Drawer").on_click(|_, _, cx| {
            drawer("Bottom Drawer", DrawerPlacement::Bottom)
                .height_sm()
                .show(cx);
        }))
}

fn drawer(title: &'static str, placement: DrawerPlacement) -> Drawer {
    Drawer::new()
        .title(title)
        .placement(placement)
        .content(move |_, _| {
            Space::new()
                .vertical()
                .gap_lg()
                .child(Text::new(format!("This is a {:?} drawer.", placement)))
                .child(
                    Button::new("Close")
                        .primary()
                        .on_click(|_, _, cx| Drawer::close(cx)),
                )
        })
}
