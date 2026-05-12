use aura_core::{Config, stable_unique_id};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, ClipboardItem, Component, ElementId, FontWeight, IntoElement, RenderOnce, SharedString,
    StyledText, TextRun, TextStyle, WhiteSpace, Window, div, prelude::*, px,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeLanguage {
    PlainText,
    Rust,
    Toml,
    Json,
    Markdown,
    Shell,
    TypeScript,
    JavaScript,
}

impl CodeLanguage {
    pub fn label(self) -> &'static str {
        match self {
            Self::PlainText => "text",
            Self::Rust => "rust",
            Self::Toml => "toml",
            Self::Json => "json",
            Self::Markdown => "markdown",
            Self::Shell => "shell",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
        }
    }

    pub fn from_label(label: &str) -> Self {
        match label.trim().to_ascii_lowercase().as_str() {
            "rs" | "rust" => Self::Rust,
            "toml" => Self::Toml,
            "json" => Self::Json,
            "md" | "markdown" => Self::Markdown,
            "sh" | "bash" | "shell" | "zsh" => Self::Shell,
            "ts" | "tsx" | "typescript" => Self::TypeScript,
            "js" | "jsx" | "javascript" => Self::JavaScript,
            _ => Self::PlainText,
        }
    }
}

impl From<&str> for CodeLanguage {
    fn from(value: &str) -> Self {
        Self::from_label(value)
    }
}

impl From<String> for CodeLanguage {
    fn from(value: String) -> Self {
        Self::from_label(&value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeFormat {
    Block,
    Inline,
}

#[derive(Clone)]
struct HighlightSpan {
    start: usize,
    end: usize,
    kind: HighlightKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HighlightKind {
    Keyword,
    String,
    Number,
    Comment,
    Function,
    Punctuation,
}

pub struct CodeBlock {
    code: SharedString,
    language: CodeLanguage,
    format: CodeFormat,
    copyable: bool,
    id: Option<ElementId>,
}

impl CodeBlock {
    pub fn new(code: impl Into<SharedString>) -> Self {
        Self {
            code: code.into(),
            language: CodeLanguage::PlainText,
            format: CodeFormat::Block,
            copyable: true,
            id: None,
        }
    }

    pub fn language(mut self, language: impl Into<CodeLanguage>) -> Self {
        self.language = language.into();
        self
    }

    pub fn rust(self) -> Self {
        self.language(CodeLanguage::Rust)
    }

    pub fn toml(self) -> Self {
        self.language(CodeLanguage::Toml)
    }

    pub fn json(self) -> Self {
        self.language(CodeLanguage::Json)
    }

    pub fn markdown(self) -> Self {
        self.language(CodeLanguage::Markdown)
    }

    pub fn shell(self) -> Self {
        self.language(CodeLanguage::Shell)
    }

    pub fn typescript(self) -> Self {
        self.language(CodeLanguage::TypeScript)
    }

    pub fn javascript(self) -> Self {
        self.language(CodeLanguage::JavaScript)
    }

    pub fn format(mut self, format: CodeFormat) -> Self {
        self.format = format;
        self
    }

    pub fn inline(mut self) -> Self {
        self.format = CodeFormat::Inline;
        self.copyable = false;
        self
    }

    pub fn copyable(mut self, copyable: bool) -> Self {
        self.copyable = copyable;
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl RenderOnce for CodeBlock {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self.id.clone().unwrap_or_else(|| {
            stable_unique_id(
                format!(
                    "aura-code-block:{}:{}:{:?}:copyable={}",
                    self.language.label(),
                    self.code.as_ref(),
                    self.format,
                    self.copyable
                ),
                "aura-code-block",
                window,
                cx,
            )
            .into()
        });

        match self.format {
            CodeFormat::Inline => render_inline_code(self.code, self.language, &theme),
            CodeFormat::Block => {
                render_block_code(id, self.code, self.language, self.copyable, &theme)
            }
        }
    }
}

impl IntoElement for CodeBlock {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn render_inline_code(
    code: SharedString,
    language: CodeLanguage,
    theme: &aura_theme::Theme,
) -> gpui::AnyElement {
    div()
        .rounded(px(theme.radius.sm))
        .bg(theme.neutral.hover)
        .px_1()
        .py_0p5()
        .child(render_highlighted_text(code, language, theme, false))
        .into_any_element()
}

fn render_block_code(
    id: ElementId,
    code: SharedString,
    language: CodeLanguage,
    copyable: bool,
    theme: &aura_theme::Theme,
) -> gpui::AnyElement {
    let copied_code = code.to_string();
    let scroll_id = format!("{id}-scroll");

    let mut header = div()
        .flex()
        .items_center()
        .justify_between()
        .gap_2()
        .px_4()
        .py_2()
        .border_b_1()
        .border_color(theme.neutral.border)
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .text_color(theme.neutral.text_3)
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .child(
                    Icon::new(IconName::FileCode)
                        .size(px(14.0))
                        .color(theme.neutral.icon),
                )
                .child(language.label()),
        );

    if copyable {
        header = header.child(
            div()
                .id(format!("{id}-copy"))
                .flex()
                .items_center()
                .gap_1()
                .px_2()
                .py_1()
                .rounded(px(theme.radius.sm))
                .text_xs()
                .text_color(theme.neutral.text_3)
                .cursor_pointer()
                .hover(|style| style.bg(theme.neutral.hover).text_color(theme.primary.base))
                .on_click(move |_, _, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(copied_code.clone()));
                })
                .child(
                    Icon::new(IconName::Copy)
                        .size(px(12.0))
                        .color(theme.neutral.icon),
                )
                .child("Copy"),
        );
    }

    div()
        .id(id)
        .w_full()
        .rounded(px(theme.radius.lg))
        .border_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .overflow_hidden()
        .child(header)
        .child(
            div()
                .id(scroll_id)
                .overflow_x_scroll()
                .p_4()
                .bg(theme.neutral.hover.opacity(0.55))
                .child(render_highlighted_text(code, language, theme, true)),
        )
        .into_any_element()
}

fn render_highlighted_text(
    code: SharedString,
    language: CodeLanguage,
    theme: &aura_theme::Theme,
    block: bool,
) -> StyledText {
    let text = code.to_string();
    let spans = highlight_spans(&text, language);
    let runs = text_runs(&text, &spans, theme, block);
    StyledText::new(code).with_runs(runs)
}

fn text_runs(
    text: &str,
    spans: &[HighlightSpan],
    theme: &aura_theme::Theme,
    block: bool,
) -> Vec<TextRun> {
    let mut runs = Vec::new();
    let mut cursor = 0;

    for span in spans {
        if span.start > cursor {
            runs.push(style_run(span.start - cursor, None, theme, block));
        }
        runs.push(style_run(
            span.end - span.start,
            Some(span.kind),
            theme,
            block,
        ));
        cursor = span.end;
    }

    if cursor < text.len() {
        runs.push(style_run(text.len() - cursor, None, theme, block));
    }

    if runs.is_empty() {
        runs.push(style_run(0, None, theme, block));
    }

    runs
}

fn style_run(
    len: usize,
    kind: Option<HighlightKind>,
    theme: &aura_theme::Theme,
    block: bool,
) -> TextRun {
    let mut style = TextStyle::default();
    style.font_family = "Monospace".into();
    style.font_size = px(if block {
        theme.font_size.sm
    } else {
        theme.font_size.md
    })
    .into();
    style.line_height = px(theme.font_size.md * 1.7).into();
    style.white_space = WhiteSpace::Nowrap;
    style.color = theme.neutral.text_1;

    match kind {
        Some(HighlightKind::Keyword) => {
            style.color = theme.primary.base;
            style.font_weight = FontWeight::BOLD;
        }
        Some(HighlightKind::String) => style.color = theme.success.base,
        Some(HighlightKind::Number) => style.color = theme.warning.base,
        Some(HighlightKind::Comment) => style.color = theme.neutral.text_3,
        Some(HighlightKind::Function) => style.color = theme.info.base,
        Some(HighlightKind::Punctuation) => style.color = theme.neutral.text_2,
        None => {}
    }

    style.to_run(len)
}

fn highlight_spans(text: &str, language: CodeLanguage) -> Vec<HighlightSpan> {
    let mut spans = lexical_spans(text);

    if matches!(language, CodeLanguage::PlainText | CodeLanguage::Markdown) {
        spans.retain(|span| matches!(span.kind, HighlightKind::String | HighlightKind::Comment));
        return spans;
    }

    spans
}

fn lexical_spans(text: &str) -> Vec<HighlightSpan> {
    let bytes = text.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        if b == b'/' && bytes.get(i + 1) == Some(&b'/') {
            let start = i;
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            spans.push(span(start, i, HighlightKind::Comment));
            continue;
        }

        if b == b'#' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            spans.push(span(start, i, HighlightKind::Comment));
            continue;
        }

        if b == b'"' || b == b'\'' || b == b'`' {
            let quote = b;
            let start = i;
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' {
                    i = (i + 2).min(bytes.len());
                    continue;
                }
                if bytes[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            spans.push(span(start, i, HighlightKind::String));
            continue;
        }

        if b.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.') {
                i += 1;
            }
            spans.push(span(start, i, HighlightKind::Number));
            continue;
        }

        if is_ident_start(b) {
            let start = i;
            i += 1;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let word = &text[start..i];
            if is_keyword(word) {
                spans.push(span(start, i, HighlightKind::Keyword));
            } else if next_non_ws(bytes, i) == Some(b'(') {
                spans.push(span(start, i, HighlightKind::Function));
            }
            continue;
        }

        if matches!(
            b,
            b'{' | b'}' | b'(' | b')' | b'[' | b']' | b':' | b';' | b',' | b'.'
        ) {
            spans.push(span(i, i + 1, HighlightKind::Punctuation));
        }

        i += 1;
    }

    spans
}

fn span(start: usize, end: usize, kind: HighlightKind) -> HighlightSpan {
    HighlightSpan { start, end, kind }
}

fn is_ident_start(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphabetic()
}

fn is_ident_continue(byte: u8) -> bool {
    byte == b'_' || byte == b'-' || byte.is_ascii_alphanumeric()
}

fn next_non_ws(bytes: &[u8], mut i: usize) -> Option<u8> {
    while i < bytes.len() {
        if !bytes[i].is_ascii_whitespace() {
            return Some(bytes[i]);
        }
        i += 1;
    }
    None
}

fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "as" | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "export"
            | "false"
            | "fn"
            | "for"
            | "from"
            | "if"
            | "impl"
            | "import"
            | "in"
            | "let"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "null"
            | "pub"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "undefined"
            | "use"
            | "where"
            | "while"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_labels_parse_common_aliases() {
        assert_eq!(CodeLanguage::from_label("rs"), CodeLanguage::Rust);
        assert_eq!(CodeLanguage::from_label("bash"), CodeLanguage::Shell);
        assert_eq!(CodeLanguage::from_label("tsx"), CodeLanguage::TypeScript);
        assert_eq!(CodeLanguage::from_label("unknown"), CodeLanguage::PlainText);
    }

    #[test]
    fn highlighter_marks_keywords_strings_numbers_comments_and_functions() {
        let spans = highlight_spans(
            "fn main() { let n = 42; // ok\n println!(\"hi\"); }",
            CodeLanguage::Rust,
        );

        assert!(spans.iter().any(|span| span.kind == HighlightKind::Keyword));
        assert!(spans.iter().any(|span| span.kind == HighlightKind::String));
        assert!(spans.iter().any(|span| span.kind == HighlightKind::Number));
        assert!(spans.iter().any(|span| span.kind == HighlightKind::Comment));
        assert!(
            spans
                .iter()
                .any(|span| span.kind == HighlightKind::Function)
        );
    }

    #[test]
    fn component_supports_copyable_block_and_inline_format() {
        let source = include_str!("code_block.rs");

        assert!(source.contains("ClipboardItem::new_string"));
        assert!(source.contains("CodeFormat::Inline"));
        assert!(source.contains("StyledText::new"));
        assert!(source.contains("with_runs"));
    }
}
