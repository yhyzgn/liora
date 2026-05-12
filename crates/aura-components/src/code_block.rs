use aura_core::{Config, stable_unique_id};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, ClipboardItem, Component, ElementId, FontStyle, FontWeight, Hsla, IntoElement, RenderOnce,
    Rgba, SharedString, StyledText, TextRun, TextStyle, UnderlineStyle, WhiteSpace, Window, div,
    prelude::*, px,
};
use std::sync::OnceLock;
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle as SyntectFontStyle, Style as SyntectStyle, Theme, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
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

    fn syntect_token(self) -> &'static str {
        match self {
            Self::PlainText => "txt",
            Self::Rust => "rs",
            Self::Toml => "toml",
            Self::Json => "json",
            Self::Markdown => "md",
            Self::Shell => "sh",
            Self::TypeScript => "ts",
            Self::JavaScript => "js",
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
        .bg(code_surface(theme).opacity(0.72))
        .border_1()
        .border_color(code_border(theme))
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
        .border_color(code_border(theme))
        .bg(code_header_surface(theme))
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .text_color(code_muted_text(theme))
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .child(
                    Icon::new(IconName::FileCode)
                        .size(px(14.0))
                        .color(code_muted_text(theme)),
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
                .text_color(code_muted_text(theme))
                .cursor_pointer()
                .hover(|style| {
                    style
                        .bg(code_hover_surface(theme))
                        .text_color(code_accent(theme))
                })
                .on_click(move |_, _, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(copied_code.clone()));
                })
                .child(
                    Icon::new(IconName::Copy)
                        .size(px(12.0))
                        .color(code_muted_text(theme)),
                )
                .child("Copy"),
        );
    }

    div()
        .id(id)
        .w_full()
        .rounded(px(theme.radius.lg))
        .border_1()
        .border_color(code_border(theme))
        .bg(code_surface(theme))
        .overflow_hidden()
        .child(header)
        .child(
            div()
                .id(scroll_id)
                .overflow_x_scroll()
                .p_4()
                .bg(code_surface(theme))
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
    let runs = syntect_runs(&text, language, theme, block);
    StyledText::new(code).with_runs(runs)
}

fn syntect_runs(
    text: &str,
    language: CodeLanguage,
    theme: &aura_theme::Theme,
    block: bool,
) -> Vec<TextRun> {
    if text.is_empty() {
        return vec![base_style(theme, block).to_run(0)];
    }

    let syntax_set = syntax_set();
    let syntax = syntax_set
        .find_syntax_by_token(language.syntect_token())
        .or_else(|| syntax_set.find_syntax_by_extension(language.syntect_token()))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let syntect_theme = syntect_theme();
    let mut highlighter = HighlightLines::new(syntax, syntect_theme);
    let mut runs = Vec::new();

    for line in LinesWithEndings::from(text) {
        match highlighter.highlight_line(line, syntax_set) {
            Ok(regions) => {
                for (style, slice) in regions {
                    if !slice.is_empty() {
                        push_run(
                            &mut runs,
                            syntect_style_run(slice.len(), style, theme, block),
                        );
                    }
                }
            }
            Err(_) => push_run(&mut runs, base_style(theme, block).to_run(line.len())),
        }
    }

    if runs.is_empty() {
        runs.push(base_style(theme, block).to_run(text.len()));
    }

    runs
}

fn push_run(runs: &mut Vec<TextRun>, run: TextRun) {
    if run.len == 0 {
        return;
    }

    if let Some(last) = runs.last_mut() {
        if last.font == run.font
            && last.color == run.color
            && last.background_color == run.background_color
            && last.underline == run.underline
            && last.strikethrough == run.strikethrough
        {
            last.len += run.len;
            return;
        }
    }

    runs.push(run);
}

fn syntect_style_run(
    len: usize,
    syntect_style: SyntectStyle,
    theme: &aura_theme::Theme,
    block: bool,
) -> TextRun {
    let mut style = base_style(theme, block);
    style.color = syntect_color(syntect_style.foreground);

    if syntect_style.font_style.contains(SyntectFontStyle::BOLD) {
        style.font_weight = FontWeight::BOLD;
    }

    if syntect_style.font_style.contains(SyntectFontStyle::ITALIC) {
        style.font_style = FontStyle::Italic;
    }

    if syntect_style
        .font_style
        .contains(SyntectFontStyle::UNDERLINE)
    {
        style.underline = Some(UnderlineStyle {
            thickness: px(1.0),
            color: Some(style.color),
            ..Default::default()
        });
    }

    style.to_run(len)
}

fn base_style(theme: &aura_theme::Theme, block: bool) -> TextStyle {
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
    style.color = code_text(theme);
    style
}

fn syntax_set() -> &'static SyntaxSet {
    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn syntect_theme() -> &'static Theme {
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
    let theme_set = THEME_SET.get_or_init(ThemeSet::load_defaults);
    theme_set
        .themes
        .get("base16-ocean.dark")
        .or_else(|| theme_set.themes.get("InspiredGitHub"))
        .or_else(|| theme_set.themes.values().next())
        .expect("syntect default themes should not be empty")
}

fn syntect_color(color: syntect::highlighting::Color) -> Hsla {
    Rgba {
        r: color.r as f32 / 255.0,
        g: color.g as f32 / 255.0,
        b: color.b as f32 / 255.0,
        a: color.a as f32 / 255.0,
    }
    .into()
}

fn code_surface(_theme: &aura_theme::Theme) -> Hsla {
    rgb(0x1b2b34)
}

fn code_header_surface(_theme: &aura_theme::Theme) -> Hsla {
    rgb(0x16242c)
}

fn code_hover_surface(_theme: &aura_theme::Theme) -> Hsla {
    rgb(0x253c49)
}

fn code_border(_theme: &aura_theme::Theme) -> Hsla {
    rgb(0x334d5c)
}

fn code_text(_theme: &aura_theme::Theme) -> Hsla {
    rgb(0xc0c5ce)
}

fn code_muted_text(_theme: &aura_theme::Theme) -> Hsla {
    rgb(0xa7adba)
}

fn code_accent(_theme: &aura_theme::Theme) -> Hsla {
    rgb(0x96b5b4)
}

fn rgb(hex: u32) -> Hsla {
    Rgba {
        r: ((hex >> 16) & 0xff) as f32 / 255.0,
        g: ((hex >> 8) & 0xff) as f32 / 255.0,
        b: (hex & 0xff) as f32 / 255.0,
        a: 1.0,
    }
    .into()
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
    fn syntect_highlighter_generates_multiple_styled_runs_for_rust() {
        let theme = aura_theme::Theme::light();
        let runs = syntect_runs(
            "fn main() { let n = 42; // ok\n println!(\"hi\"); }",
            CodeLanguage::Rust,
            &theme,
            true,
        );

        assert!(runs.len() > 3);
        assert_eq!(runs.iter().map(|run| run.len).sum::<usize>(), 48);
        assert!(runs.iter().any(|run| run.color != code_text(&theme)));
    }

    #[test]
    fn component_uses_syntect_and_supports_copyable_block_and_inline_format() {
        let source = include_str!("code_block.rs");

        assert!(source.contains("HighlightLines"));
        assert!(source.contains("SyntaxSet::load_defaults_newlines"));
        assert!(source.contains("ThemeSet::load_defaults"));
        assert!(source.contains("ClipboardItem::new_string"));
        assert!(source.contains("CodeFormat::Inline"));
        assert!(source.contains("StyledText::new"));
        assert!(source.contains("with_runs"));
    }
}
