//! Card footer with native action buttons.

use aura_components::{Button, Card, Row, RowJustify};

pub fn card_with_footer() -> Card {
    Card::new("Card body with a custom footer.")
        .title("Card with Footer")
        .width_lg()
        .footer(
            Row::new()
                .justify(RowJustify::End)
                .child(Button::new("Cancel").small())
                .child(Button::new("Save").primary().small()),
        )
}
