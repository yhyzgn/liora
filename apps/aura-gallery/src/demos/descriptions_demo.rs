use aura_components::{Button, ButtonVariant, Descriptions, DescriptionsDirection};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DescriptionsDemo).into()
}

struct DescriptionsDemo;

impl Render for DescriptionsDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .id("descriptions-scroll")
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Descriptions 描述列表"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于展示多个字段。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(
                        Descriptions::new()
                            .title("用户信息")
                            .item("用户名", "kooriookami", 1)
                            .item("手机号", "18100000000", 1)
                            .item("居住地", "苏州市", 1)
                            .item("备注", div().bg(gpui::blue().opacity(0.1)).child("学校"), 1)
                            .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("带边框样式"),
                    )
                    .child(
                        Descriptions::new()
                            .title("用户信息")
                            .border(true)
                            .extra(
                                Button::new("操作")
                                    .variant(ButtonVariant::Primary)
                                    .size(aura_theme::ButtonSize::Small),
                            )
                            .item("用户名", "kooriookami", 1)
                            .item("手机号", "18100000000", 1)
                            .item("居住地", "苏州市", 1)
                            .item("备注", "学校", 1)
                            .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("垂直带边框"),
                    )
                    .child(
                        Descriptions::new()
                            .title("垂直布局")
                            .border(true)
                            .direction(DescriptionsDirection::Vertical)
                            .item("用户名", "kooriookami", 1)
                            .item("手机号", "18100000000", 1)
                            .item("居住地", "苏州市", 1)
                            .item("备注", "学校", 1)
                            .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2),
                    ),
            )
    }
}
