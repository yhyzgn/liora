//! Text module.
//!
//! This public module implements the Liora selectable text component with typography options. It keeps the reusable
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

use crate::{
    Card, CodeBlock, CodeLanguage, Divider, Paragraph, SelectableText, SelectableTextOptions,
    SelectableTextWrap, Space, Title,
};
use gpui::{
    AnyElement, App, Component, ElementId, FontStyle, FontWeight, Hsla, IntoElement, ParentElement,
    Pixels, RenderOnce, SharedString, StrikethroughStyle, Styled, TextRun, TextStyle,
    UnderlineStyle, Window, div, prelude::*, px,
};
use liora_core::{Config, code_font_family, ui_font_family};

#[derive(Clone, Debug, PartialEq)]
/// Structural document block rendered by [`Text::document`].
pub enum TextBlock {
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

impl TextContent {
    pub(crate) fn inline(&self) -> SharedString {
        match self {
            Self::Inline(content) => content.clone(),
            Self::Document(_) => SharedString::default(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Self::Inline(content) => content.is_empty(),
            Self::Document(blocks) => blocks.iter().all(TextBlock::is_empty),
        }
    }
}

impl AsRef<str> for TextContent {
    fn as_ref(&self) -> &str {
        match self {
            Self::Inline(content) => content.as_ref(),
            Self::Document(_) => "",
        }
    }
}

impl TextBlock {
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

#[derive(Clone)]
pub(crate) enum TextContent {
    Inline(SharedString),
    Document(Vec<TextBlock>),
}

#[derive(Clone)]
/// Fluent native GPUI component for rendering Liora text.
pub struct Text {
    pub(crate) content: TextContent,
    pub(crate) color: Option<Hsla>,
    pub(crate) bg: Option<Hsla>,
    pub(crate) size: Option<Pixels>,
    pub(crate) weight: Option<FontWeight>,
    pub(crate) style: Option<FontStyle>,
    pub(crate) underline: bool,
    pub(crate) strikethrough: bool,
    pub(crate) font_family: Option<SharedString>,
    pub(crate) is_code_style: bool,
    pub(crate) wrap: bool,
    pub(crate) fill_width_on_wrap: bool,
    pub(crate) selectable: bool,
    pub(crate) framed: bool,
    pub(crate) max_width: Option<Pixels>,
    pub(crate) gap: Pixels,
    pub(crate) padding: Pixels,
    pub(crate) document_background: Option<Hsla>,
    pub(crate) id: SharedString,
}

impl Text {
    /// Creates `Text` initialized from the supplied content.
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: TextContent::Inline(content.into()),
            color: None,
            bg: None,
            size: None,
            weight: None,
            style: None,
            underline: false,
            strikethrough: false,
            font_family: None,
            is_code_style: false,
            wrap: true,
            fill_width_on_wrap: false,
            selectable: true,
            framed: false,
            max_width: None,
            gap: px(12.0),
            padding: px(16.0),
            document_background: None,
            id: liora_core::unique_id("text"),
        }
    }

    /// Creates a lightweight document view from authored structural blocks.
    pub fn document(blocks: impl IntoIterator<Item = TextBlock>) -> Self {
        let mut this = Self::new(SharedString::default());
        this.content = TextContent::Document(blocks.into_iter().collect());
        this.id = liora_core::unique_id("text-document");
        this
    }

    /// Parses a deliberately small Markdown subset for app help/about pages.
    pub fn markdown(markdown: impl AsRef<str>) -> Self {
        Self::document(parse_plain_markdown(markdown.as_ref()))
    }

    /// Applies the foreground text color.
    pub fn text_color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the bg used by the rendered component.
    pub fn bg(mut self, bg: Hsla) -> Self {
        self.bg = Some(bg);
        self
    }

    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Sets the xs value used by the component.
    pub fn xs(self) -> Self {
        self.size(px(12.0))
    }

    /// Sets the sm value used by the component.
    pub fn sm(self) -> Self {
        self.size(px(14.0))
    }

    /// Sets the weight value used by the component.
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Applies bold font weight.
    pub fn bold(mut self) -> Self {
        self.weight = Some(FontWeight::BOLD);
        self
    }

    /// Sets the font style value used by the component.
    pub fn font_style(mut self, style: FontStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Sets the italic value used by the component.
    pub fn italic(mut self) -> Self {
        self.style = Some(FontStyle::Italic);
        self
    }

    /// Sets the underline value used by the component.
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Sets the strikethrough value used by the component.
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Sets the font family value used by the component.
    pub fn font_family(mut self, family: impl Into<SharedString>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// Enable normal whitespace wrapping and let the text take the parent width.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self.fill_width_on_wrap = true;
        self
    }

    /// Alias for [`Text::wrap`].
    pub fn auto_wrap(self) -> Self {
        self.wrap()
    }

    /// Keep the text on a single line.
    pub fn nowrap(mut self) -> Self {
        self.wrap = false;
        self.fill_width_on_wrap = false;
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

    /// Convenience for inline code styling.
    ///
    /// The font family is resolved during render so the default remains GPUI's
    /// platform monospace family unless the application configured a custom
    /// Liora code font.
    pub fn code_style(mut self, theme: &liora_theme::Theme) -> Self {
        self.bg = Some(theme.neutral.hover);
        self.is_code_style = true;
        self.text_color(theme.danger.base)
    }

    /// Wraps document text in a bordered card surface.
    pub fn framed(mut self, framed: bool) -> Self {
        self.framed = framed;
        self
    }

    /// Sets a maximum readable line width for document text.
    pub fn max_width(mut self, width: impl Into<Pixels>) -> Self {
        self.max_width = Some(width.into().max(px(160.0)));
        self
    }

    /// Sets vertical spacing between document blocks.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().max(px(0.0));
        self
    }

    /// Sets inner padding used when the framed document surface is enabled.
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().max(px(0.0));
        self
    }

    /// Applies an explicit document surface background.
    pub fn background(mut self, background: Hsla) -> Self {
        self.document_background = Some(background);
        self
    }

    /// Returns authored document blocks when this text is in document mode.
    pub fn blocks(&self) -> Option<&[TextBlock]> {
        match &self.content {
            TextContent::Document(blocks) => Some(blocks),
            TextContent::Inline(_) => None,
        }
    }

    /// Returns whether the outer document frame is enabled.
    pub fn is_framed(&self) -> bool {
        self.framed
    }

    pub(crate) fn apply_to_text_style(&self, mut style: TextStyle) -> TextStyle {
        if let Some(color) = self.color {
            style.color = color;
        }

        if let Some(bg) = self.bg {
            style.background_color = Some(bg);
        }

        if let Some(weight) = self.weight {
            style.font_weight = weight;
        }

        if let Some(font_style) = self.style {
            style.font_style = font_style;
        }

        if let Some(family) = self.font_family.clone() {
            style.font_family = family;
        }

        if self.underline {
            style.underline = Some(UnderlineStyle {
                thickness: px(1.0),
                color: self.color,
                ..Default::default()
            });
        }

        if self.strikethrough {
            style.strikethrough = Some(StrikethroughStyle {
                thickness: px(1.0),
                color: self.color,
            });
        }

        style
    }

    fn inline_content(&self) -> SharedString {
        self.content.inline()
    }

    pub(crate) fn to_text_run(&self, default_style: &TextStyle) -> TextRun {
        self.apply_to_text_style(default_style.clone())
            .to_run(self.inline_content().len())
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        SelectableText::register_key_bindings(cx);
    }
}

impl RenderOnce for Text {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        if matches!(self.content, TextContent::Document(_)) {
            return render_text_document(self, theme).into_any_element();
        }

        let font_size = self.size.unwrap_or_else(|| px(theme.font_size.md));
        let line_height = font_size * 1.6;
        let text_color = self.color.unwrap_or(theme.neutral.text_2);

        let mut text = self;
        if text.is_code_style && text.font_family.is_none() {
            text.font_family = Some(code_font_family(cx));
        } else if text.font_family.is_none() {
            text.font_family = ui_font_family(cx);
        }

        if text.selectable {
            let mut base_style = TextStyle::default();
            base_style.color = text_color;
            base_style.font_size = font_size.into();
            base_style.line_height = line_height.into();
            base_style.white_space = if text.wrap {
                gpui::WhiteSpace::Normal
            } else {
                gpui::WhiteSpace::Nowrap
            };
            let run = text.to_text_run(&base_style);
            return SelectableText::view(
                SelectableTextOptions {
                    id: ElementId::from(text.id.clone()),
                    text: text.inline_content(),
                    runs: vec![run],
                    font_size,
                    line_height,
                    text_color,
                    wrap: if text.wrap {
                        SelectableTextWrap::Normal
                    } else {
                        SelectableTextWrap::NoWrap
                    },
                    key_context: "SelectableText",
                    fill_width: text.fill_width_on_wrap,
                    font_family: text.font_family.clone(),
                },
                _window,
                cx,
            );
        }

        let mut el = div()
            .child(text.inline_content())
            .text_size(font_size)
            .line_height(line_height)
            .text_color(text_color);

        if text.wrap {
            el = el.whitespace_normal();
            if text.fill_width_on_wrap {
                el = el.w_full().flex_shrink(1.0);
            }
        } else {
            el = el.whitespace_nowrap();
        }

        if let Some(bg) = text.bg {
            el = el.bg(bg).px_1().rounded(px(2.0));
        }

        if let Some(weight) = text.weight {
            el = el.font_weight(weight);
        }

        if let Some(style) = text.style {
            // In some GPUI versions, it's .italic(), in others it's .font_style(style)
            // If .font_style failed, let's try matching on style
            if style == FontStyle::Italic {
                el = el.italic();
            }
        }

        if text.underline {
            el = el.underline();
        }

        if text.strikethrough {
            el = el.line_through();
        }

        if let Some(family) = text.font_family {
            el = el.font_family(family);
        }

        el.into_any_element()
    }
}

fn render_text_document(text: Text, theme: &liora_theme::Theme) -> AnyElement {
    let TextContent::Document(blocks) = &text.content else {
        return Text::new(SharedString::default()).into_any_element();
    };
    let background = text.document_background.unwrap_or(theme.neutral.card);
    let content = Space::new()
        .vertical()
        .gap(text.gap)
        .children(blocks.iter().filter_map(|block| {
            if block.is_empty() {
                None
            } else {
                Some(render_text_block(block, text.selectable, theme))
            }
        }));

    div()
        .id(ElementId::from(text.id.clone()))
        .flex()
        .flex_col()
        .w_full()
        .when_some(text.max_width, |style, width| style.max_w(width))
        .when(text.framed, |style| {
            style
                .p(text.padding)
                .rounded_lg()
                .border_1()
                .border_color(theme.neutral.border)
                .bg(background)
        })
        .child(content)
        .into_any_element()
}

fn render_text_block(
    block: &TextBlock,
    selectable: bool,
    theme: &liora_theme::Theme,
) -> AnyElement {
    match block {
        TextBlock::Heading { level, text } => title_for_level(text.clone(), *level)
            .selectable(selectable)
            .into_any_element(),
        TextBlock::Paragraph(text) => Paragraph::with_text(text.clone())
            .selectable(selectable)
            .into_any_element(),
        TextBlock::Quote(text) => Card::new(
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
        TextBlock::Code { code, language } => CodeBlock::new(code.clone())
            .language(*language)
            .copyable(true)
            .selectable(true)
            .into_any_element(),
        TextBlock::List { ordered, items } => Space::new()
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
        TextBlock::Divider => Divider::new().into_any_element(),
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
pub fn parse_plain_markdown(markdown: &str) -> Vec<TextBlock> {
    let mut blocks = Vec::new();
    let mut paragraph = Vec::new();
    let mut list_items: Vec<SharedString> = Vec::new();
    let mut ordered_list: Option<bool> = None;
    let mut code_lines = Vec::new();
    let mut code_language = CodeLanguage::PlainText;
    let mut in_code = false;

    let flush_paragraph = |blocks: &mut Vec<TextBlock>, paragraph: &mut Vec<&str>| {
        if !paragraph.is_empty() {
            blocks.push(TextBlock::paragraph(paragraph.join(" ")));
            paragraph.clear();
        }
    };
    let flush_list = |blocks: &mut Vec<TextBlock>,
                      list_items: &mut Vec<SharedString>,
                      ordered_list: &mut Option<bool>| {
        if !list_items.is_empty() {
            blocks.push(TextBlock::List {
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
                blocks.push(TextBlock::code(code_lines.join("\n"), code_language));
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
            blocks.push(TextBlock::Divider);
            continue;
        }

        if let Some((level, text)) = parse_heading(trimmed) {
            flush_paragraph(&mut blocks, &mut paragraph);
            flush_list(&mut blocks, &mut list_items, &mut ordered_list);
            blocks.push(TextBlock::heading(level, text));
            continue;
        }

        if let Some(text) = trimmed
            .strip_prefix("> ")
            .or_else(|| trimmed.strip_prefix('>'))
        {
            flush_paragraph(&mut blocks, &mut paragraph);
            flush_list(&mut blocks, &mut list_items, &mut ordered_list);
            blocks.push(TextBlock::quote(text.trim()));
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
        blocks.push(TextBlock::code(code_lines.join("\n"), code_language));
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

impl IntoElement for Text {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod document_tests {
    use super::*;

    #[test]
    fn text_defaults_to_mouse_selectable_for_inline_and_document_content() {
        let inline = Text::new("Selectable by default");
        assert!(inline.selectable);

        let document = Text::document([
            TextBlock::heading(2, "Selectable heading"),
            TextBlock::paragraph("Selectable paragraph"),
            TextBlock::quote("Selectable quote"),
        ]);
        assert!(document.selectable);
    }

    #[test]
    fn text_document_renderer_forwards_default_selection_to_typography_blocks() {
        let source = include_str!("text.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("render_text_block(block, text.selectable, theme)"));
        assert!(source.contains("title_for_level(text.clone(), *level)"));
        assert!(source.contains(".selectable(selectable)"));
        assert!(source.contains("Paragraph::with_text(text.clone())"));
        assert!(source.contains("Paragraph::with_text(item.clone()).selectable(selectable)"));
    }

    #[test]
    fn text_document_builder_tracks_document_options() {
        let text = Text::document([
            TextBlock::heading(2, "Guide"),
            TextBlock::paragraph("Native document content"),
        ])
        .id("doc")
        .selectable(false)
        .framed(true)
        .max_width(px(720.0))
        .gap(px(18.0))
        .padding(px(24.0));

        assert_eq!(text.blocks().map(|blocks| blocks.len()), Some(2));
        assert!(text.is_framed());
    }

    #[test]
    fn text_markdown_parser_extracts_common_document_blocks() {
        let blocks = parse_plain_markdown(
            "# Title\n\nIntro paragraph wraps\nonto one line.\n\n> Note\n\n- One\n- Two\n\n```rust\nfn main() {}\n```\n---",
        );

        assert!(matches!(blocks[0], TextBlock::Heading { level: 1, .. }));
        assert_eq!(
            blocks[1],
            TextBlock::paragraph("Intro paragraph wraps onto one line.")
        );
        assert_eq!(blocks[2], TextBlock::quote("Note"));
        assert!(
            matches!(blocks[3], TextBlock::List { ordered: false, ref items } if items.len() == 2)
        );
        assert!(matches!(
            blocks[4],
            TextBlock::Code {
                language: CodeLanguage::Rust,
                ..
            }
        ));
        assert_eq!(blocks[5], TextBlock::Divider);
    }
}
