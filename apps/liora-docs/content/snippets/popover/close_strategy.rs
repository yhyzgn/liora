//! Popover close strategy and offset configuration.

use gpui::IntoElement;
use liora_components::{Button, Popover, Space, Text};
use liora_core::{Placement, clear_popover};

pub fn popover_close_strategy() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_lg()
        .child(
            Popover::new(Button::new("Manual Close Only").warning())
                .id("docs-popover-manual-close")
                .placement(Placement::Bottom)
                .close_on_click_outside(false)
                .close_on_escape(false)
                .content(|_, _| {
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Text::new("Manual close").bold())
                        .child(Text::new("Backdrop and ESC are disabled for this popover."))
                        .child(Button::new("Close Popover").primary().small().on_click(
                            |_, _, cx| {
                                clear_popover(&"docs-popover-manual-close".into(), cx);
                            },
                        ))
                }),
        )
        .child(
            Popover::new(Button::new("Custom Offset"))
                .id("docs-popover-custom-offset")
                .placement(Placement::Bottom)
                .offset_lg()
                .content(|_, _| Text::new("Offset = 20px")),
        )
}
