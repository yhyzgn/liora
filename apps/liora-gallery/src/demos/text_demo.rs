use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*, px};
use liora_components::{Divider, Space, Text, TextBlock};

use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};

pub fn render(cx: &mut App) -> Entity<TextDemo> {
    cx.new(|_| TextDemo)
}

pub struct TextDemo;

impl Render for TextDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<liora_core::Config>().theme.clone();

        page(
            "Text 文本",
            "Text 是 Liora 的基础文本与轻量文档入口。默认支持鼠标拖拽选中文字；装饰性文字可显式关闭选择。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础文本",
                    "颜色、字号、字重、斜体、下划线、删除线和背景样式。尝试拖拽选择下面任意一句文字。",
                    showcase_stack(vec![
                        showcase_card_wide(
                            "默认可选择文本",
                            "Text::new(...) 默认启用 selectable，并支持 Ctrl/Cmd + A、Ctrl/Cmd + C。",
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Text::new("Drag across this sentence: it should select like native text."))
                                .child(Text::new("Primary text").text_color(theme.primary.base).bold())
                                .child(Text::new("Success text").text_color(theme.success.base))
                                .child(Text::new("Warning italic text").text_color(theme.warning.base).italic())
                                .child(Text::new("Underlined text").underline())
                                .child(Text::new("Strikethrough text").strikethrough())
                                .child(Text::new("inline code style").code_style(&theme)),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "自动换行与 opt-out",
                            "正文建议保持默认可选择；只有状态徽标、按钮装饰文字等场景才关闭 selectable。",
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Text::new("This is a long selectable Text sample. It wraps inside the available card width while still allowing mouse drag selection across the wrapped lines.").wrap())
                                .child(
                                    Text::new("Decorative label with selection disabled")
                                        .selectable(false)
                                        .text_color(theme.neutral.text_3)
                                        .bg(theme.neutral.hover),
                                ),
                        )
                        .into_any_element(),
                    ]),
                ))
                .child(Divider::new())
                .child(section(
                    "轻量文档",
                    "Text::document 和 Text::markdown 覆盖 About、Help、Release notes 等原生文档场景。",
                    showcase_card_wide(
                        "Text::document",
                        "标题、段落、引用和列表文字默认都可选择；代码块保持自身复制能力。",
                        Text::document([
                            TextBlock::heading(2, "Release notes"),
                            TextBlock::paragraph("Text documents render as native GPUI components and keep selectable typography by default."),
                            TextBlock::quote("Use selectable(false) only for decorative guidance."),
                            TextBlock::unordered([
                                "Selectable headings and paragraphs",
                                "Theme-aware surfaces",
                                "No WebView or DOM runtime",
                            ]),
                            TextBlock::code("Text::document(blocks).framed(true)", "rust"),
                        ])
                        .framed(true)
                        .max_width(px(720.0)),
                    ),
                )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn text_demo_covers_selectable_text_and_document_mode() {
        let source = include_str!("text_demo.rs");

        assert!(source.contains("Text::new"));
        assert!(source.contains("selectable(false)"));
        assert!(source.contains("Text::document"));
        assert!(source.contains("Drag across this sentence"));
    }
}
