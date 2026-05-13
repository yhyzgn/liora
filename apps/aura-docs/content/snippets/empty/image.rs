//! Empty with a custom icon node as the illustration.

use aura_components::{Card, Empty};
use aura_icons::Icon;
use aura_icons_lucide::IconName;

pub fn empty_with_image() -> Card {
    Card::new(
        Empty::new()
            .image(Icon::new(IconName::Search))
            .description("没有找到相关内容"),
    )
    .width_md()
}
