use aura_components::{Affix, AffixPosition, Button, ButtonVariant};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| AffixDemo).into()
}

struct AffixDemo;

impl Render for AffixDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .h_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Affix 固钉"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("将内容固定在特定可视区域。"),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .id("affix-scroll-view")
                    .overflow_y_scroll()
                    .bg(theme.neutral.hover)
                    .p_4()
                    .child(
                        div()
                            .h(px(200.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.neutral.text_3)
                                    .child("向下滚动查看固钉效果"),
                            ),
                    )
                    .child(cx.new(|_| {
                        Affix::new().offset(px(80.0)).content(|_, _| {
                            Button::new("固钉在距离顶部 80px 的位置")
                                .variant(ButtonVariant::Primary)
                                .into_any_element()
                        })
                    }))
                    .child(
                        div()
                            .h(px(800.0))
                            .bg(theme.neutral.card)
                            .my_4()
                            .border_1()
                            .border_color(theme.neutral.border)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(div().child("长内容占位")),
                    )
                    .child(cx.new(|_| {
                        Affix::new()
                            .position(AffixPosition::Bottom)
                            .offset(px(20.0))
                            .content(|_, _| {
                                Button::new("固钉在距离底部 20px 的位置")
                                    .variant(ButtonVariant::Success)
                                    .into_any_element()
                            })
                    }))
                    .child(div().h(px(400.0))),
            )
    }
}
