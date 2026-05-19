use aura_components::{CodeEditor, CodeLanguage, CodeTheme};
use gpui::{App, AppContext, Entity};

const SOURCE: &str = r#"fn main() {
    println!("Hello Aura");
}
"#;

pub fn code_editor_basic(cx: &mut App) -> Entity<CodeEditor> {
    cx.new(|cx| {
        CodeEditor::new(SOURCE, cx)
            .language(CodeLanguage::Rust)
            .theme(CodeTheme::OneDark)
            .line_numbers(true)
            .tab_size(4)
            .soft_tabs(true)
    })
}
