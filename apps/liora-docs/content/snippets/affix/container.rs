//! Affix requires a scrollable native container around long content.
//!
//! Retain `Affix` as `Entity<Affix>` in the host view and pass it to this container.

use gpui::{App, Entity, IntoElement};
use liora_components::{Affix, Flex, Text};
use liora_core::Config;

pub fn affix_scroll_container(cx: &mut App, affix: Entity<Affix>) -> impl IntoElement {
    let theme = &cx.global::<Config>().theme;

    Flex::new()
        .relative()
        .height_units(360.0)
        .overflow_hidden()
        .border()
        .border_color(theme.neutral.border)
        .rounded_md()
        .bg(theme.neutral.hover)
        .child(
            Flex::new()
                .size_full()
                .id("docs-affix-scroll-view")
                .overflow_y_scroll()
                .padding_md()
                .child(
                    Flex::new()
                        .height_units(120.0)
                        .center()
                        .child(Text::new("向下滚动查看固钉效果").text_color(theme.neutral.text_3)),
                )
                .child(affix)
                .child(
                    Flex::new()
                        .height_units(520.0)
                        .bg(theme.neutral.card)
                        .margin_y_units(16.0)
                        .border()
                        .border_color(theme.neutral.border)
                        .center()
                        .child(Text::new("长内容占位")),
                )
                .child(Flex::new().height_units(180.0)),
        )
}
