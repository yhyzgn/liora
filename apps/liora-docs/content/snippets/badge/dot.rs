//! Dot badges.

use liora_components::{Avatar, Badge, Space, Text};

fn badge_dot() -> Space {
    Space::new()
        .wrap()
        .gap_lg()
        .child(Badge::new(Text::new("Query")).is_dot(true))
        .child(Badge::new(Avatar::new()).is_dot(true))
}

fn main() {
    let _ = badge_dot();
}
