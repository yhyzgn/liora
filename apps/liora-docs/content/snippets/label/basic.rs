use gpui::{IntoElement, green, px};
use liora_components::Label;
use liora_icons_lucide::IconName;

pub fn label_basic() -> impl IntoElement {
    Label::new("Build passed")
        .icon(IconName::CircleCheck)
        .gap(px(8.0))
        .size(px(14.0))
        .color(green())
}
