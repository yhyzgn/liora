//! Popover module.
//!
//! This public module implements the Liora popover positioning and popup component. It keeps the reusable
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

use crate::{button::ButtonKeyboardActivate, gpui_compat::element_id, motion::pop_in};
use gpui::{
    AnyElement, App, Bounds, Context, ElementId, FocusHandle, GlobalElementId, InspectorElementId,
    IntoElement, KeyBinding, LayoutId, MouseButton, Pixels, Render, RenderOnce, SharedString,
    ViewElement as Component, WeakFocusHandle, Window, actions, div, point, prelude::*, px,
};
use liora_core::{
    Config, Placement, clear_popover, is_popover_active, set_active_popover, stable_unique_id,
};
use std::cell::{Cell, RefCell};
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
    open_on_hover: bool,
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
    close_on_hover_outside: bool,
    content_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
    focus_scope: Option<FocusHandle>,
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
        Self::with_hover_close(
            content,
            anchor_bounds,
            placement,
            offset,
            close_on_click_outside,
            close_on_escape,
            id,
            content_padding,
            false,
            on_close,
        )
    }

    fn with_hover_close(
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        anchor_bounds: Bounds<Pixels>,
        placement: Placement,
        offset: Pixels,
        close_on_click_outside: bool,
        close_on_escape: bool,
        id: SharedString,
        content_padding: Option<Pixels>,
        close_on_hover_outside: bool,
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
            close_on_hover_outside,
            content_bounds: Rc::new(Cell::new(None)),
            on_close: Arc::new(on_close),
            focus_scope: None,
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
        let close_on_hover_outside = self.close_on_hover_outside;
        let content_bounds = self.content_bounds.clone();

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
            .when(!close_on_hover_outside, |s| {
                s.on_mouse_move(|_, _, cx| {
                    cx.stop_propagation();
                })
            })
            .when(close_on_escape, |s| {
                let on_close = on_close.clone();
                s.on_action(move |_: &PopoverClose, window, cx| {
                    on_close(window, cx);
                })
            })
            .when(close_on_click_outside, |s| {
                let on_close = on_close.clone();
                s.on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |_, _, window, cx| {
                        on_close(window, cx);
                    }),
                )
            })
            .when(close_on_hover_outside, |s| {
                let on_close = on_close.clone();
                s.on_mouse_move(move |event, window, cx| {
                    if let Some(content_bounds) = content_bounds.get()
                        && !hover_popover_safe_bounds(anchor_bounds, content_bounds)
                            .contains(&event.position)
                    {
                        on_close(window, cx);
                    }
                })
            })
            .child(
                gpui::anchored()
                    .position(popover_anchor)
                    .anchor(popover_anchor_corner)
                    .snap_to_window_with_margin(viewport_margin)
                    .child(BoundsTracker {
                        child: pop_in(
                            element_id(format!("{}-motion", id)),
                            div()
                                .id(element_id(format!("{}-content", id)))
                                .when_some(self.focus_scope.as_ref(), |s, scope| {
                                    s.track_focus(scope)
                                })
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
                        )
                        .into_any_element(),
                        bounds: self.content_bounds.clone(),
                    }),
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
            open_on_hover: false,
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

    /// Opens the popover when the pointer hovers the trigger. Click behavior remains available.
    pub fn open_on_hover(mut self, open: bool) -> Self {
        self.open_on_hover = open;
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
        let open_on_hover = self.open_on_hover;
        let content = self.content.clone();
        let trigger_id = self.trigger_id.unwrap_or_else(|| {
            stable_unique_id("popover-trigger", "popover-trigger", _window, _cx).into()
        });
        let popover_id = match &trigger_id {
            ElementId::Name(name) => name.clone(),
            _ => SharedString::from(format!("popover-{:?}", trigger_id)),
        };

        let bounds_cell = Rc::new(Cell::new(None));
        let hover_bounds_cell = bounds_cell.clone();
        let click_bounds_cell = bounds_cell.clone();
        let hover_content = content.clone();
        let hover_popover_id = popover_id.clone();
        let click_popover_id = popover_id.clone();
        let keyboard_bounds_cell = bounds_cell.clone();
        let keyboard_content = content.clone();
        let keyboard_popover_id = popover_id.clone();
        let escape_popover_id = popover_id.clone();
        let restore_subscription = _window.with_global_id(trigger_id.clone(), |global_id, window| {
            window.with_element_state::<Rc<RefCell<Option<(gpui::EntityId, gpui::Subscription)>>>, _>(global_id, |previous, _| {
                let state = previous.unwrap_or_else(|| Rc::new(RefCell::new(None)));
                (state.clone(), state)
            })
        });
        let hover_restore_subscription = restore_subscription.clone();
        let click_restore_subscription = restore_subscription.clone();
        let keyboard_restore_subscription = restore_subscription.clone();

        div()
            .id(trigger_id)
            .child(BoundsTracker {
                child: self.trigger,
                bounds: bounds_cell,
            })
            .when(open_on_hover, |s| {
                s.on_hover(move |hovered, _window, cx| {
                    if *hovered {
                        open_popover_view(OpenPopoverRequest {
                            id: hover_popover_id.clone(),
                            anchor_bounds: hover_bounds_cell.get(),
                            content: hover_content.clone(),
                            placement,
                            offset,
                            close_on_click_outside,
                            close_on_escape,
                            content_padding,
                            close_on_hover_outside: true,
                            return_focus: None,
                            cx,
                            window: _window,
                            restore_subscription: hover_restore_subscription.clone(),
                        });
                    }
                })
            })
            .on_click(move |_event, _window, cx| {
                if is_popover_active(&click_popover_id, cx) {
                    clear_popover(&click_popover_id, cx);
                    return;
                }

                open_popover_view(OpenPopoverRequest {
                    id: click_popover_id.clone(),
                    anchor_bounds: click_bounds_cell.get(),
                    content: content.clone(),
                    placement,
                    offset,
                    close_on_click_outside,
                    close_on_escape,
                    content_padding,
                    close_on_hover_outside: false,
                    return_focus: _window.focused(cx).map(|handle| handle.downgrade()),
                    restore_subscription: click_restore_subscription.clone(),
                    window: _window,
                    cx,
                });
            })
            .on_action(move |activation: &ButtonKeyboardActivate, window, cx| {
                if is_popover_active(&keyboard_popover_id, cx) {
                    clear_popover(&keyboard_popover_id, cx);
                    return;
                }
                open_popover_view(OpenPopoverRequest {
                    id: keyboard_popover_id.clone(),
                    anchor_bounds: keyboard_bounds_cell.get(),
                    content: keyboard_content.clone(),
                    placement,
                    offset,
                    close_on_click_outside,
                    close_on_escape,
                    content_padding,
                    close_on_hover_outside: false,
                    return_focus: Some(activation.origin.downgrade()),
                    restore_subscription: keyboard_restore_subscription.clone(),
                    window,
                    cx,
                });
            })
            .on_action(move |_: &PopoverClose, window, cx| {
                let view = cx
                    .global::<liora_core::ActivePopover>()
                    .0
                    .iter()
                    .find(|entry| entry.id == escape_popover_id)
                    .and_then(|entry| entry.view.clone().downcast::<PopoverView>().ok());
                let Some(view) = view else {
                    cx.propagate();
                    return;
                };
                let view = view.read(cx);
                if view.close_on_escape {
                    let on_close = view.on_close.clone();
                    on_close(window, cx);
                }
            })
    }
}

struct OpenPopoverRequest<'a> {
    id: SharedString,
    anchor_bounds: Option<Bounds<Pixels>>,
    content: Arc<dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement + 'static>,
    placement: Placement,
    offset: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
    content_padding: Option<Pixels>,
    close_on_hover_outside: bool,
    window: &'a mut Window,
    return_focus: Option<WeakFocusHandle>,
    restore_subscription: Rc<RefCell<Option<(gpui::EntityId, gpui::Subscription)>>>,
    cx: &'a mut App,
}

fn open_popover_view(request: OpenPopoverRequest<'_>) {
    let Some(anchor_bounds) = request.anchor_bounds else {
        return;
    };

    let content = request.content;
    let popover_id_for_close = request.id.clone();
    let popover_id_for_view = request.id.clone();
    let id = request.id.clone();
    let window_handle = request.window.window_handle();
    let target = request.return_focus;
    let restore_subscription = request.restore_subscription.clone();
    let view = request.cx.new(|cx| {
        let view_id = cx.entity_id();
        let mut view = PopoverView::with_hover_close(
            content,
            anchor_bounds,
            request.placement,
            request.offset,
            request.close_on_click_outside,
            request.close_on_escape,
            popover_id_for_view,
            request.content_padding,
            request.close_on_hover_outside,
            move |_window, cx| {
                if cx
                    .global::<liora_core::ActivePopover>()
                    .0
                    .iter()
                    .any(|entry| {
                        entry.id == popover_id_for_close && entry.view.entity_id() == view_id
                    })
                {
                    clear_popover(&popover_id_for_close, cx);
                }
            },
        );
        view.focus_scope = Some(cx.focus_handle());
        view
    });
    let view_id = view.entity_id();
    let focus_scope = view
        .read(request.cx)
        .focus_scope
        .clone()
        .expect("opened popover has a focus scope");
    if let Some(target) = target {
        let slot = Rc::downgrade(&restore_subscription);
        let subscription = request
            .cx
            .observe_global::<liora_core::ActivePopover>(move |cx| {
                if let Some(entry) = cx
                    .global::<liora_core::ActivePopover>()
                    .0
                    .iter()
                    .find(|entry| entry.id == id)
                {
                    if entry.view.entity_id() == view_id {
                        return;
                    }
                } else if let Some(target) = target.upgrade() {
                    let _ = window_handle.update(cx, |_, window, cx| {
                        if focus_scope.contains_focused(window, cx) {
                            target.focus(window, cx);
                        }
                    });
                }
                if let Some(slot) = slot.upgrade() {
                    cx.defer(move |_| {
                        let mut slot = slot.borrow_mut();
                        if slot.as_ref().is_some_and(|(id, _)| *id == view_id) {
                            slot.take();
                        }
                    });
                }
            });
        *restore_subscription.borrow_mut() = Some((view_id, subscription));
    }
    set_active_popover(request.id, view.into(), request.cx);
}

fn hover_popover_safe_bounds(
    anchor_bounds: Bounds<Pixels>,
    content_bounds: Bounds<Pixels>,
) -> Bounds<Pixels> {
    anchor_bounds.union(&content_bounds).inset(px(-12.0))
}

impl IntoElement for Popover {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

struct BoundsTracker {
    child: AnyElement,
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
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
        (self.child.request_layout(window, cx), ())
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
        self.child.prepaint(window, cx);
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
        self.child.paint(window, cx);
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
    fn popover_hover_builder_preserves_click_default() {
        assert!(!Popover::new("trigger").open_on_hover);
        assert!(Popover::new("trigger").open_on_hover(true).open_on_hover);
    }

    #[test]
    fn hover_safe_bounds_keeps_trigger_to_card_transition_open() {
        let anchor = Bounds {
            origin: point(px(20.0), px(20.0)),
            size: gpui::size(px(80.0), px(24.0)),
        };
        let content = Bounds {
            origin: point(px(20.0), px(52.0)),
            size: gpui::size(px(160.0), px(72.0)),
        };
        let safe = hover_popover_safe_bounds(anchor, content);

        assert!(safe.contains(&point(px(40.0), px(44.0))));
        assert!(safe.contains(&point(px(40.0), px(70.0))));
        assert!(!safe.contains(&point(px(260.0), px(180.0))));
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

    struct PopoverSurface;

    impl Render for PopoverSurface {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            liora_core::render_active_popover_in_window(window, cx);
            div().relative().size_full().child(crate::PortalLayer)
        }
    }

    #[test]
    fn stale_dispatched_close_does_not_clear_same_id_replacement() {
        use crate::Button;

        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let handle = app.open_window(gpui::size(px(400.0), px(300.0)), |_, _| PopoverSurface);
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        let stale_close = std::rc::Rc::new(std::cell::RefCell::new(None));
        let old_content: Arc<dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement> =
            Arc::new(|_, _| div().into_any_element());

        window.update(|window, cx| {
            let anchor_bounds = Some(Bounds {
                origin: point(px(10.0), px(10.0)),
                size: gpui::size(px(80.0), px(24.0)),
            });
            open_popover_view(OpenPopoverRequest {
                id: "shared-popover".into(),
                anchor_bounds,
                content: old_content,
                placement: Placement::Bottom,
                offset: px(8.0),
                close_on_click_outside: true,
                close_on_escape: true,
                content_padding: None,
                close_on_hover_outside: false,
                return_focus: None,
                restore_subscription: Rc::new(RefCell::new(None)),
                window,
                cx,
            });
            let old_view = cx.global::<liora_core::ActivePopover>().0[0]
                .view
                .clone()
                .downcast::<PopoverView>()
                .unwrap();
            *stale_close.borrow_mut() = Some(old_view.read(cx).on_close.clone());
        });

        let stale_close_for_content = stale_close.clone();
        let clicked = std::rc::Rc::new(std::cell::Cell::new(false));
        let clicked_for_content = clicked.clone();
        let replacement_content: Arc<dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement> =
            Arc::new(move |_, _| {
                let stale_close_for_button = stale_close_for_content.clone();
                let clicked_for_button = clicked_for_content.clone();
                div()
                    .debug_selector(|| "replacement-popover-content".into())
                    .child(
                        Button::new("Invoke stale close").on_click(move |_, window, cx| {
                            clicked_for_button.set(true);
                            if let Some(close) = stale_close_for_button.borrow().as_ref() {
                                close(window, cx);
                            }
                        }),
                    )
                    .into_any_element()
            });
        let replacement_id = window.update(|window, cx| {
            open_popover_view(OpenPopoverRequest {
                id: "shared-popover".into(),
                anchor_bounds: Some(Bounds {
                    origin: point(px(10.0), px(10.0)),
                    size: gpui::size(px(80.0), px(24.0)),
                }),
                content: replacement_content,
                placement: Placement::Bottom,
                offset: px(8.0),
                close_on_click_outside: true,
                close_on_escape: true,
                content_padding: None,
                close_on_hover_outside: false,
                return_focus: None,
                restore_subscription: Rc::new(RefCell::new(None)),
                window,
                cx,
            });
            cx.global::<liora_core::ActivePopover>().0[0]
                .view
                .entity_id()
        });
        window.update(|window, cx| window.draw(cx).clear(cx));
        let close_button = window
            .debug_bounds("replacement-popover-content")
            .expect("replacement action must render");
        window.simulate_click(close_button.center(), gpui::Modifiers::none());
        assert!(
            clicked.get(),
            "replacement Button callback did not receive the native click"
        );
        window.update(|window, cx| {
            let active = &cx.global::<liora_core::ActivePopover>().0;
            assert_eq!(
                active.len(),
                1,
                "stale dispatched close cleared replacement"
            );
            assert_eq!(active[0].view.entity_id(), replacement_id);
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("replacement-popover-content").is_some());
        app.quit();
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

#[cfg(test)]
#[path = "popover_interaction_tests.rs"]
mod interaction_tests;
