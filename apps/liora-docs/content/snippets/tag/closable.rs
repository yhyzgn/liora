//! Closable tags.

use liora_components::{Space, Tag};

fn closable_tags() -> Space {
    // `closable(true)` shows the close affordance; `on_close` can remove the item.
    Space::new()
        .wrap()
        .gap_md()
        .child(Tag::new("Tag 1").closable(true))
        .child(Tag::new("Tag 2").success().closable(true))
        .child(Tag::new("Tag 3").warning().closable(true))
        .child(Tag::new("Tag 4").danger().closable(true))
}

fn main() {
    let _ = closable_tags();
}
