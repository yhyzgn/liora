//! Selectable text group module.
//!
//! `SelectableTextGroup` composes multiple [`Text`] and [`Paragraph`] values
//! into one native selectable text flow. Use it for document-like surfaces where
//! users expect drag selection and copy to continue across several text blocks.

use crate::{Paragraph, SelectableText, SelectableTextOptions, SelectableTextWrap, Text};
use gpui::{
    App, Component, ElementId, IntoElement, RenderOnce, SharedString, TextRun, TextStyle, Window,
    prelude::*, px,
};
use liora_core::{Config, code_font_family, code_font_weight, ui_font_family, ui_font_weight};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    panic::Location,
};

/// A text block that can participate in a cross-block selectable text group.
pub enum SelectableTextGroupBlock {
    /// A single inline [`Text`] value.
    Text(Text),
    /// A composed [`Paragraph`] value.
    Paragraph(Paragraph),
}

impl From<Text> for SelectableTextGroupBlock {
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<Paragraph> for SelectableTextGroupBlock {
    fn from(value: Paragraph) -> Self {
        Self::Paragraph(value)
    }
}

/// Composes several text-like components into one continuous selectable range.
pub struct SelectableTextGroup {
    blocks: Vec<SelectableTextGroupBlock>,
    id: SharedString,
    separator: SharedString,
    selectable: bool,
}

impl SelectableTextGroup {
    /// Creates an empty group. Add blocks with [`Self::text`], [`Self::paragraph`], or [`Self::child`].
    #[track_caller]
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            id: default_group_id(Location::caller()),
            separator: "\n\n".into(),
            selectable: true,
        }
    }

    /// Adds a [`Text`] block to the group.
    pub fn text(mut self, text: Text) -> Self {
        self.blocks.push(SelectableTextGroupBlock::Text(text));
        self
    }

    /// Adds a [`Paragraph`] block to the group.
    pub fn paragraph(mut self, paragraph: Paragraph) -> Self {
        self.blocks
            .push(SelectableTextGroupBlock::Paragraph(paragraph));
        self
    }

    /// Adds any supported text group block.
    pub fn child(mut self, child: impl Into<SelectableTextGroupBlock>) -> Self {
        self.blocks.push(child.into());
        self
    }

    /// Adds several supported text group blocks.
    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<SelectableTextGroupBlock>>,
    ) -> Self {
        self.blocks.extend(children.into_iter().map(Into::into));
        self
    }

    /// Assigns a stable id for persisted selection state and copy shortcuts.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the text inserted between adjacent blocks. Defaults to a blank line.
    pub fn separator(mut self, separator: impl Into<SharedString>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Enables or disables the selectable behavior for the entire group.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Registers keyboard shortcuts used by grouped selectable text.
    pub fn register_key_bindings(cx: &mut App) {
        SelectableText::register_key_bindings(cx);
    }
}

impl Default for SelectableTextGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectableTextGroup {
    fn text_parts(
        self,
        cx: &impl liora_core::LocalesContext,
        theme: &liora_theme::Theme,
        code_family: SharedString,
        ui_family: Option<SharedString>,
        code_weight: Option<gpui::FontWeight>,
        ui_weight: Option<gpui::FontWeight>,
    ) -> (SharedString, Vec<TextRun>) {
        let font_size = px(theme.font_size.md);
        let line_height = font_size * 1.6;
        let mut default_style = TextStyle::default();
        default_style.color = theme.neutral.text_2;
        default_style.font_size = font_size.into();
        default_style.line_height = line_height.into();
        default_style.white_space = gpui::WhiteSpace::Normal;
        if let Some(family) = ui_family.clone() {
            default_style.font_family = family;
        }
        if let Some(weight) = ui_weight {
            default_style.font_weight = weight;
        }

        let mut full_text = String::new();
        let mut runs = Vec::<TextRun>::new();

        for block in self.blocks.into_iter() {
            let before_block_len = full_text.len();
            match block {
                SelectableTextGroupBlock::Text(mut text) => {
                    if text.content.is_empty() {
                        continue;
                    }
                    if text.weight.is_none() {
                        text.weight = if text.is_code_style {
                            code_weight.or(ui_weight)
                        } else {
                            ui_weight
                        };
                    }
                    if text.is_code_style && text.font_family.is_none() {
                        text.font_family = Some(code_family.clone());
                    } else if text.font_family.is_none() {
                        text.font_family = ui_family.clone();
                    }
                    let content = text.content.inline(cx);
                    if !content.is_empty() && !full_text.is_empty() && !self.separator.is_empty() {
                        append_text_run(
                            &mut full_text,
                            &mut runs,
                            self.separator.as_ref(),
                            default_style.to_run(self.separator.len()),
                        );
                    }
                    let run = text.to_text_run(&default_style, content.len());
                    append_text_run(&mut full_text, &mut runs, content.as_ref(), run);
                }
                SelectableTextGroupBlock::Paragraph(paragraph) => {
                    let (paragraph_text, paragraph_runs) = paragraph.selectable_text_parts(
                        cx,
                        theme,
                        Some(code_family.clone()),
                        ui_family.clone(),
                        code_weight,
                        ui_weight,
                    );
                    if paragraph_text.is_empty() {
                        continue;
                    }
                    if !full_text.is_empty() && !self.separator.is_empty() {
                        append_text_run(
                            &mut full_text,
                            &mut runs,
                            self.separator.as_ref(),
                            default_style.to_run(self.separator.len()),
                        );
                    }
                    full_text.push_str(paragraph_text.as_ref());
                    runs.extend(paragraph_runs);
                }
            }

            debug_assert!(
                full_text.len() >= before_block_len,
                "selectable text group should only append text"
            );
        }

        (full_text.into(), runs)
    }
}

impl RenderOnce for SelectableTextGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let ui_family = ui_font_family(cx);
        let code_family = code_font_family(cx);
        let ui_weight = ui_font_weight(cx);
        let code_weight = code_font_weight(cx);
        let font_size = px(theme.font_size.md);
        let line_height = font_size * 1.6;
        let selectable = self.selectable;
        let id = self.id.clone();
        let (full_text, runs) = self.text_parts(
            cx,
            &theme,
            code_family,
            ui_family.clone(),
            code_weight,
            ui_weight,
        );

        if full_text.is_empty() {
            return gpui::div().into_any_element();
        }

        if selectable {
            return SelectableText::view(
                SelectableTextOptions {
                    id: ElementId::from(id),
                    text: full_text,
                    runs,
                    font_size,
                    line_height,
                    text_color: theme.neutral.text_2,
                    wrap: SelectableTextWrap::Normal,
                    key_context: "SelectableText",
                    fill_width: true,
                    font_family: ui_family,
                },
                window,
                cx,
            );
        }

        gpui::div().child(full_text).into_any_element()
    }
}

impl IntoElement for SelectableTextGroup {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn append_text_run(full_text: &mut String, runs: &mut Vec<TextRun>, text: &str, mut run: TextRun) {
    if text.is_empty() {
        return;
    }
    full_text.push_str(text);
    run.len = text.len();
    runs.push(run);
}

fn default_group_id(location: &Location<'_>) -> SharedString {
    let mut hasher = DefaultHasher::new();
    location.file().hash(&mut hasher);
    location.line().hash(&mut hasher);
    location.column().hash(&mut hasher);
    format!(
        "selectable-text-group-{}:{}:{}-{:016x}",
        location.file(),
        location.line(),
        location.column(),
        hasher.finish()
    )
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selectable_text_group_accepts_text_and_paragraph_blocks() {
        let source = include_str!("selectable_text_group.rs");

        assert!(source.contains("pub struct SelectableTextGroup"));
        assert!(source.contains("SelectableTextGroupBlock::Text"));
        assert!(source.contains("SelectableTextGroupBlock::Paragraph"));
        assert!(source.contains("SelectableText::view"));
        assert!(source.contains(r#"separator: "\n\n".into()"#));
    }

    #[test]
    fn text_parts_join_non_empty_blocks_without_edge_separators() {
        let theme = liora_theme::Theme::light();
        let locales = liora_core::LocalesConfig::default();
        let (text, runs) = SelectableTextGroup::new()
            .separator("\n")
            .text(Text::new("Alpha"))
            .text(Text::new(""))
            .paragraph(Paragraph::with_text("Beta"))
            .text(Text::new(""))
            .text(Text::new("Gamma"))
            .text_parts(
                &locales,
                &theme,
                "Monospace".into(),
                Some("Inter".into()),
                None,
                None,
            );

        assert_eq!(text.as_ref(), "Alpha\nBeta\nGamma");
        assert_eq!(runs.iter().map(|run| run.len).sum::<usize>(), text.len());
    }

    #[test]
    fn text_parts_preserves_mixed_paragraph_runs() {
        let theme = liora_theme::Theme::light();
        let locales = liora_core::LocalesConfig::default();
        let (text, runs) = SelectableTextGroup::new()
            .separator("\n\n")
            .paragraph(
                Paragraph::new()
                    .child(Text::new("Plain "))
                    .child(Text::new("Code").code_style(&theme))
                    .child(Text::new(" Tail")),
            )
            .paragraph(Paragraph::with_text("Next"))
            .text_parts(
                &locales,
                &theme,
                "Monospace".into(),
                Some("Inter".into()),
                None,
                None,
            );

        assert_eq!(text.as_ref(), "Plain Code Tail\n\nNext");
        assert!(
            runs.len() >= 5,
            "expected paragraph style runs plus separator run"
        );
        assert_eq!(runs.iter().map(|run| run.len).sum::<usize>(), text.len());
    }
}
