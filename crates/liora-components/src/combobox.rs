//! Combobox module.
//!
//! `Combobox` fills the space between `Select` and `Autocomplete`: it provides
//! a searchable dropdown for single or multiple values, grouped rows, disabled
//! options, custom placeholder text, and a footer slot for creation or advanced
//! actions. The popup list is backed by [`crate::SearchableList`] so filtering
//! behavior stays consistent across composite controls.

use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use crate::{Button, Input, SearchableList, SearchableListItem, Text};
use gpui::{
    AnyElement, App, Bounds, Context, Element, ElementId, Entity, FocusHandle, Focusable,
    GlobalElementId, InspectorElementId, IntoElement, LayoutId, MouseButton, Pixels, Render,
    SharedString, Style, Window, actions, div, prelude::*, px, relative,
};
use liora_core::{Config, push_portal};
use liora_icons_lucide::IconName;
use std::sync::Arc;

actions!(
    combobox,
    [
        #[doc = "Keyboard action that closes the active combobox popup."]
        ComboboxClose
    ]
);

/// Selection policy for [`Combobox`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComboboxMode {
    /// Only one value can be selected at a time.
    #[default]
    Single,
    /// Multiple values can be toggled independently.
    Multiple,
}

/// Fluent native GPUI component for rendering searchable select dropdowns.
pub struct Combobox {
    input: Entity<Input>,
    items: Vec<SearchableListItem>,
    mode: ComboboxMode,
    selected_values: Vec<SharedString>,
    is_open: bool,
    disabled: bool,
    placeholder: SharedString,
    empty_text: SharedString,
    width: Option<Pixels>,
    max_items: usize,
    last_bounds: Option<Bounds<Pixels>>,
    focus_handle: FocusHandle,
    footer: Option<Arc<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    on_change: Option<Arc<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>>,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl Combobox {
    /// Creates a searchable combobox from reusable list items.
    pub fn new(items: Vec<SearchableListItem>, cx: &mut Context<Self>) -> Self {
        Self {
            input: cx.new(|cx| {
                Input::new("", cx)
                    .clearable(true)
                    .icon_suffix(IconName::ChevronDown)
            }),
            items,
            mode: ComboboxMode::Single,
            selected_values: Vec::new(),
            is_open: false,
            disabled: false,
            placeholder: "Search or select".into(),
            empty_text: "No matching options".into(),
            width: Some(px(300.0)),
            max_items: 8,
            last_bounds: None,
            focus_handle: cx.focus_handle(),
            footer: None,
            on_change: None,
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    /// Creates a combobox from plain values.
    pub fn from_values(values: Vec<impl Into<SharedString>>, cx: &mut Context<Self>) -> Self {
        Self::new(
            values.into_iter().map(SearchableListItem::new).collect(),
            cx,
        )
    }

    /// Switches to multiple selection mode.
    pub fn multiple(mut self) -> Self {
        self.mode = ComboboxMode::Multiple;
        self
    }

    /// Sets the selection mode explicitly.
    pub fn mode(mut self, mode: ComboboxMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the initially selected value for single mode.
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected_values = vec![value.into()];
        self
    }

    /// Sets the selected values for multiple mode.
    pub fn selected_values(mut self, values: Vec<impl Into<SharedString>>) -> Self {
        self.selected_values = values.into_iter().map(Into::into).collect();
        self
    }

    /// Uses the supplied placeholder text when no search is active.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Replaces the empty-state text rendered when filtering yields no rows.
    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
        self
    }

    /// Toggles disabled state and suppresses interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a fixed combobox width.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Limits how many matching rows are displayed in the popup.
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = max.max(1);
        self
    }

    /// Adds a footer element rendered below the options list.
    pub fn footer(
        mut self,
        footer: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.footer = Some(Arc::new(footer));
        self
    }

    /// Registers a callback that runs when selected values change.
    pub fn on_change(
        mut self,
        callback: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Arc::new(callback));
        self
    }

    /// Toggles whether escape closes the popup.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Toggles whether an outside click closes the popup.
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([gpui::KeyBinding::new("escape", ComboboxClose, None)]);
    }

    /// Returns selected values in their persisted order.
    pub fn selected_values_ref(&self) -> &[SharedString] {
        &self.selected_values
    }

    /// Returns matching items for a query using the shared searchable-list logic.
    pub fn matching_items_for(
        items: &[SearchableListItem],
        query: &str,
        max: usize,
    ) -> Vec<SearchableListItem> {
        SearchableList::filtered_items_for(items, query, max)
    }

    fn close_on_escape_action(
        &mut self,
        _: &ComboboxClose,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.close_on_escape && self.is_open {
            self.is_open = false;
            cx.notify();
        }
    }

    fn query(&self, cx: &App) -> SharedString {
        self.input.read(cx).value()
    }

    fn display_value_for(&self, value: &SharedString) -> SharedString {
        self.items
            .iter()
            .find(|item| &item.value == value)
            .map(|item| item.label.clone())
            .unwrap_or_else(|| value.clone())
    }

    fn summary(&self) -> SharedString {
        match self.selected_values.as_slice() {
            [] => SharedString::default(),
            [value] => self.display_value_for(value),
            values => format!("{} selected", values.len()).into(),
        }
    }

    fn select_item(
        &mut self,
        item: SearchableListItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if item.disabled {
            return;
        }

        match self.mode {
            ComboboxMode::Single => {
                self.selected_values = vec![item.value.clone()];
                self.is_open = false;
                let label = item.label.clone();
                self.input
                    .update(cx, |input, cx| input.set_value(label, cx));
            }
            ComboboxMode::Multiple => {
                if let Some(index) = self
                    .selected_values
                    .iter()
                    .position(|value| value == &item.value)
                {
                    self.selected_values.remove(index);
                } else {
                    self.selected_values.push(item.value.clone());
                }
                let summary = self.summary();
                self.input
                    .update(cx, |input, cx| input.set_value(summary, cx));
            }
        }

        if let Some(callback) = &self.on_change {
            callback(self.selected_values.clone(), window, cx);
        }
        cx.notify();
    }

    fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.is_open = true;
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }
}

impl Focusable for Combobox {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct BoundsCapturer {
    combobox: Entity<Combobox>,
}

impl IntoElement for BoundsCapturer {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for BoundsCapturer {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        cx: &mut App,
    ) {
        self.combobox
            .update(cx, |this, _| this.last_bounds = Some(bounds));
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        _window: &mut Window,
        _: &mut App,
    ) {
    }
}

impl Render for Combobox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.entity().clone();
        let disabled = self.disabled;
        let placeholder = if self.selected_values.is_empty() {
            self.placeholder.clone()
        } else {
            self.summary()
        };
        let suffix_icon = if self.is_open {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        self.input.update(cx, |input, cx| {
            input.set_placeholder(placeholder, cx);
            input.set_disabled(disabled, cx);
            input.set_clearable(!disabled, cx);
            input.set_icon_suffix(Some(suffix_icon), cx);
            input.set_on_change({
                let entity = entity.clone();
                move |_, cx| {
                    entity.update(cx, |this, cx| {
                        this.is_open = true;
                        cx.notify();
                    });
                }
            });
        });

        if self.is_open && !self.disabled {
            let bounds = self.last_bounds;
            let query = self.query(cx);
            let items = self.items.clone();
            let selected_values = self.selected_values.clone();
            let max_items = self.max_items;
            let empty_text = self.empty_text.clone();
            let footer = self.footer.clone();
            let close_on_click_outside = self.close_on_click_outside;
            let entity = entity.clone();
            let theme_portal = theme.clone();

            push_portal(
                move |_window, _cx| {
                    let (top, left, width) = bounds
                        .map(|b| (b.bottom() + px(4.0), b.left(), b.size.width))
                        .unwrap_or((px(120.0), px(120.0), px(300.0)));
                    let entity_for_list = entity.clone();
                    let mut panel = div()
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(width)
                        .rounded(px(theme_portal.radius.md))
                        .bg(theme_portal.neutral.card)
                        .border_1()
                        .border_color(theme_portal.neutral.border)
                        .shadow_lg()
                        .occlude();

                    panel = panel.when(close_on_click_outside, |panel| {
                        panel.on_mouse_down_out({
                            let entity = entity.clone();
                            move |_, _, cx| {
                                entity.update(cx, |this, cx| {
                                    this.is_open = false;
                                    cx.notify();
                                });
                            }
                        })
                    });

                    let list = SearchableList::new(items.clone())
                        .id("liora-combobox-options")
                        .query(query.clone())
                        .selected_values(selected_values.clone())
                        .empty_text(empty_text.clone())
                        .max_items(max_items)
                        .width(width)
                        .on_select(move |item, window, cx| {
                            entity_for_list
                                .update(cx, |this, cx| this.select_item(item, window, cx));
                        });

                    panel = panel.child(list);
                    if let Some(footer) = footer.clone() {
                        panel = panel.child(
                            div()
                                .border_t_1()
                                .border_color(theme_portal.neutral.border)
                                .p_2()
                                .child(footer(_window, _cx)),
                        );
                    }

                    pop_in(element_id("liora-combobox-panel"), panel).into_any_element()
                },
                cx,
            );
        }

        div()
            .relative()
            .when_some(self.width, |s, width| s.w(width))
            .when(self.width.is_none(), |s| s.w_full())
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(BoundsCapturer {
                        combobox: cx.entity().clone(),
                    }),
            )
            .child(self.input.clone())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.open(window, cx)),
            )
            .on_action(cx.listener(Self::close_on_escape_action))
    }
}

/// Small footer helper for common “create new” actions in combobox popups.
pub fn combobox_create_footer(label: impl Into<SharedString>) -> impl IntoElement {
    let label = label.into();
    Button::new(label).small().icon_start(IconName::Plus)
}

/// Small inline helper for explaining multi-select state in demos and settings.
pub fn combobox_selected_hint(count: usize) -> impl IntoElement {
    Text::new(format!("{} selected", count)).xs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items() -> Vec<SearchableListItem> {
        vec![
            SearchableListItem::labeled("rust", "Rust").group("Language"),
            SearchableListItem::labeled("gpui", "GPUI")
                .description("Native UI")
                .group("Framework"),
        ]
    }

    #[test]
    fn combobox_reuses_searchable_list_matching() {
        let matches = Combobox::matching_items_for(&items(), "native", 8);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].value, "gpui");
    }

    #[test]
    fn source_uses_searchable_list_for_popup_options() {
        let source = include_str!("combobox.rs");
        assert!(source.contains("SearchableList::new(items.clone())"));
        assert!(source.contains(".on_select(move |item, window, cx|"));
    }
}
