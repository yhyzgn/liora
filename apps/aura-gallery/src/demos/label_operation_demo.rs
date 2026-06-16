use aura_components::layout_helpers::{page, row_md, section};
use aura_components::{Button, Label, Operation, Space, Switch};
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| LabelOperationDemo {
        switch: cx.new(|cx| Switch::new(false, cx)),
    })
    .into()
}

struct LabelOperationDemo {
    switch: gpui::Entity<Switch>,
}

impl Render for LabelOperationDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Label / Operation 标签操作",
            "Label 组合 Icon + Text；Operation 提供左侧说明、右侧操作的两端对齐布局。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Label",
                    "可配置图标、间距、颜色和字号。",
                    row_md(vec![
                        Label::new("CPU")
                            .icon(IconName::Cpu)
                            .size(px(14.0))
                            .into_any_element(),
                        Label::new("Network")
                            .icon(IconName::Wifi)
                            .gap(px(10.0))
                            .into_any_element(),
                        Label::new("Build passed")
                            .icon(IconName::CircleCheck)
                            .color(gpui::green())
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "Operation",
                    "用于设置项、表单项、工具栏行等左右两端对齐场景。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(
                            Operation::new(
                                Label::new("开启状态栏驻留").icon(IconName::Bell),
                                self.switch.clone(),
                            )
                            .description("关闭主窗口后仍保留托盘入口。")
                            .success(),
                        )
                        .child(
                            Operation::with_text("执行操作", Button::new("Run").small())
                                .description("用于控制面板里的右侧操作布局。")
                                .status("手动"),
                        )
                        .child(
                            Operation::new(
                                Label::new("危险操作").icon(IconName::TriangleAlert),
                                Button::new("Delete").danger().small(),
                            )
                            .description("支持状态标签、说明文本和禁用态。")
                            .danger()
                            .disabled(true),
                        ),
                )),
        )
    }
}
