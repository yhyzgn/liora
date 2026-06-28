use gpui::{AnyView, App, Context, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{Grid, GridItem, Space, Tag, Text};
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
            "用于图标墙、卡片墙、设置入口等二维布局；支持固定列数缩放 item，以及固定 item 尺寸自动改变列数。",
            Space::new().vertical().gap_xl().child(section(
                "Grid showcase",
                "GridItem 默认正方形，适合展示 icon + label；需要内容高度自适应时可调用 rectangular()。",
                showcase_stack(vec![
                    showcase_card_wide(
                        "固定 item 尺寸，自动改变列数",
                        "fit_item_md() 保持 item 尺寸稳定，容器变宽时自动增加横向列数。",
                        Grid::new().fit_item_md().gap_md().children(tool_items()),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "固定列数，缩放 item 宽度",
                        "fit_columns(4) 保持 4 列布局，item 随容器宽度缩放。",
                        Grid::new().fit_columns(4).gap_md().children(metric_items()),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "非正方形内容卡片",
                        "GridItem::rectangular() 适合设置项、摘要卡和文本内容。",
                        Grid::new().fit_columns(3).gap_md().children(setting_items()),
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
    ]
    .into_iter()
    .map(|(icon, label)| {
        GridItem::new(
            Space::new()
                .vertical()
                .align_center()
                .gap_sm()
                .child(Icon::new(icon).size_lg())
                .child(Text::new(label).nowrap()),
        )
    })
    .collect()
}

fn metric_items() -> Vec<impl IntoElement> {
    [
        ("Build", "42s"),
        ("Tests", "128"),
        ("Coverage", "91%"),
        ("Size", "18MB"),
    ]
    .into_iter()
    .map(|(label, value)| {
        GridItem::new(
            Space::new()
                .vertical()
                .align_center()
                .gap_sm()
                .child(Text::new(value).bold())
                .child(Tag::new(label).info().plain()),
        )
    })
    .collect()
}

fn setting_items() -> Vec<impl IntoElement> {
    [
        ("Appearance", "Theme, density, and window frame"),
        ("Editor", "Font, wrapping, and code actions"),
        ("Updates", "Release channel and auto download"),
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
