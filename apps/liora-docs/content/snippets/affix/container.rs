//! Affix requires a scrollable native container around long content.
//!
//! In a real view `Affix` is stored as `Entity<Affix>` and inserted into the
//! marked location below. The container shape is still useful as a standalone
//! snippet because it shows the required scroll/relative layout contract.

use gpui::{App, IntoElement};
use liora_components::{Flex, Text};
use liora_core::Config;

pub fn affix_scroll_container(cx: &mut App) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();

    Flex::new()
        .relative()
        .height_units(360.0)
        .overflow_hidden()
        .border()
        .border_color(theme.neutral.border)
        .rounded_md()
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
                        .child(Text::new("向下滚动")),
                )
                // .child(affix_entity.clone())
                .child(
                    Flex::new()
                        .height_units(520.0)
                        .center()
                        .child(Text::new("长内容占位")),
                ),
        )
}
