//! Avatar content sources.

use aura_components::{Avatar, Space};
use aura_icons_lucide::IconName;

fn avatar_content() -> Space {
    Space::new()
        .wrap()
        .gap_md()
        .child(Avatar::new().icon(IconName::User))
        .child(Avatar::new().icon(IconName::Star))
        .child(Avatar::new().src("https://github.com/zed-industries.png"))
}

fn main() {
    let _ = avatar_content();
}
