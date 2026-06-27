use gpui::{App, AppContext, Entity, IntoElement, Render, Window};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Card, Flex, Space, Text, TitleBar};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::Theme;

pub fn render(cx: &mut App) -> Entity<TitleBarDemo> {
    cx.new(|_| TitleBarDemo)
}

pub struct TitleBarDemo;

impl Render for TitleBarDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        page(
            "TitleBar 标题栏",
            "单独展示 Liora 自定义标题栏的品牌区、窗口控制按钮、居中命令区、操作区和紧凑/无边框变体。所有示例都使用当前主题 token，切换 Light/Dark 时会同步适配。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Real window controls",
                    "展示真实最小化、最大化/还原、关闭按钮图标。嵌入 Gallery 时按钮仍是 GPUI native window control area；真实应用可直接放进自定义 frame。",
                    Card::new(
                        showcase_stack(&theme)
                            .child(control_showcase_note(&theme, "Right controls / brand shell"))
                            .child(titlebar_surface(
                                &theme,
                                TitleBar::new()
                                    .id("titlebar-controls-right-demo")
                                    .title("Liora Studio")
                                    .subtitle("Theme-aware native chrome")
                                    .icon(Icon::new(IconName::Sparkles).size_units(16.0))
                                    .height_units(62.0)
                                    .padding_x_units(20.0)
                                    .gap_units(12.0)
                                    .actions_gap_units(8.0)
                                    .background(theme.neutral.card)
                                    .border_color(theme.neutral.border)
                                    .border(true)
                                    .title_color(theme.neutral.text_1)
                                    .subtitle_color(theme.neutral.text_3)
                                    .title_size_units(14.0)
                                    .subtitle_size_units(11.0)
                                    .content_align(liora_components::TitleBarContentAlign::Start)
                                    .window_controls_position(
                                        liora_components::WindowControlsPosition::Right,
                                    )
                                    .window_controls(true)
                                    .action(Button::new("Share").small())
                                    .action(Button::new("Deploy").small().primary()),
                            ))
                            .child(control_showcase_note(
                                &theme,
                                "Left controls / utility titlebar",
                            ))
                            .child(titlebar_surface(
                                &theme,
                                TitleBar::new()
                                    .id("titlebar-controls-left-demo")
                                    .title("Inspector")
                                    .subtitle("Left controls + manual drag policy")
                                    .icon(
                                        Icon::new(IconName::SlidersHorizontal).size_units(16.0),
                                    )
                                    .compact()
                                    .draggable(false)
                                    .padding_x_units(14.0)
                                    .gap_units(8.0)
                                    .actions_gap_units(6.0)
                                    .background(theme.neutral.popover)
                                    .border_color(theme.neutral.border)
                                    .title_color(theme.neutral.text_1)
                                    .subtitle_color(theme.neutral.text_3)
                                    .title_size_units(12.0)
                                    .subtitle_size_units(10.0)
                                    .content_align(liora_components::TitleBarContentAlign::End)
                                    .window_controls_position(
                                        liora_components::WindowControlsPosition::Left,
                                    )
                                    .window_controls(true)
                                    .action(Button::new("Reset").small()),
                            )),
                    )
                    .no_shadow(),
                ))
                .child(section(
                    "Command-center titlebar",
                    "居中命令区适合编辑器、设计工具和仪表盘。leading/action slots 可以承载状态、搜索入口或全局动作。",
                    Card::new(titlebar_surface(
                        &theme,
                        TitleBar::new()
                            .id("titlebar-command-demo")
                            .title("Command shell")
                            .subtitle("Centered slot")
                            .leading(status_pill(&theme, "Online", theme.success.base))
                            .center(command_palette_hint(&theme))
                            .actions([
                                Button::new("Inspect").small(),
                                Button::new("Publish").small().primary(),
                            ])
                            .height_units(58.0)
                            .padding_x_units(18.0)
                            .gap_units(10.0)
                            .actions_gap_units(8.0)
                            .background(theme.neutral.card)
                            .border_color(theme.neutral.border)
                            .title_color(theme.neutral.text_1)
                            .subtitle_color(theme.neutral.text_3)
                            .content_align(liora_components::TitleBarContentAlign::Center)
                            .window_controls(false),
                    ))
                    .no_shadow(),
                ))
                .child(section(
                    "Borderless embedded toolbar",
                    "无边框模式适合抽屉、浮动面板或内容区内的局部工具条；这里保留完全主题化的低强调背景。",
                    Card::new(titlebar_surface(
                        &theme,
                        TitleBar::new()
                            .id("titlebar-borderless-demo")
                            .borderless()
                            .border(false)
                            .title("Preview canvas")
                            .subtitle("Embedded toolbar")
                            .height_units(46.0)
                            .padding_x_units(16.0)
                            .background(theme.primary.light_9)
                            .border_color(theme.primary.light_7)
                            .title_color(theme.neutral.text_1)
                            .subtitle_color(theme.neutral.text_2)
                            .window_controls(false)
                            .actions([Button::new("Fit").small(), Button::new("Export").small()]),
                    ))
                    .no_shadow(),
                ))
                .child(Text::new(
                    "说明：如果只想在 Gallery 中演示布局，可关闭 window_controls；如果要验证最小化/最大化/关闭图标和点击区域，请使用上方 Real window controls 示例。",
                )),
        )
    }
}

fn showcase_stack(theme: &Theme) -> Flex {
    Flex::new()
        .w_full()
        .column()
        .gap_md()
        .rounded_units(18.0)
        .border()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.body)
        .padding_md()
}

fn titlebar_surface(theme: &Theme, titlebar: TitleBar) -> Flex {
    Flex::new()
        .w_full()
        .overflow_hidden()
        .rounded_units(14.0)
        .border()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .child(titlebar)
}

fn control_showcase_note(theme: &Theme, label: &'static str) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .justify_between()
        .padding_x_units(4.0)
        .child(
            Text::new(label)
                .xs()
                .bold()
                .text_color(theme.neutral.text_2),
        )
        .child(
            Text::new("minimize · maximize · close")
                .xs()
                .text_color(theme.neutral.text_3),
        )
}

fn status_pill(theme: &Theme, label: &'static str, color: gpui::Hsla) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .padding_x_units(8.0)
        .padding_y_px(4.0)
        .rounded_pill()
        .bg(color.opacity(0.12))
        .border()
        .border_color(color.opacity(0.32))
        .child(Text::new(label).xs().bold().text_color(theme.success.base))
}

fn command_palette_hint(theme: &Theme) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .gap_sm()
        .width_px(360.0)
        .padding_x_units(12.0)
        .padding_y_px(4.0)
        .rounded_units(12.0)
        .border()
        .border_color(theme.primary.light_7)
        .bg(theme.primary.light_9)
        .child(
            Icon::new(IconName::Search)
                .size_units(14.0)
                .color(theme.primary.base),
        )
        .child(
            Text::new("Search commands or files…")
                .sm()
                .text_color(theme.neutral.text_2),
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
            ".height_units(",
            ".padding_x_units(",
            ".gap_units(",
            ".actions_gap_units(",
            ".background(",
            ".border_color(",
            ".border(",
            ".title_color(",
            ".subtitle_color(",
            ".title_size_units(",
            ".subtitle_size_units(",
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

    #[test]
    fn titlebar_demo_uses_theme_tokens_and_shows_window_controls() {
        let source = include_str!("titlebar_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("cx.global::<Config>().theme"));
        assert!(source.contains(".window_controls(true)"));
        assert!(source.contains("WindowControlsPosition::Right"));
        assert!(source.contains("WindowControlsPosition::Left"));
        assert!(source.contains("control_showcase_note"));
        assert!(!source.contains("rgb("));
    }

    #[test]
    fn titlebar_demo_does_not_build_raw_gpui_layout_elements() {
        let source = include_str!("titlebar_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(!source.contains(" div,"));
        assert!(!source.contains("div()"));
        assert!(!source.contains("gpui::Div"));
        assert!(!source.contains("gpui::div"));
    }
}
