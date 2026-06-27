use gpui::{App, AppContext, Entity};
use liora_components::{CodeCompletionItem, CodeEditor, CodeHover, CodeLanguage, CodeTheme};

const SOURCE: &str = r#"use liora_components::{Button, Space};

pub fn panel() -> impl gpui::IntoElement {
    Space::new().child(Button::new("Run"))
}
"#;

pub fn code_editor_advanced(cx: &mut App) -> Entity<CodeEditor> {
    cx.new(|cx| {
        CodeEditor::new(SOURCE, cx)
            .language(CodeLanguage::Rust)
            .theme(CodeTheme::OneDark)
            .search_query("Space")
            .completions([
                CodeCompletionItem::new("Space::new")
                    .kind("struct")
                    .detail("layout container"),
                CodeCompletionItem::new("Button::new")
                    .kind("function")
                    .detail("action control"),
                CodeCompletionItem::new("toast_info!")
                    .kind("macro")
                    .detail("show message"),
            ])
            .hover(CodeHover::new(
                "Space::new",
                "Creates a flexible native layout container.",
            ))
    })
}
