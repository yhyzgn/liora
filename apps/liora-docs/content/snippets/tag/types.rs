//! Tag semantic types.

use liora_components::{Space, Tag};

fn tag_types() -> Space {
    // Different tag types communicate lightweight status without changing layout.
    Space::new()
        .wrap()
        .gap_md()
        .child(Tag::new("Tag 1"))
        .child(Tag::new("Tag 2").success())
        .child(Tag::new("Tag 3").warning())
        .child(Tag::new("Tag 4").danger())
}

fn main() {
    let _ = tag_types();
}
