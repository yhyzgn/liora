use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card, showcase_grid};
use liora_components::{CopyButton, Grid, GridItem, Space, Tag, Text};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_antd::IconName as AntdIconName;
use liora_icons_carbon::IconName as CarbonIconName;
use liora_icons_ionic::IconName as IonicIconName;
use liora_icons_lucide::IconName;
use liora_icons_material::IconName as MaterialIconName;
use liora_icons_tabler::IconName as TablerIconName;

const COPY_PATHS: [&str; 6] = [
    "liora::icons_lucide::IconName::Settings",
    "liora::icons_antd::IconName::SaveOutlined",
    "liora::icons_ionic::IconName::AddCircleOutline",
    "liora::icons_tabler::IconName::HomeFilled",
    "liora::icons_carbon::IconName::CheckmarkFilled",
    "liora::icons_material::IconName::SearchOutlined",
];

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| IconLibrariesDemo {
        copies: COPY_PATHS.map(|path| cx.new(|cx| CopyButton::new(path, path, cx))),
    })
    .into()
}

struct IconLibrariesDemo {
    copies: [Entity<CopyButton>; 6],
}

impl Render for IconLibrariesDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        page(
            "Icon Libraries 图标库",
            "六套内置 SVG 图标库均通过同一套 liora_icons::Icon API 渲染；完整 IconName 清单在 Docs 中可点击复制。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Unified API",
                    "不同图标库只替换 IconName 来源，尺寸、颜色、主题继承、按钮图标等用法完全一致。",
                    showcase_grid(vec![
                        library_card(
                            "Lucide",
                            "liora::icons_lucide::IconName",
                            "线性、简洁，适合作为默认应用图标集。",
                            vec![
                                icon_sample("lucide", Icon::new(IconName::Settings).size_lg().color(theme.primary.base), "Settings"),
                                icon_sample("lucide", Icon::new(IconName::Search).size_lg().color(theme.info.base), "Search"),
                                icon_sample("lucide", Icon::new(IconName::CircleCheck).size_lg().color(theme.success.base), "CircleCheck"),
                            ],
                        )
                        .into_any_element(),
                        library_card(
                            "Ant Design",
                            "liora::icons_antd::IconName",
                            "Filled / Outlined / Twotone 风格保留在类型名中。",
                            vec![
                                icon_sample("antd", Icon::new(AntdIconName::SaveOutlined).size_lg().color(theme.primary.base), "SaveOutlined"),
                                icon_sample("antd", Icon::new(AntdIconName::SettingOutlined).size_lg().color(theme.info.base), "SettingOutlined"),
                                icon_sample("antd", Icon::new(AntdIconName::CheckCircleOutlined).size_lg().color(theme.success.base), "CheckCircleOutlined"),
                            ],
                        )
                        .into_any_element(),
                        library_card(
                            "Ionicons",
                            "liora::icons_ionic::IconName",
                            "Outline / Sharp 等平台风格后缀可直接选择。",
                            vec![
                                icon_sample("ionic", Icon::new(IonicIconName::AddCircleOutline).size_lg().color(theme.primary.base), "AddCircleOutline"),
                                icon_sample("ionic", Icon::new(IonicIconName::SearchOutline).size_lg().color(theme.info.base), "SearchOutline"),
                                icon_sample("ionic", Icon::new(IonicIconName::CheckmarkCircleOutline).size_lg().color(theme.success.base), "CheckmarkCircleOutline"),
                            ],
                        )
                        .into_any_element(),
                        library_card(
                            "Tabler",
                            "liora::icons_tabler::IconName",
                            "覆盖大量产品、数据和编辑场景，filled 图标追加 Filled。",
                            vec![
                                icon_sample("tabler", Icon::new(TablerIconName::HomeFilled).size_lg().color(theme.primary.base), "HomeFilled"),
                                icon_sample("tabler", Icon::new(TablerIconName::Search).size_lg().color(theme.info.base), "Search"),
                                icon_sample("tabler", Icon::new(TablerIconName::CircleCheck).size_lg().color(theme.success.base), "CircleCheck"),
                            ],
                        )
                        .into_any_element(),
                        library_card(
                            "Carbon",
                            "liora::icons_carbon::IconName",
                            "IBM Carbon 图标适合企业产品、数据平台和工具应用。",
                            vec![
                                icon_sample("carbon", Icon::new(CarbonIconName::CheckmarkFilled).size_lg().color(theme.success.base), "CheckmarkFilled"),
                                icon_sample("carbon", Icon::new(CarbonIconName::Search).size_lg().color(theme.info.base), "Search"),
                                icon_sample("carbon", Icon::new(CarbonIconName::Settings).size_lg().color(theme.primary.base), "Settings"),
                            ],
                        )
                        .into_any_element(),
                        library_card(
                            "Material",
                            "liora::icons_material::IconName",
                            "Material Symbols 支持 Outlined / Round / Sharp / Twotone 等命名后缀。",
                            vec![
                                icon_sample("material", Icon::new(MaterialIconName::SearchOutlined).size_lg().color(theme.info.base), "SearchOutlined"),
                                icon_sample("material", Icon::new(MaterialIconName::SettingsOutlined).size_lg().color(theme.primary.base), "SettingsOutlined"),
                                icon_sample("material", Icon::new(MaterialIconName::CheckCircleOutlined).size_lg().color(theme.success.base), "CheckCircleOutlined"),
                            ],
                        )
                        .into_any_element(),
                    ]),
                ))
                .child(section(
                    "Copy-ready imports",
                    "Gallery 展示常用入口；Docs 的 Icon Libraries 页面提供全量 IconName 清单，点击行或 Copy 按钮即可复制完整 Rust 路径。",
                    Space::new().wrap().gap_md().children(
                        self.copies.iter().enumerate().map(|(index, copy)| {
                            Space::new()
                                .gap_xs()
                                .child(Text::new(COPY_PATHS[index]).xs())
                                .child(copy.clone())
                        }),
                    ),
                )),
        )
    }
}

fn library_card(
    title: &'static str,
    module: &'static str,
    description: &'static str,
    icons: Vec<impl IntoElement>,
) -> impl IntoElement {
    showcase_card(
        title,
        description,
        Space::new()
            .vertical()
            .gap_md()
            .child(Tag::new(module).info().plain())
            .child(Grid::new().fit_item_lg().gap_md().children(icons)),
    )
}

fn icon_sample(library: &'static str, icon: Icon, label: &'static str) -> impl IntoElement {
    let group = format!("icon-library-sample-{library}-{label}");
    let icon_group = group.clone();
    let text_group = group.clone();

    GridItem::new(
        Space::new()
            .vertical()
            .align_center()
            .gap_md()
            .child(icon.size_xl().group_hover_primary(icon_group))
            .child(
                Text::new(label)
                    .xs()
                    .nowrap()
                    .selectable(false)
                    .group_hover_primary(text_group),
            ),
    )
    .hover_group(group)
}
