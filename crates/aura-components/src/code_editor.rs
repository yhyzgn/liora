use crate::{CodeBlock, CodeLanguage, CodeTheme, Input};
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Hsla, IntoElement, Pixels, Render, SharedString,
    Window, div, prelude::*, px, rgb,
};
use std::sync::Arc;

pub type CodeEditorChangeCallback = dyn Fn(&str, &mut Context<CodeEditor>) + 'static;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl CodeDiagnosticSeverity {
    fn label(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    fn color(self, theme: &aura_theme::Theme) -> Hsla {
        match self {
            Self::Info => theme.info.base,
            Self::Warning => theme.warning.base,
            Self::Error => theme.danger.base,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeDiagnostic {
    pub line: usize,
    pub column: usize,
    pub severity: CodeDiagnosticSeverity,
    pub message: SharedString,
}

impl CodeDiagnostic {
    pub fn new(
        line: usize,
        column: usize,
        severity: CodeDiagnosticSeverity,
        message: impl Into<SharedString>,
    ) -> Self {
        Self {
            line: line.max(1),
            column: column.max(1),
            severity,
            message: message.into(),
        }
    }

    pub fn info(line: usize, column: usize, message: impl Into<SharedString>) -> Self {
        Self::new(line, column, CodeDiagnosticSeverity::Info, message)
    }

    pub fn warning(line: usize, column: usize, message: impl Into<SharedString>) -> Self {
        Self::new(line, column, CodeDiagnosticSeverity::Warning, message)
    }

    pub fn error(line: usize, column: usize, message: impl Into<SharedString>) -> Self {
        Self::new(line, column, CodeDiagnosticSeverity::Error, message)
    }
}

/// Native code editing surface with line numbers, indentation metadata,
/// syntax-highlight preview and pluggable diagnostics.
///
/// The current MVP deliberately reuses Aura's native `Input` editing core and
/// `CodeBlock` highlighter instead of embedding a Web editor runtime. Future
/// diagnostics providers can update `set_diagnostics` without changing the UI.
pub struct CodeEditor {
    input: Entity<Input>,
    focus_handle: FocusHandle,
    language: CodeLanguage,
    theme: CodeTheme,
    line_numbers: bool,
    tab_size: usize,
    soft_tabs: bool,
    rows: usize,
    height: Option<Pixels>,
    preview: bool,
    diagnostics: Vec<CodeDiagnostic>,
    on_change: Option<Arc<CodeEditorChangeCallback>>,
}

impl CodeEditor {
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let value = value.into();
        let rows = line_count(value.as_ref()).max(8);
        let owner = cx.entity().downgrade();
        let input = cx.new(|cx| {
            Input::new(value, cx)
                .min_rows(rows)
                .on_change(move |value, cx| {
                    let _ = owner.update(cx, |editor, cx| editor.handle_input_change(value, cx));
                })
        });

        Self {
            input,
            focus_handle: cx.focus_handle(),
            language: CodeLanguage::PlainText,
            theme: CodeTheme::Auto,
            line_numbers: true,
            tab_size: 4,
            soft_tabs: true,
            rows,
            height: None,
            preview: true,
            diagnostics: Vec::new(),
            on_change: None,
        }
    }

    pub fn entity(value: impl Into<SharedString>, cx: &mut App) -> Entity<Self> {
        let value = value.into();
        cx.new(|cx| Self::new(value, cx))
    }

    pub fn value(&self, cx: &App) -> SharedString {
        self.input.read(cx).value()
    }

    pub fn set_value(&mut self, value: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_value(value, cx));
        cx.notify();
    }

    pub fn language(mut self, language: impl Into<CodeLanguage>) -> Self {
        self.language = language.into();
        self
    }

    pub fn set_language(&mut self, language: impl Into<CodeLanguage>, cx: &mut Context<Self>) {
        let language = language.into();
        if self.language != language {
            self.language = language;
            cx.notify();
        }
    }

    pub fn theme(mut self, theme: CodeTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn line_numbers(mut self, enabled: bool) -> Self {
        self.line_numbers = enabled;
        self
    }

    pub fn tab_size(mut self, size: usize) -> Self {
        self.tab_size = size.max(1);
        self
    }

    pub fn soft_tabs(mut self, enabled: bool) -> Self {
        self.soft_tabs = enabled;
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }

    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn preview(mut self, preview: bool) -> Self {
        self.preview = preview;
        self
    }

    pub fn diagnostics(mut self, diagnostics: impl IntoIterator<Item = CodeDiagnostic>) -> Self {
        self.diagnostics = diagnostics.into_iter().collect();
        self
    }

    pub fn set_diagnostics(
        &mut self,
        diagnostics: impl IntoIterator<Item = CodeDiagnostic>,
        cx: &mut Context<Self>,
    ) {
        self.diagnostics = diagnostics.into_iter().collect();
        cx.notify();
    }

    pub fn on_change(
        mut self,
        callback: impl Fn(&str, &mut Context<CodeEditor>) + 'static,
    ) -> Self {
        self.on_change = Some(Arc::new(callback));
        self
    }

    pub fn set_on_change(
        &mut self,
        callback: impl Fn(&str, &mut Context<CodeEditor>) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.on_change = Some(Arc::new(callback));
        cx.notify();
    }

    pub fn indent_unit(&self) -> String {
        if self.soft_tabs {
            " ".repeat(self.tab_size)
        } else {
            "\t".to_string()
        }
    }

    fn handle_input_change(&mut self, value: &str, cx: &mut Context<Self>) {
        if let Some(callback) = self.on_change.clone() {
            callback(value, cx);
        }
        cx.notify();
    }
}

impl Focusable for CodeEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CodeEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let value = self.value(cx);
        let line_count = line_count(value.as_ref());
        let rows = self.rows.max(line_count).max(1);
        self.input.update(cx, |input, cx| {
            if input.min_rows != rows {
                input.set_min_rows(rows, cx);
            }
        });

        let indent_label = if self.soft_tabs {
            format!("spaces:{}", self.tab_size)
        } else {
            "tabs".to_string()
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .rounded(px(theme.radius.lg))
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .overflow_hidden()
            .when_some(self.height, |s, height| s.h(height))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.hover.opacity(0.52))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(theme.neutral.text_1)
                            .child(
                                Icon::new(IconName::FileCode)
                                    .size(px(14.0))
                                    .color(theme.primary.base),
                            )
                            .child("CodeEditor"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .text_xs()
                            .text_color(theme.neutral.text_3)
                            .child(self.language.label())
                            .child(indent_label)
                            .child(format!("{} lines", line_count)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_start()
                    .min_h(px(220.0))
                    .bg(theme.neutral.hover.opacity(0.24))
                    .child(if self.line_numbers {
                        line_number_gutter(line_count, theme.neutral.text_3).into_any_element()
                    } else {
                        div().into_any_element()
                    })
                    .child(
                        div()
                            .flex_1()
                            .p_3()
                            .font_family(".ZedMono")
                            .text_sm()
                            .child(self.input.clone()),
                    ),
            )
            .when(!self.diagnostics.is_empty(), |s| {
                s.child(render_diagnostics(&self.diagnostics, &theme))
            })
            .when(self.preview, |s| {
                s.child(
                    div()
                        .border_t_1()
                        .border_color(theme.neutral.border)
                        .p_3()
                        .child(
                            div()
                                .mb_2()
                                .text_xs()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(theme.neutral.text_3)
                                .child("Syntax preview"),
                        )
                        .child(
                            CodeBlock::new(value)
                                .language(self.language)
                                .theme(self.theme)
                                .copyable(false)
                                .selectable(true),
                        ),
                )
            })
    }
}

fn line_count(value: &str) -> usize {
    value.lines().count().max(1)
}

fn line_number_gutter(line_count: usize, color: Hsla) -> gpui::Div {
    let mut gutter = div()
        .flex_none()
        .w(px(52.0))
        .px_3()
        .py_4()
        .border_r_1()
        .border_color(rgb(0xe2e8f0))
        .font_family(".ZedMono")
        .text_xs()
        .text_color(color)
        .flex()
        .flex_col()
        .items_end()
        .gap_1();

    for line in 1..=line_count {
        gutter = gutter.child(format!("{line}"));
    }

    gutter
}

fn render_diagnostics(diagnostics: &[CodeDiagnostic], theme: &aura_theme::Theme) -> gpui::Div {
    let mut panel = div()
        .flex()
        .flex_col()
        .gap_1()
        .border_t_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.hover.opacity(0.36))
        .px_4()
        .py_3();

    for diagnostic in diagnostics {
        let color = diagnostic.severity.color(theme);
        panel = panel.child(
            div()
                .flex()
                .items_start()
                .gap_2()
                .text_sm()
                .child(div().mt(px(7.0)).size(px(6.0)).rounded_full().bg(color))
                .child(
                    div()
                        .flex_1()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(color)
                                .child(format!(
                                    "{} at {}:{}",
                                    diagnostic.severity.label(),
                                    diagnostic.line,
                                    diagnostic.column
                                )),
                        )
                        .child(
                            div()
                                .text_color(theme.neutral.text_2)
                                .child(diagnostic.message.clone()),
                        ),
                ),
        );
    }

    panel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_constructors_clamp_to_one_based_locations() {
        let diagnostic = CodeDiagnostic::warning(0, 0, "missing semicolon");
        assert_eq!(diagnostic.line, 1);
        assert_eq!(diagnostic.column, 1);
        assert_eq!(diagnostic.severity, CodeDiagnosticSeverity::Warning);
    }

    #[test]
    fn code_editor_exposes_planned_mvp_api() {
        let source = include_str!("code_editor.rs");
        assert!(source.contains("pub struct CodeEditor"));
        assert!(source.contains("line_numbers"));
        assert!(source.contains("tab_size"));
        assert!(source.contains("soft_tabs"));
        assert!(source.contains("diagnostics"));
        assert!(source.contains("CodeBlock::new"));
        assert!(source.contains("on_change"));
    }
}
