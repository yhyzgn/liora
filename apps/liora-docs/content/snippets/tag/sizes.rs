//! Tag sizes.

use liora_components::{Space, Tag};

fn tag_sizes() -> Space {
    Space::new()
        .wrap()
        .gap_md()
        .child(Tag::new("Default"))
        .child(Tag::new("Large").large())
        .child(Tag::new("Small").small())
}

fn main() {
    let _ = tag_sizes();
}
