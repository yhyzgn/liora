use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::Space;

use liora_components::layout_helpers::{page, section};

#[path = "../../../liora-docs/content/snippets/badge/basic.rs"]
mod basic_example;
#[path = "../../../liora-docs/content/snippets/badge/dot.rs"]
mod dot_example;
#[path = "../../../liora-docs/content/snippets/badge/max.rs"]
mod max_example;
#[path = "../../../liora-docs/content/snippets/badge/visibility.rs"]
mod visibility_example;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| BadgeDemo { hidden: false }).into()
}

struct BadgeDemo {
    hidden: bool,
}

impl Render for BadgeDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Badge 徽章",
            "按钮和图标右上角的提示信息。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "展示五种语义类型、零值和文本徽章。",
                    basic_example::badge_basic(),
                ))
                .child(section(
                    "最大值",
                    "整数计数超过最大值时显示 max+；大整数同样封顶，其他文本保持原样。",
                    max_example::badge_max(),
                ))
                .child(section(
                    "小红点",
                    "使用 dot 模式提示有更新。",
                    dot_example::badge_dot(),
                ))
                .child(section(
                    "显示与隐藏",
                    "切换数字和圆点徽章；子控件保持可用，隐藏不改变布局。",
                    visibility_example::badge_visibility(
                        self.hidden,
                        cx.listener(|view, _, _, cx| {
                            view.hidden = !view.hidden;
                            cx.notify();
                        }),
                    ),
                )),
        )
    }
}
