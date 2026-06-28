use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{Divider, Paragraph, Space, Text};

pub fn render(cx: &mut App) -> Entity<ParagraphDemo> {
    cx.new(|_| ParagraphDemo)
}

pub struct ParagraphDemo;

impl Render for ParagraphDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<liora_core::Config>().theme.clone();

        page(
            "Paragraph 段落",
            "Paragraph 把多个 Text 片段合成为一个连续文本流，适合正文、说明、混排和可复制内容。默认支持鼠标拖拽选择整段文字。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "连续选择",
                    "下面的长段落会自动换行；从第一行拖到后续视觉行时应连续选中同一个 Paragraph，而不是只选中第一行。",
                    showcase_card_wide(
                        "自动换行长段落",
                        "Paragraph 内部使用同一个 SelectableText 状态和统一 TextRun 列表，跨视觉行选择应保持连续。",
                        Paragraph::with_text(
                            "Liora Paragraph is designed for prose. This deliberately long paragraph wraps across multiple visual lines inside the showcase card, so dragging from the first visual line into the second or third visual line should keep one continuous selection range without losing the highlighted region.",
                        ),
                    ),
                ))
                .child(Divider::new())
                .child(section(
                    "混合样式",
                    "多个 Text 片段会合并为一个段落，颜色、字重、斜体、下划线和 inline code 背景都不应破坏连续选择。",
                    showcase_stack(vec![
                        showcase_card_wide(
                            "多 Text 片段混排",
                            "尝试从普通文字拖过高亮、链接式下划线和 inline code 区域。",
                            Paragraph::new()
                                .child(Text::new("Build native desktop apps with "))
                                .child(Text::new("Liora").bold().text_color(theme.primary.base))
                                .child(Text::new(" and official "))
                                .child(Text::new("GPUI").code_style(&theme))
                                .child(Text::new(". Styled Text fragments are rendered as one selectable paragraph, so copy and mouse selection preserve the full prose flow across wrapping lines. "))
                                .child(Text::new("Selectable typography").underline())
                                .child(Text::new(" stays theme-aware in both light and dark modes.")),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "关闭选择",
                            "装饰性提示或不可复制说明可显式关闭 selection。",
                            Paragraph::with_text("This decorative paragraph has selection disabled.")
                                .selectable(false),
                        )
                        .into_any_element(),
                    ]),
                )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn paragraph_demo_is_dedicated_and_covers_wrapped_selection() {
        let source = include_str!("paragraph_demo.rs");

        assert!(source.contains("Paragraph 段落"));
        assert!(source.contains("Paragraph::with_text"));
        assert!(source.contains("Paragraph::new"));
        assert!(source.contains("automatically") || source.contains("自动换行"));
        assert!(source.contains("selectable(false)"));
        assert!(source.contains("code_style"));
    }
}
