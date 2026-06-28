use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Space, Text};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_antd::IconName as AntdIconName;
use liora_icons_carbon::IconName as CarbonIconName;
use liora_icons_ionic::IconName as IonicIconName;
use liora_icons_lucide::IconName;
use liora_icons_material::IconName as MaterialIconName;
use liora_icons_tabler::IconName as TablerIconName;

use liora_components::layout_helpers::{page, row, section};

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
                ))
                .child(section(
                    "Additional icon libraries",
                    "Ant Design、Ionicons、Tabler、Carbon 和 Material 图标库与 Lucide 使用同一套 Icon API。",
                    row(vec![
                        icon_labeled(Icon::new(AntdIconName::SaveOutlined).size_lg(), "AntD"),
                        icon_labeled(Icon::new(IonicIconName::AddCircleOutline).size_lg(), "Ionic"),
                        icon_labeled(Icon::new(TablerIconName::HomeFilled).size_lg(), "Tabler"),
                        icon_labeled(Icon::new(CarbonIconName::CheckmarkFilled).size_lg(), "Carbon"),
                        icon_labeled(Icon::new(MaterialIconName::SearchOutlined).size_lg(), "Material"),
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
