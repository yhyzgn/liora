//! Text View module.
//!
//! This public module implements the Liora document/text-view component for lightweight native document rendering. It keeps the reusable
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

use crate::{Card, CodeBlock, CodeLanguage, Divider, Paragraph, Space, Text, Title};
use gpui::{
    AnyElement, App, Component, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    Pixels, RenderOnce, SharedString, Styled, Window, div, prelude::*, px,
};
use liora_core::{Config, unique_id};

/// Structural block rendered by [`TextView`].
#[derive(Clone, Debug, PartialEq)]
pub enum TextViewBlock {
    /// Heading text with a clamped level from 1 to 6.
    Heading { level: u8, text: SharedString },
    /// Plain paragraph text rendered with selectable wrapping.
    Paragraph(SharedString),
    /// Quote paragraph rendered in a subtle card-like rail.
    Quote(SharedString),
    /// Fenced or authored code block with syntax highlighting.
    Code {
        /// Code text rendered inside the block.
        code: SharedString,
        /// Language label resolved by [`CodeLanguage::from_label`].
        language: CodeLanguage,
    },
    /// Ordered or unordered list items.
    List {
        /// Whether the list uses numeric markers.
        ordered: bool,
        /// Item text rendered for each row.
        items: Vec<SharedString>,
    },
    /// Visual separator between sections.
    Divider,
}

impl TextViewBlock {
    /// Creates a heading block and clamps the level to the supported h1-h6 range.
    pub fn heading(level: u8, text: impl Into<SharedString>) -> Self {
        Self::Heading {
            level: level.clamp(1, 6),
            text: text.into(),
        }
    }

    /// Creates a paragraph block.
    pub fn paragraph(text: impl Into<SharedString>) -> Self {
        Self::Paragraph(text.into())
    }

    /// Creates a quote block.
    pub fn quote(text: impl Into<SharedString>) -> Self {
        Self::Quote(text.into())
    }

    /// Creates a syntax-highlighted code block.
    pub fn code(code: impl Into<SharedString>, language: impl Into<CodeLanguage>) -> Self {
        Self::Code {
            code: code.into(),
            language: language.into(),
        }
    }

    /// Creates an unordered list block.
    pub fn unordered(items: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        Self::List {
            ordered: false,
            items: items.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates an ordered list block.
    pub fn ordered(items: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        Self::List {
            ordered: true,
            items: items.into_iter().map(Into::into).collect(),
        }
    }

    /// Returns whether this block should be skipped by document rendering.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Heading { text, .. } | Self::Paragraph(text) | Self::Quote(text) => {
                text.trim().is_empty()
            }
            Self::Code { code, .. } => code.is_empty(),
            Self::List { items, .. } => items.iter().all(|item| item.trim().is_empty()),
            Self::Divider => false,
        }
    }
}

/// Fluent native GPUI component for rendering lightweight document content.
#[derive(Clone)]
pub struct TextView {
    blocks: Vec<TextViewBlock>,
    id: SharedString,
    selectable: bool,
    framed: bool,
    max_width: Option<Pixels>,
    gap: Pixels,
    padding: Pixels,
    background: Option<Hsla>,
}

impl TextView {
    /// Creates a document view from authored structural blocks.
    pub fn new(blocks: impl IntoIterator<Item = TextViewBlock>) -> Self {
        Self {
            blocks: blocks.into_iter().collect(),
            id: unique_id("text-view"),
            selectable: true,
            framed: false,
            max_width: None,
            gap: px(12.0),
            padding: px(16.0),
            background: None,
        }
    }

    /// Parses a deliberately small Markdown subset for app help/about pages.
    pub fn from_plain_markdown(markdown: impl AsRef<str>) -> Self {
        Self::new(parse_plain_markdown(markdown.as_ref()))
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Toggles native text selection for title and paragraph blocks.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Wraps the document in a bordered card surface.
    pub fn framed(mut self, framed: bool) -> Self {
        self.framed = framed;
        self
    }

    /// Sets a maximum readable line width for the document body.
    pub fn max_width(mut self, width: impl Into<Pixels>) -> Self {
        self.max_width = Some(width.into().max(px(160.0)));
        self
    }

    /// Sets vertical spacing between document blocks.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().max(px(0.0));
        self
    }

    /// Sets inner padding used when the framed surface is enabled.
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().max(px(0.0));
        self
    }

    /// Applies an explicit document surface background.
    pub fn background(mut self, background: Hsla) -> Self {
        self.background = Some(background);
        self
    }

    /// Returns the authored blocks owned by this text view.
    pub fn blocks(&self) -> &[TextViewBlock] {
        &self.blocks
    }

    /// Returns whether the outer frame is enabled.
    pub fn is_framed(&self) -> bool {
        self.framed
    }
}

impl IntoElement for TextView {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for TextView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let background = self.background.unwrap_or(theme.neutral.card);
        let content =
            Space::new()
                .vertical()
                .gap(self.gap)
                .children(self.blocks.iter().filter_map(|block| {
                    if block.is_empty() {
                        None
                    } else {
                        Some(render_text_view_block(block, self.selectable, &theme))
                    }
                }));

        let shell = div()
            .id(ElementId::from(self.id.clone()))
            .flex()
            .flex_col()
            .w_full()
            .when_some(self.max_width, |style, width| style.max_w(width))
            .when(self.framed, |style| {
                style
                    .p(self.padding)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.neutral.border)
                    .bg(background)
            })
            .child(content);

        shell.into_any_element()
    }
}

fn render_text_view_block(
    block: &TextViewBlock,
    selectable: bool,
    theme: &liora_theme::Theme,
) -> AnyElement {
    match block {
        TextViewBlock::Heading { level, text } => title_for_level(text.clone(), *level)
            .selectable(selectable)
            .into_any_element(),
        TextViewBlock::Paragraph(text) => Paragraph::with_text(text.clone())
            .selectable(selectable)
            .into_any_element(),
        TextViewBlock::Quote(text) => Card::new(
            div()
                .border_l_4()
                .border_color(theme.primary.base)
                .pl_3()
                .child(
                    Paragraph::with_text(text.clone())
                        .selectable(selectable)
                        .into_any_element(),
                ),
        )
        .no_shadow()
        .into_any_element(),
        TextViewBlock::Code { code, language } => CodeBlock::new(code.clone())
            .language(*language)
            .copyable(true)
            .selectable(true)
            .into_any_element(),
        TextViewBlock::List { ordered, items } => Space::new()
            .vertical()
            .gap_xs()
            .children(items.iter().enumerate().filter_map(|(index, item)| {
                if item.trim().is_empty() {
                    return None;
                }
                let marker = if *ordered {
                    format!("{}.", index + 1)
                } else {
                    "•".to_string()
                };
                Some(
                    Space::new()
                        .gap_sm()
                        .align_start()
                        .child(Text::new(marker).text_color(theme.neutral.text_3).nowrap())
                        .child(Paragraph::with_text(item.clone()).selectable(selectable))
                        .into_any_element(),
                )
            }))
            .into_any_element(),
        TextViewBlock::Divider => Divider::new().into_any_element(),
    }
}

fn title_for_level(text: SharedString, level: u8) -> Title {
    match level.clamp(1, 6) {
        1 => Title::new(text).h1(),
        2 => Title::new(text).h2(),
        3 => Title::new(text).h3(),
        4 => Title::new(text).h4(),
        5 => Title::new(text).h5(),
        _ => Title::new(text).h6(),
    }
}

/// Parses headings, paragraphs, quotes, simple lists, horizontal rules, and fenced code blocks.
pub fn parse_plain_markdown(markdown: &str) -> Vec<TextViewBlock> {
    let mut blocks = Vec::new();
    let mut paragraph = Vec::new();
    let mut list_items: Vec<SharedString> = Vec::new();
    let mut ordered_list: Option<bool> = None;
    let mut code_lines = Vec::new();
    let mut code_language = CodeLanguage::PlainText;
    let mut in_code = false;

    let flush_paragraph = |blocks: &mut Vec<TextViewBlock>, paragraph: &mut Vec<&str>| {
        if !paragraph.is_empty() {
            blocks.push(TextViewBlock::paragraph(paragraph.join(" ")));
            paragraph.clear();
        }
    };
    let flush_list = |blocks: &mut Vec<TextViewBlock>,
                      list_items: &mut Vec<SharedString>,
                      ordered_list: &mut Option<bool>| {
        if !list_items.is_empty() {
            blocks.push(TextViewBlock::List {
                ordered: ordered_list.unwrap_or(false),
                items: std::mem::take(list_items),
            });
        }
        *ordered_list = None;
    };

    for line in markdown.lines() {
        let trimmed = line.trim();
        if in_code {
            if trimmed.starts_with("```") {
                blocks.push(TextViewBlock::code(code_lines.join("\n"), code_language));
                code_lines.clear();
                code_language = CodeLanguage::PlainText;
                in_code = false;
            } else {
                code_lines.push(line);
            }
            continue;
        }

        if trimmed.starts_with("```") {
            flush_paragraph(&mut blocks, &mut paragraph);
            flush_list(&mut blocks, &mut list_items, &mut ordered_list);
            in_code = true;
            code_language = CodeLanguage::from_label(trimmed.trim_start_matches('`'));
            continue;
        }

        if trimmed.is_empty() {
            flush_paragraph(&mut blocks, &mut paragraph);
            flush_list(&mut blocks, &mut list_items, &mut ordered_list);
            continue;
        }

        if matches!(trimmed, "---" | "***" | "___") {
            flush_paragraph(&mut blocks, &mut paragraph);
            flush_list(&mut blocks, &mut list_items, &mut ordered_list);
            blocks.push(TextViewBlock::Divider);
            continue;
        }

        if let Some((level, text)) = parse_heading(trimmed) {
            flush_paragraph(&mut blocks, &mut paragraph);
            flush_list(&mut blocks, &mut list_items, &mut ordered_list);
            blocks.push(TextViewBlock::heading(level, text));
            continue;
        }

        if let Some(text) = trimmed
            .strip_prefix("> ")
            .or_else(|| trimmed.strip_prefix('>'))
        {
            flush_paragraph(&mut blocks, &mut paragraph);
            flush_list(&mut blocks, &mut list_items, &mut ordered_list);
            blocks.push(TextViewBlock::quote(text.trim()));
            continue;
        }

        if let Some(item) = parse_unordered_item(trimmed) {
            flush_paragraph(&mut blocks, &mut paragraph);
            if ordered_list == Some(true) {
                flush_list(&mut blocks, &mut list_items, &mut ordered_list);
            }
            ordered_list = Some(false);
            list_items.push(item.into());
            continue;
        }

        if let Some(item) = parse_ordered_item(trimmed) {
            flush_paragraph(&mut blocks, &mut paragraph);
            if ordered_list == Some(false) {
                flush_list(&mut blocks, &mut list_items, &mut ordered_list);
            }
            ordered_list = Some(true);
            list_items.push(item.into());
            continue;
        }

        paragraph.push(trimmed);
    }

    if in_code {
        blocks.push(TextViewBlock::code(code_lines.join("\n"), code_language));
    }
    flush_paragraph(&mut blocks, &mut paragraph);
    flush_list(&mut blocks, &mut list_items, &mut ordered_list);
    blocks
}

fn parse_heading(line: &str) -> Option<(u8, &str)> {
    let hashes = line.chars().take_while(|ch| *ch == '#').count();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    let text = line.get(hashes..)?.trim();
    (!text.is_empty()).then_some((hashes as u8, text))
}

fn parse_unordered_item(line: &str) -> Option<&str> {
    line.strip_prefix("- ")
        .or_else(|| line.strip_prefix("* "))
        .or_else(|| line.strip_prefix("+ "))
        .map(str::trim)
        .filter(|item| !item.is_empty())
}

fn parse_ordered_item(line: &str) -> Option<&str> {
    let dot = line.find('.')?;
    if dot == 0 || !line[..dot].chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    line.get(dot + 1..)
        .map(str::trim)
        .filter(|item| !item.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_view_builder_tracks_document_options() {
        let view = TextView::new([
            TextViewBlock::heading(2, "Guide"),
            TextViewBlock::paragraph("Native document content"),
        ])
        .id("doc")
        .selectable(false)
        .framed(true)
        .max_width(px(720.0))
        .gap(px(18.0))
        .padding(px(24.0));

        assert_eq!(view.blocks().len(), 2);
        assert!(view.is_framed());
    }

    #[test]
    fn plain_markdown_parser_extracts_common_document_blocks() {
        let blocks = parse_plain_markdown(
            "# Title\n\nIntro paragraph wraps\nonto one line.\n\n> Note\n\n- One\n- Two\n\n```rust\nfn main() {}\n```\n---",
        );

        assert!(matches!(blocks[0], TextViewBlock::Heading { level: 1, .. }));
        assert_eq!(
            blocks[1],
            TextViewBlock::paragraph("Intro paragraph wraps onto one line.")
        );
        assert_eq!(blocks[2], TextViewBlock::quote("Note"));
        assert!(
            matches!(blocks[3], TextViewBlock::List { ordered: false, ref items } if items.len() == 2)
        );
        assert!(matches!(
            blocks[4],
            TextViewBlock::Code {
                language: CodeLanguage::Rust,
                ..
            }
        ));
        assert_eq!(blocks[5], TextViewBlock::Divider);
    }

    #[test]
    fn text_view_blocks_ignore_empty_content() {
        assert!(TextViewBlock::paragraph("   ").is_empty());
        assert!(TextViewBlock::unordered([""]).is_empty());
        assert!(!TextViewBlock::Divider.is_empty());
    }
}
