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

use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{
    AnyElement, App, Bounds, Context, Element, ElementId, Entity, FocusHandle, Focusable,
    IntoElement, KeyBinding, MouseButton, Pixels, Render, SharedString, Window, actions, div,
    prelude::*, px,
};
use liora_core::{Config, push_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

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

    /// Performs the popup item id operation used by this component.
    pub fn popup_item_id(prefix: impl AsRef<str>, path: &[SharedString]) -> SharedString {
        let mut id = prefix.as_ref().to_string();
        id.push_str("-item");
        for value in path {
            id.push('-');
            id.push_str(&sanitize_id_segment(value.as_ref()));
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
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.close_on_escape && self.is_open {
            self.is_open = false;
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
        self.search_query = query.into();
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

    fn columns_for_active_path(&self) -> Vec<Vec<CascaderOption>> {
        let mut columns = vec![self.options.clone()];
        let mut siblings = self.options.as_slice();

        for value in &self.active_path {
            let Some(option) = siblings.iter().find(|option| &option.value == value) else {
                break;
            };
            if option.disabled || option.loading || option.children.is_empty() {
                break;
            }
            columns.push(option.children.clone());
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

    fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.is_open = !self.is_open;
        if self.is_open {
            self.active_path = self.selected_path.clone();
            window.focus(&self.focus_handle, cx);
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
            let columns = self.columns_for_active_path();
            let active_path = self.active_path.clone();
            let selected_path = self.selected_path.clone();
            let filterable = self.filterable;
            let matches = self.matching_leaf_paths();
            let close_on_click_outside = self.close_on_click_outside;

            let trigger_width = trigger_bounds.map_or(px(240.0), |bounds| bounds.size.width);
            let popup_height = if filterable && !matches.is_empty() {
                popup_content_height(matches.len())
            } else {
                popup_content_height(columns.iter().map(Vec::len).max().unwrap_or_default())
            };
            let requested_panel_width =
                px((columns.len().max(1) as f32 * 180.0).max(f32::from(trigger_width)));

            push_portal(
                move |window, _cx| {
                    let viewport = window.viewport_size();
                    let panel_width = popup_panel_width(requested_panel_width, viewport.width);
                    let (top, left) =
                        popup_position(trigger_bounds, panel_width, popup_height, viewport);
                    let panel_height =
                        popup_height.min((viewport.height - top - px(8.0)).max(px(1.0)));
                    let theme = theme_portal.clone();
                    let entity = entity.clone();

                    let close_entity = entity.clone();
                    let panel = div()
                        .id(element_id(format!("{}-panel", cascader_id)))
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(panel_width)
                        .max_h(panel_height)
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
                        .when(filterable && !matches.is_empty(), |panel| {
                            panel.child(render_match_list(
                                cascader_id.clone(),
                                matches.clone(),
                                entity.clone(),
                                theme.clone(),
                                (panel_height - px(2.0)).max(px(1.0)),
                            ))
                        })
                        .when(!filterable || matches.is_empty(), |panel| {
                            panel.child(render_columns(
                                cascader_id.clone(),
                                columns.clone(),
                                active_path.clone(),
                                selected_path.clone(),
                                entity.clone(),
                                theme.clone(),
                                (panel_height - px(2.0)).max(px(1.0)),
                            ))
                        });

                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .bg(gpui::transparent_black())
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
    columns: Vec<Vec<CascaderOption>>,
    active_path: Vec<SharedString>,
    selected_path: Vec<SharedString>,
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
            let active_value = active_path.get(depth).cloned();
            let selected_value = selected_path.get(depth).cloned();
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
                .children(column.into_iter().map({
                    let entity = entity.clone();
                    let theme = theme.clone();
                    let item_id_prefix = item_id_prefix.clone();
                    move |option| {
                        let mut path = path_prefix.clone();
                        path.push(option.value.clone());
                        let is_active = Some(option.value.clone()) == active_value;
                        let is_selected = Some(option.value.clone()) == selected_value;
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
        .children(matches.into_iter().map(move |(path, labels)| {
            let entity = entity.clone();
            let theme = theme.clone();
            let text = labels
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" / ");
            let item_id = Cascader::popup_item_id(cascader_id.as_ref(), &path);

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

fn sanitize_id_segment(segment: &str) -> String {
    segment
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn collect_leaf_matches(
    options: &[CascaderOption],
    query: &str,
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

        if option.children.is_empty() {
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
            collect_leaf_matches(&option.children, query, path, labels, matches);
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
            let Some(bounds) = window.debug_bounds("stale-search-item-parent-leaf") else {
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
            for selector in ["enabled-path-item-parent", "enabled-path-item-parent-leaf"] {
                let Some(bounds) = window.debug_bounds(selector) else {
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
}
