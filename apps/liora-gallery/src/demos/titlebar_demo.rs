use gpui::{App, Entity, IntoElement, Render, Window, div, prelude::*, px};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Card, Space, Text, TitleBar};

pub fn render(cx: &mut App) -> Entity<TitleBarDemo> {
    cx.new(|_| TitleBarDemo)
}

pub struct TitleBarDemo;

impl Render for TitleBarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        page(
            "TitleBar 标题栏",
            "单独展示 Liora 自定义标题栏的标题、副标题、操作区和紧凑/无边框变体。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Standard titlebar",
                    "用于 WindowFrameMode::Custom 的应用标题栏。示例中隐藏窗口控制，避免嵌入 demo 时影响宿主窗口。",
                    Card::new(
                        div().overflow_hidden().rounded(px(12.0)).border_1().child(
                            TitleBar::new()
                                .title("Project Atlas")
                                .subtitle("Custom native chrome")
                                .height(px(58.0))
                                .padding_x(px(20.0))
                                .gap(px(12.0))
                                .actions_gap(px(6.0))
                                .background(gpui::transparent_black())
                                .border(false)
                                .content_align(liora_components::TitleBarContentAlign::Start)
                                .window_controls_position(liora_components::WindowControlsPosition::Right)
                                .window_controls(false)
                                .action(Button::new("Share").small())
                                .action(Button::new("Deploy").small().primary()),
                        ),
                    )
                    .no_shadow(),
                ))
                .child(section(
                    "Compact + borderless",
                    "用于工具窗、浮动面板或嵌入式原生窗口。",
                    Card::new(
                        div().overflow_hidden().rounded(px(12.0)).border_1().child(
                            TitleBar::new()
                                .compact()
                                .borderless()
                                .title("Inspector")
                                .subtitle("Compact")
                                .window_controls(false)
                                .action(Button::new("Reset").small()),
                        ),
                    )
                    .no_shadow(),
                ))
                .child(Text::new(
                    "在真实窗口中，TitleBar 会提供拖动区域、双击标题栏行为和窗口控制按钮。",
                )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn titlebar_demo_is_standalone() {
        let source = include_str!("titlebar_demo.rs");

        assert!(source.contains("TitleBar::new()"));
        assert!(source.contains(".window_controls(false)"));
        assert!(source.contains(".padding_x(px(20.0))"));
        assert!(source.contains(".actions_gap(px(6.0))"));
        assert!(source.contains(".background(gpui::transparent_black())"));
        assert!(!source.contains(concat!("Sidebar", "::new()")));
        assert!(!source.contains(concat!("AppWindow", "Frame::new")));
    }
}
