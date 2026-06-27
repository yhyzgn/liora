use gpui::{App, Entity, IntoElement, Render, Window, div, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Card, Space, Text, TitleBar};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> Entity<TitleBarDemo> {
    cx.new(|_| TitleBarDemo)
}

pub struct TitleBarDemo;

impl Render for TitleBarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        page(
            "TitleBar 标题栏",
            "单独展示 Liora 自定义标题栏的品牌区、居中内容、操作区、窗口控制位置和紧凑/无边框变体。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Brand application chrome",
                    "品牌型应用标题栏：图标、标题、副标题、尺寸、背景、边框和操作区都可由应用决定。",
                    Card::new(
                        div().overflow_hidden().rounded(px(14.0)).border_1().child(
                            TitleBar::new()
                                .id("titlebar-brand-demo")
                                .title("Project Atlas")
                                .subtitle("Native GPUI workspace")
                                .icon(brand_mark(rgb(0x6366f1).into(), rgb(0x22d3ee).into()))
                                .height(px(62.0))
                                .padding_x(px(20.0))
                                .gap(px(12.0))
                                .actions_gap(px(6.0))
                                .background(rgb(0xf8fafc).into())
                                .border_color(rgb(0xdbeafe).into())
                                .border(true)
                                .title_color(rgb(0x0f172a).into())
                                .subtitle_color(rgb(0x64748b).into())
                                .title_size(px(14.0))
                                .subtitle_size(px(11.0))
                                .content_align(liora_components::TitleBarContentAlign::Start)
                                .window_controls_position(
                                    liora_components::WindowControlsPosition::Right,
                                )
                                .window_controls(false)
                                .action(Button::new("Share").small())
                                .action(Button::new("Deploy").small().primary()),
                        ),
                    )
                    .no_shadow(),
                ))
                .child(section(
                    "Command-center titlebar",
                    "居中命令区适合编辑器、设计工具和仪表盘。leading/action slots 可以承载状态、搜索入口或全局动作。",
                    Card::new(
                        div().overflow_hidden().rounded(px(14.0)).border_1().child(
                            TitleBar::new()
                                .id("titlebar-command-demo")
                                .title("Command shell")
                                .subtitle("Centered slot")
                                .leading(status_pill("Online", rgb(0x16a34a).into()))
                                .center(command_palette_hint())
                                .actions([
                                    Button::new("Inspect").small(),
                                    Button::new("Publish").small().primary(),
                                ])
                                .height(px(56.0))
                                .padding_x(px(18.0))
                                .gap(px(10.0))
                                .actions_gap(px(8.0))
                                .background(rgb(0xffffff).into())
                                .border_color(rgb(0xe2e8f0).into())
                                .content_align(liora_components::TitleBarContentAlign::Center)
                                .window_controls(false),
                        ),
                    )
                    .no_shadow(),
                ))
                .child(section(
                    "Left controls / utility window",
                    "平台或工具窗需要左侧控制区时，可切换窗口按钮位置；嵌入式面板也可以禁用拖动区域。",
                    Card::new(
                        div().overflow_hidden().rounded(px(14.0)).border_1().child(
                            TitleBar::new()
                                .id("titlebar-left-controls-demo")
                                .title("Inspector")
                                .subtitle("Left controls + manual drag policy")
                                .icon(Icon::new(IconName::SlidersHorizontal).size(px(16.0)))
                                .compact()
                                .draggable(false)
                                .padding_x(px(14.0))
                                .gap(px(8.0))
                                .background(rgb(0x111827).into())
                                .border_color(rgb(0x334155).into())
                                .title_color(rgb(0xf8fafc).into())
                                .subtitle_color(rgb(0x94a3b8).into())
                                .title_size(px(12.0))
                                .subtitle_size(px(10.0))
                                .content_align(liora_components::TitleBarContentAlign::End)
                                .window_controls_position(
                                    liora_components::WindowControlsPosition::Left,
                                )
                                .window_controls(false)
                                .action(Button::new("Reset").small()),
                        ),
                    )
                    .no_shadow(),
                ))
                .child(section(
                    "Borderless embedded toolbar",
                    "无边框模式适合抽屉、浮动面板或内容区内的局部工具条。",
                    Card::new(
                        div().overflow_hidden().rounded(px(14.0)).border_1().child(
                            TitleBar::new()
                                .id("titlebar-borderless-demo")
                                .borderless()
                                .border(false)
                                .title("Preview canvas")
                                .subtitle("Embedded toolbar")
                                .height(px(44.0))
                                .padding_x(px(16.0))
                                .background(rgb(0xf5f3ff).into())
                                .border_color(rgb(0xc4b5fd).into())
                                .title_color(rgb(0x4c1d95).into())
                                .subtitle_color(rgb(0x6d28d9).into())
                                .window_controls(false)
                                .actions([
                                    Button::new("Fit").small(),
                                    Button::new("Export").small(),
                                ]),
                        ),
                    )
                    .no_shadow(),
                ))
                .child(Text::new(
                    "在真实窗口中，TitleBar 会提供拖动区域、双击标题栏行为和窗口控制按钮；Gallery 示例默认隐藏窗口控制，避免嵌入 demo 时影响宿主窗口。",
                )),
        )
    }
}

fn brand_mark(from: gpui::Hsla, to: gpui::Hsla) -> impl IntoElement {
    div()
        .size(px(30.0))
        .rounded(px(10.0))
        .bg(from)
        .shadow_md()
        .child(
            div()
                .m(px(7.0))
                .size(px(16.0))
                .rounded_full()
                .bg(to)
                .opacity(0.86),
        )
}

fn status_pill(label: &'static str, color: gpui::Hsla) -> impl IntoElement {
    div()
        .px_2()
        .py_1()
        .rounded_full()
        .bg(color.opacity(0.12))
        .border_1()
        .border_color(color.opacity(0.32))
        .child(Text::new(label).xs().bold().text_color(color))
}

fn command_palette_hint() -> impl IntoElement {
    div()
        .min_w(px(240.0))
        .max_w(px(360.0))
        .px_3()
        .py_1()
        .rounded(px(10.0))
        .border_1()
        .border_color(rgb(0xdbeafe))
        .bg(rgb(0xeff6ff))
        .flex()
        .items_center()
        .gap_2()
        .child(
            Icon::new(IconName::Search)
                .size(px(14.0))
                .color(rgb(0x2563eb).into()),
        )
        .child(
            Text::new("Search commands or files…")
                .sm()
                .text_color(rgb(0x1d4ed8).into()),
        )
}

#[cfg(test)]
mod tests {
    #[test]
    fn titlebar_demo_is_standalone() {
        let source = include_str!("titlebar_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        for builder in [
            ".id(",
            ".title(",
            ".subtitle(",
            ".icon(",
            ".leading(",
            ".center(",
            ".action(",
            ".actions(",
            ".window_controls(",
            ".draggable(",
            ".height(",
            ".padding_x(",
            ".gap(",
            ".actions_gap(",
            ".background(",
            ".border_color(",
            ".border(",
            ".title_color(",
            ".subtitle_color(",
            ".title_size(",
            ".subtitle_size(",
            ".content_align(",
            ".window_controls_position(",
            ".compact()",
            ".borderless()",
        ] {
            assert!(
                source.contains(builder),
                "missing TitleBar builder {builder}"
            );
        }
        assert!(source.contains("TitleBar::new()"));
        assert!(!source.contains(concat!("Sidebar", "::new()")));
        assert!(!source.contains(concat!("AppWindow", "Frame::new")));
    }
}
