use aura_core::Config;
use gpui::{
    div, prelude::*, AnyElement, App, Context, IntoElement, Pixels, Render, ScrollHandle, Window,
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
            render_content: Box::new(move |window, cx| render_content(window, cx).into_any_element()),
            height: None,
        }
    }

    pub fn height(mut self, h: impl Into<Pixels>) -> Self {
        self.height = Some(h.into());
        self
    }
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
                    .child(content)
            )
            .child(
                ScrollbarThumb {
                    scroll_handle: self.scroll_handle.clone(),
                }
            )
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
