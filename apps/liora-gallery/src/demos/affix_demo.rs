use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::{Affix, Flex, Text};
use liora_core::Config;

use liora_components::layout_helpers::page;

#[path = "../../../liora-docs/content/snippets/affix/bottom.rs"]
mod bottom_example;
#[path = "../../../liora-docs/content/snippets/affix/top.rs"]
mod top_example;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| AffixDemo {
        top_affix: cx.new(|_| top_example::top_affix()),
        bottom_affix: cx.new(|_| bottom_example::bottom_affix()),
    })
    .into()
}

struct AffixDemo {
    top_affix: Entity<Affix>,
    bottom_affix: Entity<Affix>,
}

impl Render for AffixDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        page(
            "Affix 固钉",
            "将内容固定在特定可视区域。",
            Flex::new()
                .relative()
                .height_units(560.0)
                .overflow_hidden()
                .border()
                .border_color(theme.neutral.border)
                .rounded_md()
                .bg(theme.neutral.hover)
                .child(
                    Flex::new()
                        .size_full()
                        .id("affix-scroll-view")
                        .overflow_y_scroll()
                        .padding_md()
                        .child(Flex::new().height_units(200.0).center().child(
                            Text::new("向下滚动查看固钉效果").text_color(theme.neutral.text_3),
                        ))
                        .child(self.top_affix.clone())
                        .child(
                            Flex::new()
                                .height_units(800.0)
                                .bg(theme.neutral.card)
                                .margin_y_units(16.0)
                                .border()
                                .border_color(theme.neutral.border)
                                .center()
                                .child(Text::new("长内容占位")),
                        )
                        .child(self.bottom_affix.clone())
                        .child(Flex::new().height_units(400.0)),
                ),
        )
    }
}
