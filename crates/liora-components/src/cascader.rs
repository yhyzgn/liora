//! Cascader module.
//!
//! This public module implements the Liora multi-level cascading selection popup component. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
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
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::Input;
use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{
    AnyElement, App, Bounds, Context, Element, ElementId, Entity, FocusHandle, Focusable,
    IntoElement, KeyBinding, MouseButton, Pixels, Render, ScrollHandle, SharedString, Window, actions, div,
    prelude::*, px,
};
use liora_core::{Config, push_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

const SEARCH_INPUT_HEIGHT: f32 = 34.0;
// Input plus 8px padding on each side and the header's bottom border.
const SEARCH_HEADER_HEIGHT: f32 = SEARCH_INPUT_HEIGHT + 17.0;

actions!(
    cascader,
    [
        #[doc = "Keyboard action that closes the active cascader popup."]
        CascaderClose
    ]
);

#[derive(Debug, Clone, PartialEq, Eq)]
/// Fluent native GPUI component for rendering Liora cascader option.
pub struct CascaderOption {
    /// Machine-readable value represented by this item.
    pub value: SharedString,
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Nested child items rendered beneath this item.
    pub children: Vec<CascaderOption>,
    /// Whether user interaction is disabled for this item.
    pub disabled: bool,
    /// Whether this cascader option is currently loading lazy children.
    pub loading: bool,
    /// Whether this cascader option is known to have no children.
    pub leaf: bool,
}

/// Fluent native GPUI component for rendering Liora cascader.
pub struct Cascader {
    id: SharedString,
    options: Vec<CascaderOption>,
    selected_path: Vec<SharedString>,
    active_path: Vec<SharedString>,
    placeholder: SharedString,
    separator: SharedString,
    is_open: bool,
    disabled: bool,
    clearable: bool,
    filterable: bool,
    search_query: SharedString,
    search_input: Option<Entity<Input>>,
    highlighted_result: Option<Vec<SharedString>>,
    search_scroll: Option<ScrollHandle>,
    width: Option<Pixels>,
    focus_handle: FocusHandle,
    last_bounds: Option<Bounds<Pixels>>,
    lazy: bool,
    close_on_click_outside: bool,
    close_on_escape: bool,
    on_change: Option<Arc<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>>,
    on_lazy_load: Option<
        Arc<
            dyn Fn(&mut Cascader, Vec<SharedString>, &mut Window, &mut Context<Cascader>) + 'static,
        >,
    >,
}

impl CascaderOption {
    /// Creates `CascaderOption` initialized from the supplied value, and label.
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            children: vec![],
            disabled: false,
            loading: false,
            leaf: false,
        }
    }

    /// Adds a child element to the component body.
    pub fn child(mut self, child: CascaderOption) -> Self {
        self.children.push(child);
        self
    }

    /// Replaces or appends child elements rendered by the component.
    pub fn children(mut self, children: impl IntoIterator<Item = CascaderOption>) -> Self {
        self.children.extend(children);
        self
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Toggles the loading state and associated spinner treatment.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets the leaf value used by the component.
    pub fn leaf(mut self, leaf: bool) -> Self {
        self.leaf = leaf;
        self
    }

    fn find_child(&self, value: &SharedString) -> Option<&CascaderOption> {
        self.children.iter().find(|child| &child.value == value)
    }
}

impl Cascader {
    /// Creates `Cascader` that renders the supplied options collection.
    pub fn new(options: Vec<CascaderOption>, cx: &mut Context<Self>) -> Self {
        Self {
            id: liora_core::unique_id("cascader"),
            options,
            selected_path: vec![],
            active_path: vec![],
            placeholder: "请选择".into(),
            separator: " / ".into(),
            is_open: false,
            disabled: false,
            clearable: false,
            filterable: false,
            search_query: SharedString::default(),
            search_input: None,
            highlighted_result: None,
            search_scroll: None,
            width: None,
            focus_handle: cx.focus_handle(),
            last_bounds: None,
            lazy: false,
            close_on_click_outside: true,
            close_on_escape: true,
            on_change: None,
            on_lazy_load: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Derives an opaque, stable item identity from the complete prefix and value path.
    pub fn popup_item_id(prefix: impl AsRef<str>, path: &[SharedString]) -> SharedString {
        let mut id = String::new();
        append_id_segment(&mut id, prefix.as_ref());
        id.push_str("-item");
        for value in path {
            id.push('-');
            append_id_segment(&mut id, value.as_ref());
        }
        id.into()
    }

    /// Returns the filesystem path for the selected resource.
    pub fn selected_path(
        mut self,
        path: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.selected_path = path.into_iter().map(Into::into).collect();
        self.active_path = self.selected_path.clone();
        self
    }

    /// Uses the supplied placeholder text when the value is empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Creates a tray menu separator item specification.
    pub fn separator(mut self, separator: impl Into<SharedString>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Toggles whether the component renders a clear affordance.
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Enables built-in filtering behavior.
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    /// Sets the search query value used by the component.
    pub fn search_query(mut self, query: impl Into<SharedString>) -> Self {
        self.search_query = query.into();
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Applies the predefined width md sizing preset.
    pub fn width_md(self) -> Self {
        self.width(px(360.0))
    }

    /// Enables lazy loading for child data.
    pub fn lazy(mut self, lazy: bool) -> Self {
        self.lazy = lazy;
        self
    }

    /// Toggles whether the popup closes when escape occurs.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Toggles whether the popup closes when click outside occurs.
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", CascaderClose, Some("Cascader"))]);
    }

    fn close_on_escape_action(
        &mut self,
        _: &CascaderClose,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .search_input
            .as_ref()
            .is_some_and(|input| input.read(cx).is_composing())
        {
            cx.propagate();
            return;
        }
        if self.close_on_escape && self.is_open {
            self.is_open = false;
            self.restore_query_focus(window, cx);
            cx.notify();
        } else {
            cx.propagate();
        }
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(
        mut self,
        f: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Arc::new(f));
        self
    }

    /// Registers a callback that runs when lazy load occurs.
    pub fn on_lazy_load(
        mut self,
        f: impl Fn(&mut Cascader, Vec<SharedString>, &mut Window, &mut Context<Cascader>) + 'static,
    ) -> Self {
        self.on_lazy_load = Some(Arc::new(f));
        self
    }

    /// Updates the stored on lazy load value and keeps the existing component identity.
    pub fn set_on_lazy_load(
        &mut self,
        f: impl Fn(&mut Cascader, Vec<SharedString>, &mut Window, &mut Context<Cascader>) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.on_lazy_load = Some(Arc::new(f));
        cx.notify();
    }

    /// Updates the stored options value and keeps the existing component identity.
    pub fn set_options(&mut self, options: Vec<CascaderOption>, cx: &mut Context<Self>) {
        self.options = options;
        self.highlighted_result = None;
        if !Self::is_selectable_path(&self.options, &self.selected_path) {
            self.selected_path.clear();
            self.active_path.clear();
        }
        cx.notify();
    }

    /// Updates the stored children at path value and keeps the existing component identity.
    pub fn set_children_at_path(
        &mut self,
        path: &[SharedString],
        children: Vec<CascaderOption>,
        cx: &mut Context<Self>,
    ) -> bool {
        let changed = Self::set_children_in_options(&mut self.options, path, children);
        if changed {
            Self::set_loading_in_options(&mut self.options, path, false);
            self.highlighted_result = None;
            cx.notify();
        }
        changed
    }

    /// Updates the stored loading at path value and keeps the existing component identity.
    pub fn set_loading_at_path(
        &mut self,
        path: &[SharedString],
        loading: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let changed = Self::set_loading_in_options(&mut self.options, path, loading);
        if changed {
            self.highlighted_result = None;
            cx.notify();
        }
        changed
    }

    /// Updates the stored selected path value and keeps the existing component identity.
    pub fn set_selected_path(
        &mut self,
        path: impl IntoIterator<Item = impl Into<SharedString>>,
        cx: &mut Context<Self>,
    ) {
        self.selected_path = path.into_iter().map(Into::into).collect();
        self.active_path = self.selected_path.clone();
        cx.notify();
    }

    /// Updates the stored search query value and keeps the existing component identity.
    pub fn set_search_query(&mut self, query: impl Into<SharedString>, cx: &mut Context<Self>) {
        let query = query.into();
        if self.search_query != query {
            self.highlighted_result = None;
            self.search_query = query;
        }
        if let Some(input) = &self.search_input {
            input.update(cx, |input, cx| {
                input.set_value(self.search_query.clone(), cx)
            });
        }
        cx.notify();
    }

    /// Returns the filesystem path for the labels for resource.
    pub fn labels_for_path(options: &[CascaderOption], path: &[SharedString]) -> Vec<SharedString> {
        let mut labels = Vec::new();
        let mut siblings = options;

        for value in path {
            let Some(option) = siblings.iter().find(|option| &option.value == value) else {
                return vec![];
            };
            labels.push(option.label.clone());
            siblings = &option.children;
        }

        labels
    }

    /// Returns whether the path ends at a selectable leaf and every ancestor is enabled and not loading.
    pub fn is_selectable_path(options: &[CascaderOption], path: &[SharedString]) -> bool {
        let Some(option) = Self::option_for_path(options, path) else {
            return false;
        };
        Self::is_selectable_option(option, false)
    }

    /// Returns whether selectable option is currently true for this value.
    pub fn is_selectable_option(option: &CascaderOption, lazy: bool) -> bool {
        !option.disabled
            && !option.loading
            && (option.leaf || (!lazy && option.children.is_empty()))
    }

    /// Returns whether the cascader option should request lazy children.
    pub fn should_lazy_load_option(option: &CascaderOption, lazy: bool) -> bool {
        lazy && !option.disabled && !option.loading && !option.leaf && option.children.is_empty()
    }

    /// Updates the stored children in options value and keeps the existing component identity.
    pub fn set_children_in_options(
        options: &mut [CascaderOption],
        path: &[SharedString],
        children: Vec<CascaderOption>,
    ) -> bool {
        let Some(option) = Self::option_for_path_mut(options, path) else {
            return false;
        };
        option.children = children;
        option.loading = false;
        true
    }

    /// Updates the stored loading in options value and keeps the existing component identity.
    pub fn set_loading_in_options(
        options: &mut [CascaderOption],
        path: &[SharedString],
        loading: bool,
    ) -> bool {
        let Some(option) = Self::option_for_path_mut(options, path) else {
            return false;
        };
        option.loading = loading;
        true
    }

    fn option_for_path<'a>(
        options: &'a [CascaderOption],
        path: &[SharedString],
    ) -> Option<&'a CascaderOption> {
        let (first, rest) = path.split_first()?;
        let mut option = options.iter().find(|option| &option.value == first)?;
        if option.disabled || option.loading {
            return None;
        }

        for value in rest {
            option = option.find_child(value)?;
            if option.disabled || option.loading {
                return None;
            }
        }

        Some(option)
    }

    fn option_for_path_mut<'a>(
        options: &'a mut [CascaderOption],
        path: &[SharedString],
    ) -> Option<&'a mut CascaderOption> {
        let (first, rest) = path.split_first()?;
        let option = options.iter_mut().find(|option| &option.value == first)?;
        Self::option_for_path_mut_from_option(option, rest)
    }

    fn option_for_path_mut_from_option<'a>(
        option: &'a mut CascaderOption,
        path: &[SharedString],
    ) -> Option<&'a mut CascaderOption> {
        let Some((first, rest)) = path.split_first() else {
            return Some(option);
        };
        let child = option
            .children
            .iter_mut()
            .find(|child| &child.value == first)?;
        Self::option_for_path_mut_from_option(child, rest)
    }

    fn columns_for_active_path(&self) -> Vec<&[CascaderOption]> {
        let mut columns = vec![self.options.as_slice()];
        let mut siblings = self.options.as_slice();

        for value in &self.active_path {
            let Some(option) = siblings.iter().find(|option| &option.value == value) else {
                break;
            };
            if option.disabled || option.loading || option.children.is_empty() {
                break;
            }
            columns.push(option.children.as_slice());
            siblings = &option.children;
        }

        columns
    }

    fn matching_leaf_paths(&self) -> Vec<(Vec<SharedString>, Vec<SharedString>)> {
        let query = self.search_query.to_string().to_lowercase();
        if query.trim().is_empty() {
            return vec![];
        }

        let mut matches = Vec::new();
        collect_leaf_matches(
            &self.options,
            &query,
            self.lazy,
            &mut vec![],
            &mut vec![],
            &mut matches,
        );
        matches
    }

    fn display_text(&self) -> SharedString {
        let labels = Self::labels_for_path(&self.options, &self.selected_path);
        if labels.is_empty() {
            return self.placeholder.clone();
        }
        labels
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(self.separator.as_ref())
            .into()
    }

    fn search_key_down(
        &mut self,
        key: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.is_open
            || self.disabled
            || !self.filterable
            || !matches!(key, "up" | "down" | "enter")
        {
            return false;
        }
        let Some(input) = &self.search_input else {
            return false;
        };
        let input = input.read(cx);
        if !input.focus_handle(cx).is_focused(window) || input.is_composing() {
            // Input retains Enter's no-op callback, so passthrough cannot select a result.
            return false;
        }
        let query = input.value();
        if self.search_query != query {
            // A key can precede the editor's deferred on_change notification.
            self.search_query = query;
            self.highlighted_result = None;
            cx.notify();
        }
        if self.search_query.trim().is_empty()
            || (key == "enter" && self.highlighted_result.is_none())
        {
            return false;
        }
        let mut matches = self.matching_leaf_paths();
        if matches.is_empty() {
            self.highlighted_result = None;
            return false;
        }
        if key == "enter" {
            if let Some(path) = self.highlighted_result.take() {
                if matches.iter().any(|(current, _)| current == &path) {
                    self.choose_path(path, window, cx);
                }
            }
        } else {
            let current = self.highlighted_result.as_ref().and_then(|path| {
                matches.iter().position(|(current, _)| current == path)
            });
            let index = if key == "down" {
                current.map_or(0, |index| (index + 1).min(matches.len() - 1))
            } else {
                current.map_or(matches.len() - 1, |index| index.saturating_sub(1))
            };
            self.highlighted_result = Some(matches.swap_remove(index).0);
            self.search_scroll
                .as_ref()
                .expect("search Input initializes scroll handle")
                .scroll_to_item(index);
        }
        cx.notify();
        true
    }

    fn ensure_search_input(&mut self, cx: &mut Context<Self>) {
        if !self.filterable || self.search_input.is_some() {
            return;
        }
        self.search_scroll = Some(ScrollHandle::new());
        let owner = cx.entity().downgrade();
        let query = self.search_query.clone();
        let id = format!("{}-query", self.id);
        self.search_input = Some(cx.new(move |cx| {
            Input::new(query, cx)
                .id(id)
                .aria_label("Search options")
                .placeholder("Search options")
                .height(px(SEARCH_INPUT_HEIGHT))
                .clearable(true)
                .on_enter(|_, _, _, _| {})
                .on_change(move |query, cx| {
                    let owner = owner.clone();
                    let input = cx.entity().downgrade();
                    let query = SharedString::from(query.to_owned());
                    cx.defer(move |cx| {
                        let Some(input) = input.upgrade() else {
                            return;
                        };
                        if input.read(cx).value() != query {
                            return;
                        }
                        let _ = owner.update(cx, |this, cx| {
                            // User edits already live in Input, including marked text.
                            if this.search_query != query {
                                this.search_query = query;
                                this.highlighted_result = None;
                            }
                            cx.notify();
                        });
                    });
                })
        }));
    }

    fn restore_query_focus(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .search_input
            .as_ref()
            .is_some_and(|input| input.focus_handle(cx).is_focused(window))
        {
            window.focus(&self.focus_handle, cx);
        }
    }

    fn dismiss_and_traverse(&mut self, reverse: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.is_open = false;
        window.focus(&self.focus_handle, cx);
        if reverse {
            window.focus_prev(cx);
        } else {
            window.focus_next(cx);
        }
        // Traversal still uses the painted frame; skip its one retiring editor stop.
        if self
            .search_input
            .as_ref()
            .is_some_and(|input| input.focus_handle(cx).is_focused(window))
        {
            if reverse {
                window.focus_prev(cx);
            } else {
                window.focus_next(cx);
            }
        }
        cx.notify();
    }

    fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.is_open = !self.is_open;
        if self.is_open {
            self.active_path = self.selected_path.clone();
            self.highlighted_result = None;
            self.ensure_search_input(cx);
            let focus = self
                .search_input
                .as_ref()
                .map(|input| input.focus_handle(cx))
                .unwrap_or_else(|| self.focus_handle.clone());
            window.focus(&focus, cx);
        } else {
            self.restore_query_focus(window, cx);
        }
        cx.notify();
    }

    fn choose_path(
        &mut self,
        path: Vec<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let Some(option) = Self::option_for_path(&self.options, &path) else {
            return;
        };

        if Self::should_lazy_load_option(option, self.lazy) {
            self.active_path = path.clone();
            Self::set_loading_in_options(&mut self.options, &path, true);
            if let Some(on_lazy_load) = self.on_lazy_load.clone() {
                on_lazy_load(self, path, window, cx);
            } else {
                cx.notify();
            }
            return;
        }

        if !Self::is_selectable_option(option, self.lazy) {
            self.active_path = path;
            cx.notify();
            return;
        }

        self.selected_path = path.clone();
        self.active_path = path.clone();
        self.is_open = false;
        if let Some(on_change) = &self.on_change {
            on_change(path, window, cx);
        }
        self.restore_query_focus(window, cx);
        cx.notify();
    }

    fn clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_path.is_empty() {
            return;
        }
        self.selected_path.clear();
        self.active_path.clear();
        if let Some(on_change) = &self.on_change {
            on_change(vec![], window, cx);
        }
        cx.notify();
    }
}

impl Focusable for Cascader {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Cascader {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        self.focus_handle = self.focus_handle.clone().tab_stop(!self.disabled);
        let focused = self.focus_handle.is_focused(_window);
        let display_text = self.display_text();
        let has_value = !self.selected_path.is_empty();
        let text_color = if has_value {
            theme.neutral.text_1
        } else {
            theme.neutral.text_3
        };
        let border_color = if focused || self.is_open {
            theme.primary.base
        } else {
            theme.neutral.border
        };

        if self.is_open {
            let entity = cx.entity().clone();
            let theme_portal = theme.clone();
            let trigger_bounds = self.last_bounds;
            let cascader_id = self.id.clone();
            let close_on_click_outside = self.close_on_click_outside;

            push_portal(
                move |window, cx| {
                    // The one-shot portal builds owned elements before this entity borrow ends.
                    let this = entity.read(cx);
                    let columns = this.columns_for_active_path();
                    let active_path = &this.active_path;
                    let selected_path = &this.selected_path;
                    let filterable = this.filterable;
                    let searching = filterable && !this.search_query.trim().is_empty();
                    let matches = this.matching_leaf_paths();
                    let trigger_width =
                        trigger_bounds.map_or(px(240.0), |bounds| bounds.size.width);
                    let header_height = if filterable {
                        px(SEARCH_HEADER_HEIGHT)
                    } else {
                        px(0.0)
                    };
                    let popup_height = header_height
                        + if searching {
                            popup_content_height(matches.len())
                        } else {
                            popup_content_height(
                                columns
                                    .iter()
                                    .map(|column| column.len())
                                    .max()
                                    .unwrap_or_default(),
                            )
                        };
                    let requested_panel_width =
                        px((columns.len().max(1) as f32 * 180.0).max(f32::from(trigger_width)));
                    let viewport = window.viewport_size();
                    let panel_width = popup_panel_width(requested_panel_width, viewport.width);
                    let (top, left) =
                        popup_position(trigger_bounds, panel_width, popup_height, viewport);
                    let panel_height =
                        popup_height.min((viewport.height - top - px(8.0)).max(px(0.0)));
                    let results_height = (panel_height - px(2.0) - header_height).max(px(0.0));
                    let theme = theme_portal.clone();
                    let entity = entity.clone();

                    let close_entity = entity.clone();
                    let key_entity = entity.clone();
                    let escape_entity = entity.clone();
                    let panel = div();
                    #[cfg(test)]
                    let panel = panel.debug_selector(|| "cascader-panel".to_string());
                    let panel = panel
                        .id(element_id(format!("{}-panel", cascader_id)))
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(panel_width)
                        .h(panel_height)
                        .flex()
                        .flex_col()
                        .overflow_hidden()
                        .bg(theme.neutral.card)
                        .rounded(px(theme.radius.md))
                        .border_1()
                        .border_color(theme.neutral.border)
                        .shadow(vec![gpui::BoxShadow {
                            color: theme.neutral.border,
                            offset: gpui::point(px(0.0), px(4.0)),
                            blur_radius: px(14.0),
                            spread_radius: px(0.0),
                            inset: false,
                        }])
                        .occlude()
                        .on_mouse_down(MouseButton::Left, |_, _, cx| {
                            cx.stop_propagation();
                        })
                        .when_some(
                            this.search_input.as_ref().filter(|_| filterable),
                            |panel, input| {
                                let header = div();
                                #[cfg(test)]
                                let header =
                                    header.debug_selector(|| "cascader-search-header".to_string());
                                panel.child(
                                    header
                                        .h(header_height)
                                        .flex_shrink_0()
                                        .p_2()
                                        .border_b_1()
                                        .border_color(theme.neutral.divider)
                                        .child(input.clone()),
                                )
                            },
                        )
                        .when(results_height > px(0.0), |panel| {
                            if searching && matches.is_empty() {
                                let empty = div();
                                #[cfg(test)]
                                let empty =
                                    empty.debug_selector(|| "cascader-no-results".to_string());
                                panel.child(
                                    empty
                                        .h(results_height)
                                        .flex_shrink_0()
                                        .px_3()
                                        .flex()
                                        .items_center()
                                        .text_sm()
                                        .text_color(theme.neutral.text_3)
                                        .child("No results"),
                                )
                            } else if searching {
                                panel.child(render_match_list(
                                    cascader_id.clone(),
                                    matches,
                                    this.highlighted_result.as_deref(),
                                    this.search_scroll
                                        .as_ref()
                                        .expect("search Input initializes scroll handle"),
                                    entity.clone(),
                                    theme.clone(),
                                    results_height,
                                ))
                            } else {
                                panel.child(render_columns(
                                    cascader_id.clone(),
                                    columns,
                                    active_path,
                                    selected_path,
                                    entity.clone(),
                                    theme.clone(),
                                    results_height,
                                ))
                            }
                        });

                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .bg(gpui::transparent_black())
                        .key_context("Cascader")
                        .on_action(move |action: &CascaderClose, window, cx| {
                            escape_entity.update(cx, |this, cx| {
                                this.close_on_escape_action(action, window, cx)
                            });
                        })
                        // Bound Input actions are dispatched before raw key listeners.
                        .capture_action({
                            let entity = entity.clone();
                            move |_: &crate::input::InputUp, window, cx| {
                                if entity.update(cx, |this, cx| this.search_key_down("up", window, cx)) {
                                    cx.stop_propagation();
                                }
                            }
                        })
                        .capture_action({
                            let entity = entity.clone();
                            move |_: &crate::input::InputDown, window, cx| {
                                if entity.update(cx, |this, cx| this.search_key_down("down", window, cx)) {
                                    cx.stop_propagation();
                                }
                            }
                        })
                        .capture_action({
                            let entity = entity.clone();
                            move |_: &crate::input::Enter, window, cx| {
                                if entity.update(cx, |this, cx| this.search_key_down("enter", window, cx)) {
                                    cx.stop_propagation();
                                }
                            }
                        })
                        .capture_key_down(move |event, window, cx| {
                            let modifiers = event.keystroke.modifiers;
                            if event.keystroke.key == "tab"
                                && !modifiers.control
                                && !modifiers.alt
                                && !modifiers.platform
                                && !modifiers.function
                            {
                                if key_entity
                                    .read(cx)
                                    .search_input
                                    .as_ref()
                                    .is_some_and(|input| input.read(cx).is_composing())
                                {
                                    return;
                                }
                                key_entity.update(cx, |this, cx| {
                                    this.dismiss_and_traverse(modifiers.shift, window, cx)
                                });
                                window.prevent_default();
                                cx.stop_propagation();
                            }
                        })
                        .when(close_on_click_outside, |s| {
                            s.on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                close_entity.update(cx, |this, cx| {
                                    this.is_open = false;
                                    cx.notify();
                                });
                            })
                        })
                        .child(pop_in(
                            element_id(format!("{}-panel-motion", cascader_id)),
                            panel,
                        ))
                        .into_any_element()
                },
                cx,
            );
        }

        div()
            .id(element_id(self.id.clone()))
            .accessibility_id(self.id.clone())
            .role(gpui::Role::Button)
            .aria_label(display_text.clone())
            .aria_expanded(self.is_open)
            .when(self.disabled, |s| {
                s.a11y_synthetic_children(|builder| {
                    builder.parent_node().set_disabled();
                })
            })
            .track_focus(&self.focus_handle)
            .key_context("Cascader")
            .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, window, cx| {
                let modifiers = event.keystroke.modifiers;
                if event.keystroke.key == "tab"
                    && !modifiers.control
                    && !modifiers.alt
                    && !modifiers.platform
                    && !modifiers.function
                {
                    if modifiers.shift {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                    cx.stop_propagation();
                } else if !this.disabled
                    && !modifiers.modified()
                    && matches!(event.keystroke.key.as_str(), "enter" | "space")
                {
                    // Native GPUI delivers activation on key-up through on_click.
                    cx.stop_propagation();
                }
            }))
            .when(!self.disabled, |s| {
                s.on_click(cx.listener(|this, event, window, cx| {
                    if matches!(event, gpui::ClickEvent::Keyboard(_)) {
                        this.toggle_open(window, cx);
                        cx.stop_propagation();
                    }
                }))
            })
            .relative()
            .when_some(self.width, |s, width| s.w(width))
            .when(self.width.is_none(), |s| s.w_full())
            .h(px(34.0))
            .flex()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .bg(if self.disabled {
                theme.neutral.hover
            } else {
                theme.neutral.card
            })
            .border_1()
            .border_color(border_color)
            .rounded(px(theme.radius.md))
            .text_size(px(theme.font_size.md))
            .text_color(text_color)
            .when(!self.disabled, |s| {
                s.cursor_pointer()
                    .hover(|s| s.border_color(theme.primary.base).cursor_pointer())
            })
            .when(self.disabled, |s| s.cursor_not_allowed())
            .child(div().flex_1().overflow_hidden().child(display_text))
            .when(self.clearable && has_value && !self.disabled, |s| {
                s.child(
                    div()
                        .id(element_id(format!("{}-clear", self.id)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .hover(|s| s.bg(theme.neutral.hover))
                        .child(
                            Icon::new(IconName::X)
                                .size(px(14.0))
                                .color(theme.neutral.icon),
                        )
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, window, cx| {
                                this.clear(window, cx);
                                cx.stop_propagation();
                            }),
                        ),
                )
            })
            .child(
                Icon::new(if self.is_open {
                    IconName::ChevronUp
                } else {
                    IconName::ChevronDown
                })
                .size(px(14.0))
                .color(theme.neutral.icon),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(BoundsCapturer {
                        cascader: cx.entity().clone(),
                    }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.toggle_open(window, cx);
                    // Suppress the tracked trigger's later native mouse-focus default.
                    window.prevent_default();
                }),
            )
            .on_action(cx.listener(Self::close_on_escape_action))
    }
}

fn popup_panel_width(requested: Pixels, viewport_width: Pixels) -> Pixels {
    requested.min((viewport_width - px(16.0)).max(px(1.0)))
}

fn popup_content_height(row_count: usize) -> Pixels {
    if row_count > 8 {
        px(280.0)
    } else {
        px((row_count.max(1) as f32 * 34.0) + 2.0)
    }
}

fn popup_position(
    trigger_bounds: Option<Bounds<Pixels>>,
    panel_width: Pixels,
    panel_height: Pixels,
    viewport: gpui::Size<Pixels>,
) -> (Pixels, Pixels) {
    let margin = px(8.0);
    let max_left = (viewport.width - panel_width - margin).max(margin);
    let max_top = (viewport.height - panel_height - margin).max(margin);
    let (preferred_top, preferred_left) = trigger_bounds.map_or((margin, margin), |bounds| {
        (bounds.bottom() + px(4.0), bounds.left())
    });
    let top = if preferred_top <= max_top {
        preferred_top
    } else {
        trigger_bounds
            .map(|bounds| (bounds.top() - panel_height - px(4.0)).max(margin))
            .unwrap_or(max_top)
            .min(max_top)
    };
    (top, preferred_left.min(max_left).max(margin))
}

fn render_columns(
    cascader_id: SharedString,
    columns: Vec<&[CascaderOption]>,
    active_path: &[SharedString],
    selected_path: &[SharedString],
    entity: Entity<Cascader>,
    theme: liora_theme::Theme,
    panel_height: Pixels,
) -> AnyElement {
    div()
        .id(element_id(format!("{}-columns", cascader_id)))
        .w_full()
        .flex()
        .flex_row()
        .overflow_x_scroll()
        .children(columns.into_iter().enumerate().map(|(depth, column)| {
            let active_value = active_path.get(depth);
            let selected_value = selected_path.get(depth);
            let path_prefix = active_path.iter().take(depth).cloned().collect::<Vec<_>>();
            let column_id = format!("{}-column-{depth}", cascader_id);
            let item_id_prefix = cascader_id.clone();
            div()
                .id(element_id(column_id))
                .w(px(180.0))
                .flex_shrink_0()
                .max_h(panel_height)
                .overflow_y_scroll()
                .border_r_1()
                .border_color(theme.neutral.divider)
                .children(column.iter().map({
                    let entity = entity.clone();
                    let theme = theme.clone();
                    let item_id_prefix = item_id_prefix.clone();
                    move |option| {
                        let mut path = path_prefix.clone();
                        path.push(option.value.clone());
                        let is_active = Some(&option.value) == active_value;
                        let is_selected = Some(&option.value) == selected_value;
                        let has_children = !option.children.is_empty();
                        let disabled = option.disabled || option.loading;
                        let entity = entity.clone();
                        let theme = theme.clone();

                        let item_id = Cascader::popup_item_id(item_id_prefix.as_ref(), &path);

                        let item = div();
                        #[cfg(test)]
                        let item = item.debug_selector(|| item_id.to_string());
                        item.id(item_id)
                            .h(px(34.0))
                            .px_3()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_2()
                            .bg(if is_active || is_selected {
                                theme.primary.base.opacity(0.1)
                            } else {
                                theme.neutral.card
                            })
                            .text_color(if disabled {
                                theme.neutral.text_3
                            } else if is_active || is_selected {
                                theme.primary.base
                            } else {
                                theme.neutral.text_1
                            })
                            .when(!disabled, |s| {
                                s.cursor_pointer()
                                    .hover(|s| s.bg(theme.neutral.hover).cursor_pointer())
                            })
                            .when(disabled, |s| s.cursor_not_allowed())
                            .child(div().flex_1().text_sm().child(option.label.clone()))
                            .when(option.loading, |s| {
                                s.child(
                                    Icon::new(IconName::LoaderCircle)
                                        .size(px(14.0))
                                        .color(theme.neutral.icon),
                                )
                            })
                            .when(has_children && !option.loading, |s| {
                                s.child(
                                    Icon::new(IconName::ChevronRight)
                                        .size(px(14.0))
                                        .color(theme.neutral.icon),
                                )
                            })
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                if disabled {
                                    return;
                                }
                                entity.update(cx, |this, cx| {
                                    this.choose_path(path.clone(), window, cx);
                                });
                                cx.stop_propagation();
                            })
                    }
                }))
        }))
        .into_any_element()
}

fn render_match_list(
    cascader_id: SharedString,
    matches: Vec<(Vec<SharedString>, Vec<SharedString>)>,
    highlighted_result: Option<&[SharedString]>,
    scroll: &ScrollHandle,
    entity: Entity<Cascader>,
    theme: liora_theme::Theme,
    panel_height: Pixels,
) -> AnyElement {
    let list = div();
    #[cfg(test)]
    let list = list.debug_selector(|| "cascader-search-results".to_string());
    list.id("cascader-search-results")
        .flex()
        .flex_col()
        .max_h(panel_height)
        .overflow_y_scroll()
        .track_scroll(scroll)
        .children(matches.into_iter().map(move |(path, labels)| {
            let entity = entity.clone();
            let theme = theme.clone();
            let text = labels
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" / ");
            let item_id = Cascader::popup_item_id(cascader_id.as_ref(), &path);
            let highlighted = highlighted_result == Some(path.as_slice());

            let item = div();
            #[cfg(test)]
            let item = item.debug_selector(|| item_id.to_string());
            item.id(item_id)
                .h(px(34.0))
                .flex_shrink_0()
                .px_3()
                .flex()
                .items_center()
                .cursor_pointer()
                .when(highlighted, |s| s.bg(theme.primary.base.opacity(0.1)))
                .hover(|s| s.bg(theme.neutral.hover).cursor_pointer())
                .child(div().text_sm().text_color(theme.neutral.text_1).child(text))
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    entity.update(cx, |this, cx| {
                        this.choose_path(path.clone(), window, cx);
                    });
                    cx.stop_propagation();
                })
        }))
        .into_any_element()
}

fn append_id_segment(id: &mut String, segment: &str) {
    for ch in segment.chars() {
        match ch {
            '%' => id.push_str("%25"),
            '-' => id.push_str("%2D"),
            _ => id.push(ch),
        }
    }
}

fn collect_leaf_matches(
    options: &[CascaderOption],
    query: &str,
    lazy: bool,
    path: &mut Vec<SharedString>,
    labels: &mut Vec<SharedString>,
    matches: &mut Vec<(Vec<SharedString>, Vec<SharedString>)>,
) {
    for option in options {
        if option.disabled || option.loading {
            continue;
        }
        path.push(option.value.clone());
        labels.push(option.label.clone());

        if option.children.is_empty() && Cascader::is_selectable_option(option, lazy) {
            let haystack = labels
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" /")
                .to_lowercase();
            if haystack.contains(query) {
                matches.push((path.clone(), labels.clone()));
            }
        } else {
            collect_leaf_matches(&option.children, query, lazy, path, labels, matches);
        }

        path.pop();
        labels.pop();
    }
}

struct BoundsCapturer {
    cascader: Entity<Cascader>,
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
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, ()) {
        let mut style = gpui::Style::default();
        style.size.width = gpui::relative(1.0).into();
        style.size.height = gpui::relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        cx: &mut App,
    ) {
        self.cascader.update(cx, |this, _| {
            this.last_bounds = Some(bounds);
        });
    }

    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        _window: &mut Window,
        _: &mut App,
    ) {
    }
}

#[cfg(test)]
mod popup_layout_tests {
    use super::*;

    #[test]
    fn popup_height_matches_the_rendered_scroll_limit() {
        assert_eq!(popup_content_height(0), px(36.0));
        assert_eq!(popup_content_height(3), px(104.0));
        assert_eq!(popup_content_height(20), px(280.0));
    }

    #[test]
    fn popup_flips_above_trigger_and_stays_inside_viewport() {
        let bounds = Bounds::new(
            gpui::point(px(320.0), px(760.0)),
            gpui::size(px(360.0), px(34.0)),
        );
        assert_eq!(
            popup_position(
                Some(bounds),
                px(360.0),
                px(104.0),
                gpui::size(px(800.0), px(900.0)),
            ),
            (px(652.0), px(320.0))
        );
    }

    #[test]
    fn popup_clamps_wide_panel_to_viewport_margin() {
        let bounds = Bounds::new(
            gpui::point(px(700.0), px(200.0)),
            gpui::size(px(360.0), px(34.0)),
        );
        let panel_width = popup_panel_width(px(900.0), px(800.0));
        assert_eq!(panel_width, px(784.0));
        assert_eq!(
            popup_position(
                Some(bounds),
                panel_width,
                px(104.0),
                gpui::size(px(800.0), px(900.0)),
            ),
            (px(238.0), px(8.0))
        );
    }

    #[test]
    fn popup_flips_inside_a_short_viewport() {
        let bounds = Bounds::new(
            gpui::point(px(700.0), px(50.0)),
            gpui::size(px(360.0), px(34.0)),
        );
        assert_eq!(
            popup_position(
                Some(bounds),
                px(784.0),
                px(280.0),
                gpui::size(px(800.0), px(200.0)),
            ),
            (px(8.0), px(8.0))
        );
    }
}
#[cfg(test)]
mod interaction_tests {
    use super::*;
    use std::{cell::Cell, cell::RefCell, rc::Rc};

    #[test]
    fn queued_portal_reads_latest_lazy_columns_and_rechecks_blocked_ancestors() {
        struct UpdatingSurface {
            cascader: Entity<Cascader>,
            renders: Rc<Cell<usize>>,
        }

        impl Render for UpdatingSurface {
            fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
                let trigger = self
                    .cascader
                    .update(cx, |this, cx| this.render(window, cx).into_any_element());
                if self.renders.get() == 0 {
                    // Update after queuing the portal, before PortalLayer consumes it.
                    self.cascader.update(cx, |this, cx| {
                        assert!(this.set_children_at_path(
                            &["parent".into()],
                            vec![CascaderOption::new("branch", "Loaded branch")
                                .child(CascaderOption::new("leaf", "Loaded leaf").leaf(true))],
                            cx,
                        ));
                        this.set_selected_path(["parent", "branch", "leaf"], cx);
                    });
                }
                self.renders.set(self.renders.get() + 1);
                div()
                    .relative()
                    .size_full()
                    .child(trigger)
                    .child(crate::PortalLayer)
            }
        }

        for blocker in ["enabled", "disabled", "loading"] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let changes = Rc::new(RefCell::new(Vec::new()));
            let callbacks = changes.clone();
            let cascader = app.new(|cx| {
                let mut this = Cascader::new(
                    vec![CascaderOption::new("parent", "Parent").loading(true)],
                    cx,
                )
                .id("queued-lazy")
                .lazy(true)
                .on_change(move |path, _, _| callbacks.borrow_mut().push(path));
                this.is_open = true;
                this.active_path = vec!["parent".into()];
                this
            });
            let renders = Rc::new(Cell::new(0));
            let root = cascader.clone();
            let count = renders.clone();
            let handle = app.add_window(move |_, _| UpdatingSurface {
                cascader: root,
                renders: count,
            });
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
            assert_eq!(
                renders.get(),
                1,
                "test must inspect the first queued portal"
            );
            static SELECTOR: std::sync::LazyLock<SharedString> = std::sync::LazyLock::new(|| {
                Cascader::popup_item_id(
                    "queued-lazy",
                    &["parent".into(), "branch".into(), "leaf".into()],
                )
            });
            let bounds = window
                .debug_bounds(SELECTOR.as_ref())
                .expect("latest lazy path was not painted");
            assert!(
                changes.borrow().is_empty(),
                "data updates emitted on_change"
            );
            window.update(|window, cx| {
                cascader.update(cx, |this, _| {
                    this.options[0].disabled = blocker == "disabled";
                    this.options[0].loading = blocker == "loading";
                });
                window.dispatch_event(
                    gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                        position: bounds.center(),
                        button: MouseButton::Left,
                        click_count: 1,
                        ..Default::default()
                    }),
                    cx,
                );
            });
            assert_eq!(changes.borrow().len(), usize::from(blocker == "enabled"));
            if blocker == "enabled" {
                assert_eq!(
                    changes.borrow()[0],
                    ["parent", "branch", "leaf"].map(SharedString::from),
                );
            }
            assert_eq!(
                cascader.read_with(&app, |this, _| this.is_open),
                blocker != "enabled",
            );
            app.quit();
        }
    }

    struct CascaderSurface {
        cascader: Entity<Cascader>,
        after_clicks: Rc<Cell<usize>>,
        unhandled_keys: Rc<Cell<usize>>,
    }

    impl Render for CascaderSurface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let clicks = self.after_clicks.clone();
            let keys = self.unhandled_keys.clone();
            let escapes = self.unhandled_keys.clone();
            div()
                .relative()
                .size_full()
                .on_key_down(move |event, _, _| {
                    if event.keystroke.key == "f6" {
                        keys.set(keys.get() + 1);
                    }
                })
                .on_action(move |_: &CascaderClose, _, _| {
                    escapes.set(escapes.get() + 1);
                })
                .child(crate::Button::new("Before").id("cascader-before"))
                .child(self.cascader.clone())
                .child(
                    crate::Button::new("After")
                        .id("cascader-after")
                        .on_click(move |_, _, _| clicks.set(clicks.get() + 1)),
                )
                .child(crate::PortalLayer)
        }
    }

    fn surface(
        app: &mut gpui::TestAppContext,
        cascader: &Entity<Cascader>,
    ) -> (gpui::VisualTestContext, Rc<Cell<usize>>, Rc<Cell<usize>>) {
        let after_clicks = Rc::new(Cell::new(0));
        let unhandled_keys = Rc::new(Cell::new(0));
        let root = cascader.clone();
        let clicks = after_clicks.clone();
        let keys = unhandled_keys.clone();
        let handle = app.add_window(move |_, _| CascaderSurface {
            cascader: root,
            after_clicks: clicks,
            unhandled_keys: keys,
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        (window, after_clicks, unhandled_keys)
    }

    fn press(window: &mut gpui::VisualTestContext, key: &str) {
        window.simulate_keystrokes(key);
        window.update(|window, cx| {
            let Ok(keystroke) = gpui::Keystroke::parse_with_os(key, cx.platform_info().os) else {
                panic!("invalid test keystroke: {key}");
            };
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
        });
    }

    fn open_with_pointer(window: &mut gpui::VisualTestContext, cascader: &Entity<Cascader>) {
        let Some(bounds) = window.update(|_, cx| cascader.read(cx).last_bounds) else {
            panic!("Cascader trigger was not painted");
        };
        let center = bounds.center();
        window.simulate_click(center, gpui::Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
    }

    fn options(disabled: bool, loading: bool) -> Vec<CascaderOption> {
        vec![
            CascaderOption::new("parent", "Parent")
                .disabled(disabled)
                .loading(loading)
                .child(CascaderOption::new("leaf", "Needle").leaf(true)),
            CascaderOption::new("other", "Other").leaf(true),
        ]
    }

    #[test]
    fn tab_reaches_cascader_and_enter_space_open_then_escape_dismisses_after_redraw() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let cascader = app.new(|cx| Cascader::new(options(false, false), cx));
        let (mut window, after, unhandled) = surface(&mut app, &cascader);
        window.update(|window, cx| window.focus_next(cx));
        window.update(|window, cx| window.draw(cx).clear(cx));
        press(&mut window, "tab");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(window.update(|window, cx| cascader.read(cx).focus_handle.is_focused(window)));

        for key in ["enter", "space"] {
            press(&mut window, key);
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert!(cascader.read_with(&app, |this, _| this.is_open), "{key}");
            press(&mut window, "escape");
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert!(!cascader.read_with(&app, |this, _| this.is_open));
        }
        press(&mut window, "ctrl-enter");
        assert!(!cascader.read_with(&app, |this, _| this.is_open));
        press(&mut window, "f6");
        assert_eq!(unhandled.get(), 1, "unhandled input must reach the parent");
        press(&mut window, "escape");
        assert_eq!(unhandled.get(), 2, "closed trigger swallowed Escape");
        press(&mut window, "tab");
        window.update(|window, cx| window.draw(cx).clear(cx));
        press(&mut window, "enter");
        assert_eq!(after.get(), 1);
        app.quit();
    }

    #[test]
    fn pointer_opened_cascader_escape_respects_dismissal_policy_after_redraw() {
        for close in [true, false] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let cascader = app.new(|cx| {
                Cascader::new(options(false, false), cx)
                    .close_on_escape(close)
                    .close_on_click_outside(close)
            });
            let (mut window, _, unhandled) = surface(&mut app, &cascader);
            open_with_pointer(&mut window, &cascader);
            assert!(cascader.read_with(&app, |this, _| this.is_open));
            press(&mut window, "escape");
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert_eq!(cascader.read_with(&app, |this, _| this.is_open), !close);
            assert_eq!(unhandled.get(), usize::from(!close));
            if close {
                open_with_pointer(&mut window, &cascader);
            }
            let outside = window.update(|window, _| {
                let size = window.viewport_size();
                gpui::point(size.width - px(8.0), size.height - px(8.0))
            });
            window.simulate_click(outside, gpui::Modifiers::none());
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert_eq!(cascader.read_with(&app, |this, _| this.is_open), !close);
            app.quit();
        }
    }

    #[test]
    fn disabled_cascader_is_skipped_and_ignores_pointer_and_retained_focus_activation() {
        for initially_disabled in [true, false] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let changes = Rc::new(Cell::new(0));
            let callbacks = changes.clone();
            let cascader = app.new(|cx| {
                Cascader::new(options(false, false), cx)
                    .disabled(initially_disabled)
                    .on_change(move |_, _, _| callbacks.set(callbacks.get() + 1))
            });
            let (mut window, after, _) = surface(&mut app, &cascader);
            window.update(|window, cx| window.focus_next(cx));
            window.update(|window, cx| window.draw(cx).clear(cx));
            press(&mut window, "tab");
            window.update(|window, cx| window.draw(cx).clear(cx));
            if !initially_disabled {
                assert!(
                    window.update(|window, cx| cascader.read(cx).focus_handle.is_focused(window))
                );
                cascader.update(&mut app, |this, cx| {
                    this.disabled = true;
                    cx.notify();
                });
                window.update(|window, cx| window.draw(cx).clear(cx));
                press(&mut window, "enter");
                press(&mut window, "space");
                assert!(!cascader.read_with(&app, |this, _| this.is_open));
                press(&mut window, "tab");
                window.update(|window, cx| window.draw(cx).clear(cx));
            }
            press(&mut window, "enter");
            assert_eq!(after.get(), 1, "disabled trigger remained in Tab order");
            open_with_pointer(&mut window, &cascader);
            assert!(!cascader.read_with(&app, |this, _| this.is_open));
            assert_eq!(changes.get(), 0);
            app.quit();
        }
    }

    #[test]
    fn search_and_selectable_paths_exclude_disabled_or_loading_ancestors() {
        for (disabled, loading) in [(true, false), (false, true)] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let cascader = app.new(|cx| {
                Cascader::new(options(disabled, loading), cx)
                    .filterable(true)
                    .search_query("needle")
            });
            let (mut window, _, _) = surface(&mut app, &cascader);
            open_with_pointer(&mut window, &cascader);
            cascader.read_with(&app, |this, _| {
                assert!(this.matching_leaf_paths().is_empty());
                assert!(!Cascader::is_selectable_path(
                    &this.options,
                    &["parent".into(), "leaf".into()],
                ));
            });
            assert!(window.debug_bounds("cascader-search-results").is_none());
            app.quit();
        }
    }

    #[test]
    fn stale_search_result_rechecks_changed_ancestors_without_notifying_selection() {
        for loading in [false, true] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let changes = Rc::new(RefCell::new(Vec::new()));
            let callbacks = changes.clone();
            let cascader = app.new(|cx| {
                Cascader::new(options(false, false), cx)
                    .id("stale-search")
                    .filterable(true)
                    .search_query("needle")
                    .on_change(move |path, _, _| callbacks.borrow_mut().push(path))
            });
            let (mut window, _, _) = surface(&mut app, &cascader);
            open_with_pointer(&mut window, &cascader);
            assert!(cascader.read_with(&app, |this, _| this.is_open));
            static SELECTOR: std::sync::LazyLock<SharedString> = std::sync::LazyLock::new(|| {
                Cascader::popup_item_id("stale-search", &["parent".into(), "leaf".into()])
            });
            let Some(bounds) = window.debug_bounds(SELECTOR.as_ref()) else {
                panic!("stale search result was not painted");
            };
            let position = bounds.center();
            // Change live data after paint, before the already-rendered listener receives input.
            window.update(|window, cx| {
                cascader.update(cx, |this, cx| {
                    if loading {
                        assert!(this.set_loading_at_path(&["parent".into()], true, cx));
                    } else {
                        this.set_options(options(true, false), cx);
                    }
                });
                window.dispatch_event(
                    gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                        position,
                        button: MouseButton::Left,
                        click_count: 1,
                        ..Default::default()
                    }),
                    cx,
                );
            });
            assert!(
                changes.borrow().is_empty(),
                "rejected path emitted on_change"
            );
            cascader.read_with(&app, |this, _| {
                assert!(this.selected_path.is_empty());
                assert!(this.active_path.is_empty());
                assert!(this.is_open);
            });
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert!(cascader.read_with(&app, |this, _| this.matching_leaf_paths().is_empty()));
            assert!(window.debug_bounds("cascader-search-results").is_none());
            app.quit();
        }
    }

    #[test]
    fn native_column_selection_preserves_enabled_and_synchronous_lazy_paths() {
        for lazy in [false, true] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let changes = Rc::new(RefCell::new(Vec::new()));
            let callbacks = changes.clone();
            let loads = Rc::new(Cell::new(0));
            let requests = loads.clone();
            let cascader = app.new(|cx| {
                Cascader::new(
                    vec![if lazy {
                        CascaderOption::new("parent", "Parent")
                    } else {
                        options(false, false).remove(0)
                    }],
                    cx,
                )
                .id("enabled-path")
                .lazy(lazy)
                .on_change(move |path, _, _| callbacks.borrow_mut().push(path))
                .on_lazy_load(move |this, path, _, cx| {
                    requests.set(requests.get() + 1);
                    assert!(this.set_children_at_path(
                        &path,
                        vec![CascaderOption::new("leaf", "Needle").leaf(true)],
                        cx,
                    ));
                })
            });
            let (mut window, _, _) = surface(&mut app, &cascader);
            open_with_pointer(&mut window, &cascader);
            static SELECTORS: std::sync::LazyLock<[SharedString; 2]> =
                std::sync::LazyLock::new(|| {
                    [
                        Cascader::popup_item_id("enabled-path", &["parent".into()]),
                        Cascader::popup_item_id("enabled-path", &["parent".into(), "leaf".into()]),
                    ]
                });
            for selector in SELECTORS.iter() {
                let Some(bounds) = window.debug_bounds(selector.as_ref()) else {
                    panic!("Cascader option was not painted: {selector}");
                };
                let position = bounds.center();
                window.simulate_click(position, gpui::Modifiers::none());
                window.update(|window, cx| window.draw(cx).clear(cx));
            }
            assert_eq!(loads.get(), usize::from(lazy));
            assert_eq!(
                *changes.borrow(),
                [vec![SharedString::from("parent"), "leaf".into()]]
            );
            cascader.read_with(&app, |this, _| {
                assert_eq!(this.display_text(), "Parent / Needle");
                assert!(!this.is_open);
            });
            cascader.update(&mut app, |this, cx| {
                this.set_options(options(true, false), cx)
            });
            assert!(cascader.read_with(&app, |this, _| this.selected_path.is_empty()));
            assert_eq!(
                changes.borrow().len(),
                1,
                "data invalidation is not a user selection"
            );
            app.quit();
        }
    }

    #[test]
    fn filterable_open_types_query_without_enter_committing_selection() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let changes = Rc::new(Cell::new(0));
        let callbacks = changes.clone();
        let cascader = app.new(|cx| {
            Cascader::new(options(false, false), cx)
                .filterable(true)
                .on_change(move |_, _, _| callbacks.set(callbacks.get() + 1))
        });
        let (mut window, _, _) = surface(&mut app, &cascader);
        open_with_pointer(&mut window, &cascader);
        window.simulate_input("Other");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            cascader.read_with(&app, |this, _| this.search_query.clone()),
            "Other"
        );
        press(&mut window, "enter");
        assert_eq!(
            cascader.read_with(&app, |this, _| this.search_query.clone()),
            "Other"
        );
        assert!(cascader.read_with(&app, |this, _| this.is_open
            && this.selected_path.is_empty()));
        assert_eq!(changes.get(), 0);
        app.quit();
    }

    #[test]
    fn seeded_builder_orders_setter_before_open_and_closed_updates_retain_editor() {
        for order in 0..3 {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let cascader = app.new(|cx| {
                let mut this = Cascader::new(options(false, false), cx);
                assert!(this.search_scroll.is_none());
                this = match order {
                    0 => this.filterable(true).search_query("needle"),
                    1 => this.search_query("needle").filterable(true),
                    _ => {
                        this.set_search_query("needle", cx);
                        this.filterable(true)
                    }
                };
                assert!(this.search_input.is_none());
                assert!(this.search_scroll.is_none());
                this
            });
            let (mut window, _, _) = surface(&mut app, &cascader);
            open_with_pointer(&mut window, &cascader);
            let input = cascader.read_with(&app, |this, _| this.search_input.clone().unwrap());
            assert!(cascader.read_with(&app, |this, _| this.search_scroll.is_some()));
            assert_eq!(
                input.read_with(&app, |input, _| input.value().clone()),
                "needle"
            );
            assert!(window.update(|window, cx| input.focus_handle(cx).is_focused(window)));
            press(&mut window, "escape");
            window.update(|window, cx| window.draw(cx).clear(cx));
            cascader.update(&mut app, |this, cx| this.set_search_query("Other", cx));
            assert_eq!(
                input.read_with(&app, |input, _| input.value().clone()),
                "Other"
            );
            open_with_pointer(&mut window, &cascader);
            assert_eq!(
                cascader.read_with(&app, |this, _| this
                    .search_input
                    .as_ref()
                    .unwrap()
                    .entity_id()),
                input.entity_id()
            );
            app.quit();
        }
    }

    #[test]
    fn composition_survives_query_rerender_and_escape_then_stale_edit_loses_to_host_setter() {
        use gpui::EntityInputHandler;
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let cascader = app.new(|cx| Cascader::new(options(false, false), cx).filterable(true));
        let (mut window, _, unhandled) = surface(&mut app, &cascader);
        open_with_pointer(&mut window, &cascader);
        let input = cascader.read_with(&app, |this, _| this.search_input.clone().unwrap());
        let selection = window.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.replace_and_mark_text_in_range(None, "候補", Some(0..1), window, cx);
                input.selected_range()
            })
        });
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            cascader.read_with(&app, |this, _| this.search_query.clone()),
            "候補"
        );
        window.update(|window, cx| {
            input.update(cx, |input, cx| {
                assert!(input.is_composing());
                assert_eq!(input.marked_text_range(window, cx), Some(0..2));
                assert_eq!(input.selected_range(), selection);
            })
        });
        press(&mut window, "enter");
        // dispatch_keystroke synthesizes committed text for unconsumed Tab;
        // dispatch raw platform events to test ownership before the native IME decides.
        for key in ["tab", "shift-tab"] {
            window.update(|window, cx| {
                assert!(input.focus_handle(cx).is_focused(window));
                let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
                let result = window.dispatch_event(
                    gpui::PlatformInput::KeyDown(gpui::KeyDownEvent {
                        keystroke: keystroke.clone(),
                        is_held: false,
                        prefer_character_input: false,
                    }),
                    cx,
                );
                assert!(result.propagate);
                assert!(!window.default_prevented());
                window.dispatch_event(
                    gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                    cx,
                );
                assert!(input.focus_handle(cx).is_focused(window));
                input.update(cx, |input, cx| {
                    assert_eq!(input.value(), "候補");
                    assert_eq!(input.marked_text_range(window, cx), Some(0..2));
                    assert_eq!(input.selected_range(), selection);
                });
            });
        }
        assert!(cascader.read_with(&app, |this, _| this.is_open));
        assert!(input.read_with(&app, |input, _| input.is_composing()));
        press(&mut window, "escape");
        assert!(cascader.read_with(&app, |this, _| this.is_open));
        assert!(input.read_with(&app, |input, _| input.is_composing()));
        assert_eq!(
            unhandled.get(),
            1,
            "IME Escape must propagate rather than be consumed"
        );
        window.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.unmark_text(window, cx);
                input.replace_text_in_range(None, "stale", window, cx);
            });
            cascader.update(cx, |this, cx| this.set_search_query("Other", cx));
        });
        assert_eq!(
            cascader.read_with(&app, |this, _| this.search_query.clone()),
            "Other"
        );
        assert_eq!(
            input.read_with(&app, |input, _| input.value().clone()),
            "Other"
        );
        press(&mut window, "escape");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(!cascader.read_with(&app, |this, _| this.is_open));
        assert!(window.update(|window, cx| cascader.focus_handle(cx).is_focused(window)));
        app.quit();
    }

    #[test]
    fn query_tab_dismisses_to_trigger_neighbors_in_both_directions() {
        for reverse in [false, true] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let cascader = app.new(|cx| Cascader::new(options(false, false), cx).filterable(true));
            let (mut window, _, _) = surface(&mut app, &cascader);
            let expected = window.update(|window, cx| {
                window.focus(&cascader.focus_handle(cx), cx);
                if reverse {
                    window.focus_prev(cx);
                } else {
                    window.focus_next(cx);
                }
                window.focused(cx).unwrap()
            });
            open_with_pointer(&mut window, &cascader);
            press(&mut window, if reverse { "shift-tab" } else { "tab" });
            assert!(window.update(|window, _| expected.is_focused(window)));
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert!(window.update(|window, _| expected.is_focused(window)));
            assert!(window.debug_bounds("cascader-search-header").is_none());
            assert!(!cascader.read_with(&app, |this, _| this.is_open));
            app.quit();
        }
    }

    #[test]
    fn query_tab_skips_retiring_editor_when_trigger_is_the_only_host_stop() {
        struct Only(Entity<Cascader>);
        impl Render for Only {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .size_full()
                    .child(self.0.clone())
                    .child(crate::PortalLayer)
            }
        }
        for key in ["tab", "shift-tab"] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let cascader = app.new(|cx| Cascader::new(options(false, false), cx).filterable(true));
            let handle = app.add_window(|_, _| Only(cascader.clone()));
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &mut app);
            open_with_pointer(&mut window, &cascader);
            press(&mut window, key);
            assert!(window.update(|window, cx| cascader.focus_handle(cx).is_focused(window)));
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert!(window.update(|window, cx| cascader.focus_handle(cx).is_focused(window)));
            assert!(window.debug_bounds("cascader-search-header").is_none());
            app.quit();
        }
    }

    #[test]
    fn no_matches_are_inert_whitespace_restores_columns_and_header_budget_clips_tiny_viewports() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let cascader = app.new(|cx| {
            Cascader::new(options(false, false), cx)
                .filterable(true)
                .search_query("absent")
        });
        let (mut window, _, _) = surface(&mut app, &cascader);
        open_with_pointer(&mut window, &cascader);
        assert!(window.debug_bounds("cascader-no-results").is_some());
        assert!(window.debug_bounds("cascader-search-results").is_none());
        cascader.update(&mut app, |this, cx| this.set_search_query("  ", cx));
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(window.debug_bounds("cascader-no-results").is_none());
        static ROOT: std::sync::LazyLock<SharedString> = std::sync::LazyLock::new(|| {
            Cascader::popup_item_id("short-search", &["parent".into()])
        });
        cascader.update(&mut app, |this, cx| {
            this.id = "short-search".into();
            cx.notify();
        });
        for height in [120.0, 40.0] {
            window.simulate_resize(gpui::size(px(400.0), px(height)));
            window.update(|window, cx| window.draw(cx).clear(cx));
            let panel = window.debug_bounds("cascader-panel").unwrap();
            let header = window.debug_bounds("cascader-search-header").unwrap();
            assert!(panel.top() >= px(0.0) && panel.bottom() <= px(height));
            assert_eq!(header.size.height, px(SEARCH_HEADER_HEIGHT));
            if height == 40.0 {
                assert!(window.debug_bounds(ROOT.as_ref()).is_none());
            } else {
                let row = window.debug_bounds(ROOT.as_ref()).unwrap();
                assert!(row.top() >= header.bottom());
            }
        }
        press(&mut window, "escape");
        assert!(!cascader.read_with(&app, |this, _| this.is_open));
        app.quit();
    }

    #[test]
    fn pointer_selection_restores_editor_focus_unless_callback_moves_it_and_outside_preserves_destination()
     {
        for move_focus in [false, true] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let destination = Rc::new(RefCell::new(None::<FocusHandle>));
            let target = destination.clone();
            let cascader = app.new(|cx| {
                Cascader::new(options(false, false), cx)
                    .id("focus-search")
                    .filterable(true)
                    .search_query("Other")
                    .on_change(move |_, window, cx| {
                        if move_focus {
                            window.focus(target.borrow().as_ref().unwrap(), cx);
                        }
                    })
            });
            let (mut window, _, _) = surface(&mut app, &cascader);
            let after = window.update(|window, cx| {
                window.focus(&cascader.focus_handle(cx), cx);
                window.focus_next(cx);
                window.focused(cx).unwrap()
            });
            *destination.borrow_mut() = Some(after.clone());
            open_with_pointer(&mut window, &cascader);
            static OTHER: std::sync::LazyLock<SharedString> = std::sync::LazyLock::new(|| {
                Cascader::popup_item_id("focus-search", &["other".into()])
            });
            let row = window.debug_bounds(OTHER.as_ref()).unwrap();
            window.simulate_click(row.center(), gpui::Modifiers::none());
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert!(!cascader.read_with(&app, |this, _| this.is_open));
            assert!(window.update(|window, cx| {
                if move_focus {
                    after.is_focused(window)
                } else {
                    cascader.focus_handle(cx).is_focused(window)
                }
            }));
            let before = window.update(|window, cx| {
                window.focus(&cascader.focus_handle(cx), cx);
                window.focus_prev(cx);
                window.focused(cx).unwrap()
            });
            open_with_pointer(&mut window, &cascader);
            // The active PortalLayer occludes underlying controls; preserve a host's
            // deliberate focus destination rather than assuming outside click-through.
            window.update(|window, cx| window.focus(&before, cx));
            let outside = window.debug_bounds("cascader-before").unwrap();
            window.simulate_click(outside.center(), gpui::Modifiers::none());
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert!(!cascader.read_with(&app, |this, _| this.is_open));
            assert!(window.update(|window, _| before.is_focused(window)));
            app.quit();
        }
    }

    #[test]
    fn latest_deferred_user_edit_wins_and_native_clear_restores_columns_without_selection() {
        use gpui::EntityInputHandler;
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let cascader = app.new(|cx| Cascader::new(options(false, false), cx).filterable(true));
        let (mut window, _, _) = surface(&mut app, &cascader);
        open_with_pointer(&mut window, &cascader);
        let input = cascader.read_with(&app, |this, _| this.search_input.clone().unwrap());
        window.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.replace_text_in_range(None, "First", window, cx);
                input.replace_text_in_range(Some(0..5), "Other", window, cx);
            })
        });
        assert_eq!(
            cascader.read_with(&app, |this, _| this.search_query.clone()),
            "Other"
        );
        press(&mut window, "ctrl-a");
        press(&mut window, "backspace");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            cascader.read_with(&app, |this, _| this.search_query.is_empty()
                && this.selected_path.is_empty()
                && this.is_open)
        );
        assert!(input.read_with(&app, |input, _| input.value().is_empty()));
        assert!(window.debug_bounds("cascader-search-results").is_none());
        assert!(window.debug_bounds("cascader-no-results").is_none());
        app.quit();
    }

    #[test]
    fn result_navigation_clamps_and_commits_only_after_an_explicit_arrow() {
        for (keys, expected) in [
            (vec!["down"], "a"),
            (vec!["up"], "c"),
            (vec!["down", "up", "up"], "a"),
            (vec!["up", "down", "down"], "c"),
            (vec!["down", "down", "up"], "a"),
        ] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let changes = Rc::new(RefCell::new(Vec::new()));
            let callbacks = changes.clone();
            let cascader = app.new(|cx| {
                Cascader::new(
                    vec![
                        CascaderOption::new("blocked", "Needle blocked")
                            .disabled(true)
                            .child(CascaderOption::new("leaf", "Needle")),
                        CascaderOption::new("a", "Needle A"),
                        CascaderOption::new("pending", "Needle pending").loading(true),
                        CascaderOption::new("b", "Needle B"),
                        CascaderOption::new("c", "Needle C"),
                    ],
                    cx,
                )
                .filterable(true)
                .search_query("Needle")
                .on_change(move |path, _, _| callbacks.borrow_mut().push(path))
            });
            let (mut window, _, _) = surface(&mut app, &cascader);
            open_with_pointer(&mut window, &cascader);
            let input = cascader.read_with(&app, |this, _| this.search_input.clone().unwrap());
            input.update(&mut app, |input, cx| input.set_selection(1..3, cx));
            press(&mut window, "enter");
            assert!(changes.borrow().is_empty());
            for key in keys {
                press(&mut window, key);
            }
            assert!(changes.borrow().is_empty(), "navigation selected a value");
            assert_eq!(input.read_with(&app, |input, _| input.value().clone()), "Needle");
            assert_eq!(input.read_with(&app, |input, _| input.selected_range()), 1..3);
            assert!(window.update(|window, cx| input.focus_handle(cx).is_focused(window)));
            press(&mut window, "enter");
            assert_eq!(*changes.borrow(), vec![vec![SharedString::from(expected)]]);
            assert!(!cascader.read_with(&app, |this, _| this.is_open));
            assert!(window.update(|window, cx| cascader.focus_handle(cx).is_focused(window)));
            app.quit();
        }
    }

    #[test]
    fn result_navigation_does_not_retarget_after_query_or_option_changes() {
        for change in ["query", "typed", "blank", "no-match", "options", "children", "loading", "blocked", "reopen", "clear"] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let changes = Rc::new(Cell::new(0));
            let callbacks = changes.clone();
            let cascader = app.new(|cx| {
                Cascader::new(options(false, false), cx)
                    .filterable(true)
                    .search_query("Needle")
                    .on_change(move |_, _, _| callbacks.set(callbacks.get() + 1))
            });
            let (mut window, _, _) = surface(&mut app, &cascader);
            open_with_pointer(&mut window, &cascader);
            press(&mut window, "down");
            match change {
                "query" => cascader.update(&mut app, |this, cx| this.set_search_query("Other", cx)),
                "typed" => {
                    press(&mut window, "ctrl-a");
                    window.simulate_input("Other");
                }
                "blank" => cascader.update(&mut app, |this, cx| this.set_search_query("   ", cx)),
                "no-match" => cascader.update(&mut app, |this, cx| this.set_search_query("missing", cx)),
                "options" => cascader.update(&mut app, |this, cx| {
                    this.set_options(vec![CascaderOption::new("replacement", "Needle")], cx)
                }),
                "children" => cascader.update(&mut app, |this, cx| {
                    assert!(this.set_children_at_path(
                        &["parent".into()],
                        vec![CascaderOption::new("replacement", "Needle")],
                        cx,
                    ));
                }),
                "loading" => cascader.update(&mut app, |this, cx| {
                    assert!(this.set_loading_at_path(&["parent".into()], true, cx));
                }),
                "blocked" => cascader.update(&mut app, |this, cx| {
                    this.options[0].disabled = true;
                    cx.notify();
                }),
                "reopen" => {
                    press(&mut window, "escape");
                    window.update(|window, cx| window.draw(cx).clear(cx));
                    open_with_pointer(&mut window, &cascader);
                }
                "clear" => {
                    press(&mut window, "ctrl-a");
                    press(&mut window, "backspace");
                }
                _ => unreachable!(),
            }
            window.update(|window, cx| window.draw(cx).clear(cx));
            press(&mut window, "enter");
            assert_eq!(changes.get(), 0, "stale highlight committed after {change}");
            assert!(cascader.read_with(&app, |this, _| this.is_open && this.selected_path.is_empty()));
            app.quit();
        }
    }

    #[test]
    fn result_navigation_keeps_modified_keys_and_marked_text_owned_by_input() {
        use gpui::EntityInputHandler;
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let changes = Rc::new(Cell::new(0));
        let callbacks = changes.clone();
        let cascader = app.new(|cx| {
            Cascader::new(options(false, false), cx)
                .filterable(true)
                .search_query("Needle")
                .on_change(move |_, _, _| callbacks.set(callbacks.get() + 1))
        });
        let (mut window, _, _) = surface(&mut app, &cascader);
        open_with_pointer(&mut window, &cascader);
        let input = cascader.read_with(&app, |this, _| this.search_input.clone().unwrap());
        for key in ["shift-down", "ctrl-up", "enter"] {
            press(&mut window, key);
        }
        assert_eq!(changes.get(), 0);
        for key in ["shift-enter", "ctrl-enter"] {
            cascader.update(&mut app, |this, cx| this.set_search_query("Needle", cx));
            press(&mut window, "down");
            assert!(cascader.read_with(&app, |this, _| this.highlighted_result.is_some()));
            press(&mut window, key);
            assert_eq!(changes.get(), 0, "modified Enter selected a result");
            assert!(cascader.read_with(&app, |this, _| this.is_open));
            assert!(window.update(|window, cx| input.focus_handle(cx).is_focused(window)));
        }
        // Unbound modified Enter may edit text through GPUI's native Input fallback.
        cascader.update(&mut app, |this, cx| this.set_search_query("Needle", cx));
        press(&mut window, "down");
        let highlight = cascader.read_with(&app, |this, _| this.highlighted_result.clone());
        assert!(highlight.is_some());
        input.update(&mut app, |input, cx| input.set_selection(1..3, cx));
        for (key, expected) in [("shift-down", 1..3), ("shift-up", 1..3), ("shift-left", 1..2)] {
            press(&mut window, key);
            assert_eq!(input.read_with(&app, |input, _| input.selected_range()), expected);
            assert_eq!(cascader.read_with(&app, |this, _| this.highlighted_result.clone()), highlight);
            assert_eq!(changes.get(), 0, "modified arrow selected a result");
        }
        window.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.replace_and_mark_text_in_range(Some(0..6), "Needle", Some(0..6), window, cx);
            });
        });
        for key in ["down", "up", "enter"] {
            press(&mut window, key);
        }
        assert_eq!(changes.get(), 0, "Cascader selected while marked text was active");
        assert!(input.read_with(&app, |input, _| input.is_composing()));
        assert!(cascader.read_with(&app, |this, _| this.is_open && this.selected_path.is_empty()));
        assert!(window.update(|window, cx| input.focus_handle(cx).is_focused(window)));
        app.quit();
    }

    #[test]
    fn result_navigation_scrolls_the_highlight_into_the_results_viewport() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let cascader = app.new(|cx| {
            Cascader::new(
                (0..30).map(|index| CascaderOption::new(format!("{index}"), "Needle")).collect(),
                cx,
            )
            .id("result-scroll")
            .filterable(true)
            .search_query("Needle")
        });
        let (mut window, _, _) = surface(&mut app, &cascader);
        open_with_pointer(&mut window, &cascader);
        press(&mut window, "up");
        window.update(|window, cx| window.draw(cx).clear(cx));
        static LAST: std::sync::LazyLock<SharedString> = std::sync::LazyLock::new(|| {
            Cascader::popup_item_id("result-scroll", &["29".into()])
        });
        let row = window.debug_bounds(LAST.as_ref()).expect("last result was not laid out");
        let list = window.debug_bounds("cascader-search-results").unwrap();
        assert!(row.top() >= list.top() && row.bottom() <= list.bottom(), "highlight stayed outside results viewport: {row:?} vs {list:?}");
        app.quit();
    }

    #[test]
    fn result_navigation_reads_editor_changes_before_the_deferred_query_notification() {
        use gpui::EntityInputHandler;
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let changes = Rc::new(RefCell::new(Vec::new()));
        let callbacks = changes.clone();
        let cascader = app.new(|cx| {
            Cascader::new(options(false, false), cx)
                .filterable(true)
                .search_query("Needle")
                .on_change(move |path, _, _| callbacks.borrow_mut().push(path))
        });
        let (mut window, _, _) = surface(&mut app, &cascader);
        open_with_pointer(&mut window, &cascader);
        let input = cascader.read_with(&app, |this, _| this.search_input.clone().unwrap());
        window.update(|window, cx| {
            input.update(cx, |input, cx| {
                input.replace_text_in_range(Some(0..6), "Other", window, cx);
            });
            assert_eq!(cascader.read(cx).search_query, "Needle");
            let keystroke = gpui::Keystroke::parse_with_os("down", cx.platform_info().os).unwrap();
            window.dispatch_event(
                gpui::PlatformInput::KeyDown(gpui::KeyDownEvent {
                    keystroke: keystroke.clone(),
                    is_held: false,
                    prefer_character_input: false,
                }),
                cx,
            );
            window.dispatch_event(gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }), cx);
        });
        // The deferred same-query notification must retain the new highlight.
        press(&mut window, "enter");
        assert_eq!(*changes.borrow(), vec![vec![SharedString::from("other")]]);
        assert_eq!(cascader.read_with(&app, |this, _| this.search_query.clone()), "Other");
        app.quit();
    }

    #[test]
    fn result_navigation_preserves_callback_focus_and_rejects_disabled_retained_input() {
        for disabled in [false, true] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let changes = Rc::new(Cell::new(0));
            let callbacks = changes.clone();
            let destination = Rc::new(RefCell::new(None::<FocusHandle>));
            let target = destination.clone();
            let cascader = app.new(|cx| {
                Cascader::new(options(false, false), cx)
                    .filterable(true)
                    .search_query("Needle")
                    .on_change(move |_, window, cx| {
                        callbacks.set(callbacks.get() + 1);
                        window.focus(target.borrow().as_ref().unwrap(), cx);
                    })
            });
            let (mut window, _, _) = surface(&mut app, &cascader);
            let after = window.update(|window, cx| {
                window.focus(&cascader.focus_handle(cx), cx);
                window.focus_next(cx);
                window.focused(cx).unwrap()
            });
            *destination.borrow_mut() = Some(after.clone());
            open_with_pointer(&mut window, &cascader);
            press(&mut window, "down");
            cascader.update(&mut app, |this, _| this.disabled = disabled);
            press(&mut window, "enter");
            assert_eq!(changes.get(), usize::from(!disabled));
            assert_eq!(cascader.read_with(&app, |this, _| this.is_open), disabled);
            if !disabled {
                assert!(window.update(|window, _| after.is_focused(window)));
            }
            app.quit();
        }
    }
}
