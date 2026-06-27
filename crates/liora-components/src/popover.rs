//! Popover module.
//!
//! This public module implements the Liora popover positioning and popup component. It keeps the reusable
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

use crate::{gpui_compat::element_id, motion::pop_in};
use gpui::{
    AnyElement, App, Bounds, Component, Context, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, KeyBinding, LayoutId, MouseButton, Pixels, Render, RenderOnce, SharedString,
    Window, actions, div, point, prelude::*, px,
};
use liora_core::{
    Config, Placement, clear_popover, is_popover_active, set_active_popover, stable_unique_id,
};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

actions!(
    popover,
    [
        #[doc = "Keyboard action that closes the active popover."]
        PopoverClose
    ]
);

/// Fluent native GPUI component for rendering Liora popover.
pub struct Popover {
    trigger: AnyElement,
    content: Arc<dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement + 'static>,
    placement: Placement,
    offset: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
    trigger_id: Option<ElementId>,
    content_padding: Option<Pixels>,
}

/// Fluent native GPUI component for rendering Liora popover view.
pub struct PopoverView {
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    anchor_bounds: Bounds<Pixels>,
    placement: Placement,
    offset: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
    id: SharedString,
    content_padding: Option<Pixels>,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl PopoverView {
    /// Creates `PopoverView` with default theme-driven styling and no optional callbacks attached.
    pub fn new(
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        anchor_bounds: Bounds<Pixels>,
        placement: Placement,
        offset: Pixels,
        close_on_click_outside: bool,
        close_on_escape: bool,
        id: SharedString,
        content_padding: Option<Pixels>,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            content,
            anchor_bounds,
            placement,
            offset,
            close_on_click_outside,
            close_on_escape,
            id,
            content_padding,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for PopoverView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let anchor_bounds = self.anchor_bounds;
        let placement = self.placement;
        let offset = self.offset;
        let on_close = self.on_close.clone();
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;
        let id = self.id.clone();
        let content_padding = self.content_padding;

        let content = (self.content)(_window, cx);
        let viewport_size = _window.viewport_size();
        let popover_anchor = popover_anchor_point(anchor_bounds, placement, offset);
        let popover_anchor_corner = popover_anchor_corner(placement);
        let viewport_margin = px(4.0);
        let max_w = (viewport_size.width - viewport_margin * 2.0).max(px(0.0));

        div()
            .id(id.clone())
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .cursor_default()
            .occlude()
            .bg(gpui::transparent_black())
            .on_hover(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_move(|_, _, cx| {
                cx.stop_propagation();
            })
            .when(close_on_escape, |s| {
                let on_close = on_close.clone();
                s.on_action(move |_: &PopoverClose, window, cx| {
                    on_close(window, cx);
                })
            })
            .when(close_on_click_outside, |s| {
                s.on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |_, _, window, cx| {
                        on_close(window, cx);
                    }),
                )
            })
            .child(
                gpui::anchored()
                    .position(popover_anchor)
                    .anchor(popover_anchor_corner)
                    .snap_to_window_with_margin(viewport_margin)
                    .child(pop_in(
                        element_id(format!("{}-motion", id)),
                        div()
                            .id(element_id(format!("{}-content", id)))
                            .flex()
                            .flex_col()
                            .flex_none()
                            .self_start()
                            .cursor_default()
                            .occlude()
                            .max_w(max_w)
                            .on_hover(|_, _, cx| {
                                cx.stop_propagation();
                            })
                            .on_mouse_move(|_, _, cx| {
                                cx.stop_propagation();
                            })
                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                cx.stop_propagation();
                            }) // Consume click so it doesn't trigger the background
                            .bg(theme.neutral.popover)
                            .text_color(theme.neutral.text_1)
                            .border_1()
                            .border_color(theme.neutral.border)
                            .rounded(px(theme.radius.md))
                            .shadow_lg()
                            .when_some(content_padding, |s, padding| s.p(padding))
                            .child(content),
                    )),
            )
    }
}

fn popover_anchor_point(
    anchor_bounds: Bounds<Pixels>,
    placement: Placement,
    offset: Pixels,
) -> gpui::Point<Pixels> {
    match placement {
        Placement::Top => point(
            anchor_bounds.left() + anchor_bounds.size.width / 2.0,
            anchor_bounds.top() - offset,
        ),
        Placement::TopStart => point(anchor_bounds.left(), anchor_bounds.top() - offset),
        Placement::TopEnd => point(anchor_bounds.right(), anchor_bounds.top() - offset),
        Placement::Bottom => point(
            anchor_bounds.left() + anchor_bounds.size.width / 2.0,
            anchor_bounds.bottom() + offset,
        ),
        Placement::BottomStart => point(anchor_bounds.left(), anchor_bounds.bottom() + offset),
        Placement::BottomEnd => point(anchor_bounds.right(), anchor_bounds.bottom() + offset),
        Placement::Left => point(
            anchor_bounds.left() - offset,
            anchor_bounds.top() + anchor_bounds.size.height / 2.0,
        ),
        Placement::LeftStart => point(anchor_bounds.left() - offset, anchor_bounds.top()),
        Placement::LeftEnd => point(anchor_bounds.left() - offset, anchor_bounds.bottom()),
        Placement::Right => point(
            anchor_bounds.right() + offset,
            anchor_bounds.top() + anchor_bounds.size.height / 2.0,
        ),
        Placement::RightStart => point(anchor_bounds.right() + offset, anchor_bounds.top()),
        Placement::RightEnd => point(anchor_bounds.right() + offset, anchor_bounds.bottom()),
    }
}

fn popover_anchor_corner(placement: Placement) -> gpui::Anchor {
    match placement {
        Placement::Top => gpui::Anchor::BottomCenter,
        Placement::TopStart => gpui::Anchor::BottomLeft,
        Placement::TopEnd => gpui::Anchor::BottomRight,
        Placement::Bottom => gpui::Anchor::TopCenter,
        Placement::BottomStart => gpui::Anchor::TopLeft,
        Placement::BottomEnd => gpui::Anchor::TopRight,
        Placement::Left => gpui::Anchor::RightCenter,
        Placement::LeftStart => gpui::Anchor::TopRight,
        Placement::LeftEnd => gpui::Anchor::BottomRight,
        Placement::Right => gpui::Anchor::LeftCenter,
        Placement::RightStart => gpui::Anchor::TopLeft,
        Placement::RightEnd => gpui::Anchor::BottomLeft,
    }
}

impl Popover {
    /// Creates `Popover` initialized from the supplied trigger.
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            content: Arc::new(|_, _| div().child("Popover Content").into_any_element()),
            placement: Placement::Bottom,
            offset: px(8.0),
            close_on_click_outside: true,
            close_on_escape: true,
            trigger_id: None,
            content_padding: Some(px(16.0)),
        }
    }

    /// Sets the rendered content element or text for this component.
    pub fn content<F, E>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<PopoverView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| f(window, cx).into_any_element());
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

    /// Applies the predefined offset lg sizing preset.
    pub fn offset_lg(self) -> Self {
        self.offset(px(20.0))
    }

    /// Toggles whether the popup closes when click outside occurs.
    pub fn close_on_click_outside(mut self, c: bool) -> Self {
        self.close_on_click_outside = c;
        self
    }

    /// Toggles whether the popup closes when escape occurs.
    pub fn close_on_escape(mut self, c: bool) -> Self {
        self.close_on_escape = c;
        self
    }

    /// Removes the shared content padding so composite popup bodies can own their spacing.
    pub fn flush_content(mut self) -> Self {
        self.content_padding = None;
        self
    }

    /// Overrides the shared content padding applied around plain popover bodies.
    pub fn content_padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.content_padding = Some(padding.into());
        self
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", PopoverClose, None)]);
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.trigger_id = Some(ElementId::from(id.into()));
        self
    }
}

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let placement = self.placement;
        let offset = self.offset;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;
        let content_padding = self.content_padding;
        let content = self.content.clone();
        let trigger_id = self.trigger_id.unwrap_or_else(|| {
            stable_unique_id("popover-trigger", "popover-trigger", _window, _cx).into()
        });
        let popover_id = match &trigger_id {
            ElementId::Name(name) => name.clone(),
            _ => SharedString::from(format!("popover-{:?}", trigger_id)),
        };

        let bounds_cell = Rc::new(Cell::new(None));
        let bounds_cell_clone = bounds_cell.clone();

        div()
            .id(trigger_id)
            .child(BoundsTracker {
                trigger: self.trigger,
                bounds: bounds_cell,
            })
            .on_click(move |_event, _window, cx| {
                if is_popover_active(&popover_id, cx) {
                    clear_popover(&popover_id, cx);
                    return;
                }

                if let Some(anchor_bounds) = bounds_cell_clone.get() {
                    let content = content.clone();
                    let popover_id_for_close = popover_id.clone();
                    let popover_id_for_view = popover_id.clone();
                    let view = cx.new(|_cx| {
                        PopoverView::new(
                            content,
                            anchor_bounds,
                            placement,
                            offset,
                            close_on_click_outside,
                            close_on_escape,
                            popover_id_for_view,
                            content_padding,
                            move |_window, _cx| {
                                clear_popover(&popover_id_for_close, _cx);
                            },
                        )
                    });
                    set_active_popover(popover_id.clone(), view.into(), cx);
                }
            })
    }
}

impl IntoElement for Popover {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

struct BoundsTracker {
    trigger: AnyElement,
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
}

impl IntoElement for BoundsTracker {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for BoundsTracker {
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
        self.bounds.set(Some(bounds));
        self.trigger.paint(window, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popover_offset_lg_sets_demo_offset() {
        assert_eq!(Popover::new("trigger").offset_lg().offset, px(20.0));
    }

    #[test]
    fn popover_padding_builders_track_plain_and_composite_modes() {
        assert_eq!(Popover::new("trigger").content_padding, Some(px(16.0)));
        assert_eq!(
            Popover::new("trigger").flush_content().content_padding,
            None
        );
        assert_eq!(
            Popover::new("trigger")
                .content_padding(px(8.0))
                .content_padding,
            Some(px(8.0))
        );
    }

    #[test]
    fn centered_placements_anchor_content_by_center_edges() {
        assert_eq!(
            popover_anchor_corner(Placement::Top),
            gpui::Anchor::BottomCenter
        );
        assert_eq!(
            popover_anchor_corner(Placement::Bottom),
            gpui::Anchor::TopCenter
        );
        assert_eq!(
            popover_anchor_corner(Placement::Left),
            gpui::Anchor::RightCenter
        );
        assert_eq!(
            popover_anchor_corner(Placement::Right),
            gpui::Anchor::LeftCenter
        );
    }

    #[test]
    fn popover_supports_flush_content_for_composite_bubbles() {
        let source = include_str!("popover.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("content_padding: Option<Pixels>"));
        assert!(source.contains("pub fn flush_content"));
        assert!(source.contains(".when_some(content_padding"));
        assert!(!source.contains(
            ".p_4()
                            .child(content)"
        ));
    }

    #[test]
    fn popover_shell_is_intrinsic_sized_and_theme_tokened() {
        let source = include_str!("popover.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains(".flex_none()"));
        assert!(source.contains(".flex()"));
        assert!(source.contains(".flex_col()"));
        assert!(source.contains(".self_start()"));
        assert!(source.contains(".bg(theme.neutral.popover)"));
        assert!(source.contains(".text_color(theme.neutral.text_1)"));
        assert!(!source.contains(".bg(theme.neutral.card)"));
    }

    #[test]
    fn popover_content_uses_liora_motion() {
        let source = include_str!("popover.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("pop_in("));
        assert!(source.contains("-motion"));
    }
}

#[cfg(test)]
mod spacing_regression_tests {
    #[test]
    fn popover_content_has_default_spacing() {
        let source = include_str!("popover.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(
            source.contains("content_padding: Some(px(16.0))"),
            "plain popover content should keep default spacing so bubble content is not cramped"
        );
        assert!(
            source.contains(".when_some(content_padding, |s, padding| s.p(padding))"),
            "shared popover padding should be optional for composite popup layouts"
        );
    }
}
