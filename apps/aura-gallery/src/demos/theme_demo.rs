use aura_components::layout_helpers::{page, section};
use aura_components::{Button, Card, Space, Tag, Text, Title};
use aura_core::{Config, ThemeMode};
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ThemeDemo).into()
}

struct ThemeDemo;

impl Render for ThemeDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config = cx.global::<Config>();
        let theme = config.theme.clone();
        let mode = config.theme_mode;

        page(
            "Theme 主题系统",
            "验证 System / Light / Dark 三种主题模式下的语义色、表面、文字和交互状态。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "当前模式",
                    "Gallery 顶部 Theme 分段控件会切换全局 Config；System 模式会跟随操作系统外观变化。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(Title::new(format!("{} · {}", mode.label(), theme.name)).h4())
                            .child(Text::new(match mode {
                                ThemeMode::System => "System resolves through GPUI WindowAppearance and observes system changes.",
                                ThemeMode::Light => "Explicit light mode ignores later OS appearance changes.",
                                ThemeMode::Dark => "Explicit dark mode ignores later OS appearance changes.",
                            }))
                            .child(
                                Space::new()
                                    .wrap()
                                    .gap_sm()
                                    .child(Tag::new("System").round(true))
                                    .child(Tag::new("Light").success().round(true))
                                    .child(Tag::new("Dark").info().round(true)),
                            ),
                    ),
                ))
                .child(section(
                    "语义色与状态",
                    "这些色块使用主题 token，而不是写死浅色背景；暗色模式下 subtle 背景保持低透明度。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(color_row("Primary", theme.primary.base, theme.primary.hover, theme.primary.light_9))
                            .child(color_row("Info", theme.info.base, theme.info.hover, theme.info.light_9))
                            .child(color_row("Success", theme.success.base, theme.success.hover, theme.success.light_9))
                            .child(color_row("Warning", theme.warning.base, theme.warning.hover, theme.warning.light_9))
                            .child(color_row("Danger", theme.danger.base, theme.danger.hover, theme.danger.light_9)),
                    ),
                ))
                .child(section(
                    "交互状态",
                    "Button / surface / overlay 等控件应从主题推导 hover、pressed、disabled 和遮罩。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(
                                Space::new()
                                    .wrap()
                                    .gap_sm()
                                    .child(Button::new("Default"))
                                    .child(Button::new("Primary").primary())
                                    .child(Button::new("Secondary").primary().secondary())
                                    .child(Button::new("Danger").danger())
                                    .child(Button::new("Disabled").primary().disabled(true)),
                            )
                            .child(surface_sample()),
                    ),
                )),
        )
    }
}

fn color_row(
    label: &'static str,
    base: gpui::Hsla,
    hover: gpui::Hsla,
    subtle: gpui::Hsla,
) -> impl IntoElement {
    Space::new()
        .gap_md()
        .wrap()
        .child(div().w(px(72.0)).child(Text::new(label).bold()))
        .child(color_chip("base", base))
        .child(color_chip("hover", hover))
        .child(color_chip("subtle", subtle))
}

fn color_chip(label: &'static str, color: gpui::Hsla) -> impl IntoElement {
    div()
        .w(px(112.0))
        .h(px(44.0))
        .rounded_lg()
        .border_1()
        .border_color(color)
        .bg(color)
        .flex()
        .items_center()
        .justify_center()
        .child(Text::new(label).size(px(12.0)))
}

fn surface_sample() -> impl IntoElement {
    div().rounded_lg().border_1().p_4().child(
        Space::new()
            .vertical()
            .gap_sm()
            .child(Text::new("Themed surface").bold())
            .child(Text::new(
                "Hover and selected backgrounds should stay readable in both light and dark modes.",
            )),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn theme_demo_uses_config_theme_mode_and_tokens() {
        let source = include_str!("theme_demo.rs");
        let production = source.split("#[cfg(test)]").next().unwrap_or(source);
        assert!(production.contains("ThemeMode::System"));
        assert!(production.contains("config.theme_mode"));
        assert!(production.contains("theme.primary.light_9"));
        assert!(!production.contains("rgb(0xf8fafc"));
    }
}
