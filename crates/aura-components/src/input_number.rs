use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{prelude::*, px, App, Render, Window, Context, Focusable, FocusHandle, Entity, MouseButton, AnyElement};
use crate::Input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputNumberControlsPosition {
    Horizontal,
    Right,
}

pub struct InputNumber {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    precision: usize,
    disabled: bool,
    controls_position: InputNumberControlsPosition,
    input: Entity<Input>,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(f64, &mut Window, &mut App) + 'static>>,
}

impl InputNumber {
    pub fn new(value: f64, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            Input::new(format!("{:.*}", 0, value), cx)
                .filter(|text| {
                    // Only allow digits, one decimal point, and leading minus sign
                    text.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-')
                })
        });

        let focus_handle = cx.focus_handle();
        
        Self {
            value,
            min: f64::MIN,
            max: f64::MAX,
            step: 1.0,
            precision: 0,
            disabled: false,
            controls_position: InputNumberControlsPosition::Horizontal,
            input,
            focus_handle,
            on_change: None,
        }
    }

    pub fn min(mut self, min: f64) -> Self { self.min = min; self }
    pub fn max(mut self, max: f64) -> Self { self.max = max; self }
    pub fn step(mut self, step: f64) -> Self { self.step = step; self }
    pub fn precision(mut self, p: usize) -> Self { 
        self.precision = p; 
        self
    }
    pub fn disabled(mut self, d: bool, cx: &mut Context<Self>) -> Self {
        self.disabled = d;
        self.input.update(cx, |input, cx| { input.set_disabled(d, cx); });
        self
    }
    pub fn controls_position(mut self, pos: InputNumberControlsPosition) -> Self {
        self.controls_position = pos;
        self
    }

    pub fn on_change(mut self, cb: impl Fn(f64, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    fn set_value(&mut self, val: f64, window: &mut Window, cx: &mut Context<Self>) {
        let val = val.clamp(self.min, self.max);
        if (val - self.value).abs() > f64::EPSILON || self.value == 0.0 {
            self.value = val;
            let formatted = format!("{:.*}", self.precision, self.value);
            self.input.update(cx, |input, cx| {
                input.set_value(formatted, cx);
            });
            if let Some(ref cb) = self.on_change {
                cb(self.value, window, cx);
            }
            cx.notify();
        }
    }

    fn increment(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled && self.value < self.max {
            self.set_value(self.value + self.step, window, cx);
        }
    }

    fn decrement(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled && self.value > self.min {
            self.set_value(self.value - self.step, window, cx);
        }
    }
}

impl Focusable for InputNumber {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl Render for InputNumber {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        
        match self.controls_position {
            InputNumberControlsPosition::Horizontal => self.render_horizontal(&theme, cx).into_any_element(),
            InputNumberControlsPosition::Right => self.render_right(&theme, cx).into_any_element(),
        }
    }
}

impl InputNumber {
    fn render_horizontal(&self, theme: &aura_theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let icon_sz = 12.0;
        let can_inc = !self.disabled && self.value < self.max;
        let can_dec = !self.disabled && self.value > self.min;

        let mut row = gpui::div()
            .flex().flex_row().items_center()
            .h(px(34.0))
            .rounded(px(theme.radius.md))
            .border_1().border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .overflow_hidden();

        // Decrement button
        let mut dec_btn = gpui::div()
            .flex().items_center().justify_center()
            .w(px(32.0)).h_full()
            .bg(theme.neutral.hover)
            .border_color(theme.neutral.border).border_r_1();

        if can_dec {
            dec_btn = dec_btn.cursor_pointer().hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    this.decrement(window, cx);
                }));
        } else {
            dec_btn = dec_btn.cursor_not_allowed().opacity(0.5);
        }
        
        row = row.child(dec_btn.child(Icon::new(IconName::Minus).size(px(icon_sz)).color(if can_dec { theme.neutral.text_1 } else { theme.neutral.text_disabled })));
        row = row.child(gpui::div().flex_1().child(self.input.clone()));

        // Increment button
        let mut inc_btn = gpui::div()
            .flex().items_center().justify_center()
            .w(px(32.0)).h_full()
            .bg(theme.neutral.hover)
            .border_color(theme.neutral.border).border_l_1();

        if can_inc {
            inc_btn = inc_btn.cursor_pointer().hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    this.increment(window, cx);
                }));
        } else {
            inc_btn = inc_btn.cursor_not_allowed().opacity(0.5);
        }

        row = row.child(inc_btn.child(Icon::new(IconName::Plus).size(px(icon_sz)).color(if can_inc { theme.neutral.text_1 } else { theme.neutral.text_disabled })));
        
        row
    }

    fn render_right(&self, theme: &aura_theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let icon_sz = 10.0;
        let can_inc = !self.disabled && self.value < self.max;
        let can_dec = !self.disabled && self.value > self.min;

        let mut row = gpui::div()
            .flex().flex_row().items_center()
            .h(px(34.0))
            .rounded(px(theme.radius.md))
            .border_1().border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .overflow_hidden();

        row = row.child(gpui::div().flex_1().child(self.input.clone()));

        let mut controls = gpui::div()
            .flex().flex_col()
            .w(px(32.0)).h_full()
            .border_color(theme.neutral.border).border_l_1();

        // Increment small button
        let mut inc_btn = gpui::div()
            .flex_1().flex().items_center().justify_center()
            .bg(theme.neutral.hover)
            .border_color(theme.neutral.border).border_b_1();

        if can_inc {
            inc_btn = inc_btn.cursor_pointer().hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    this.increment(window, cx);
                }));
        } else {
            inc_btn = inc_btn.cursor_not_allowed().opacity(0.5);
        }

        // Decrement small button
        let mut dec_btn = gpui::div()
            .flex_1().flex().items_center().justify_center()
            .bg(theme.neutral.hover);

        if can_dec {
            dec_btn = dec_btn.cursor_pointer().hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    this.decrement(window, cx);
                }));
        } else {
            dec_btn = dec_btn.cursor_not_allowed().opacity(0.5);
        }

        controls = controls.child(inc_btn.child(Icon::new(IconName::ChevronUp).size(px(icon_sz)).color(if can_inc { theme.neutral.text_1 } else { theme.neutral.text_disabled })));
        controls = controls.child(dec_btn.child(Icon::new(IconName::ChevronDown).size(px(icon_sz)).color(if can_dec { theme.neutral.text_1 } else { theme.neutral.text_disabled })));

        row = row.child(controls);
        
        row
    }
}
