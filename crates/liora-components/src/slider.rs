//! Slider module.
//!
//! This public module implements the Liora slider input component for numeric ranges. It keeps the reusable
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
    fill, point, prelude::*, px, relative, size, App, Bounds, Context, Element, ElementId, Entity,
    FocusHandle, Focusable, GlobalElementId, InspectorElementId, LayoutId, MouseButton,
    MouseMoveEvent, Pixels, Point, Render, Style, Window,
};
use liora_core::Config;

/// Fluent native GPUI component for rendering Liora slider.
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
    /// Creates `Slider` initialized from the supplied value.
    pub fn new(value: f64, cx: &mut Context<Self>) -> Self {
        Self {
            value: if value.is_finite() { value } else { 0.0 },
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

    /// Sets the lower numeric boundary.
    pub fn min(mut self, min: f64) -> Self {
        if min.is_finite() {
            self.min = min;
        }
        self
    }
    /// Sets the upper numeric boundary.
    pub fn max(mut self, max: f64) -> Self {
        if max.is_finite() {
            self.max = max;
        }
        self
    }
    /// Sets the increment used by numeric or time controls.
    pub fn step(mut self, step: f64) -> Self {
        if step.is_finite() && step > 0.0 {
            self.step = step;
        }
        self
    }
    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, cb: impl Fn(f64, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    fn normalized_value(&self, value: f64) -> f64 {
        let min = self.min.min(self.max);
        let max = self.min.max(self.max);
        if value.is_nan() || value <= min {
            return min;
        }
        if value >= max {
            return max;
        }
        let distance = value - min;
        let steps = distance / self.step;
        if distance.is_finite() && steps.is_finite() {
            return (min + steps.round() * self.step).clamp(min, max);
        }
        // Remainders retain the min-origin grid without overflowing distance or step count.
        let remainder =
            (value.rem_euclid(self.step) - min.rem_euclid(self.step)).rem_euclid(self.step);
        if remainder == 0.0 {
            value
        } else if remainder < self.step - remainder {
            (value - remainder).clamp(min, max)
        } else {
            (value + (self.step - remainder)).clamp(min, max)
        }
    }

    fn range_fraction(value: f64, min: f64, max: f64) -> f64 {
        if min == max {
            return 0.0;
        }
        let span = max - min;
        if span.is_finite() {
            ((value - min) / span).clamp(0.0, 1.0)
        } else {
            ((value / 2.0 - min / 2.0) / (max / 2.0 - min / 2.0)).clamp(0.0, 1.0)
        }
    }

    fn set_value(&mut self, val: f64, window: &mut Window, cx: &mut Context<Self>) {
        let val = self.normalized_value(val);
        if val != self.value {
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
        if let Some(bounds) = self
            .last_bounds
            .filter(|bounds| bounds.size.width > px(0.0))
        {
            let relative_x = pos.x - bounds.left();
            let width = bounds.size.width;
            let percentage = (relative_x / width).clamp(0.0, 1.0);
            let min = self.min.min(self.max);
            let max = self.min.max(self.max);
            let val = min * (1.0 - percentage as f64) + max * percentage as f64;
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
        let min = slider.min.min(slider.max);
        let max = slider.min.max(slider.max);
        let percentage = Slider::range_fraction(slider.value, min, max);

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
        self.value = self.normalized_value(self.value);
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[test]
    fn snapping_uses_range_origin_and_keeps_endpoints_reachable() {
        let mut app = TestAppContext::single();
        let slider = app.new(|cx| Slider::new(3.0, cx).min(3.0).max(10.0).step(4.0));
        app.update(|cx| {
            let slider = slider.read(cx);
            assert_eq!(slider.normalized_value(3.0), 3.0);
            assert_eq!(slider.normalized_value(6.0), 7.0);
            assert_eq!(slider.normalized_value(9.5), 10.0);
            assert_eq!(slider.normalized_value(10.0), 10.0);
            assert_eq!(slider.normalized_value(-100.0), 3.0);
        });
    }

    #[test]
    fn extreme_finite_ranges_keep_midpoint_and_values_in_range() {
        let mut app = TestAppContext::single();
        let slider = app.new(|cx| Slider::new(0.0, cx).min(-f64::MAX).max(f64::MAX));
        app.update(|cx| {
            let slider = slider.read(cx);
            assert_eq!(slider.normalized_value(0.0), 0.0);
            assert_eq!(Slider::range_fraction(0.0, -f64::MAX, f64::MAX), 0.5);
            assert_eq!(slider.normalized_value(1e308), 1e308);
            assert!(
                (Slider::range_fraction(1e308, -f64::MAX, f64::MAX) - 0.7781342323134002).abs()
                    < 1e-15
            );
            assert_eq!(slider.normalized_value(0.0), 0.0);
        });
    }

    #[test]
    fn overflowing_step_ratio_and_reversed_huge_range_remain_finite() {
        let mut app = TestAppContext::single();
        for step in [1.0, f64::from_bits(1)] {
            let slider = app.new(|cx| Slider::new(0.0, cx).min(f64::MAX).max(-f64::MAX).step(step));
            app.update(|cx| {
                let slider = slider.read(cx);
                assert_eq!(slider.normalized_value(0.0), 0.0);
                assert_eq!(slider.normalized_value(1e308), 1e308);
                assert_eq!(slider.normalized_value(f64::MAX), f64::MAX);
                assert_eq!(slider.normalized_value(-f64::MAX), -f64::MAX);
            });
        }
    }

    #[test]
    fn subnormal_step_rounds_to_nearest_when_step_count_overflows() {
        let mut app = TestAppContext::single();
        let unit = f64::from_bits(1);
        let slider = app.new(|cx| Slider::new(0.0, cx).min(-1.0).max(1.0).step(unit * 3.0));
        app.update(|cx| {
            let slider = slider.read(cx);
            let origin = (-1.0_f64).rem_euclid(unit * 3.0);
            for bits in 0..9 {
                let value = f64::from_bits(bits);
                let remainder = (value.rem_euclid(unit * 3.0) - origin).rem_euclid(unit * 3.0);
                let expected = if remainder <= unit {
                    value - remainder
                } else {
                    value + (unit * 3.0 - remainder)
                };
                assert_eq!(slider.normalized_value(value), expected);
            }
        });
    }

    #[test]
    fn tiny_value_change_emits_once_and_repeated_value_is_noop() {
        use std::{cell::RefCell, rc::Rc};
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let changes = Rc::new(RefCell::new(Vec::new()));
        let recorded = changes.clone();
        let slider = app.new(|cx| {
            Slider::new(0.0, cx)
                .step(f64::from_bits(1))
                .on_change(move |value, _, _| recorded.borrow_mut().push(value))
        });
        struct Surface(Entity<Slider>);
        impl Render for Surface {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                gpui::div().w_full().child(self.0.clone())
            }
        }
        let handle = app.open_window(size(px(400.), px(100.)), |_, _| Surface(slider.clone()));
        app.update_window(handle.into(), |_, window, cx| {
            slider.update(cx, |slider, cx| {
                slider.set_value(f64::from_bits(1), window, cx);
                slider.set_value(f64::from_bits(1), window, cx);
            });
            assert_eq!(slider.read(cx).value, f64::from_bits(1));
        })
        .unwrap();
        assert_eq!(*changes.borrow(), vec![f64::from_bits(1)]);
    }

    #[test]
    fn invalid_configuration_has_finite_bounded_values() {
        let mut app = TestAppContext::single();
        for step in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let slider = app.new(|cx| Slider::new(f64::NAN, cx).min(10.0).max(3.0).step(step));
            app.update(|cx| {
                let slider = slider.read(cx);
                for value in [f64::NAN, f64::INFINITY, -10.0, 5.0, 100.0] {
                    let value = slider.normalized_value(value);
                    assert!(value.is_finite() && (3.0..=10.0).contains(&value));
                }
            });
        }
    }
}
