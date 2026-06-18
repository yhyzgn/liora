use gpui::{
    App, Bounds, Context, Element, ElementId, Entity, FocusHandle, Focusable, GlobalElementId,
    InspectorElementId, LayoutId, MouseButton, MouseMoveEvent, Pixels, Point, Render, Style,
    Window, fill, point, prelude::*, px, relative, size,
};
use liora_core::Config;

pub struct Slider {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    disabled: bool,
    focus_handle: FocusHandle,
    is_dragging: bool,
    last_bounds: Option<Bounds<Pixels>>,
    on_change: Option<Box<dyn Fn(f64, &mut Window, &mut App) + 'static>>,
}

impl Slider {
    pub fn new(value: f64, cx: &mut Context<Self>) -> Self {
        Self {
            value,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            focus_handle: cx.focus_handle(),
            is_dragging: false,
            last_bounds: None,
            on_change: None,
        }
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn on_change(mut self, cb: impl Fn(f64, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    fn set_value(&mut self, val: f64, window: &mut Window, cx: &mut Context<Self>) {
        let val = val.clamp(self.min, self.max);
        let val = (val / self.step).round() * self.step;
        if (val - self.value).abs() > f64::EPSILON {
            self.value = val;
            if let Some(ref cb) = self.on_change {
                cb(self.value, window, cx);
            }
            cx.notify();
        }
    }

    fn handle_mouse_down(
        &mut self,
        event: &gpui::MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        window.focus(&self.focus_handle, cx);
        self.is_dragging = true;
        self.update_value_from_mouse(event.position, window, cx);
    }

    fn handle_mouse_up(&mut self, _: &gpui::MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.is_dragging = false;
        cx.notify();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_dragging && !self.disabled {
            self.update_value_from_mouse(event.position, window, cx);
        }
    }

    fn update_value_from_mouse(
        &mut self,
        pos: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self.last_bounds {
            let relative_x = pos.x - bounds.left();
            let width = bounds.size.width;
            let percentage = (relative_x / width).clamp(0.0, 1.0);
            let val = self.min + percentage as f64 * (self.max - self.min);
            self.set_value(val, window, cx);
        }
    }
}

impl Focusable for Slider {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct SliderElement {
    slider: Entity<Slider>,
}

impl IntoElement for SliderElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for SliderElement {
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
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = px(16.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        cx: &mut App,
    ) -> () {
        self.slider.update(cx, |this, _| {
            this.last_bounds = Some(bounds);
        });
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let slider = self.slider.read(cx);
        let theme = &cx.global::<Config>().theme;
        let percentage = (slider.value - slider.min) / (slider.max - slider.min);

        let h = 6.0;
        let thumb_sz = 16.0;

        let track_bg = theme.neutral.hover;
        let active_bg = if slider.disabled {
            theme.neutral.border
        } else {
            theme.primary.base
        };
        let thumb_bg = if slider.disabled {
            theme.neutral.text_disabled
        } else {
            theme.primary.base
        };

        // Paint track
        let track_bounds = Bounds::new(
            point(
                bounds.left(),
                bounds.top() + (bounds.size.height - px(h)) / 2.0,
            ),
            size(bounds.size.width, px(h)),
        );
        window.paint_quad(fill(track_bounds, track_bg));

        // Paint active track
        let active_bounds = Bounds::new(
            track_bounds.origin,
            size(bounds.size.width * percentage as f32, px(h)),
        );
        window.paint_quad(fill(active_bounds, active_bg));

        // Paint thumb
        let thumb_origin = point(
            bounds.left() + bounds.size.width * percentage as f32 - px(thumb_sz / 2.0),
            bounds.top() + (bounds.size.height - px(thumb_sz)) / 2.0,
        );
        let thumb_bounds = Bounds::new(thumb_origin, size(px(thumb_sz), px(thumb_sz)));
        window.paint_quad(fill(thumb_bounds, thumb_bg));
    }
}

impl Render for Slider {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fh = self.focus_handle.clone();

        gpui::div()
            .relative()
            .w_full()
            .h(px(16.0))
            .track_focus(&fh)
            .when(!self.disabled, |s| s.cursor_pointer())
            .when(self.disabled, |s| s.cursor_not_allowed())
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .child(SliderElement {
                slider: cx.entity().clone(),
            })
    }
}
