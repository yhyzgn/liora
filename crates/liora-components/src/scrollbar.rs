//! Scrollbar module.
//!
//! This public module implements the Liora custom scrollbar wrappers for GPUI virtual scroll areas. It keeps the reusable
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
    AnyElement, App, Bounds, Context, CursorStyle, DispatchPhase, Element, EntityId,
    GlobalElementId, Hitbox, HitboxBehavior, InspectorElementId, IntoElement, LayoutId, ListState,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point, Render,
    ScrollHandle, Style, Window, div, point, prelude::*, px, relative, size,
};
use liora_core::Config;
use std::cell::Cell;

thread_local! {
    static VIRTUAL_SCROLLBAR_DRAG_STATE: Cell<Option<ScrollbarDragState>> = const { Cell::new(None) };
    static SCROLLBAR_DRAG_STATE: Cell<Option<ScrollbarDragState>> = const { Cell::new(None) };
}

const SCROLLBAR_THUMB_WIDTH: Pixels = px(4.0);
const SCROLLBAR_THUMB_HOVER_WIDTH: Pixels = px(12.0);
const SCROLLBAR_HIT_WIDTH: Pixels = px(20.0);
const SCROLLBAR_MIN_THUMB_HEIGHT: Pixels = px(24.0);

/// Returns the interactive width reserved for a virtual scrollbar track.
pub fn virtual_scrollbar_hit_width() -> Pixels {
    SCROLLBAR_HIT_WIDTH + px(2.0)
}

/// Fluent native GPUI component for rendering Liora scrollbar.
pub struct Scrollbar {
    scroll_handle: ScrollHandle,
    render_content: Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>,
    height: Option<Pixels>,
}

impl Scrollbar {
    /// Creates `Scrollbar` initialized from the supplied render content.
    pub fn new<F, E>(_cx: &mut Context<Self>, render_content: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> E + 'static,
        E: IntoElement,
    {
        Self {
            scroll_handle: ScrollHandle::new(),
            render_content: Box::new(move |window, cx| {
                render_content(window, cx).into_any_element()
            }),
            height: None,
        }
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, h: impl Into<Pixels>) -> Self {
        self.height = Some(h.into());
        self
    }
}

/// Paints and drives a scrollbar for GPUI's virtual [`ListState`].
///
/// This lets host applications use GPUI's native virtual list for visible-area rendering
/// while still bootstrapping the visual scrollbar from Liora's component layer.
pub struct VirtualScrollbar {
    list_state: ListState,
}

impl VirtualScrollbar {
    /// Creates `VirtualScrollbar` initialized from the supplied list state.
    pub fn new(list_state: ListState) -> Self {
        Self { list_state }
    }
}

/// Fluent native GPUI component for rendering Liora virtual scrollbar prepaint.
pub struct VirtualScrollbarPrepaint {
    thumb_bounds: Option<Bounds<Pixels>>,
    hover_bounds: Bounds<Pixels>,
    hitbox: Hitbox,
    owner: u64,
    active: bool,
    dragging: bool,
}

#[derive(Clone, Copy)]
struct ThumbMetrics {
    bounds: Bounds<Pixels>,
    viewport_bounds: Bounds<Pixels>,
    max_offset: Pixels,
    track_height: Pixels,
    track_top: Pixels,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ScrollbarDragState {
    owner: u64,
    grab_offset: Pixels,
}

impl Element for VirtualScrollbar {
    type RequestLayoutState = ();
    type PrepaintState = VirtualScrollbarPrepaint;

    fn id(&self) -> Option<gpui::ElementId> {
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
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.position = gpui::Position::Absolute;
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let metrics = virtual_thumb_metrics(&self.list_state, SCROLLBAR_THUMB_WIDTH);
        let thumb = metrics.map(|metrics| metrics.bounds);
        let hitbox_bounds = metrics
            .map(|metrics| scrollbar_track_hitbox(metrics.viewport_bounds))
            .unwrap_or_else(|| scrollbar_track_hitbox(bounds));
        let hitbox = window.insert_hitbox(hitbox_bounds, HitboxBehavior::Normal);
        let owner = scrollbar_owner(window.current_view(), hitbox_bounds);
        let dragging = virtual_scrollbar_drag_state().is_some_and(|state| state.owner == owner);
        let active = hitbox.is_hovered(window)
            || dragging
            || hitbox_bounds.contains(&window.mouse_position());
        let thumb_bounds = thumb.map(|thumb| {
            let target_width = if active {
                SCROLLBAR_THUMB_HOVER_WIDTH
            } else {
                SCROLLBAR_THUMB_WIDTH
            };
            scrollbar_thumb_bounds_for_width(thumb, target_width)
        });
        VirtualScrollbarPrepaint {
            thumb_bounds,
            hover_bounds: hitbox_bounds,
            hitbox,
            owner,
            active,
            dragging,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(thumb_bounds) = prepaint.thumb_bounds else {
            return;
        };

        let thumb_color = cx
            .global::<Config>()
            .theme
            .neutral
            .border
            .opacity(if prepaint.dragging { 1.0 } else { 0.8 });
        window.paint_quad(PaintQuad {
            bounds: thumb_bounds,
            corner_radii: gpui::Corners::all(thumb_bounds.size.width / 2.0),
            background: thumb_color.into(),
            border_widths: gpui::Edges::all(px(0.0)),
            border_color: gpui::transparent_black(),
            border_style: gpui::BorderStyle::Solid,
        });
        set_scrollbar_cursor(window, &prepaint.hitbox, prepaint.active, prepaint.dragging);

        let was_active = prepaint.active;
        let hover_bounds = prepaint.hover_bounds;
        let owner = prepaint.owner;
        let current_view = window.current_view();
        window.on_mouse_event(move |_: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                let active = virtual_scrollbar_drag_state()
                    .is_some_and(|state| state.owner == owner)
                    || hover_bounds.contains(&window.mouse_position());
                if active != was_active {
                    cx.notify(current_view);
                    window.refresh();
                }
            }
        });

        let list_state = self.list_state.clone();
        let hitbox = prepaint.hitbox.clone();
        let hover_bounds = prepaint.hover_bounds;
        let raw_thumb_bounds = thumb_bounds;
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture
                && event.button == MouseButton::Left
                && (hitbox.is_hovered(window) || hover_bounds.contains(&event.position))
            {
                let grab_offset = if raw_thumb_bounds.contains(&event.position) {
                    event.position.y - raw_thumb_bounds.top()
                } else {
                    raw_thumb_bounds.size.height / 2.0
                };
                set_virtual_scrollbar_drag_state(Some(ScrollbarDragState { owner, grab_offset }));
                list_state.scrollbar_drag_started();
                set_virtual_scrollbar_position(&list_state, event.position, grab_offset);

                cx.stop_propagation();
                window.refresh();
            }
        });

        let list_state = self.list_state.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                let Some(drag_state) =
                    virtual_scrollbar_drag_state().filter(|state| state.owner == owner)
                else {
                    return;
                };
                set_virtual_scrollbar_position(&list_state, event.position, drag_state.grab_offset);
                cx.stop_propagation();
                window.refresh();
            }
        });

        let list_state = self.list_state.clone();
        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture
                && event.button == MouseButton::Left
                && virtual_scrollbar_drag_state().is_some_and(|state| state.owner == owner)
            {
                list_state.scrollbar_drag_ended();
                set_virtual_scrollbar_drag_state(None);
                cx.stop_propagation();
                window.refresh();
            }
        });
    }
}

fn set_scrollbar_cursor(window: &mut Window, hitbox: &Hitbox, active: bool, dragging: bool) {
    if dragging {
        window.set_window_cursor_style(CursorStyle::ResizeUpDown);
    } else if active {
        window.set_cursor_style(CursorStyle::PointingHand, hitbox);
    }
}

fn set_virtual_scrollbar_position(
    list_state: &ListState,
    position: Point<Pixels>,
    grab_offset: Pixels,
) {
    let Some(metrics) = virtual_thumb_metrics(list_state, SCROLLBAR_THUMB_WIDTH) else {
        return;
    };
    let Some(offset) = scrollbar_offset_from_position(position, metrics, grab_offset) else {
        return;
    };
    list_state.set_offset_from_scrollbar(point(px(0.0), offset));
}

fn virtual_thumb_metrics(list_state: &ListState, width: Pixels) -> Option<ThumbMetrics> {
    thumb_metrics_from_values(
        list_state.viewport_bounds(),
        list_state.max_offset_for_scrollbar().y,
        list_state.scroll_px_offset_for_scrollbar().y,
        width,
    )
}

fn thumb_metrics_from_values(
    viewport_bounds: Bounds<Pixels>,
    max_offset_y: Pixels,
    scroll_offset_y: Pixels,
    width: Pixels,
) -> Option<ThumbMetrics> {
    let viewport_h = viewport_bounds.size.height;
    let max_offset_y = max_offset_y.max(px(0.0));
    let content_h = viewport_h + max_offset_y;

    if content_h <= viewport_h || content_h <= px(0.0) || viewport_h <= px(0.0) {
        return None;
    }

    let ratio = viewport_h / content_h;
    let thumb_h = (viewport_h * ratio)
        .max(SCROLLBAR_MIN_THUMB_HEIGHT)
        .min(viewport_h);
    let current_offset = (-scroll_offset_y).clamp(px(0.0), max_offset_y);
    let track_height = (viewport_h - thumb_h).max(px(0.0));
    let thumb_top = if max_offset_y > px(0.0) {
        track_height * (current_offset / max_offset_y)
    } else {
        px(0.0)
    };

    let bounds = Bounds {
        origin: point(
            viewport_bounds.right() - width - px(2.0),
            viewport_bounds.top() + thumb_top,
        ),
        size: size(width, thumb_h),
    };

    Some(ThumbMetrics {
        bounds,
        viewport_bounds,
        max_offset: max_offset_y,
        track_height,
        track_top: viewport_bounds.top(),
    })
}

fn scrollbar_thumb_bounds_for_width(target: Bounds<Pixels>, width: Pixels) -> Bounds<Pixels> {
    Bounds {
        origin: point(target.right() - width, target.top()),
        size: size(width, target.size.height),
    }
}

fn scrollbar_track_hitbox(bounds: Bounds<Pixels>) -> Bounds<Pixels> {
    Bounds {
        origin: point(bounds.right() - SCROLLBAR_HIT_WIDTH - px(2.0), bounds.top()),
        size: size(virtual_scrollbar_hit_width(), bounds.size.height),
    }
}

fn scrollbar_offset_from_position(
    position: Point<Pixels>,
    metrics: ThumbMetrics,
    grab_offset: Pixels,
) -> Option<Pixels> {
    if metrics.max_offset <= px(0.0) || metrics.track_height <= px(0.0) {
        return None;
    }

    let y = (position.y - metrics.track_top - grab_offset).clamp(px(0.0), metrics.track_height);
    let content_offset = y / metrics.track_height * metrics.max_offset;
    Some(-content_offset)
}

fn scrollbar_owner(view_id: EntityId, bounds: Bounds<Pixels>) -> u64 {
    let mut hash = view_id.as_u64();
    for value in [
        bounds.origin.x.as_f32(),
        bounds.origin.y.as_f32(),
        bounds.size.width.as_f32(),
        bounds.size.height.as_f32(),
    ] {
        hash = hash.wrapping_mul(16_777_619) ^ u64::from(value.to_bits());
    }
    hash
}

fn virtual_scrollbar_drag_state() -> Option<ScrollbarDragState> {
    VIRTUAL_SCROLLBAR_DRAG_STATE.with(Cell::get)
}

fn set_virtual_scrollbar_drag_state(state: Option<ScrollbarDragState>) {
    VIRTUAL_SCROLLBAR_DRAG_STATE.with(|slot| slot.set(state));
}

fn scrollbar_drag_state() -> Option<ScrollbarDragState> {
    SCROLLBAR_DRAG_STATE.with(Cell::get)
}

fn set_scrollbar_drag_state(state: Option<ScrollbarDragState>) {
    SCROLLBAR_DRAG_STATE.with(|slot| slot.set(state));
}

fn scroll_handle_thumb_metrics(
    scroll_handle: &ScrollHandle,
    width: Pixels,
) -> Option<ThumbMetrics> {
    thumb_metrics_from_values(
        scroll_handle.bounds(),
        scroll_handle.max_offset().y,
        scroll_handle.offset().y,
        width,
    )
}

fn set_scroll_handle_position(
    scroll_handle: &ScrollHandle,
    position: Point<Pixels>,
    grab_offset: Pixels,
) {
    let Some(metrics) = scroll_handle_thumb_metrics(scroll_handle, SCROLLBAR_THUMB_WIDTH) else {
        return;
    };
    if metrics.max_offset <= px(0.0) || metrics.track_height <= px(0.0) {
        return;
    }

    let Some(offset) = scrollbar_offset_from_position(position, metrics, grab_offset) else {
        return;
    };
    scroll_handle.set_offset(point(px(0.0), offset));
}

impl Render for Scrollbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let scroll_handle = self.scroll_handle.clone();
        let content = (self.render_content)(_window, cx);

        let mut container = div().flex().flex_col().overflow_hidden();
        if let Some(h) = self.height {
            container = container.h(h);
        } else {
            container = container.h_full();
        }

        container
            .relative()
            .child(
                div()
                    .flex_1()
                    .id("scroll-viewport")
                    .overflow_y_scroll()
                    .track_scroll(&scroll_handle)
                    .on_scroll(cx.listener(|_, _, _, cx| {
                        cx.notify();
                    }))
                    .child(content),
            )
            .child(ScrollbarThumb {
                scroll_handle: self.scroll_handle.clone(),
            })
    }
}

struct ScrollbarThumb {
    scroll_handle: ScrollHandle,
}

struct ScrollbarThumbPrepaint {
    thumb_bounds: Option<Bounds<Pixels>>,
    hover_bounds: Bounds<Pixels>,
    hitbox: Hitbox,
    owner: u64,
    active: bool,
    dragging: bool,
}

impl gpui::Element for ScrollbarThumb {
    type RequestLayoutState = ();
    type PrepaintState = ScrollbarThumbPrepaint;

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _id2: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let mut style = gpui::Style::default();
        style.position = gpui::Position::Absolute;
        style.size.width = gpui::relative(1.0).into();
        style.size.height = gpui::relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _id2: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let metrics = scroll_handle_thumb_metrics(&self.scroll_handle, SCROLLBAR_THUMB_WIDTH);
        let thumb = metrics.map(|metrics| metrics.bounds);
        let hover_bounds = metrics
            .map(|metrics| scrollbar_track_hitbox(metrics.viewport_bounds))
            .unwrap_or_else(|| scrollbar_track_hitbox(bounds));
        let hitbox = window.insert_hitbox(hover_bounds, HitboxBehavior::Normal);
        let owner = scrollbar_owner(window.current_view(), hover_bounds);
        let dragging = scrollbar_drag_state().is_some_and(|state| state.owner == owner);
        let active = hitbox.is_hovered(window)
            || dragging
            || hover_bounds.contains(&window.mouse_position());
        let thumb_bounds = thumb.map(|thumb| {
            let target_width = if active {
                SCROLLBAR_THUMB_HOVER_WIDTH
            } else {
                SCROLLBAR_THUMB_WIDTH
            };
            scrollbar_thumb_bounds_for_width(thumb, target_width)
        });

        ScrollbarThumbPrepaint {
            thumb_bounds,
            hover_bounds,
            hitbox,
            owner,
            active,
            dragging,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _id2: Option<&gpui::InspectorElementId>,
        _bounds: gpui::Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) -> () {
        let Some(thumb_bounds) = prepaint.thumb_bounds else {
            return;
        };

        let thumb_color = cx
            .global::<Config>()
            .theme
            .neutral
            .border
            .opacity(if prepaint.dragging { 1.0 } else { 0.8 });

        let was_active = prepaint.active;
        let hover_bounds = prepaint.hover_bounds;
        let owner = prepaint.owner;
        let current_view = window.current_view();
        window.on_mouse_event(move |_: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                let active = scrollbar_drag_state().is_some_and(|state| state.owner == owner)
                    || hover_bounds.contains(&window.mouse_position());
                if active != was_active {
                    cx.notify(current_view);
                    window.refresh();
                }
            }
        });

        let scroll_handle = self.scroll_handle.clone();
        let hitbox = prepaint.hitbox.clone();
        let hover_bounds = prepaint.hover_bounds;
        let raw_thumb_bounds = thumb_bounds;
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture
                && event.button == MouseButton::Left
                && (hitbox.is_hovered(window) || hover_bounds.contains(&event.position))
            {
                let grab_offset = if raw_thumb_bounds.contains(&event.position) {
                    event.position.y - raw_thumb_bounds.top()
                } else {
                    raw_thumb_bounds.size.height / 2.0
                };
                set_scrollbar_drag_state(Some(ScrollbarDragState { owner, grab_offset }));
                set_scroll_handle_position(&scroll_handle, event.position, grab_offset);

                cx.stop_propagation();
                window.refresh();
            }
        });

        let scroll_handle = self.scroll_handle.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                let Some(drag_state) = scrollbar_drag_state().filter(|state| state.owner == owner)
                else {
                    return;
                };
                set_scroll_handle_position(&scroll_handle, event.position, drag_state.grab_offset);
                cx.stop_propagation();
                window.refresh();
            }
        });

        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture
                && event.button == MouseButton::Left
                && scrollbar_drag_state().is_some_and(|state| state.owner == owner)
            {
                set_scrollbar_drag_state(None);
                cx.stop_propagation();
                window.refresh();
            }
        });

        set_scrollbar_cursor(window, &prepaint.hitbox, prepaint.active, prepaint.dragging);

        window.paint_quad(gpui::PaintQuad {
            bounds: thumb_bounds,
            corner_radii: gpui::Corners::all(thumb_bounds.size.width / 2.0),
            background: thumb_color.into(),
            border_widths: gpui::Edges::all(gpui::px(0.0)),
            border_color: gpui::hsla(0.0, 0.0, 0.0, 0.0),
            border_style: gpui::BorderStyle::Solid,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_viewport(height: f32) -> Bounds<Pixels> {
        Bounds {
            origin: point(px(10.0), px(20.0)),
            size: size(px(100.0), px(height)),
        }
    }

    #[test]
    fn virtual_scrollbar_bootstraps_gpui_list_state_scrolling() {
        let source = include_str!("scrollbar.rs");

        assert!(source.contains("pub struct VirtualScrollbar"));
        assert!(source.contains("ListState"));
        assert!(source.contains("scroll_px_offset_for_scrollbar"));
        assert!(source.contains("max_offset_for_scrollbar"));
        assert!(source.contains("set_offset_from_scrollbar"));
        assert!(source.contains("scrollbar_track_hitbox(metrics.viewport_bounds)"));
        assert!(source.contains("scrollbar_drag_started"));
        assert!(source.contains("scrollbar_drag_ended"));
        assert!(source.contains("virtual_scrollbar_drag_state"));
    }

    #[test]
    fn thumb_metrics_match_gpui_scrollbar_model_and_clamp_offsets() {
        let metrics = thumb_metrics_from_values(
            test_viewport(200.0),
            px(300.0),
            px(-150.0),
            SCROLLBAR_THUMB_WIDTH,
        )
        .expect("scrollable content should produce a thumb");

        assert_eq!(metrics.bounds.size.height, px(80.0));
        assert_eq!(metrics.viewport_bounds, test_viewport(200.0));
        assert_eq!(metrics.track_height, px(120.0));
        assert_eq!(metrics.bounds.top(), px(80.0));
        assert_eq!(metrics.bounds.right(), px(108.0));

        let before_start = thumb_metrics_from_values(
            test_viewport(200.0),
            px(300.0),
            px(64.0),
            SCROLLBAR_THUMB_WIDTH,
        )
        .expect("positive scroll offsets are clamped to the top");
        assert_eq!(before_start.bounds.top(), px(20.0));

        let past_end = thumb_metrics_from_values(
            test_viewport(200.0),
            px(300.0),
            px(-999.0),
            SCROLLBAR_THUMB_WIDTH,
        )
        .expect("overscrolled offsets are clamped to the bottom");
        assert_eq!(past_end.bounds.top(), px(140.0));
    }

    #[test]
    fn scrollbar_drag_uses_negative_gpui_content_offsets() {
        let metrics = thumb_metrics_from_values(
            test_viewport(200.0),
            px(300.0),
            px(0.0),
            SCROLLBAR_THUMB_WIDTH,
        )
        .expect("scrollable content should produce a thumb");

        assert_eq!(
            scrollbar_offset_from_position(point(px(105.0), px(80.0)), metrics, px(0.0)),
            Some(px(-150.0))
        );
        assert_eq!(
            scrollbar_offset_from_position(point(px(105.0), px(999.0)), metrics, px(0.0)),
            Some(px(-300.0))
        );
        assert_eq!(
            scrollbar_offset_from_position(point(px(105.0), px(-999.0)), metrics, px(0.0)),
            Some(px(0.0))
        );
    }

    #[test]
    fn scrollbar_hitbox_covers_full_track_for_easy_hover_and_drag() {
        let hitbox = scrollbar_track_hitbox(test_viewport(200.0));

        assert_eq!(hitbox.top(), px(20.0));
        assert_eq!(hitbox.size.height, px(200.0));
        assert_eq!(hitbox.size.width, virtual_scrollbar_hit_width());
        assert_eq!(SCROLLBAR_THUMB_HOVER_WIDTH, px(12.0));
    }

    #[test]
    fn scrollbar_drag_state_is_scoped_to_the_owning_track() {
        let source = include_str!("scrollbar.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("production source should precede tests");

        assert!(source.contains("owner: u64"));
        assert!(source.contains("scrollbar_owner(window.current_view(), hitbox_bounds)"));
        assert!(source.contains("scrollbar_owner(window.current_view(), hover_bounds)"));
        assert!(source.contains("filter(|state| state.owner == owner)"));
        assert!(source.contains("is_some_and(|state| state.owner == owner)"));
        assert!(!source.contains("static VIRTUAL_SCROLLBAR_GRAB_OFFSET"));
        assert!(!source.contains("static SCROLLBAR_GRAB_OFFSET"));
    }

    #[test]
    fn scrollbars_expand_on_hover_and_drag_without_smoothing() {
        let source = include_str!("scrollbar.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("production source should precede tests");

        assert!(source.contains("SCROLLBAR_THUMB_HOVER_WIDTH"));
        assert!(source.contains("SCROLLBAR_HIT_WIDTH"));
        assert!(source.contains("scrollbar_thumb_bounds_for_width"));
        assert!(source.contains("set_scrollbar_drag_state"));
        assert!(source.contains("set_virtual_scrollbar_drag_state"));
        assert!(source.contains("set_scroll_handle_position"));
        assert!(source.contains("set_virtual_scrollbar_position"));
        assert!(source.contains("CursorStyle::PointingHand"));
        assert!(source.contains("CursorStyle::ResizeUpDown"));
        assert!(source.contains("hover_bounds.contains(&window.mouse_position())"));
        assert!(source.contains("cx.notify(current_view)"));
        assert!(!source.contains("lerp_pixels"));
        assert!(!source.contains("request_animation_frame"));
    }
}
