//! Basic Popover with card content.

use liora_components::{Button, Popover, Space, Text};
use liora_core::Placement;

pub fn basic_popover() -> Popover {
    Popover::new(Button::new("Bottom Center").primary())
        .id("docs-popover-basic")
        .placement(Placement::Bottom)
        .content(|_, _| {
            Space::new()
                .vertical()
                .gap_sm()
                .child(Text::new("Title").bold())
                .child(Text::new("This is native GPUI popover content."))
                .child(Button::new("Confirm").primary().small())
        })
}

fn main() {
    let _ = basic_popover();
}
