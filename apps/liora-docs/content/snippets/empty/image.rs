//! Empty with a custom icon node as the illustration.

use liora_components::{Card, Empty};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn empty_with_image() -> Card {
    Card::new(
        Empty::new()
            .image(Icon::new(IconName::Search))
            .description("没有找到相关内容"),
    )
    .width_md()
}
