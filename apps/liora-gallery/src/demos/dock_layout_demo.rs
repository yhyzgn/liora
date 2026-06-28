use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_grid};
use liora_components::{DockEdge, DockLayout, DockPanel, DockTab, Space, Tag, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DockLayoutDemo).into()
}

struct DockLayoutDemo;

impl Render for DockLayoutDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "DockLayout 停靠布局",
            "面向 IDE、数据工作台和后台工具的原生工作区布局：侧边面板、底部面板和中心文档 tabs。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Dock showcase",
                    "工作台示例统一使用宽卡片，保持面板边界、说明和主体宽度稳定。",
                    showcase_grid(vec![
                        showcase_card_wide(
                            "Workbench 工作台",
                            "静态 dock 区域适合先搭出稳定信息架构；拖拽重排可后续在此 API 上继续增强。",
                            workbench_layout(),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "左右检查器 + 底部日志",
                            "多个 edge panel 可以组合成常见的 Explorer / Inspector / Terminal 桌面应用结构。",
                            inspector_layout(),
                        )
                        .into_any_element(),
                    ]),
                )),
        )
    }
}

fn workbench_layout() -> DockLayout {
    DockLayout::new()
        .id("gallery-dock-workbench")
        .height_lg()
        .panel_gap(px(6.0))
        .panel(
            DockPanel::new(
                "explorer",
                "Explorer",
                DockEdge::Left,
                panel_list(["src", "crates", "apps", "README.md"]),
            )
            .size(px(220.0)),
        )
        .panel(
            DockPanel::new(
                "terminal",
                "Terminal",
                DockEdge::Bottom,
                Text::new("cargo check --workspace --all-targets"),
            )
            .size(px(132.0)),
        )
        .tab(DockTab::new(
            "main",
            "main.rs",
            editor_preview("fn main() {\n    init_liora(cx);\n}"),
        ))
        .tab(DockTab::new(
            "readme",
            "README.md",
            Text::new("# Liora\nNative GPUI component library.").wrap(),
        ))
}

fn inspector_layout() -> DockLayout {
    DockLayout::new()
        .id("gallery-dock-inspector")
        .height(px(460.0))
        .panel_gap(px(6.0))
        .panel(
            DockPanel::new(
                "outline",
                "Outline",
                DockEdge::Left,
                panel_list(["App", "Shell", "Sidebar", "Content"]),
            )
            .size(px(180.0)),
        )
        .panel(
            DockPanel::new(
                "props",
                "Properties",
                DockEdge::Right,
                panel_list(["theme", "spacing", "state", "events"]),
            )
            .size(px(220.0)),
        )
        .panel(
            DockPanel::new(
                "logs",
                "Logs",
                DockEdge::Bottom,
                Text::new("Ready • 0 errors • 0 warnings"),
            )
            .size(px(96.0)),
        )
        .tab(DockTab::new(
            "preview",
            "Preview",
            Space::new()
                .vertical()
                .gap_sm()
                .child(Tag::new("Live").success())
                .child(Text::new("Center content remains flexible.")),
        ))
}

fn panel_list(items: impl IntoIterator<Item = &'static str>) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_xs()
        .children(items.into_iter().map(Text::new))
}

fn editor_preview(code: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(Tag::new("Rust").info())
        .child(
            Text::new(code)
                .code_style(&liora_theme::Theme::light())
                .wrap(),
        )
}
