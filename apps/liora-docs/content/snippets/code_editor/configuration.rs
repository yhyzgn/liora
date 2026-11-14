use gpui::{App, AppContext, Entity};
use liora_components::{
    CodeCompletionItem, CodeEditor, CodeEditorOptions, CodeLanguage, CodeTheme,
};

const SOURCE: &str = r#"use liora_components::{CodeEditor, CodeEditorOptions};

let options = CodeEditorOptions::default();
// Read-only editors still support selection, copy, scroll and hover.
"#;

pub fn code_editor_configuration(cx: &mut App) -> Entity<CodeEditor> {
    cx.new(|cx| {
        CodeEditor::new(SOURCE, cx)
            .language(CodeLanguage::Rust)
            .theme(CodeTheme::OneDark)
            .options(CodeEditorOptions {
                read_only: true,
                header: false,
                status_bar: true,
                line_numbers: false,
                diagnostics_panel: false,
                completions_panel: true,
                hover_panel: false,
                current_line_highlight: true,
                completion_limit: 3,
                ..CodeEditorOptions::default()
            })
            .completions([
                CodeCompletionItem::new("CodeEditorOptions::default")
                    .kind("config")
                    .detail("start from full chrome"),
                CodeCompletionItem::new("read_only(true)")
                    .kind("builder")
                    .detail("disable mutation commands"),
                CodeCompletionItem::new("current_line_highlight(true)")
                    .kind("builder")
                    .detail("highlight the active row"),
            ])
    })
}
