//! Dropdown module.
//!
//! This public module implements the Liora dropdown menu component backed by Liora popover behavior. It keeps the reusable
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

use crate::Popover;
use crate::gpui_compat::element_id;
use gpui::{
    AnyElement, App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
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

/// Fluent native GPUI component for rendering Liora dropdown.
pub struct Dropdown {
    trigger: AnyElement,
    items: Vec<DropdownItem>,
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
        self.items.push(DropdownItem {
            label: label.into(),
            on_click: Arc::new(on_click),
        });
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

        let close_on_click_outside = self.close_on_click_outside;
        Popover::new(self.trigger)
            .id(dropdown_id.clone())
            .placement(self.placement)
            .offset(px(4.0))
            .close_on_click_outside(close_on_click_outside)
            .close_on_escape(close_on_escape)
            .content(move |_window, _cx| {
                let theme = theme.clone();
                div()
                    .id(element_id(format!("{}-menu", dropdown_id)))
                    .cursor_default()
                    .occlude()
                    .flex()
                    .flex_col()
                    .py_1()
                    .min_w(px(168.0))
                    .max_h(px(200.0))
                    .children(items.iter().enumerate().map(|(i, item)| {
                        let on_click = item.on_click.clone();
                        let label = item.label.clone();
                        let dropdown_id = dropdown_id.clone();
                        let item_id = format!("{}-item-{}", dropdown_id, i);

                        div()
                            .id(element_id(item_id))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .min_h(px(34.0))
                            .px_3()
                            .py_2()
                            .text_size(px(theme.font_size.md))
                            .text_color(theme.neutral.text_1)
                            .hover(|s| {
                                s.cursor_pointer()
                                    .bg(theme.neutral.hover)
                                    .text_color(theme.primary.base)
                            })
                            .on_click(move |_, window, cx| {
                                on_click(window, cx);
                                clear_popover(&dropdown_id, cx);
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

#[cfg(test)]
mod tests {
    #[test]
    fn dropdown_inherits_popover_motion_shell() {
        let source = include_str!("dropdown.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("Popover::new(self.trigger)"));
        assert!(source.contains(".content(move |_window, _cx|"));
    }
}
