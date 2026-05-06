use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use aura_theme::Theme;
use gpui::{App, Context, Entity, Hsla, IntoElement, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> Entity<IconDemo> {
    cx.new(|_| IconDemo)
}

pub struct IconDemo;

impl Render for IconDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let icons: &[(IconName, &str)] = &[
            (IconName::House, "Home"),
            (IconName::User, "User"),
            (IconName::Search, "Search"),
            (IconName::Check, "Check"),
            (IconName::ChevronDown, "ChevronDown"),
            (IconName::Settings, "Settings"),
            (IconName::X, "X"),
            (IconName::Star, "Star"),
        ];

        let mut page = div().flex().flex_col().gap_3();
        page = page.child(hdr(theme, "Lucide Icons"));
        page = page.child(
            div()
                .flex()
                .flex_row()
                .gap_4()
                .flex_wrap()
                .items_center()
                .children(icons.iter().map(|(icon, name)| {
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .items_center()
                        .child(build_icon(theme, *icon, 24.0, theme.primary.base))
                        .child(
                            div()
                                .text_size(px(10.0))
                                .text_color(theme.neutral.text_3)
                                .child(*name),
                        )
                })),
        );
        page = page.child(hdr(theme, "Default Color"));
        page = page.child(
            div()
                .flex()
                .flex_row()
                .gap_4()
                .flex_wrap()
                .items_center()
                .children(icons.iter().map(|(icon, name)| {
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .items_center()
                        .child(build_icon_default_color(theme, *icon, 24.0))
                        .child(
                            div()
                                .text_size(px(10.0))
                                .text_color(theme.neutral.text_3)
                                .child(*name),
                        )
                })),
        );
        page = page.child(hdr(theme, "Sizes"));
        page = page.child(
            div()
                .flex()
                .flex_row()
                .gap_4()
                .items_end()
                .child(icon_labeled(theme, IconName::House, 12.0, "12"))
                .child(icon_labeled(theme, IconName::House, 18.0, "18"))
                .child(icon_labeled(theme, IconName::House, 24.0, "24"))
                .child(icon_labeled(theme, IconName::House, 32.0, "32")),
        );
        page = page.child(hdr(theme, "Colors"));
        page = page.child(
            div()
                .flex()
                .flex_row()
                .gap_4()
                .items_end()
                .child(icon_labeled_color(
                    theme,
                    IconName::Star,
                    theme.primary.base,
                    "Primary",
                ))
                .child(icon_labeled_color(
                    theme,
                    IconName::Star,
                    theme.info.base,
                    "Info",
                ))
                .child(icon_labeled_color(
                    theme,
                    IconName::Star,
                    theme.success.base,
                    "Success",
                ))
                .child(icon_labeled_color(
                    theme,
                    IconName::Star,
                    theme.warning.base,
                    "Warning",
                ))
                .child(icon_labeled_color(
                    theme,
                    IconName::Star,
                    theme.danger.base,
                    "Danger",
                )),
        );
        page
    }
}

fn hdr(theme: &Theme, s: &str) -> impl IntoElement {
    div()
        .text_size(px(theme.font_size.lg))
        .text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD)
        .mt_2()
        .child(s.to_string())
}

fn build_icon(_theme: &Theme, icon: IconName, sz: f32, color: Hsla) -> impl IntoElement {
    Icon::new(icon).size(px(sz)).color(color)
}

fn build_icon_default_color(_theme: &Theme, icon: IconName, sz: f32) -> impl IntoElement {
    Icon::new(icon).size(px(sz))
}

fn icon_labeled(theme: &Theme, icon: IconName, sz: f32, label: &str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .items_center()
        .child(build_icon(theme, icon, sz, theme.neutral.icon))
        .child(
            div()
                .text_size(px(10.0))
                .text_color(theme.neutral.text_3)
                .child(label.to_string()),
        )
}

fn icon_labeled_color(theme: &Theme, icon: IconName, color: Hsla, label: &str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .items_center()
        .child(build_icon(theme, icon, 24.0, color))
        .child(
            div()
                .text_size(px(10.0))
                .text_color(theme.neutral.text_3)
                .child(label.to_string()),
        )
}
