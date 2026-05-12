//! Avatar shapes.

use aura_components::{Avatar, Space};

fn avatar_shapes() -> Space {
    Space::new()
        .wrap()
        .gap_md()
        .child(Avatar::new())
        .child(Avatar::new().square())
}

fn main() {
    let _ = avatar_shapes();
}
