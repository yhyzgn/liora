//! Card footer with native action buttons.

use liora_components::{Button, Card, Row, RowJustify, toast_info};

pub fn card_with_footer() -> Card {
    Card::new("Card body with a custom footer.")
        .title("Card with Footer")
        .width_lg()
        .footer(
            Row::new()
                .justify(RowJustify::End)
                .child(Button::new("Cancel").small().on_click(|_, _, _| {
                    toast_info!("Cancel requested (demo)");
                }))
                .child(Button::new("Save").primary().small().on_click(|_, _, _| {
                    toast_info!("Save requested (demo)");
                })),
        )
}
