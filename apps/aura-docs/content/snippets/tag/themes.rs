//! Tag visual themes.

use aura_components::{Space, Tag};

fn tag_themes() -> Space {
    // Dark and plain effects are useful for denser status surfaces.
    Space::new()
        .vertical()
        .gap_md()
        .child(
            Space::new()
                .wrap()
                .gap_md()
                .child(Tag::new("Dark").dark())
                .child(Tag::new("Success").success().dark())
                .child(Tag::new("Warning").warning().dark())
                .child(Tag::new("Danger").danger().dark()),
        )
        .child(
            Space::new()
                .wrap()
                .gap_md()
                .child(Tag::new("Plain").plain())
                .child(Tag::new("Success").success().plain())
                .child(Tag::new("Warning").warning().plain())
                .child(Tag::new("Danger").danger().plain()),
        )
}

fn main() {
    let _ = tag_themes();
}
