use gpui::{App, AppContext, Entity};
use liora_components::{
    CodeCompletionEdit, CodeCompletionItem, CodeEditor, CodeHover, CodeLanguage, CodeTheme,
};

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

/// Deliver an asynchronous host response on the GPUI app thread.
pub fn deliver_completion(
    editor: &Entity<CodeEditor>,
    token: &liora_components::CodeEditorRequestToken,
    range: std::ops::Range<usize>,
    cx: &mut App,
) -> bool {
    editor.update(cx, |editor, cx| {
        editor.set_completion_edits_for_request(
            token,
            [CodeCompletionEdit::new(
                CodeCompletionItem::new("Space constructor"),
                range,
                "Space::new()",
            )],
            cx,
        )
    })
}

pub fn replace_matching_text(editor: &Entity<CodeEditor>, cx: &mut App) -> usize {
    editor.update(cx, |editor, cx| {
        editor.set_search_query("Space", cx);
        editor.find_next(cx);
        editor.replace_all_matches("Flex", cx)
    })
}
