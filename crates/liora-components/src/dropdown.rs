//! Dropdown module.
//!
//! This public module implements the Liora dropdown menu component backed by Liora popover behavior. It keeps the reusable
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

use crate::Popover;
use crate::gpui_compat::element_id;
use gpui::{
    AnyElement, App, Entity, FocusHandle, IntoElement, MouseButton, RenderOnce, Role, ScrollHandle,
    SharedString, ViewElement as Component, Window, div, prelude::*, px,
};
use liora_core::{Config, Placement, clear_popover, stable_unique_id};
use std::sync::Arc;

/// Data model used by dropdown item rendering.
pub struct DropdownItem {
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Callback invoked when the component is activated by pointer or keyboard input.
    pub on_click: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

enum DropdownEntry {
    Enabled(DropdownItem),
    Disabled(SharedString),
}

impl DropdownEntry {
    fn label(&self) -> &SharedString {
        match self {
            Self::Enabled(item) => &item.label,
            Self::Disabled(label) => label,
        }
    }
}

/// Fluent native GPUI component for rendering Liora dropdown.
pub struct Dropdown {
    trigger: AnyElement,
    items: Vec<DropdownEntry>,
    placement: Placement,
    close_on_click_outside: bool,
    close_on_escape: bool,
    id: Option<SharedString>,
}

impl Dropdown {
    /// Creates `Dropdown` initialized from the supplied trigger.
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            items: vec![],
            placement: Placement::BottomStart,
            close_on_click_outside: true,
            close_on_escape: true,
            id: None,
        }
    }

    /// Performs the item operation used by this component.
    pub fn item(
        mut self,
        label: impl Into<SharedString>,
        on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(DropdownEntry::Enabled(DropdownItem {
            label: label.into(),
            on_click: Arc::new(on_click),
        }));
        self
    }

    /// Adds a visible, non-executable menu item. It is skipped by keyboard navigation.
    pub fn disabled_item(mut self, label: impl Into<SharedString>) -> Self {
        self.items.push(DropdownEntry::Disabled(label.into()));
        self
    }

    /// Selects the popup, label, or overlay placement.
    pub fn placement(mut self, p: Placement) -> Self {
        self.placement = p;
        self
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
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
}

impl RenderOnce for Dropdown {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let items = self.items;
        let close_on_escape = self.close_on_escape;
        let dropdown_id = self.id.clone().unwrap_or_else(|| {
            stable_unique_id(format!("dropdown:{}", items.len()), "dropdown", _window, cx)
        });

        let model =
            _window.use_keyed_state(element_id(format!("{dropdown_id}-items")), cx, |_, _| {
                Vec::<DropdownEntry>::new()
            });
        model.update(cx, |current, _| *current = items);
        let close_on_click_outside = self.close_on_click_outside;
        Popover::new(self.trigger)
            .id(dropdown_id.clone())
            .placement(self.placement)
            .offset(px(4.0))
            .flush_content()
            .close_on_click_outside(close_on_click_outside)
            .close_on_escape(close_on_escape)
            .content(move |window, cx| {
                let theme = theme.clone();
                let focus = menu_focus(
                    &dropdown_id,
                    model.read(cx).len(),
                    window,
                    cx,
                    |index, cx| matches!(model.read(cx)[index], DropdownEntry::Enabled(_)),
                );
                let root_focus = focus.read(cx).root.clone();
                let scroll = focus.read(cx).scroll.clone();
                let rows = &focus.read(cx).rows;
                let navigation = focus.clone();
                let navigation_id = dropdown_id.clone();
                let enabled = &focus.read(cx).enabled;
                div()
                    .id(element_id(format!("{dropdown_id}-menu")))
                    .debug_selector(|| "dropdown-menu-viewport".to_string())
                    .role(Role::Menu)
                    .track_focus(&root_focus)
                    .cursor_default()
                    .occlude()
                    .flex()
                    .flex_col()
                    .py_1()
                    .min_w(px(168.0))
                    .max_h(px(200.0))
                    .overflow_y_scroll()
                    .track_scroll(&scroll)
                    .on_key_down(move |event, window, cx| {
                        navigate_menu(&navigation_id, &navigation, event, window, cx);
                    })
                    .children(model.read(cx).iter().enumerate().map(|(i, item)| {
                        let label = item.label().clone();
                        let disabled = !enabled[i];
                        let dropdown_id = dropdown_id.clone();
                        let model = model.clone();
                        let item_id = format!("{dropdown_id}-item-{i}");
                        let focus_color = theme.primary.base;
                        div()
                            .id(element_id(item_id.clone()))
                            .debug_selector(|| item_id.clone())
                            .accessibility_id(item_id)
                            .role(Role::MenuItem)
                            .aria_label(label.clone())
                            .a11y_synthetic_children(move |builder| {
                                if disabled {
                                    builder.parent_node().set_disabled();
                                }
                            })
                            .flex()
                            .items_center()
                            .min_h(px(34.0))
                            .flex_shrink_0()
                            .px_3()
                            .py_2()
                            .text_size(px(theme.font_size.md))
                            .text_color(if disabled {
                                theme.neutral.text_disabled
                            } else {
                                theme.neutral.text_1
                            })
                            .border_1()
                            .border_color(gpui::transparent_black())
                            .when(disabled, |s| {
                                s.cursor_not_allowed()
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                            })
                            .when(!disabled, |s| {
                                s.track_focus(&rows[i])
                                    .cursor_pointer()
                                    .focus_visible(move |s| s.border_color(focus_color))
                                    .hover(|s| {
                                        s.bg(theme.neutral.hover).text_color(theme.primary.base)
                                    })
                                    .on_click(move |_, window, cx| {
                                        let callback = match model.read(cx).get(i) {
                                            Some(DropdownEntry::Enabled(item)) => {
                                                Some(item.on_click.clone())
                                            }
                                            _ => None,
                                        };
                                        if let Some(callback) = callback {
                                            clear_popover(&dropdown_id, cx);
                                            callback(window, cx);
                                        }
                                        cx.stop_propagation();
                                    })
                            })
                            .child(div().text_sm().child(label))
                    }))
            })
    }
}

impl IntoElement for Dropdown {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

// Shared only by the two dropdown bodies; popup lifetime and dismissal stay in Popover.
pub(super) struct MenuFocus {
    pub(super) root: FocusHandle,
    pub(super) rows: Vec<FocusHandle>,
    pub(super) scroll: ScrollHandle,
    initialized: bool,
    enabled: Vec<bool>,
}

pub(super) fn menu_focus(
    id: &SharedString,
    count: usize,
    window: &mut Window,
    cx: &mut App,
    enabled: impl Fn(usize, &App) -> bool,
) -> Entity<MenuFocus> {
    let focus = window.use_keyed_state(element_id(format!("{id}-menu-focus")), cx, |_, cx| {
        MenuFocus {
            root: cx.focus_handle(),
            rows: Vec::new(),
            scroll: ScrollHandle::new(),
            enabled: Vec::new(),
            initialized: false,
        }
    });
    focus.update(cx, |state, cx| {
        let invalid =
            state.rows.iter().enumerate().any(|(index, row)| {
                (index >= count || !enabled(index, cx)) && row.is_focused(window)
            });
        state.rows.resize_with(count, || cx.focus_handle());
        state.enabled.resize(count, false);
        for (index, available) in state.enabled.iter_mut().enumerate() {
            *available = enabled(index, cx);
        }
        if !state.initialized || invalid {
            let pending = focus.downgrade();
            let id = id.clone();
            let view = cx
                .global::<liora_core::ActivePopover>()
                .0
                .iter()
                .find(|entry| entry.id == id)
                .map(|entry| entry.view.entity_id());
            let previous = window.focused(cx);
            // Portal content renders after the trigger's a11y prepaint; focus between frames.
            window.on_next_frame(move |window, cx| {
                let current = cx
                    .global::<liora_core::ActivePopover>()
                    .0
                    .iter()
                    .find(|entry| entry.id == id)
                    .map(|entry| entry.view.entity_id());
                if view.is_none() || current != view || window.focused(cx) != previous {
                    return;
                }
                if let Some(pending) = pending.upgrade() {
                    let state = pending.read(cx);
                    let index = state.enabled.iter().position(|enabled| *enabled);
                    if let Some(index) = index {
                        state.scroll.scroll_to_item(index);
                    }
                    let target = index
                        .map(|index| state.rows[index].clone())
                        .unwrap_or_else(|| state.root.clone());
                    window.focus(&target, cx);
                }
            });
            state.initialized = true;
        }
    });
    focus
}

pub(super) fn navigate_menu(
    id: &SharedString,
    focus: &Entity<MenuFocus>,
    event: &gpui::KeyDownEvent,
    window: &mut Window,
    cx: &mut App,
) {
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
        clear_popover(id, cx);
        window.prevent_default();
        cx.stop_propagation();
        return;
    }
    if modifiers.modified() {
        return;
    }
    let state = focus.read(cx);
    let current = state.rows.iter().position(|row| row.is_focused(window));
    let enabled = &state.enabled;
    let target = match event.keystroke.key.as_str() {
        "home" => enabled.iter().position(|enabled| *enabled),
        "end" => enabled.iter().rposition(|enabled| *enabled),
        "down" | "up" => {
            let forward = event.keystroke.key == "down";
            (1..=enabled.len())
                .map(|offset| {
                    let current = current.unwrap_or(if forward { enabled.len() - 1 } else { 0 });
                    if forward {
                        (current + offset) % enabled.len()
                    } else {
                        (current + enabled.len() - offset) % enabled.len()
                    }
                })
                .find(|index| enabled[*index])
        }
        _ => return,
    };
    if let Some(index) = target {
        state.scroll.scroll_to_item(index);
        let target = state.rows[index].clone();
        window.focus(&target, cx);
        window.refresh();
    }
    window.prevent_default();
    cx.stop_propagation();
}

#[cfg(test)]
#[path = "dropdown/interaction_tests.rs"]
mod interaction_tests;

#[cfg(test)]
#[path = "dropdown/disabled_item_tests.rs"]
mod disabled_item_tests;
