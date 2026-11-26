use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::Space;

use liora_components::layout_helpers::{page, section};

#[path = "../../../liora-docs/content/snippets/breadcrumb/basic.rs"]
mod breadcrumb_basic_example;
#[path = "../../../liora-docs/content/snippets/breadcrumb/clickable.rs"]
mod breadcrumb_clickable_example;
#[path = "../../../liora-docs/content/snippets/breadcrumb/icon.rs"]
mod breadcrumb_icon_example;
#[path = "../../../liora-docs/content/snippets/breadcrumb/separator.rs"]
mod breadcrumb_separator_example;
#[path = "../../../liora-docs/content/snippets/breadcrumb/separator_icon.rs"]
mod breadcrumb_separator_icon_example;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| BreadcrumbDemo).into()
}

struct BreadcrumbDemo;

impl Render for BreadcrumbDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Breadcrumb 面包屑",
            "显示当前页面的路径，快速返回之前的页面。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "默认使用斜杠分隔各级路径。",
                    breadcrumb_basic_example::basic_breadcrumb(),
                ))
                .child(section(
                    "图标类型",
                    "首项可以携带图标来强化入口语义。",
                    breadcrumb_icon_example::icon_breadcrumb(),
                ))
                .child(section(
                    "自定义分隔符 (String)",
                    "使用文本作为路径层级之间的分隔符。",
                    breadcrumb_separator_example::text_separator_breadcrumb(),
                ))
                .child(section(
                    "自定义分隔符 (Icon)",
                    "使用图标作为路径层级之间的分隔符。",
                    breadcrumb_separator_icon_example::icon_separator_breadcrumb(),
                ))
                .child(section(
                    "点击事件",
                    "前置路径项可通过鼠标或 Tab / Enter 执行回调；末项只表示当前位置。",
                    breadcrumb_clickable_example::clickable_breadcrumb(),
                )),
        )
    }
}
