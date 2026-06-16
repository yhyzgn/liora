use aura_components::Label;
use aura_icons_lucide::IconName;
use gpui::{IntoElement, green, px};

pub fn label_basic() -> impl IntoElement {
    Label::new("Build passed")
        .icon(IconName::CircleCheck)
        .gap(px(8.0))
        .size(px(14.0))
        .color(green())
}
