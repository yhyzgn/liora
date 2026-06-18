//! Preview can wrap any native trigger element.

use gpui::{App, IntoElement};
use liora_components::{Card, Preview, Space, Text};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn preview_custom_trigger(cx: &mut App) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();
    let local = format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR"));

    Preview::new(local).child(
        Card::new(
            Space::new()
                .gap_md()
                .child(
                    Icon::new(IconName::Image)
                        .size_lg()
                        .color(theme.primary.base),
                )
                .child(
                    Space::new()
                        .vertical()
                        .gap_xs()
                        .child(Text::new("点击查看大图").bold())
                        .child(Text::new("Preview 可以包裹卡片、按钮或其他元素。")),
                ),
        )
        .no_shadow(),
    )
}
