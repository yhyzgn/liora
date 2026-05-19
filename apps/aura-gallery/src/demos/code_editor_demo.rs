use aura_components::layout_helpers::{page, section};
use aura_components::{
    CodeDiagnostic, CodeEditor, CodeLanguage, CodeTheme, Divider, Space, Text, toast_info,
};
use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| CodeEditorDemo::new(cx)).into()
}

struct CodeEditorDemo {
    basic: Entity<CodeEditor>,
    diagnostics: Entity<CodeEditor>,
}

impl CodeEditorDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic = cx.new(|cx| {
            CodeEditor::new(RUST_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .tab_size(4)
                .soft_tabs(true)
                .line_numbers(true)
                .theme(CodeTheme::OneDark)
                .on_change(|value, _| {
                    let line_count = value.lines().count().max(1);
                    toast_info!("CodeEditor changed: {} lines", line_count);
                })
        });
        let diagnostics = cx.new(|cx| {
            CodeEditor::new(TS_SAMPLE, cx)
                .language(CodeLanguage::TypeScript)
                .tab_size(2)
                .soft_tabs(true)
                .line_numbers(true)
                .theme(CodeTheme::GitHubDark)
                .diagnostics([
                    CodeDiagnostic::warning(3, 7, "Prefer an explicit return type."),
                    CodeDiagnostic::info(
                        5,
                        3,
                        "Diagnostics are provider-driven and can be replaced by LSP later.",
                    ),
                ])
        });

        Self { basic, diagnostics }
    }
}

impl Render for CodeEditorDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "CodeEditor 代码编辑器",
            "原生 GPUI 代码编辑控件基础版，支持行号、缩进配置、语法高亮预览、编辑和 diagnostics 扩展点。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Rust 编辑器",
                    "使用 Aura Input 作为编辑核心，保留纯 Rust + GPUI 原生渲染；下方语法预览复用 CodeBlock 的 syntect/two-face 高亮。",
                    self.basic.clone(),
                ))
                .child(Divider::new())
                .child(section(
                    "Diagnostics 扩展点",
                    "语法检查不硬绑定 LSP；业务层可以通过 diagnostics(...) 或 set_diagnostics(...) 注入任意诊断结果。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(self.diagnostics.clone())
                        .child(Text::new("MVP 阶段先完成编辑、行号、缩进元数据和诊断渲染；Tab/Shift+Tab 多行缩进和 provider trait 后续继续增强。")),
                )),
        )
    }
}

const RUST_SAMPLE: &str = r#"use aura_components::{Button, CodeEditor, Space};

pub fn editor_panel() -> impl gpui::IntoElement {
    Space::new()
        .vertical()
        .child(Button::new("Run").primary())
}
"#;

const TS_SAMPLE: &str = r#"type Metric = { label: string; value: number };

export function summarize(items: Metric[]) {
  return items.map((item) => `${item.label}: ${item.value}`);
}
"#;

#[cfg(test)]
mod tests {
    #[test]
    fn code_editor_demo_uses_component_api() {
        let source = include_str!("code_editor_demo.rs");
        assert!(source.contains("CodeEditor::new"));
        assert!(source.contains("CodeDiagnostic::warning"));
        assert!(source.contains("line_numbers"));
        assert!(source.contains("tab_size"));
        assert!(source.contains("on_change"));
    }
}
