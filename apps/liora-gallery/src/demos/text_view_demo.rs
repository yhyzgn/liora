use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{Space, TextView, TextViewBlock};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TextViewDemo).into()
}

struct TextViewDemo;

impl Render for TextViewDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "TextView 文档视图",
            "面向帮助页、About、配置说明和内置文档的轻量原生文档渲染组件。",
            Space::new().vertical().gap_xl().child(section(
                "Document showcase",
                "文档示例统一放入宽卡片，避免不同 TextView 模式在页面上随机占宽。",
                showcase_stack(vec![
                    structured_document_card(),
                    markdown_subset_card(),
                    inline_document_card(),
                ]),
            )),
        )
    }
}

fn structured_document_card() -> gpui::AnyElement {
    showcase_card_wide(
        "结构化文档块",
        "直接用 block model 组合标题、段落、引用、列表和代码块。",
        TextView::new([
            TextViewBlock::heading(2, "Application bootstrap"),
            TextViewBlock::paragraph("Initialize Liora once, then compose native GPUI windows with reusable Liora components."),
            TextViewBlock::quote("TextView is intentionally lightweight: use Docs for full documentation chrome, TextView for app-local documents."),
            TextViewBlock::unordered([
                "Native selectable text",
                "Theme-aware quote and code surfaces",
                "Copyable code blocks",
            ]),
            TextViewBlock::code("liora_components::init_liora(cx);", "rust"),
        ])
        .framed(true)
        .max_width(px(720.0)),
    )
    .into_any_element()
}

fn markdown_subset_card() -> gpui::AnyElement {
    showcase_card_wide(
        "快速 Markdown 子集",
        "适合应用内 help/about，不支持完整 CommonMark，也不会引入 WebView。",
        TextView::from_plain_markdown(
            "# Release notes\n\nLiora renders documents as native GPUI elements.\n\n> Keep SDK docs close to product behavior.\n\n1. Parse a small Markdown subset\n2. Render reusable component blocks\n\n```rust\nTextView::from_plain_markdown(markdown)\n```",
        )
        .framed(true)
        .max_width(px(720.0)),
    )
    .into_any_element()
}

fn inline_document_card() -> gpui::AnyElement {
    showcase_card_wide(
        "无边框嵌入模式",
        "在设置页、侧栏详情或空状态中嵌入文档内容时可以关闭外框。",
        TextView::new([
            TextViewBlock::heading(3, "Inline help"),
            TextViewBlock::paragraph("Use max_width for readable line length and selectable(false) for purely decorative guidance."),
            TextViewBlock::Divider,
            TextViewBlock::ordered([
                "Keep content concise",
                "Use real components",
                "Avoid browser runtime",
            ]),
        ])
        .selectable(false)
        .max_width(px(720.0)),
    )
    .into_any_element()
}
