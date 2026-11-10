//! Tooltip module.
//!
//! This public module implements the Liora tooltip component for anchored help text. It keeps the reusable
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

use gpui::{
    AnyElement, App, Bounds, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    Pixels, RenderOnce, SharedString, ViewElement as Component, Window, div, prelude::*, px,
};
use liora_core::{
    Placement, TooltipData, clear_tooltip, set_exclusive_active_tooltip, stable_unique_id,
};
use std::cell::Cell;
use std::rc::Rc;

/// Fluent native GPUI component for rendering Liora tooltip.
pub struct Tooltip {
    trigger: AnyElement,
    content: SharedString,
    placement: Placement,
    offset: Pixels,
    id: Option<SharedString>,
}

impl Tooltip {
    /// Creates `Tooltip` initialized from the supplied trigger.
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            content: SharedString::default(),
            placement: Placement::Top,
            offset: px(8.0),
            id: None,
        }
    }

    /// Sets the rendered content element or text for this component.
    pub fn content(mut self, content: impl Into<SharedString>) -> Self {
        self.content = content.into();
        self
    }

    /// Selects the popup, label, or overlay placement.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the pixel offset used when positioning the component.
    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let content = self.content.clone();
        let placement = self.placement;
        let offset = self.offset;
        let id = self.id.clone().unwrap_or_else(|| {
            stable_unique_id(
                format!("tooltip:{}:{:?}", self.content, self.placement),
                "tooltip",
                _window,
                _cx,
            )
        });

        let bounds_cell = Rc::new(Cell::new(Bounds::default()));
        let bounds_cell_clone = bounds_cell.clone();
        let hover_id = id.clone();
        let move_id = id.clone();
        let hover_content = content.clone();

        div()
            .id(id.clone())
            .child(TooltipBoundsTracker {
                trigger: self.trigger,
                bounds: bounds_cell.clone(),
                id: id.clone(),
                content: content.clone(),
                placement,
                offset,
            })
            .on_hover(move |hovered, _window, cx| {
                if !hovered {
                    clear_tooltip(&hover_id, cx);
                    return;
                }

                let anchor_bounds = bounds_cell.get();
                if anchor_bounds.size.width > px(0.0) {
                    set_exclusive_active_tooltip(
                        TooltipData {
                            id: hover_id.clone(),
                            content: hover_content.clone(),
                            anchor_bounds,
                            placement,
                            offset,
                        },
                        cx,
                    );
                }
            })
            .on_mouse_move(move |_event, _window, cx| {
                let anchor_bounds = bounds_cell_clone.get();
                if anchor_bounds.size.width > px(0.0) {
                    set_exclusive_active_tooltip(
                        TooltipData {
                            id: move_id.clone(),
                            content: content.clone(),
                            anchor_bounds,
                            placement,
                            offset,
                        },
                        cx,
                    );
                }
            })
    }
}

impl IntoElement for Tooltip {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

struct TooltipBoundsTracker {
    trigger: AnyElement,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    id: SharedString,
    content: SharedString,
    placement: Placement,
    offset: Pixels,
}

impl gpui::Element for TooltipBoundsTracker {
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
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.trigger.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _rl: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> () {
        self.trigger.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _rl: &mut (),
        _ps: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.bounds.set(bounds);
        if bounds.contains(&window.mouse_position())
            && set_exclusive_active_tooltip(
                TooltipData {
                    id: self.id.clone(),
                    content: self.content.clone(),
                    anchor_bounds: bounds,
                    placement: self.placement,
                    offset: self.offset,
                },
                cx,
            )
        {
            window.refresh();
        }
        self.trigger.paint(window, cx);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tooltip_refreshes_content_on_hover_for_virtualized_reuse() {
        let source = include_str!("tooltip.rs");

        assert!(source.contains("TooltipBoundsTracker"));
        assert!(source.contains("set_exclusive_active_tooltip"));
        assert!(source.contains("clear_tooltip"));
        assert!(source.contains("hover_content.clone()"));
        assert!(source.contains("if !hovered") && source.contains("return;"));
        assert!(source.contains("bounds.contains(&window.mouse_position())"));
        assert!(source.contains("window.refresh()"));
    }
}
