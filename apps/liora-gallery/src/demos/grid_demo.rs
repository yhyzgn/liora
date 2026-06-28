use gpui::{AnyView, App, Context, IntoElement, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{Grid, GridItem, Space, Tag, Text, toast_success};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| GridDemo).into()
}

struct GridDemo;

impl Render for GridDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Grid 栅格墙",
            "二维内容墙组件：既支持固定 item 尺寸并按宽度自动增减列数，也支持固定列数并缩放 item 宽度。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "横向自适应策略",
                    "同一套 Grid 可以覆盖 icon wall、dashboard tile、设置入口和内容卡片等不同密度场景。",
                    showcase_stack(vec![
                        showcase_card_wide(
                            "固定 item 尺寸，列数随宽度变化",
                            "fit_item_md() 保持每个 item 约 104px；容器变宽时自然出现更多列。",
                            Grid::new().fit_item_md().gap_md().children(tool_items()),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "固定列数，item 随宽度缩放",
                            "fit_columns(4) 适合仪表盘、固定信息密度和响应式摘要区域。",
                            Grid::new().fit_columns(4).gap_md().children(metric_items()),
                        )
                        .into_any_element(),
                    ]),
                ))
                .child(section(
                    "尺寸、间距与形态",
                    "展示不同 item preset、gap preset、正方形/矩形卡片和内容对齐方式。",
                    showcase_stack(vec![
                        showcase_card_wide(
                            "紧凑工具墙",
                            "fit_item_sm() + gap_sm() 适合高密度工具入口。",
                            Grid::new().fit_item_sm().gap_sm().children(compact_items()),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "大尺寸图标墙",
                            "fit_item(148px) + gap_lg() 给图标和名称留出更舒展的空间。",
                            Grid::new().fit_item(px(148.0)).gap_lg().children(large_icon_items()),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "矩形设置卡片",
                            "GridItem::rectangular() 取消正方形约束，适合长文本、开关说明和摘要内容。",
                            Grid::new().fit_columns(3).gap_md().children(setting_items()),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "点击复制与 hover group",
                            "GridItem::hover_group(...) 可让内部 Icon/Text 与卡片 hover 同步变成 primary 色。",
                            Grid::new().fit_item_lg().gap_md().children(clickable_items()),
                        )
                        .into_any_element(),
                    ]),
                )),
        )
    }
}

fn tool_items() -> Vec<impl IntoElement> {
    [
        (IconName::Search, "Search"),
        (IconName::Settings, "Settings"),
        (IconName::Palette, "Theme"),
        (IconName::Bell, "Notify"),
        (IconName::FolderOpen, "Files"),
        (IconName::Terminal, "Terminal"),
        (IconName::ChartNoAxesColumn, "Stats"),
        (IconName::CircleQuestionMark, "Help"),
        (IconName::Sparkles, "AI"),
        (IconName::ShieldCheck, "Security"),
    ]
    .into_iter()
    .map(|(icon, label)| icon_tile(icon, label, "grid-tool"))
    .collect()
}

fn compact_items() -> Vec<impl IntoElement> {
    [
        (IconName::Plus, "New"),
        (IconName::Copy, "Copy"),
        (IconName::Save, "Save"),
        (IconName::RefreshCw, "Sync"),
        (IconName::Download, "Export"),
        (IconName::Trash2, "Delete"),
    ]
    .into_iter()
    .map(|(icon, label)| icon_tile(icon, label, "grid-compact"))
    .collect()
}

fn large_icon_items() -> Vec<impl IntoElement> {
    [
        (IconName::LayoutDashboard, "Dashboard"),
        (IconName::PanelsTopLeft, "Workspace"),
        (IconName::Database, "Data Source"),
        (IconName::ChartSpline, "Analytics"),
        (IconName::PackageCheck, "Release"),
        (IconName::LifeBuoy, "Support"),
    ]
    .into_iter()
    .map(|(icon, label)| icon_tile(icon, label, "grid-large"))
    .collect()
}

fn metric_items() -> Vec<impl IntoElement> {
    [
        ("Build", "42s", "success"),
        ("Tests", "128", "info"),
        ("Coverage", "91%", "success"),
        ("Size", "18MB", "warning"),
    ]
    .into_iter()
    .map(|(label, value, tone)| {
        GridItem::new(
            Space::new()
                .vertical()
                .align_center()
                .gap_sm()
                .child(Text::new(value).bold())
                .child(match tone {
                    "success" => Tag::new(label).success().plain(),
                    "warning" => Tag::new(label).warning().plain(),
                    _ => Tag::new(label).info().plain(),
                }),
        )
    })
    .collect()
}

fn setting_items() -> Vec<impl IntoElement> {
    [
        ("Appearance", "Theme, density, window frame, and icon scale"),
        ("Editor", "Font family fallback, wrapping, and code actions"),
        (
            "Updates",
            "Release channel, auto download, and installer policy",
        ),
        (
            "Shortcuts",
            "Keyboard profiles and command palette integration",
        ),
        (
            "Privacy",
            "Telemetry, local cache, and workspace permissions",
        ),
        (
            "Extensions",
            "Plugin discovery, sandboxing, and runtime assets",
        ),
    ]
    .into_iter()
    .map(|(title, description)| {
        GridItem::new(
            Space::new()
                .vertical()
                .align_start()
                .gap_sm()
                .child(Text::new(title).bold())
                .child(Text::new(description).wrap()),
        )
        .rectangular()
        .align_start()
    })
    .collect()
}

fn clickable_items() -> Vec<impl IntoElement> {
    [
        (IconName::MousePointerClick, "Click"),
        (IconName::ClipboardCopy, "Copy"),
        (IconName::BadgeCheck, "Selected"),
        (IconName::Paintbrush, "Hover"),
    ]
    .into_iter()
    .map(|(icon, label)| {
        let group = format!("grid-clickable-{label}");
        let icon_group = group.clone();
        let text_group = group.clone();
        let copied = label.to_string();
        GridItem::new(
            Space::new()
                .vertical()
                .align_center()
                .gap_md()
                .child(Icon::new(icon).size_xl().group_hover_primary(icon_group))
                .child(
                    Text::new(label)
                        .bold()
                        .nowrap()
                        .selectable(false)
                        .group_hover_primary(text_group),
                ),
        )
        .hover_group(group)
        .on_click(move |_, cx| {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(copied.clone()));
            toast_success!("Copied {}", copied);
        })
    })
    .collect()
}

fn icon_tile(icon: IconName, label: &'static str, prefix: &'static str) -> GridItem {
    let group = format!("{prefix}-{label}");
    let icon_group = group.clone();
    let text_group = group.clone();
    GridItem::new(
        Space::new()
            .vertical()
            .align_center()
            .gap_md()
            .child(Icon::new(icon).size_lg().group_hover_primary(icon_group))
            .child(
                Text::new(label)
                    .nowrap()
                    .selectable(false)
                    .group_hover_primary(text_group),
            ),
    )
    .hover_group(group)
    .on_click(move |_, _| {})
}
