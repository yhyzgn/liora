use aura_components::{Space, Text};
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| IconDemo).into()
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

        page(
            "Icon 图标",
            "基于 Lucide 的图标系统，支持尺寸与颜色配置。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Lucide Icons",
                    "常用图标展示。",
                    row(icons
                        .iter()
                        .map(|(icon, name)| {
                            icon_labeled(Icon::new(*icon).size_lg().color(theme.primary.base), name)
                        })
                        .collect::<Vec<_>>()),
                ))
                .child(section(
                    "Default Color",
                    "未指定颜色时使用图标默认颜色。",
                    row(icons
                        .iter()
                        .map(|(icon, name)| icon_labeled(Icon::new(*icon).size_lg(), name))
                        .collect::<Vec<_>>()),
                ))
                .child(section(
                    "Sizes",
                    "使用语义化尺寸快捷方法。",
                    row(vec![
                        icon_labeled(Icon::new(IconName::House).size_xs(), "12"),
                        icon_labeled(Icon::new(IconName::House).size_md(), "18"),
                        icon_labeled(Icon::new(IconName::House).size_lg(), "24"),
                        icon_labeled(Icon::new(IconName::House).size_xl(), "32"),
                    ]),
                ))
                .child(section(
                    "Colors",
                    "图标颜色可以跟随主题语义色。",
                    row(vec![
                        icon_labeled(
                            Icon::new(IconName::Star)
                                .size_lg()
                                .color(theme.primary.base),
                            "Primary",
                        ),
                        icon_labeled(
                            Icon::new(IconName::Star).size_lg().color(theme.info.base),
                            "Info",
                        ),
                        icon_labeled(
                            Icon::new(IconName::Star)
                                .size_lg()
                                .color(theme.success.base),
                            "Success",
                        ),
                        icon_labeled(
                            Icon::new(IconName::Star)
                                .size_lg()
                                .color(theme.warning.base),
                            "Warning",
                        ),
                        icon_labeled(
                            Icon::new(IconName::Star).size_lg().color(theme.danger.base),
                            "Danger",
                        ),
                    ]),
                )),
        )
    }
}

fn icon_labeled(icon: Icon, label: &str) -> impl IntoElement {
    Space::new()
        .vertical()
        .align_center()
        .gap_xs()
        .child(icon)
        .child(Text::new(label.to_string()).nowrap())
}
