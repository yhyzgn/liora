//! Basic Card examples, including hover feedback.

use aura_components::{Card, Space};
use gpui::IntoElement;

pub fn basic_cards() -> impl IntoElement {
    Space::new().wrap().gap_md().children(vec![
        Card::new("Standard card content goes here.")
            .title("Standard Card")
            .width_md(),
        Card::new("This card will change shadow on hover.")
            .title("Hoverable Card")
            .hoverable()
            .width_md(),
    ])
}
