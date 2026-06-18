//! Avatar sizes.

use liora_components::{Avatar, Space};

fn avatar_sizes() -> Space {
    Space::new()
        .wrap()
        .gap_md()
        .child(Avatar::new().small())
        .child(Avatar::new())
        .child(Avatar::new().large())
}

fn main() {
    let _ = avatar_sizes();
}
