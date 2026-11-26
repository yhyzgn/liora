//! Native selection scope for multiple styled text blocks.
//!
//! Text, Paragraph and Title retain their own GPUI layout while sharing a
//! selectable range and plain-text clipboard projection.

use crate::{Paragraph, SelectableText, Space, Text, Title};
use gpui::{App, IntoElement, RenderOnce, SharedString, ViewElement as Component, Window};
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
    /// A native heading at its authored font size.
    Title(Title),
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
impl From<Title> for SelectableTextGroupBlock {
    fn from(value: Title) -> Self {
        Self::Title(value)
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

    /// Adds a heading without flattening its font size or line height.
    pub fn title(mut self, title: Title) -> Self {
        self.blocks.push(SelectableTextGroupBlock::Title(title));
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
impl RenderOnce for SelectableTextGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let scope = self
            .selectable
            .then(|| SelectableText::selection_scope(self.id.clone(), _window, _cx));
        let mut content = Space::new().vertical().gap_md();
        for (order, block) in self.blocks.into_iter().enumerate() {
            let leaf_id = format!("{}-{order}", self.id);
            let scope = scope.clone();
            let separator = self.separator.clone();
            let child = match block {
                SelectableTextGroupBlock::Text(text) => {
                    let selectable = self.selectable && text.selectable;
                    text.id(leaf_id)
                        .in_selection_scope(scope, order, separator)
                        .selectable(selectable)
                        .into_any_element()
                }
                SelectableTextGroupBlock::Paragraph(paragraph) => {
                    let paragraph = if self.selectable {
                        paragraph
                    } else {
                        paragraph.selectable(false)
                    };
                    paragraph
                        .id(leaf_id)
                        .in_selection_scope(scope, order, separator)
                        .into_any_element()
                }
                SelectableTextGroupBlock::Title(title) => {
                    let title = if self.selectable {
                        title
                    } else {
                        title.selectable(false)
                    };
                    title
                        .id(leaf_id)
                        .in_selection_scope(scope, order, separator)
                        .into_any_element()
                }
            };
            content = content.child(child);
        }
        content
    }
}

impl IntoElement for SelectableTextGroup {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
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
