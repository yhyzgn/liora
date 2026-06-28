//! Searchable list module.
//!
//! `SearchableList` is the shared filtering and option-list primitive used by
//! higher-level controls such as [`crate::Select::searchable`]. It intentionally keeps
//! query matching, grouping, disabled rows, empty states, and selection visuals
//! in one SDK component so application controls do not duplicate list logic.

use crate::gpui_compat::element_id;
use gpui::{
    App, Component, Hsla, IntoElement, MouseButton, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

/// Data model rendered by [`SearchableList`] and reused by [`crate::Select::searchable`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchableListItem {
    /// Stable machine value used by callbacks, forms, and persistence.
    pub value: SharedString,
    /// User-facing primary label.
    pub label: SharedString,
    /// Optional secondary text rendered at the row end.
    pub description: Option<SharedString>,
    /// Optional group header used for grouped rendering.
    pub group: Option<SharedString>,
    /// Whether this row is visible but cannot be selected.
    pub disabled: bool,
}

impl SearchableListItem {
    /// Creates an item whose label and value are the same text.
    pub fn new(value: impl Into<SharedString>) -> Self {
        let value = value.into();
        Self {
            label: value.clone(),
            value,
            description: None,
            group: None,
            disabled: false,
        }
    }

    /// Creates an item with separate value and label fields.
    pub fn labeled(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            group: None,
            disabled: false,
        }
    }

    /// Adds a secondary description to this item.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Assigns the item to a display group.
    pub fn group(mut self, group: impl Into<SharedString>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Toggles disabled state for this row.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn matches_query(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let query = query.to_lowercase();
        self.value.to_string().to_lowercase().contains(&query)
            || self.label.to_string().to_lowercase().contains(&query)
            || self
                .description
                .as_ref()
                .is_some_and(|description| description.to_string().to_lowercase().contains(&query))
            || self
                .group
                .as_ref()
                .is_some_and(|group| group.to_string().to_lowercase().contains(&query))
    }
}

/// Fluent native GPUI component for rendering searchable, grouped option lists.
pub struct SearchableList {
    id: SharedString,
    items: Vec<SearchableListItem>,
    query: SharedString,
    selected_values: Vec<SharedString>,
    empty_text: SharedString,
    max_items: usize,
    width: Option<Pixels>,
    max_height: Pixels,
    item_height: Pixels,
    background: Option<Hsla>,
    on_select: Option<Arc<dyn Fn(SearchableListItem, &mut Window, &mut App) + 'static>>,
}

impl SearchableList {
    /// Creates a searchable list from item data.
    pub fn new(items: Vec<SearchableListItem>) -> Self {
        Self {
            id: "liora-searchable-list".into(),
            items,
            query: SharedString::default(),
            selected_values: Vec::new(),
            empty_text: "No matching results".into(),
            max_items: usize::MAX,
            width: None,
            max_height: px(260.0),
            item_height: px(36.0),
            background: None,
            on_select: None,
        }
    }

    /// Creates items from plain string values.
    pub fn from_values(values: Vec<impl Into<SharedString>>) -> Self {
        Self::new(values.into_iter().map(SearchableListItem::new).collect())
    }

    /// Assigns a stable element id used by GPUI state and tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the active query used to filter rendered rows.
    pub fn query(mut self, query: impl Into<SharedString>) -> Self {
        self.query = query.into();
        self
    }

    /// Marks a single value as selected.
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected_values = vec![value.into()];
        self
    }

    /// Marks multiple values as selected.
    pub fn selected_values(mut self, values: Vec<impl Into<SharedString>>) -> Self {
        self.selected_values = values.into_iter().map(Into::into).collect();
        self
    }

    /// Replaces the empty-state text.
    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
        self
    }

    /// Limits the number of visible matching rows.
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = max.max(1);
        self
    }

    /// Sets a fixed list width.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the maximum list height before clipping/scrolling.
    pub fn max_height(mut self, height: impl Into<Pixels>) -> Self {
        self.max_height = height.into().max(px(80.0));
        self
    }

    /// Sets row height for dense or relaxed lists.
    pub fn item_height(mut self, height: impl Into<Pixels>) -> Self {
        self.item_height = height.into().max(px(28.0));
        self
    }

    /// Overrides the list panel background.
    pub fn background(mut self, background: Hsla) -> Self {
        self.background = Some(background);
        self
    }

    /// Registers a callback invoked for enabled row selection.
    pub fn on_select(
        mut self,
        callback: impl Fn(SearchableListItem, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Arc::new(callback));
        self
    }

    /// Returns the filtered rows without rendering. This keeps matching behavior
    /// testable and lets composite controls reuse the exact same query logic.
    pub fn filtered_items_for(
        items: &[SearchableListItem],
        query: &str,
        max_items: usize,
    ) -> Vec<SearchableListItem> {
        let query = query.trim();
        items
            .iter()
            .filter(|item| item.matches_query(query))
            .take(max_items.max(1))
            .cloned()
            .collect()
    }

    /// Returns whether a value is selected.
    pub fn is_value_selected(&self, value: &str) -> bool {
        self.selected_values
            .iter()
            .any(|selected| selected == value)
    }

    fn filtered_items(&self) -> Vec<SearchableListItem> {
        Self::filtered_items_for(&self.items, self.query.as_ref(), self.max_items)
    }
}

impl RenderOnce for SearchableList {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let items = self.filtered_items();
        let selected_values = self.selected_values.clone();
        let on_select = self.on_select.clone();
        let mut previous_group: Option<SharedString> = None;

        let mut list = div()
            .id(element_id(self.id.clone()))
            .flex()
            .flex_col()
            .overflow_hidden()
            .max_h(self.max_height)
            .rounded(px(theme.radius.md))
            .bg(self.background.unwrap_or(theme.neutral.card))
            .border_1()
            .border_color(theme.neutral.border)
            .when_some(self.width, |s, width| s.w(width));

        if items.is_empty() {
            return list
                .child(
                    div()
                        .px_3()
                        .py_3()
                        .text_sm()
                        .text_color(theme.neutral.text_3)
                        .child(self.empty_text),
                )
                .into_any_element();
        }

        for (index, item) in items.into_iter().enumerate() {
            if item.group != previous_group {
                if let Some(group) = item.group.clone() {
                    list = list.child(
                        div()
                            .px_3()
                            .pt_2()
                            .pb_1()
                            .text_xs()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(theme.neutral.text_3)
                            .child(group),
                    );
                }
                previous_group = item.group.clone();
            }

            let selected = selected_values.iter().any(|value| *value == item.value);
            let disabled = item.disabled;
            let row_item = item.clone();
            let callback = on_select.clone();
            let id = format!("{}-item-{}", self.id, index);
            let row_bg = if selected {
                theme.primary.base.opacity(0.10)
            } else {
                theme.neutral.card
            };
            let row_text = if disabled {
                theme.neutral.text_disabled
            } else if selected {
                theme.primary.base
            } else {
                theme.neutral.text_1
            };

            let mut row = div()
                .id(element_id(id))
                .flex()
                .items_center()
                .justify_between()
                .gap_3()
                .min_h(self.item_height)
                .px_3()
                .py_2()
                .bg(row_bg)
                .text_color(row_text)
                .when(!disabled, |s| {
                    s.cursor_pointer()
                        .hover(move |s| s.cursor_pointer().bg(theme.neutral.hover))
                })
                .when(disabled, |s| s.cursor_not_allowed())
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.0))
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(div().text_sm().child(item.label.clone()))
                        .when_some(item.description.clone(), |s, description| {
                            s.child(
                                div()
                                    .text_xs()
                                    .text_color(theme.neutral.text_3)
                                    .child(description),
                            )
                        }),
                )
                .when(selected, |s| {
                    s.child(
                        Icon::new(IconName::Check)
                            .size(px(14.0))
                            .color(theme.primary.base),
                    )
                });

            if !disabled {
                row = row.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    if let Some(callback) = &callback {
                        callback(row_item.clone(), window, cx);
                    }
                    cx.stop_propagation();
                });
            }

            list = list.child(row);
        }

        list.into_any_element()
    }
}

impl IntoElement for SearchableList {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_items() -> Vec<SearchableListItem> {
        vec![
            SearchableListItem::labeled("button", "Button").group("Basic"),
            SearchableListItem::labeled("select-search", "Select::searchable")
                .description("Searchable select")
                .group("Input"),
            SearchableListItem::labeled("status-bar", "StatusBar").group("Shell"),
        ]
    }

    #[test]
    fn filters_by_value_label_description_and_group() {
        let items = sample_items();
        assert_eq!(
            SearchableList::filtered_items_for(&items, "searchable", 8).len(),
            1
        );
        assert_eq!(
            SearchableList::filtered_items_for(&items, "select", 8).len(),
            1
        );
        assert_eq!(
            SearchableList::filtered_items_for(&items, "shell", 8).len(),
            1
        );
        assert_eq!(
            SearchableList::filtered_items_for(&items, "missing", 8).len(),
            0
        );
    }

    #[test]
    fn filtering_is_trimmed_case_insensitive_and_limited() {
        let items = sample_items();
        let matches = SearchableList::filtered_items_for(&items, "  BAR ", 1);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].value, "status-bar");
    }

    #[test]
    fn selected_values_support_single_and_multiple_modes() {
        let list =
            SearchableList::new(sample_items()).selected_values(vec!["button", "select-search"]);
        assert!(list.is_value_selected("button"));
        assert!(list.is_value_selected("select-search"));
        assert!(!list.is_value_selected("status-bar"));
    }
}
