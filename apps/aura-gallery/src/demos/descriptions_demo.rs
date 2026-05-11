use aura_components::{Button, ButtonVariant, Descriptions, DescriptionsDirection, Space, Text};
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DescriptionsDemo).into()
}

struct DescriptionsDemo;

impl Render for DescriptionsDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Descriptions 描述列表",
            "用于展示多个字段。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "无边框模式适合轻量信息展示。",
                    Descriptions::new()
                        .title("用户信息")
                        .item("用户名", "kooriookami", 1)
                        .item("手机号", "18100000000", 1)
                        .item("居住地", "苏州市", 1)
                        .item("备注", Text::new("学校").bg(gpui::blue().opacity(0.1)), 1)
                        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2),
                ))
                .child(section(
                    "带边框样式",
                    "带标题操作区和边框分隔。",
                    Descriptions::new()
                        .title("用户信息")
                        .border(true)
                        .extra(Button::new("操作").variant(ButtonVariant::Primary).small())
                        .item("用户名", "kooriookami", 1)
                        .item("手机号", "18100000000", 1)
                        .item("居住地", "苏州市", 1)
                        .item("备注", "学校", 1)
                        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2),
                ))
                .child(section(
                    "垂直带边框",
                    "标签与内容上下排列。",
                    Descriptions::new()
                        .title("垂直布局")
                        .border(true)
                        .direction(DescriptionsDirection::Vertical)
                        .item("用户名", "kooriookami", 1)
                        .item("手机号", "18100000000", 1)
                        .item("居住地", "苏州市", 1)
                        .item("备注", "学校", 1)
                        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2),
                )),
        )
    }
}
