//! Input Number module.
//!
//! This public module implements the Liora numeric input component with increment/decrement controls. It keeps the reusable
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

use crate::Input;
use gpui::{
    prelude::*, px, App, Context, Entity, FocusHandle, Focusable, MouseButton, Render, Window,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control input number controls position behavior.
pub enum InputNumberControlsPosition {
    /// Lays out content in the horizontal direction.
    Horizontal,
    /// Places the overlay to the right of the anchor.
    Right,
}

/// Fluent native GPUI component for rendering Liora input number.
pub struct InputNumber {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    precision: usize,
    disabled: bool,
    controls_position: InputNumberControlsPosition,
    input: Entity<Input>,
    initialized: bool,
    on_change: Option<Box<dyn Fn(f64, &mut Window, &mut App) + 'static>>,
}

impl InputNumber {
    /// Creates `InputNumber` initialized from the supplied value.
    pub fn new(value: f64, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            Input::new(format!("{:.*}", 0, value), cx).filter(|text| {
                if text.is_empty() {
                    return true;
                }
                let Some(first) = text.chars().next() else {
                    return true;
                };
                if first == '+' || first == '-' {
                    let rest: String = text.chars().skip(1).collect();
                    if rest.is_empty() {
                        return true;
                    }
                    rest.chars().all(|c| c.is_ascii_digit() || c == '.')
                        && rest.matches('.').count() <= 1
                } else {
                    text.chars().all(|c| c.is_ascii_digit() || c == '.')
                        && text.matches('.').count() <= 1
                }
            })
        });

        Self {
            value,
            min: f64::MIN,
            max: f64::MAX,
            step: 1.0,
            precision: 0,
            disabled: false,
            controls_position: InputNumberControlsPosition::Horizontal,
            input,
            initialized: false,
            on_change: None,
        }
    }

    /// Sets the lower numeric boundary.
    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }
    /// Sets the upper numeric boundary.
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }
    /// Sets the increment used by numeric or time controls.
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }
    /// Sets the precision value used by the component.
    pub fn precision(mut self, p: usize) -> Self {
        self.precision = p;
        self
    }
    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool, cx: &mut Context<Self>) -> Self {
        self.disabled = d;
        self.input.update(cx, |input, cx| {
            input.set_disabled(d, cx);
        });
        self
    }
    /// Sets the controls position value used by the component.
    pub fn controls_position(mut self, pos: InputNumberControlsPosition) -> Self {
        self.controls_position = pos;
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, cb: impl Fn(f64, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    fn set_numeric_value(&mut self, val: f64, window: &mut Window, cx: &mut Context<Self>) {
        let scale = 10_f64.powi(self.precision.min(308) as i32);
        // Use decimal values inside the bounds; when none exist retain exact bounds and display.
        let min = (self.min * scale).ceil() / scale;
        let min = if min.is_finite() { min } else { self.min };
        let max = (self.max * scale).floor() / scale;
        let max = if max.is_finite() { max } else { self.max };
        let val = if min.is_finite() && max.is_finite() && min <= max {
            let rounded = format!("{:.*}", self.precision, val.clamp(min, max))
                .parse::<f64>()
                .unwrap_or(self.value);
            rounded.clamp(min, max)
        } else {
            val.clamp(self.min, self.max)
        };
        if val != self.value {
            self.value = val;
            if let Some(cb) = &self.on_change {
                cb(val, window, cx);
            }
            cx.notify();
        }
    }

    fn set_value(&mut self, val: f64, window: &mut Window, cx: &mut Context<Self>) {
        self.set_numeric_value(val, window, cx);
        let rounded = format!("{:.*}", self.precision, self.value);
        let formatted = if rounded.parse::<f64>().ok() == Some(self.value) {
            rounded
        } else {
            self.value.to_string()
        };
        self.input
            .update(cx, |input, cx| input.set_value(formatted, cx));
    }

    fn reconcile_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let value = self.input.read(cx).value().parse::<f64>();
        if let Ok(value) = value {
            if !value.is_finite() {
                return;
            }
            self.set_numeric_value(value, window, cx);
        }
    }

    fn increment(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.reconcile_input(window, cx);
        }
        if !self.disabled && self.value < self.max {
            self.set_value(self.value + self.step, window, cx);
        }
    }

    fn decrement(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.reconcile_input(window, cx);
        }
        if !self.disabled && self.value > self.min {
            self.set_value(self.value - self.step, window, cx);
        }
    }
}

impl Focusable for InputNumber {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.read(cx).focus_handle(cx)
    }
}

impl Render for InputNumber {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        if !self.initialized {
            self.set_value(self.value, window, cx);
            let owner = cx.weak_entity();
            let handle = window.window_handle();
            self.input.update(cx, |input, _| {
                input.set_on_change(move |text, cx| {
                    let Ok(value) = text.parse::<f64>() else {
                        return;
                    };
                    if !value.is_finite() {
                        return;
                    }
                    let owner = owner.clone();
                    let text = text.to_owned();
                    // Defer until the Input change callback returns, before mutating its owner.
                    cx.defer(move |cx| {
                        let _ = handle.update(cx, |_, window, cx| {
                            let _ = owner.update(cx, |number, cx| {
                                if !number.disabled
                                    && number.input.read(cx).value().as_ref() == text
                                {
                                    number.set_numeric_value(value, window, cx);
                                }
                            });
                        });
                    });
                });
            });
            let focus = self.input.read(cx).focus_handle(cx);
            cx.on_blur(&focus, window, |number, window, cx| {
                number.reconcile_input(window, cx);
                number.set_value(number.value, window, cx);
            })
            .detach();
            self.initialized = true;
        }

        match self.controls_position {
            InputNumberControlsPosition::Horizontal => {
                self.render_horizontal(&theme, cx).into_any_element()
            }
            InputNumberControlsPosition::Right => self.render_right(&theme, cx).into_any_element(),
        }
    }
}

impl InputNumber {
    fn render_horizontal(
        &self,
        theme: &liora_theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let icon_sz = 12.0;
        let can_inc = !self.disabled && self.value < self.max;
        let can_dec = !self.disabled && self.value > self.min;

        let mut row = gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(34.0))
            .rounded(px(theme.radius.md))
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .overflow_hidden();

        let mut dec_btn = gpui::div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(32.0))
            .h_full()
            .bg(theme.neutral.hover)
            .border_color(theme.neutral.border)
            .border_r_1();

        if can_dec {
            dec_btn = dec_btn
                .cursor_pointer()
                .hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        this.decrement(window, cx);
                    }),
                );
        } else {
            dec_btn = dec_btn.cursor_not_allowed().opacity(0.5);
        }

        row = row.child(
            dec_btn.child(
                Icon::new(IconName::Minus)
                    .size(px(icon_sz))
                    .color(if can_dec {
                        theme.neutral.text_1
                    } else {
                        theme.neutral.text_disabled
                    }),
            ),
        );
        row = row.child(gpui::div().flex_1().child(self.input.clone()));

        let mut inc_btn = gpui::div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(32.0))
            .h_full()
            .bg(theme.neutral.hover)
            .border_color(theme.neutral.border)
            .border_l_1();

        if can_inc {
            inc_btn = inc_btn
                .cursor_pointer()
                .hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        this.increment(window, cx);
                    }),
                );
        } else {
            inc_btn = inc_btn.cursor_not_allowed().opacity(0.5);
        }

        row = row.child(
            inc_btn.child(
                Icon::new(IconName::Plus)
                    .size(px(icon_sz))
                    .color(if can_inc {
                        theme.neutral.text_1
                    } else {
                        theme.neutral.text_disabled
                    }),
            ),
        );

        row
    }

    fn render_right(&self, theme: &liora_theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let icon_sz = 10.0;
        let can_inc = !self.disabled && self.value < self.max;
        let can_dec = !self.disabled && self.value > self.min;

        let mut row = gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(34.0))
            .rounded(px(theme.radius.md))
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .overflow_hidden();

        row = row.child(gpui::div().flex_1().child(self.input.clone()));

        let mut controls = gpui::div()
            .flex()
            .flex_col()
            .w(px(32.0))
            .h_full()
            .border_color(theme.neutral.border)
            .border_l_1();

        let mut inc_btn = gpui::div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .bg(theme.neutral.hover)
            .border_color(theme.neutral.border)
            .border_b_1();

        if can_inc {
            inc_btn = inc_btn
                .cursor_pointer()
                .hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        this.increment(window, cx);
                    }),
                );
        } else {
            inc_btn = inc_btn.cursor_not_allowed().opacity(0.5);
        }

        let mut dec_btn = gpui::div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .bg(theme.neutral.hover);

        if can_dec {
            dec_btn = dec_btn
                .cursor_pointer()
                .hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        this.decrement(window, cx);
                    }),
                );
        } else {
            dec_btn = dec_btn.cursor_not_allowed().opacity(0.5);
        }

        controls = controls.child(
            inc_btn.child(
                Icon::new(IconName::ChevronUp)
                    .size(px(icon_sz))
                    .color(if can_inc {
                        theme.neutral.text_1
                    } else {
                        theme.neutral.text_disabled
                    }),
            ),
        );
        controls =
            controls.child(
                dec_btn.child(Icon::new(IconName::ChevronDown).size(px(icon_sz)).color(
                    if can_dec {
                        theme.neutral.text_1
                    } else {
                        theme.neutral.text_disabled
                    },
                )),
            );

        row = row.child(controls);

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{size, EntityInputHandler, TestAppContext};
    use std::{cell::RefCell, rc::Rc};

    struct Surface(Entity<InputNumber>);
    impl Render for Surface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            gpui::div().w_full().child(self.0.clone())
        }
    }

    #[test]
    fn manual_drafts_update_numeric_steps_without_rewriting_text_or_duplicate_zero() {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let changes = Rc::new(RefCell::new(Vec::new()));
        let recorded = changes.clone();
        let number = app.new(|cx| {
            InputNumber::new(1.23, cx)
                .precision(2)
                .step(0.01)
                .min(-10.0)
                .max(10.0)
                .on_change(move |value, _, _| recorded.borrow_mut().push(value))
        });
        let handle = app.open_window(size(px(400.), px(200.)), |_, _| Surface(number.clone()));
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
        })
        .unwrap();
        for text in ["-", ".", "2.", "2.345", "0", "0"] {
            app.update_window(handle.into(), |_, window, cx| {
                let input = number.read(cx).input.clone();
                input.update(cx, |input, cx| {
                    let len = input.value().encode_utf16().count();
                    input.replace_text_in_range(Some(0..len), text, window, cx);
                });
            })
            .unwrap();
            app.update_window(handle.into(), |_, _, cx| {
                assert_eq!(number.read(cx).input.read(cx).value().as_ref(), text);
            })
            .unwrap();
        }
        assert_eq!(*changes.borrow(), vec![2.0, 2.35, 0.0]);
        app.update_window(handle.into(), |_, window, cx| {
            number.update(cx, |number, cx| number.increment(window, cx));
            assert_eq!(number.read(cx).value, 0.01);
            assert_eq!(number.read(cx).input.read(cx).value().as_ref(), "0.01");
        })
        .unwrap();
    }

    #[test]
    fn newer_replacement_wins_over_deferred_edit_and_bounds_match_display() {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let number = app.new(|cx| {
            InputNumber::new(0.01, cx)
                .precision(2)
                .step(0.01)
                .min(0.005)
                .max(0.025)
        });
        let handle = app.open_window(size(px(400.), px(200.)), |_, _| Surface(number.clone()));
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            let input = number.read(cx).input.clone();
            input.update(cx, |input, cx| {
                let len = input.value().encode_utf16().count();
                input.replace_text_in_range(Some(0..len), "0.015", window, cx);
            });
            number.update(cx, |number, cx| number.increment(window, cx));
            assert_eq!(number.read(cx).value, 0.02);
            number.update(cx, |number, cx| number.set_value(0.01, window, cx));
            input.update(cx, |input, cx| {
                let len = input.value().encode_utf16().count();
                input.replace_text_in_range(Some(0..len), "0.005", window, cx);
            });
            number.update(cx, |number, cx| number.set_value(0.02, window, cx));
        })
        .unwrap();
        app.update_window(handle.into(), |_, window, cx| {
            assert_eq!(number.read(cx).value, 0.02);
            assert_eq!(number.read(cx).input.read(cx).value().as_ref(), "0.02");
            number.update(cx, |number, cx| number.decrement(window, cx));
            assert_eq!(number.read(cx).value, 0.01);
            number.update(cx, |number, cx| number.decrement(window, cx));
            assert_eq!(number.read(cx).value, 0.01);
            assert_eq!(number.read(cx).input.read(cx).value().as_ref(), "0.01");
            number.update(cx, |number, cx| {
                number.min = 0.005;
                number.max = 0.006;
                number.set_value(0.005, window, cx);
            });
            assert_eq!(
                number
                    .read(cx)
                    .input
                    .read(cx)
                    .value()
                    .parse::<f64>()
                    .unwrap(),
                number.read(cx).value
            );
        })
        .unwrap();
    }
}
