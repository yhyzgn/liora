//! Badges with max value.

use liora_components::{Badge, Button, Space};

fn badge_max() -> Space {
    Space::new()
        .wrap()
        .gap_lg()
        .child(Badge::new(Button::new("Messages")).value("200").max(99))
        .child(Badge::new(Button::new("Updates")).value("50").max(10))
}

fn main() {
    let _ = badge_max();
}
