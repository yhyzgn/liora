//! Paragraph module.
//!
//! This public module implements the Liora selectable paragraph/text-run composition component. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::{SelectableText, SelectableTextOptions, SelectableTextWrap, Text};
use gpui::{
    App, Component, ElementId, IntoElement, RenderOnce, SharedString, StyledText, TextRun,
    TextStyle, WhiteSpace, Window, div, prelude::*, px,
};
use liora_core::{Config, code_font_family, ui_font_family};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    panic::Location,
};

/// Fluent native GPUI component for rendering Liora paragraph.
pub struct Paragraph {
    children: Vec<Text>,
    selectable: bool,
    id: SharedString,
}

impl Paragraph {
    /// Creates `Paragraph` with default theme-driven styling and no optional callbacks attached.
    #[track_caller]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            selectable: true,
            id: default_paragraph_id("", Location::caller()),
        }
    }

    /// Applies the text preset.
    #[track_caller]
    pub fn with_text(text: impl Into<SharedString>) -> Self {
        let text = text.into();
        let id = default_paragraph_id(text.as_ref(), Location::caller());
        Self {
            children: vec![Text::new(text)],
            selectable: true,
            id,
        }
    }

    /// Adds a child element to the component body.
    pub fn child(mut self, child: Text) -> Self {
        self.children.push(child);
        self
    }

    /// Replaces or appends child elements rendered by the component.
    pub fn children(mut self, children: impl IntoIterator<Item = Text>) -> Self {
        self.children.extend(children);
        self
    }

    /// Toggles whether the rendered text can be selected.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        SelectableText::register_key_bindings(cx);
    }

    fn default_text_style(
        theme: &liora_theme::Theme,
        ui_family: Option<SharedString>,
    ) -> TextStyle {
        let font_size = px(theme.font_size.md);
        let mut style = TextStyle::default();
        style.color = theme.neutral.text_2;
        style.font_size = font_size.into();
        style.line_height = px(theme.font_size.md * 1.6).into();
        style.white_space = WhiteSpace::Normal;
        style.text_overflow = None;
        style.line_clamp = None;
        if let Some(family) = ui_family {
            style.font_family = family;
        }
        style
    }

    fn styled_text_parts(
        &self,
        theme: &liora_theme::Theme,
        code_family: Option<SharedString>,
        ui_family: Option<SharedString>,
    ) -> (SharedString, Vec<TextRun>) {
        let default_style = Self::default_text_style(theme, ui_family);
        let mut full_text = String::new();
        let mut runs: Vec<TextRun> = Vec::new();

        for segment in &self.children {
            if segment.content.is_empty() {
                continue;
            }

            let segment_text = segment.content.clone();
            let text = segment_text.as_ref();
            let leading_glue_len = if runs.is_empty() {
                0
            } else {
                leading_no_line_start_len(text)
            };

            if leading_glue_len > 0 {
                full_text.push_str(&text[..leading_glue_len]);
                if let Some(previous_run) = runs.last_mut() {
                    previous_run.len += leading_glue_len;
                }
            }

            let remaining = &text[leading_glue_len..];
            if !remaining.is_empty() {
                full_text.push_str(remaining);
                let mut segment = segment.clone();
                if segment.is_code_style && segment.font_family.is_none() {
                    if let Some(code_family) = code_family.clone() {
                        segment.font_family = Some(code_family);
                    }
                }
                let mut run = segment.to_text_run(&default_style);
                run.len = remaining.len();
                runs.push(run);
            }
        }

        (full_text.into(), runs)
    }
}

fn default_paragraph_id(seed: &str, location: &Location<'_>) -> SharedString {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    format!(
        "paragraph-{}:{}:{}-{:016x}",
        location.file(),
        location.line(),
        location.column(),
        hasher.finish()
    )
    .into()
}

fn leading_no_line_start_len(text: &str) -> usize {
    let Some(first) = text.chars().next() else {
        return 0;
    };

    if !is_no_line_start_punctuation(first) {
        return 0;
    }

    let mut end = 0;
    let mut saw_punctuation = false;
    for (index, ch) in text.char_indices() {
        if is_no_line_start_punctuation(ch) {
            saw_punctuation = true;
            end = index + ch.len_utf8();
            continue;
        }

        if saw_punctuation && ch.is_whitespace() {
            end = index + ch.len_utf8();
            continue;
        }

        break;
    }

    end
}

fn is_no_line_start_punctuation(ch: char) -> bool {
    matches!(
        ch,
        ':' | '：'
            | ','
            | '，'
            | '.'
            | '。'
            | ';'
            | '；'
            | '!'
            | '！'
            | '?'
            | '？'
            | '、'
            | ')'
            | '）'
            | ']'
            | '】'
            | '}'
            | '》'
            | '」'
            | '』'
            | '”'
            | '’'
    )
}

impl RenderOnce for Paragraph {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let code_family = code_font_family(cx);
        let ui_family = ui_font_family(cx);
        let (full_text, runs) = self.styled_text_parts(theme, Some(code_family), ui_family.clone());
        let font_size = px(theme.font_size.md);

        if self.selectable {
            return SelectableText::view(
                SelectableTextOptions {
                    id: ElementId::from(self.id.clone()),
                    text: full_text,
                    runs,
                    font_size,
                    line_height: font_size * 1.6,
                    text_color: theme.neutral.text_2,
                    wrap: SelectableTextWrap::Normal,
                    key_context: "SelectableText",
                    fill_width: true,
                    font_family: ui_family,
                },
                _window,
                cx,
            );
        }

        let mut paragraph = div()
            .w_full()
            .text_size(font_size)
            .line_height(font_size * 1.6)
            .text_color(theme.neutral.text_2)
            .whitespace_normal()
            .child(StyledText::new(full_text).with_runs(runs));

        if let Some(family) = ui_family {
            paragraph = paragraph.font_family(family);
        }

        paragraph.into_any_element()
    }
}

impl IntoElement for Paragraph {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{FontStyle, FontWeight};

    #[test]
    fn paragraph_default_id_is_stable_across_render_rebuilds() {
        let source = include_str!("paragraph.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("#[track_caller]"));
        assert!(source.contains("fn default_paragraph_id"));
        assert!(!source.contains(r#"liora_core::unique_id("paragraph")"#));
    }

    #[test]
    fn paragraph_defaults_to_mouse_selectable() {
        assert!(Paragraph::new().selectable);
        assert!(Paragraph::with_text("Selectable paragraph").selectable);
    }

    #[test]
    fn text_and_paragraph_use_selectable_text_for_native_selection() {
        let text_source = include_str!("text.rs");
        let paragraph_source = include_str!("paragraph.rs");
        let selectable_source = include_str!("selectable_text.rs");

        assert!(text_source.contains("SelectableText::view"));
        assert!(paragraph_source.contains("SelectableText::view"));
        assert!(text_source.contains("pub fn selectable"));
        assert!(paragraph_source.contains("pub fn selectable"));
        assert!(selectable_source.contains("event.click_count == 2"));
        assert!(selectable_source.contains("ClipboardItem::new_string"));
        assert!(selectable_source.contains(r#"KeyBinding::new("ctrl-c""#));
        assert!(selectable_source.contains("window.capture_pointer"));
        assert!(selectable_source.contains("cx.on_blur(&self.focus_handle"));
        assert!(selectable_source.contains("fn clear_selection"));
    }

    #[test]
    fn paragraph_composes_segments_into_one_styled_text_run_list() {
        let theme = liora_theme::Theme::light();
        let (text, runs) = Paragraph::new()
            .child(Text::new("Hello ").bold())
            .child(Text::new("世界").italic())
            .styled_text_parts(&theme, Some("Monospace".into()), None);

        assert_eq!(text.as_ref(), "Hello 世界");
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0].len, "Hello ".len());
        assert_eq!(runs[1].len, "世界".len());
        assert_eq!(runs[0].font.weight, FontWeight::BOLD);
        assert_eq!(runs[1].font.style, FontStyle::Italic);
    }

    #[test]
    fn paragraph_glues_line_start_forbidden_punctuation_to_previous_run() {
        let theme = liora_theme::Theme::light();
        let (text, runs) = Paragraph::new()
            .child(Text::new("crates/liora-components").code_style(&theme))
            .child(Text::new("：所有可复用组件，例如 "))
            .child(Text::new("Button").code_style(&theme))
            .child(Text::new("、"))
            .child(Text::new("Input").code_style(&theme))
            .child(Text::new("。"))
            .styled_text_parts(&theme, Some("Monospace".into()), None);

        assert_eq!(
            text.as_ref(),
            "crates/liora-components：所有可复用组件，例如 Button、Input。"
        );
        assert_eq!(runs[0].len, "crates/liora-components：".len());
        assert_eq!(runs[2].len, "Button、".len());
        assert_eq!(runs[3].len, "Input。".len());
    }

    #[test]
    fn text_segments_map_inline_code_style_to_text_runs_without_forcing_app_font() {
        let theme = liora_theme::Theme::light();
        let default_style = Paragraph::default_text_style(&theme, None);
        let run = Text::new("code")
            .code_style(&theme)
            .bold()
            .underline()
            .to_text_run(&default_style);

        assert_eq!(run.len, "code".len());
        assert_eq!(run.font.family.as_ref(), ".SystemUIFont");
        assert_eq!(run.font.weight, FontWeight::BOLD);
        assert_eq!(run.color, theme.danger.base);
        assert_eq!(run.background_color, Some(theme.neutral.hover));
        assert!(run.underline.is_some());
    }

    #[test]
    fn paragraph_default_style_accepts_app_ui_font_family() {
        let theme = liora_theme::Theme::light();
        let style = Paragraph::default_text_style(&theme, Some("PingFang SC".into()));

        assert_eq!(style.font_family.as_ref(), "PingFang SC");
    }

    #[test]
    fn paragraph_default_style_keeps_native_wrapping_without_truncation() {
        let theme = liora_theme::Theme::light();
        let style = Paragraph::default_text_style(&theme, None);

        assert_eq!(style.white_space, WhiteSpace::Normal);
        assert!(style.text_overflow.is_none());
        assert!(style.line_clamp.is_none());
    }
}
