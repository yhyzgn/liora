//! Badges with max value.

use liora_components::{Badge, Button, Space};

pub fn badge_max() -> Space {
    Space::new()
        .wrap()
        .gap_lg()
        .child(
            Badge::new(Button::new("Messages").id("badge-max-messages"))
                .value("200")
                .max(99),
        )
        .child(
            Badge::new(Button::new("Updates").id("badge-max-updates"))
                .value("50")
                .max(10),
        )
        .child(
            Badge::new(Button::new("Large count"))
                .value("4294967295")
                .max(99),
        )
        .child(Badge::new(Button::new("Status")).value("New").max(99))
}
