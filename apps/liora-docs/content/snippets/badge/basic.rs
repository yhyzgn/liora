//! Basic badges.

use liora_components::{Badge, BadgeType, Button, Space};

fn badge_basic() -> Space {
    Space::new()
        .wrap()
        .gap_lg()
        .child(Badge::new(Button::new("Messages")).value("5"))
        .child(
            Badge::new(Button::new("Updates"))
                .value("10")
                .badge_type(BadgeType::Primary),
        )
        .child(
            Badge::new(Button::new("Alerts"))
                .value("2")
                .badge_type(BadgeType::Warning),
        )
}

fn main() {
    let _ = badge_basic();
}
