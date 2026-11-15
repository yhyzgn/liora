use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, row_md, section, showcase_card, showcase_grid};
use liora_components::{HighlightedText, Space};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| HighlightedTextDemo).into()
}

struct HighlightedTextDemo;

impl Render for HighlightedTextDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "HighlightedText 搜索高亮",
            "原生 GPUI 字符位置匹配；整行可选择和复制，不依赖网页引擎。",
            Space::new().vertical().gap_xl().child(section(
                "搜索结果",
                "匹配位置按 Unicode 字符计数，重复位置、越界位置被忽略。",
                showcase_grid(vec![
                    showcase_card(
                        "匹配强调",
                        "加粗和主题强调色两种语义。",
                        row_md(vec![
                            HighlightedText::new("在 Liora 搜索中文🙂结果")
                                .indices([3, 4, 5, 6])
                                .into_any_element(),
                            HighlightedText::new("Find results in native GPUI")
                                .indices([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
                                .accent()
                                .into_any_element(),
                        ]),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}
