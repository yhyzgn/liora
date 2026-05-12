//! Rounded tags.

use aura_components::{Space, Tag};

fn round_tags() -> Space {
    // `round(true)` switches the radius to a pill-like shape.
    Space::new()
        .wrap()
        .gap_md()
        .child(Tag::new("Tag 1").round(true))
        .child(Tag::new("Tag 2").success().round(true))
        .child(Tag::new("Tag 3").warning().round(true))
        .child(Tag::new("Tag 4").danger().round(true))
}

fn main() {
    let _ = round_tags();
}
