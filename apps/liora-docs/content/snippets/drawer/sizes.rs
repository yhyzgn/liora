//! Configure drawer width or height depending on placement.

use gpui::IntoElement;
use liora_components::{Button, Drawer, DrawerPlacement, Space, Text};

pub fn drawer_size_buttons() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("Wide Drawer").on_click(|_, _, cx| {
            drawer("Wide Drawer", DrawerPlacement::Right)
                .width_lg()
                .show(cx);
        }))
        .child(Button::new("Tall Top Drawer").on_click(|_, _, cx| {
            drawer("Tall Top Drawer", DrawerPlacement::Top)
                .height_lg()
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
