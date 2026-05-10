use aura_components::{Affix, AffixPosition, Button, ButtonVariant, Flex, Text};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*};

use super::common::page;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| AffixDemo {
        top_affix: cx.new(|_| {
            Affix::new().offset_lg().content(|_, _| {
                Button::new("固钉在距离顶部 80px 的位置")
                    .variant(ButtonVariant::Primary)
                    .into_any_element()
            })
        }),
        bottom_affix: cx.new(|_| {
            Affix::new()
                .position(AffixPosition::Bottom)
                .offset_md()
                .content(|_, _| {
                    Button::new("固钉在距离底部 20px 的位置")
                        .variant(ButtonVariant::Success)
                        .into_any_element()
                })
        }),
    })
    .into()
}

struct AffixDemo {
    top_affix: Entity<Affix>,
    bottom_affix: Entity<Affix>,
}

impl Render for AffixDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

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
