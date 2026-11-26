//! Full-width, non-shrinking cards in a narrow native scroll container.

use gpui::IntoElement;
use liora_components::{Card, Flex, Text};

pub fn card_layout() -> impl IntoElement {
    Flex::new()
        .column()
        .width_px(320.0)
        .height_px(220.0)
        .id("card-layout-scroll")
        .overflow_y_scroll()
        .gap_md()
        .child(
            Card::new(
                Flex::new()
                    .height_px(150.0)
                    .child(Text::new("Full-width body stays 150 px tall.").wrap()),
            )
            .id("card-layout-primary")
            .header(Text::new("Custom Header").bold())
            .footer(Text::new("Custom Footer"))
            .full_width()
            .no_shrink()
            .no_shadow(),
        )
        .child(
            Card::new("Scroll to this second card.")
                .id("card-layout-secondary")
                .title("Second Card")
                .full_width()
                .no_shrink()
                .no_shadow(),
        )
}
