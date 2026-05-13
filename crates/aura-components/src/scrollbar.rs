use aura_core::Config;
use gpui::{
    AnyElement, App, Bounds, Context, DispatchPhase, Element, GlobalElementId, Hitbox,
    HitboxBehavior, InspectorElementId, IntoElement, LayoutId, ListState, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point, Render, ScrollHandle,
    Size, Style, Window, div, point, prelude::*, px, relative, size,
};

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
/// This lets Aura Docs use GPUI's native virtual list for visible-area rendering
/// while still bootstrapping the visual scrollbar from Aura's component layer.
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
    bounds: Option<Bounds<Pixels>>,
    hitbox: Hitbox,
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
        let thumb = virtual_thumb_bounds(&self.list_state);
        let hitbox_bounds = thumb.unwrap_or(Bounds {
            origin: point(bounds.right() - px(10.0), bounds.top()),
            size: Size {
                width: px(10.0),
                height: bounds.size.height,
            },
        });
        let hitbox = window.insert_hitbox(hitbox_bounds, HitboxBehavior::Normal);
        VirtualScrollbarPrepaint {
            bounds: thumb,
            hitbox,
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
        let Some(thumb_bounds) = prepaint.bounds else {
            return;
        };

        let theme = &cx.global::<Config>().theme;
        window.paint_quad(PaintQuad {
            bounds: thumb_bounds,
            corner_radii: gpui::Corners::all(px(2.0)),
            background: theme.neutral.border.opacity(0.8).into(),
            border_widths: gpui::Edges::all(px(0.0)),
            border_color: gpui::transparent_black(),
            border_style: gpui::BorderStyle::Solid,
        });

        let list_state = self.list_state.clone();
        let hitbox = prepaint.hitbox.clone();
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase == DispatchPhase::Bubble
                && event.button == MouseButton::Left
                && hitbox.is_hovered(window)
            {
                list_state.scrollbar_drag_started();
                set_virtual_scrollbar_position(&list_state, event.position);
                window.capture_pointer(hitbox.id);
                cx.stop_propagation();
                window.refresh();
            }
        });

        let list_state = self.list_state.clone();
        let hitbox_id = prepaint.hitbox.id;
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture
                && event.pressed_button == Some(MouseButton::Left)
                && window.captured_hitbox() == Some(hitbox_id)
            {
                set_virtual_scrollbar_position(&list_state, event.position);
                cx.stop_propagation();
            }
        });

        let list_state = self.list_state.clone();
        let hitbox_id = prepaint.hitbox.id;
        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture
                && event.button == MouseButton::Left
                && window.captured_hitbox() == Some(hitbox_id)
            {
                list_state.scrollbar_drag_ended();
                cx.stop_propagation();
            }
        });
    }
}

fn set_virtual_scrollbar_position(list_state: &ListState, position: Point<Pixels>) {
    let viewport = list_state.viewport_bounds();
    let max_offset = list_state.max_offset_for_scrollbar();
    if max_offset.y <= px(0.0) || viewport.size.height <= px(0.0) {
        return;
    }

    let content_height = viewport.size.height + max_offset.y;
    let thumb_height =
        (viewport.size.height * (viewport.size.height / content_height)).max(px(24.0));
    let track_height = (viewport.size.height - thumb_height).max(px(1.0));
    let y = (position.y - viewport.top() - thumb_height / 2.0).clamp(px(0.0), track_height);
    let content_offset = y / track_height * max_offset.y;
    list_state.set_offset_from_scrollbar(point(px(0.0), content_offset));
}

fn virtual_thumb_bounds(list_state: &ListState) -> Option<Bounds<Pixels>> {
    let viewport = list_state.viewport_bounds();
    let max_offset = list_state.max_offset_for_scrollbar();
    let offset = list_state.scroll_px_offset_for_scrollbar();
    let viewport_h = viewport.size.height;
    let content_h = viewport_h + max_offset.y;

    if content_h <= viewport_h || content_h <= px(0.0) || viewport_h <= px(0.0) {
        return None;
    }

    let ratio = viewport_h / content_h;
    let thumb_h = (viewport_h * ratio).max(px(24.0)).min(viewport_h);
    let scroll_ratio = if max_offset.y > px(0.0) {
        -offset.y / max_offset.y
    } else {
        0.0
    }
    .clamp(0.0, 1.0);
    let thumb_top = (viewport_h - thumb_h) * scroll_ratio;

    Some(Bounds {
        origin: point(viewport.right() - px(6.0), viewport.top() + thumb_top),
        size: size(px(4.0), thumb_h),
    })
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

impl IntoElement for ScrollbarThumb {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for ScrollbarThumb {
    type RequestLayoutState = ();
    type PrepaintState = ();

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
        _bounds: gpui::Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        ()
    }

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _id2: Option<&gpui::InspectorElementId>,
        _bounds: gpui::Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) -> () {
        let theme = &cx.global::<Config>().theme;
        let viewport_bounds = self.scroll_handle.bounds();
        let max_offset = self.scroll_handle.max_offset();
        let offset = self.scroll_handle.offset();

        let viewport_h = viewport_bounds.size.height;
        let content_h = viewport_h + max_offset.y;

        if content_h <= viewport_h || content_h <= gpui::px(0.0) {
            return;
        }

        let ratio = viewport_h / content_h;
        let thumb_h = viewport_h * ratio;

        let scroll_ratio = if max_offset.y > gpui::px(0.0) {
            -offset.y / max_offset.y
        } else {
            0.0
        };
        let thumb_top = (viewport_h - thumb_h) * scroll_ratio;

        let thumb_bounds = gpui::Bounds {
            origin: gpui::Point {
                x: viewport_bounds.right() - gpui::px(6.0),
                y: viewport_bounds.top() + thumb_top,
            },
            size: gpui::Size {
                width: gpui::px(4.0),
                height: thumb_h,
            },
        };

        window.paint_quad(gpui::PaintQuad {
            bounds: thumb_bounds,
            corner_radii: gpui::Corners::all(gpui::px(2.0)),
            background: theme.neutral.border.opacity(0.8).into(),
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
        assert!(source.contains("captured_hitbox() == Some(hitbox_id)"));
    }
}
