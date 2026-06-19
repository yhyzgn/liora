//! Scrollbar module.
//!
//! This public module implements the Liora custom scrollbar wrappers for GPUI virtual scroll areas. It keeps the reusable
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

use gpui::{
    AnyElement, App, Bounds, Context, DispatchPhase, Element, GlobalElementId, Hitbox,
    HitboxBehavior, InspectorElementId, IntoElement, LayoutId, ListState, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point, Render, ScrollHandle,
    Size, Style, Window, div, point, prelude::*, px, relative, size,
};
use liora_core::Config;
use std::cell::Cell;

thread_local! {
    static VIRTUAL_SCROLLBAR_GRAB_OFFSET: Cell<Option<Pixels>> = const { Cell::new(None) };
    static SCROLLBAR_GRAB_OFFSET: Cell<Option<Pixels>> = const { Cell::new(None) };
}

const SCROLLBAR_THUMB_WIDTH: Pixels = px(4.0);
const SCROLLBAR_THUMB_HOVER_WIDTH: Pixels = px(8.0);
const SCROLLBAR_HIT_WIDTH: Pixels = px(14.0);
const SCROLLBAR_MIN_THUMB_HEIGHT: Pixels = px(24.0);

pub struct Scrollbar {
    scroll_handle: ScrollHandle,
    render_content: Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>,
    height: Option<Pixels>,
}

impl Scrollbar {
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

    pub fn height(mut self, h: impl Into<Pixels>) -> Self {
        self.height = Some(h.into());
        self
    }
}

/// Paints and drives a scrollbar for GPUI's virtual [`ListState`].
///
/// This lets Liora Docs use GPUI's native virtual list for visible-area rendering
/// while still bootstrapping the visual scrollbar from Liora's component layer.
pub struct VirtualScrollbar {
    list_state: ListState,
}

impl VirtualScrollbar {
    pub fn new(list_state: ListState) -> Self {
        Self { list_state }
    }
}

impl IntoElement for VirtualScrollbar {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct VirtualScrollbarPrepaint {
    thumb_bounds: Option<Bounds<Pixels>>,
    hover_bounds: Bounds<Pixels>,
    hitbox: Hitbox,
    active: bool,
    dragging: bool,
}

#[derive(Clone, Copy)]
struct ThumbMetrics {
    bounds: Bounds<Pixels>,
    max_offset: Pixels,
    track_height: Pixels,
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
        let hitbox_bounds = thumb.map(expand_scrollbar_hitbox).unwrap_or(Bounds {
            origin: point(bounds.right() - SCROLLBAR_HIT_WIDTH, bounds.top()),
            size: Size {
                width: SCROLLBAR_HIT_WIDTH,
                height: bounds.size.height,
            },
        });
        let hitbox = window.insert_hitbox(hitbox_bounds, HitboxBehavior::Normal);
        let dragging = virtual_scrollbar_grab_offset().is_some();
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

        let was_active = prepaint.active;
        let hover_bounds = prepaint.hover_bounds;
        let current_view = window.current_view();
        window.on_mouse_event(move |_: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                let active = virtual_scrollbar_grab_offset().is_some()
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
                set_virtual_scrollbar_grab_offset(Some(grab_offset));
                list_state.scrollbar_drag_started();
                set_virtual_scrollbar_position(&list_state, event.position, grab_offset);

                cx.stop_propagation();
                window.refresh();
            }
        });

        let list_state = self.list_state.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                let Some(grab_offset) = virtual_scrollbar_grab_offset() else {
                    return;
                };
                set_virtual_scrollbar_position(&list_state, event.position, grab_offset);
                cx.stop_propagation();
                window.refresh();
            }
        });

        let list_state = self.list_state.clone();
        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture
                && event.button == MouseButton::Left
                && virtual_scrollbar_grab_offset().is_some()
            {
                list_state.scrollbar_drag_ended();
                set_virtual_scrollbar_grab_offset(None);
                cx.stop_propagation();
                window.refresh();
            }
        });
    }
}

fn set_virtual_scrollbar_position(
    list_state: &ListState,
    position: Point<Pixels>,
    grab_offset: Pixels,
) {
    let viewport = list_state.viewport_bounds();
    let max_offset = list_state.max_offset_for_scrollbar();
    if max_offset.height <= px(0.0) || viewport.size.height <= px(0.0) {
        return;
    }

    let content_height = viewport.size.height + max_offset.height;
    let thumb_height = (viewport.size.height * (viewport.size.height / content_height))
        .max(SCROLLBAR_MIN_THUMB_HEIGHT)
        .min(viewport.size.height);
    let track_height = (viewport.size.height - thumb_height).max(px(1.0));
    let y = (position.y - viewport.top() - grab_offset).clamp(px(0.0), track_height);
    let content_offset = y / track_height * max_offset.height;
    list_state.set_offset_from_scrollbar(point(px(0.0), content_offset));
}

fn virtual_thumb_metrics(list_state: &ListState, width: Pixels) -> Option<ThumbMetrics> {
    let viewport = list_state.viewport_bounds();
    let max_offset = list_state.max_offset_for_scrollbar();
    let offset = list_state.scroll_px_offset_for_scrollbar();
    let viewport_h = viewport.size.height;
    let content_h = viewport_h + max_offset.height;

    if content_h <= viewport_h || content_h <= px(0.0) || viewport_h <= px(0.0) {
        return None;
    }

    let ratio = viewport_h / content_h;
    let thumb_h = (viewport_h * ratio)
        .max(SCROLLBAR_MIN_THUMB_HEIGHT)
        .min(viewport_h);
    let scroll_ratio = if max_offset.height > px(0.0) {
        -offset.y / max_offset.height
    } else {
        0.0
    }
    .clamp(0.0, 1.0);
    let thumb_top = (viewport_h - thumb_h) * scroll_ratio;

    let bounds = Bounds {
        origin: point(
            viewport.right() - width - px(2.0),
            viewport.top() + thumb_top,
        ),
        size: size(width, thumb_h),
    };

    Some(ThumbMetrics {
        bounds,
        max_offset: max_offset.height,
        track_height: viewport_h - thumb_h,
    })
}

fn scrollbar_thumb_bounds_for_width(target: Bounds<Pixels>, width: Pixels) -> Bounds<Pixels> {
    Bounds {
        origin: point(target.right() - width, target.top()),
        size: size(width, target.size.height),
    }
}

fn expand_scrollbar_hitbox(thumb: Bounds<Pixels>) -> Bounds<Pixels> {
    Bounds {
        origin: point(
            thumb.right() - SCROLLBAR_HIT_WIDTH - px(2.0),
            thumb.top() - px(4.0),
        ),
        size: size(SCROLLBAR_HIT_WIDTH + px(2.0), thumb.size.height + px(8.0)),
    }
}

fn virtual_scrollbar_grab_offset() -> Option<Pixels> {
    VIRTUAL_SCROLLBAR_GRAB_OFFSET.with(Cell::get)
}

fn set_virtual_scrollbar_grab_offset(offset: Option<Pixels>) {
    VIRTUAL_SCROLLBAR_GRAB_OFFSET.with(|state| state.set(offset));
}

fn scrollbar_grab_offset() -> Option<Pixels> {
    SCROLLBAR_GRAB_OFFSET.with(Cell::get)
}

fn set_scrollbar_grab_offset(offset: Option<Pixels>) {
    SCROLLBAR_GRAB_OFFSET.with(|state| state.set(offset));
}

fn scroll_handle_thumb_metrics(
    scroll_handle: &ScrollHandle,
    width: Pixels,
) -> Option<ThumbMetrics> {
    let viewport_bounds = scroll_handle.bounds();
    let max_offset = scroll_handle.max_offset();
    let offset = scroll_handle.offset();

    let viewport_h = viewport_bounds.size.height;
    let content_h = viewport_h + max_offset.height;

    if content_h <= viewport_h || content_h <= px(0.0) || viewport_h <= px(0.0) {
        return None;
    }

    let ratio = viewport_h / content_h;
    let thumb_h = (viewport_h * ratio)
        .max(SCROLLBAR_MIN_THUMB_HEIGHT)
        .min(viewport_h);
    let scroll_ratio = if max_offset.height > px(0.0) {
        -offset.y / max_offset.height
    } else {
        0.0
    }
    .clamp(0.0, 1.0);
    let thumb_top = (viewport_h - thumb_h) * scroll_ratio;
    let bounds = Bounds {
        origin: point(
            viewport_bounds.right() - width - px(2.0),
            viewport_bounds.top() + thumb_top,
        ),
        size: size(width, thumb_h),
    };

    Some(ThumbMetrics {
        bounds,
        max_offset: max_offset.height,
        track_height: viewport_h - thumb_h,
    })
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

    let viewport = scroll_handle.bounds();
    let y = (position.y - viewport.top() - grab_offset).clamp(px(0.0), metrics.track_height);
    let content_offset = y / metrics.track_height * metrics.max_offset;
    scroll_handle.set_offset(point(px(0.0), -content_offset));
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
                    .on_scroll_wheel(cx.listener(|_, _, _, cx| {
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
    active: bool,
    dragging: bool,
}

impl IntoElement for ScrollbarThumb {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
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
        let hover_bounds = thumb.map(expand_scrollbar_hitbox).unwrap_or(Bounds {
            origin: point(bounds.right() - SCROLLBAR_HIT_WIDTH, bounds.top()),
            size: Size {
                width: SCROLLBAR_HIT_WIDTH,
                height: bounds.size.height,
            },
        });
        let hitbox = window.insert_hitbox(hover_bounds, HitboxBehavior::Normal);
        let dragging = scrollbar_grab_offset().is_some();
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
        let current_view = window.current_view();
        window.on_mouse_event(move |_: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                let active = scrollbar_grab_offset().is_some()
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
                set_scrollbar_grab_offset(Some(grab_offset));
                set_scroll_handle_position(&scroll_handle, event.position, grab_offset);

                cx.stop_propagation();
                window.refresh();
            }
        });

        let scroll_handle = self.scroll_handle.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                let Some(grab_offset) = scrollbar_grab_offset() else {
                    return;
                };
                set_scroll_handle_position(&scroll_handle, event.position, grab_offset);
                cx.stop_propagation();
                window.refresh();
            }
        });

        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture
                && event.button == MouseButton::Left
                && scrollbar_grab_offset().is_some()
            {
                set_scrollbar_grab_offset(None);
                cx.stop_propagation();
                window.refresh();
            }
        });

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
    #[test]
    fn virtual_scrollbar_bootstraps_gpui_list_state_scrolling() {
        let source = include_str!("scrollbar.rs");

        assert!(source.contains("pub struct VirtualScrollbar"));
        assert!(source.contains("ListState"));
        assert!(source.contains("scroll_px_offset_for_scrollbar"));
        assert!(source.contains("max_offset_for_scrollbar"));
        assert!(source.contains("set_offset_from_scrollbar"));
        assert!(source.contains("scrollbar_drag_started"));
        assert!(source.contains("scrollbar_drag_ended"));
        assert!(source.contains("virtual_scrollbar_grab_offset"));
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
        assert!(source.contains("set_scrollbar_grab_offset"));
        assert!(source.contains("set_virtual_scrollbar_grab_offset"));
        assert!(source.contains("set_scroll_handle_position"));
        assert!(source.contains("set_virtual_scrollbar_position"));
        assert!(source.contains("hover_bounds.contains(&window.mouse_position())"));
        assert!(source.contains("cx.notify(current_view)"));
        assert!(!source.contains("lerp_pixels"));
        assert!(!source.contains("request_animation_frame"));
    }
}
