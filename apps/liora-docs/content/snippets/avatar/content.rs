//! Avatar content sources.

use liora_components::{Avatar, Space};
use liora_icons_lucide::IconName;

fn avatar_content() -> Space {
    Space::new()
        .wrap()
        .gap_md()
        .child(Avatar::new().icon(IconName::User))
        .child(Avatar::new().icon(IconName::Star))
        .child(Avatar::new().src("https://github.com/zed-industries.png"))
        .child(
            Avatar::new()
                .src("http://127.0.0.1:0/missing-avatar.png")
                .alt("AB"),
        )
}

fn main() {
    let _ = avatar_content();
}
