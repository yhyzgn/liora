use aura_components::{CodeDiagnostic, CodeEditor, CodeLanguage};
use gpui::{App, AppContext, Entity};

const SOURCE: &str = r#"export function run(value: number) {
  return value.toString()
}
"#;

pub fn code_editor_diagnostics(cx: &mut App) -> Entity<CodeEditor> {
    cx.new(|cx| {
        CodeEditor::new(SOURCE, cx)
            .language(CodeLanguage::TypeScript)
            .tab_size(2)
            .diagnostics_provider(|source| {
                source
                    .lines()
                    .enumerate()
                    .filter(|(_, line)| {
                        line.trim_start().starts_with("return") && !line.ends_with(';')
                    })
                    .map(|(index, _)| {
                        CodeDiagnostic::warning(
                            index + 1,
                            1,
                            "Return line should end with a semicolon.",
                        )
                    })
                    .collect()
            })
            .diagnostics([
                CodeDiagnostic::warning(2, 3, "Prefer an explicit return type."),
                CodeDiagnostic::error(2, 25, "Missing semicolon according to project style."),
            ])
    })
}
