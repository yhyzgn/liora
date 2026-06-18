use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Label, Operation, Space, Switch};
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| OperationDemo {
        tray_switch: cx.new(|cx| Switch::new(false, cx)),
        sync_switch: cx.new(|cx| Switch::new(true, cx)),
    })
    .into()
}

struct OperationDemo {
    tray_switch: Entity<Switch>,
    sync_switch: Entity<Switch>,
}

impl Render for OperationDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Operation 操作行",
            "左侧说明 + 右侧操作区域的两端对齐布局，适合设置项、控制面板和工具栏行。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "设置项布局",
                    "右侧 action 完全由调用方提供，可以是 Switch、Button 或任意 GPUI 元素。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(
                            Operation::new(
                                Label::new("开启状态栏驻留").icon(IconName::Bell),
                                self.tray_switch.clone(),
                            )
                            .description("关闭主窗口后仍保留托盘入口。")
                            .success(),
                        )
                        .child(
                            Operation::new(
                                Label::new("后台同步").icon(IconName::RefreshCcw),
                                self.sync_switch.clone(),
                            )
                            .description("自动同步本地配置与远端状态。")
                            .status("自动")
                            .status_color(gpui::blue()),
                        ),
                ))
                .child(section(
                    "操作状态",
                    "支持说明文本、状态标签、状态颜色、危险/禁用等展示。",
                    Space::new()
                        .vertical()
                        .gap_md()
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
                            .description("禁用态会整体降低透明度，避免误触。")
                            .danger()
                            .disabled(true),
                        )
                        .child(
                            Operation::new(
                                Label::new("紧凑操作").icon(IconName::SlidersHorizontal),
                                Button::new("Config").primary().small(),
                            )
                            .description("no_padding 可用于嵌入表格、卡片或自定义容器。")
                            .warning()
                            .no_padding(),
                        ),
                )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn operation_demo_covers_action_and_state_configurations() {
        let source = include_str!("operation_demo.rs");
        assert!(source.contains("Operation::new"));
        assert!(source.contains("Operation::with_text"));
        assert!(source.contains("Switch::new"));
        assert!(source.contains(".description("));
        assert!(source.contains(".status_color("));
        assert!(source.contains(".disabled(true)"));
        assert!(source.contains(".no_padding()"));
    }
}
