//! Basic badges.

use liora_components::{Badge, BadgeType, Button, Space};

pub fn badge_basic() -> Space {
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
        .child(
            Badge::new(Button::new("Completed"))
                .value("0")
                .badge_type(BadgeType::Success),
        )
        .child(
            Badge::new(Button::new("Info"))
                .value("New")
                .badge_type(BadgeType::Info),
        )
}
