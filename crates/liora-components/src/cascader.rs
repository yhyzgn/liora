//! Cascader module.
//!
//! This public module implements the Liora multi-level cascading selection popup component. It keeps the reusable
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
/// Public builder and render state for the Liora cascader option component.
pub struct CascaderOption {
    /// Current value represented by this option or component state.
    pub value: SharedString,
    /// Human-readable label shown in the component UI.
    pub label: SharedString,
    /// Nested child items rendered beneath this item.
    pub children: Vec<CascaderOption>,
    /// Whether user interaction is disabled for this item.
    pub disabled: bool,
    /// Loading for this data model.
    pub loading: bool,
    /// Leaf for this data model.
    pub leaf: bool,
}

/// Public builder and render state for the Liora cascader component.
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
    /// Creates a new value with the required baseline configuration.
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

    /// Configures the child option.
    pub fn child(mut self, child: CascaderOption) -> Self {
        self.children.push(child);
        self
    }

    /// Configures the children option.
    pub fn children(mut self, children: impl IntoIterator<Item = CascaderOption>) -> Self {
        self.children.extend(children);
        self
    }

    /// Configures the disabled option.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Configures the loading option.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Configures the leaf option.
    pub fn leaf(mut self, leaf: bool) -> Self {
        self.leaf = leaf;
        self
    }

    fn find_child(&self, value: &SharedString) -> Option<&CascaderOption> {
        self.children.iter().find(|child| &child.value == value)
    }
}

impl Cascader {
    /// Creates a new value with the required baseline configuration.
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

    /// Returns the stable tray command identifier used for menu event routing.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Configures the popup item id option.
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

    /// Configures the placeholder option.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Creates a tray menu separator item specification.
    pub fn separator(mut self, separator: impl Into<SharedString>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Configures the disabled option.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Configures the clearable option.
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Configures the filterable option.
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    /// Configures the search query option.
    pub fn search_query(mut self, query: impl Into<SharedString>) -> Self {
        self.search_query = query.into();
        self
    }

    /// Returns the width token used for component sizing.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Applies the predefined width md sizing preset.
    pub fn width_md(self) -> Self {
        self.width(px(360.0))
    }

    /// Configures the lazy option.
    pub fn lazy(mut self, lazy: bool) -> Self {
        self.lazy = lazy;
        self
    }

    /// Configures the close on escape option.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Configures the close on click outside option.
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Configures the register key bindings option.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", CascaderClose, None)]);
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

    /// Returns whether selectable path is currently true for this value.
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

    /// Configures the should lazy load option option.
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

        for value in rest {
            option = option.find_child(value)?;
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
            if option.children.is_empty() {
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
            window.focus(&self.focus_handle);
        }
        cx.notify();
    }

    fn choose_path(
        &mut self,
        path: Vec<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

            push_portal(
                move |_window, _cx| {
                    let (top, left, trigger_width) = if let Some(bounds) = trigger_bounds {
                        (bounds.bottom() + px(4.0), bounds.left(), bounds.size.width)
                    } else {
                        (px(100.0), px(100.0), px(240.0))
                    };
                    let panel_width =
                        px((columns.len().max(1) as f32 * 180.0).max(f32::from(trigger_width)));
                    let theme = theme_portal.clone();
                    let entity = entity.clone();

                    let close_entity = entity.clone();
                    let panel = div()
                        .id(element_id(format!("{}-panel", cascader_id)))
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(panel_width)
                        .max_h(px(280.0))
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

fn render_columns(
    cascader_id: SharedString,
    columns: Vec<Vec<CascaderOption>>,
    active_path: Vec<SharedString>,
    selected_path: Vec<SharedString>,
    entity: Entity<Cascader>,
    theme: liora_theme::Theme,
) -> AnyElement {
    div()
        .flex()
        .flex_row()
        .children(columns.into_iter().enumerate().map(|(depth, column)| {
            let active_value = active_path.get(depth).cloned();
            let selected_value = selected_path.get(depth).cloned();
            let path_prefix = active_path.iter().take(depth).cloned().collect::<Vec<_>>();
            let column_id = format!("{}-column-{depth}", cascader_id);
            let item_id_prefix = cascader_id.clone();
            div()
                .id(element_id(column_id))
                .w(px(180.0))
                .max_h(px(280.0))
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

                        div()
                            .id(item_id)
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
) -> AnyElement {
    div()
        .id("cascader-search-results")
        .flex()
        .flex_col()
        .max_h(px(280.0))
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

            div()
                .id(item_id)
                .h(px(34.0))
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
        path.push(option.value.clone());
        labels.push(option.label.clone());

        if option.children.is_empty() {
            let haystack = labels
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" /")
                .to_lowercase();
            if !option.disabled && !option.loading && haystack.contains(query) {
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
mod cursor_regression_tests {
    #[test]
    fn cascader_disabled_states_use_not_allowed_cursor() {
        let source = include_str!("cascader.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(
            source.contains(".when(self.disabled, |s| s.cursor_not_allowed())"),
            "disabled cascader trigger should show a not-allowed cursor"
        );
        assert!(
            source.contains(".when(disabled, |s| s.cursor_not_allowed())"),
            "disabled cascader options should show a not-allowed cursor"
        );
    }
}
