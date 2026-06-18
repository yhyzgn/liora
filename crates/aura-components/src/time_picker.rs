use crate::motion::pop_in;
use aura_core::{Config, push_portal};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, Bounds, Context, Element, ElementId, Entity, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, MouseButton, Pixels, Render, SharedString, Window, actions, div,
    prelude::*, px,
};

actions!(time_picker, [TimePickerClose]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeValue {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

pub struct TimePicker {
    id: SharedString,
    value: Option<TimeValue>,
    is_open: bool,
    placeholder: SharedString,
    display_format: SharedString,
    width: Option<Pixels>,
    disabled: bool,
    minute_step: u32,
    second_step: u32,
    show_seconds: bool,
    last_bounds: Option<Bounds<Pixels>>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    on_change: Option<Box<dyn Fn(Option<TimeValue>, &mut Window, &mut App) + 'static>>,
}

impl TimeValue {
    pub fn new(hour: u32, minute: u32, second: u32) -> Option<Self> {
        if hour > 23 || minute > 59 || second > 59 {
            return None;
        }
        Some(Self {
            hour,
            minute,
            second,
        })
    }

    pub fn format(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl TimePicker {
    pub fn new() -> Self {
        Self {
            id: aura_core::unique_id("time-picker"),
            value: None,
            is_open: false,
            placeholder: "请选择时间".into(),
            display_format: "HH:mm:ss".into(),
            width: None,
            disabled: false,
            minute_step: 1,
            second_step: 1,
            show_seconds: true,
            last_bounds: None,
            close_on_click_outside: true,
            close_on_escape: true,
            on_change: None,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn value(mut self, value: TimeValue) -> Self {
        self.value = Some(value);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn format(mut self, format: impl Into<SharedString>) -> Self {
        self.display_format = format.into();
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn width_md(self) -> Self {
        self.width(px(240.0))
    }

    pub fn width_lg(self) -> Self {
        self.width(px(280.0))
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn minute_step(mut self, step: u32) -> Self {
        self.minute_step = step.clamp(1, 60);
        self
    }

    pub fn second_step(mut self, step: u32) -> Self {
        self.second_step = step.clamp(1, 60);
        self
    }

    pub fn without_seconds(mut self) -> Self {
        self.show_seconds = false;
        self.display_format = "HH:mm".into();
        self
    }

    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([gpui::KeyBinding::new("escape", TimePickerClose, None)]);
    }

    fn close_on_escape_action(
        &mut self,
        _: &TimePickerClose,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.close_on_escape && self.is_open {
            self.close(cx);
        }
    }

    pub fn on_change(
        mut self,
        f: impl Fn(Option<TimeValue>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn set_on_change(
        &mut self,
        f: impl Fn(Option<TimeValue>, &mut Window, &mut App) + 'static,
        _cx: &mut Context<Self>,
    ) {
        self.on_change = Some(Box::new(f));
    }

    pub fn set_value(&mut self, value: Option<TimeValue>, cx: &mut Context<Self>) {
        self.value = value;
        cx.notify();
    }

    pub fn value_ref(&self) -> Option<TimeValue> {
        self.value
    }

    fn display_text(&self) -> String {
        self.value
            .map(|value| format_time_value(value, self.display_format.as_ref()))
            .unwrap_or_else(|| self.placeholder.to_string())
    }

    fn toggle_open(&mut self, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.is_open = !self.is_open;
        cx.notify();
    }

    fn close(&mut self, cx: &mut Context<Self>) {
        if self.is_open {
            self.is_open = false;
            cx.notify();
        }
    }

    fn select_time(&mut self, value: TimeValue, window: &mut Window, cx: &mut Context<Self>) {
        self.value = Some(value);
        self.is_open = false;
        if let Some(ref on_change) = self.on_change {
            on_change(Some(value), window, cx);
        }
        cx.notify();
    }
}

impl Render for TimePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.entity().clone();
        let display = self.display_text();
        let has_value = self.value.is_some();
        let border_color = if self.is_open {
            theme.primary.base
        } else {
            theme.neutral.border
        };

        if self.is_open {
            let entity = entity.clone();
            let picker_id = self.id.clone();
            let bounds = self.last_bounds;
            let panel_min_width = if self.show_seconds {
                px(312.0)
            } else {
                px(232.0)
            };
            let close_on_click_outside = self.close_on_click_outside;
            push_portal(
                move |_window, _cx| {
                    let (top, left, width) = if let Some(bounds) = bounds {
                        (bounds.bottom() + px(4.0), bounds.left(), bounds.size.width)
                    } else {
                        (px(100.0), px(100.0), px(220.0))
                    };
                    let close_entity = entity.clone();

                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .bg(gpui::transparent_black())
                        .when(close_on_click_outside, |s| {
                            s.on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                close_entity.update(cx, |picker, cx| picker.close(cx));
                            })
                        })
                        .child(pop_in(
                            format!("{}-panel-motion", picker_id),
                            div()
                                .absolute()
                                .top(top)
                                .left(left)
                                .w(width.max(panel_min_width))
                                .child(render_time_panel(picker_id, entity, _cx)),
                        ))
                        .into_any_element()
                },
                cx,
            );
        }

        div()
            .relative()
            .when_some(self.width, |s, width| s.w(width))
            .when(self.width.is_none(), |s| s.w(px(220.0)))
            .h(px(34.0))
            .id(format!("{}-trigger", self.id))
            .flex()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .bg(if self.disabled {
                theme.neutral.hover
            } else {
                theme.neutral.card
            })
            .border_1()
            .border_color(border_color)
            .rounded(px(theme.radius.md))
            .cursor_pointer()
            .hover(|s| s.cursor_pointer().border_color(theme.primary.base))
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .text_size(px(theme.font_size.md))
                    .text_color(if has_value {
                        theme.neutral.text_1
                    } else {
                        theme.neutral.placeholder
                    })
                    .child(display),
            )
            .child(
                Icon::new(IconName::Clock)
                    .size(px(16.0))
                    .color(theme.neutral.icon),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(TimePickerBoundsCapturer { picker: entity }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.toggle_open(cx);
                }),
            )
            .on_action(cx.listener(Self::close_on_escape_action))
    }
}

fn render_time_panel(
    id: SharedString,
    picker: Entity<TimePicker>,
    cx: &mut App,
) -> gpui::AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let (selected, minute_step, second_step, show_seconds, display_format) =
        picker.update(cx, |picker, _| {
            (
                picker.value,
                picker.minute_step,
                picker.second_step,
                picker.show_seconds,
                picker.display_format.clone(),
            )
        });

    let hours: Vec<u32> = (0..24).collect();
    let minutes: Vec<u32> = stepped_values(minute_step);
    let seconds: Vec<u32> = stepped_values(second_step);
    let preview = selected
        .map(|value| format_time_value(value, display_format.as_ref()))
        .unwrap_or_else(|| "--:--".to_string());

    div()
        .id(format!("{}-panel", id))
        .cursor_default()
        .occlude()
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
        .flex()
        .flex_col()
        .gap_2()
        .p_2()
        .bg(theme.neutral.card)
        .border_1()
        .border_color(theme.neutral.border)
        .rounded(px(theme.radius.lg))
        .shadow_lg()
        .child(
            div()
                .h(px(34.0))
                .flex()
                .items_center()
                .justify_between()
                .px_2()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(theme.neutral.text_1)
                        .child("时间"),
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(theme.radius.sm))
                        .bg(theme.primary.light_9)
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(theme.primary.base)
                        .child(preview),
                ),
        )
        .child(
            div()
                .flex()
                .gap_1()
                .p_1()
                .rounded(px(theme.radius.md))
                .border_1()
                .border_color(theme.neutral.border)
                .bg(theme.neutral.body)
                .child(time_column(
                    format!("{}-hour", id),
                    "时",
                    hours,
                    selected.map(|value| value.hour),
                    &theme,
                    picker.clone(),
                    move |current, hour| TimeValue {
                        hour,
                        minute: current.map(|value| value.minute).unwrap_or(0),
                        second: current.map(|value| value.second).unwrap_or(0),
                    },
                ))
                .child(time_column(
                    format!("{}-minute", id),
                    "分",
                    minutes,
                    selected.map(|value| value.minute),
                    &theme,
                    picker.clone(),
                    move |current, minute| TimeValue {
                        hour: current.map(|value| value.hour).unwrap_or(0),
                        minute,
                        second: current.map(|value| value.second).unwrap_or(0),
                    },
                ))
                .when(show_seconds, |s| {
                    s.child(time_column(
                        format!("{}-second", id),
                        "秒",
                        seconds,
                        selected.map(|value| value.second),
                        &theme,
                        picker.clone(),
                        move |current, second| TimeValue {
                            hour: current.map(|value| value.hour).unwrap_or(0),
                            minute: current.map(|value| value.minute).unwrap_or(0),
                            second,
                        },
                    ))
                }),
        )
        .into_any_element()
}

fn time_column(
    id: impl Into<SharedString>,
    title: &'static str,
    values: Vec<u32>,
    selected: Option<u32>,
    theme: &aura_theme::Theme,
    picker: Entity<TimePicker>,
    build_value: impl Fn(Option<TimeValue>, u32) -> TimeValue + Clone + 'static,
) -> impl IntoElement {
    let id = id.into();
    div()
        .flex_1()
        .min_w(px(64.0))
        .flex()
        .flex_col()
        .child(
            div()
                .h(px(24.0))
                .flex()
                .items_center()
                .justify_center()
                .text_xs()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(theme.neutral.text_3)
                .child(title),
        )
        .child(
            div()
                .id(format!("{}-scroll", id))
                .max_h(px(210.0))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .gap_1()
                .children(values.into_iter().map(move |value| {
                    let is_selected = selected == Some(value);
                    let picker = picker.clone();
                    let build_value = build_value.clone();
                    time_option(
                        format!("{}-{}", id, value),
                        value,
                        is_selected,
                        theme.clone(),
                        picker,
                        build_value,
                    )
                })),
        )
}

fn time_option(
    id: impl Into<SharedString>,
    value: u32,
    is_selected: bool,
    theme: aura_theme::Theme,
    picker: Entity<TimePicker>,
    build_value: impl Fn(Option<TimeValue>, u32) -> TimeValue + 'static,
) -> impl IntoElement {
    div()
        .id(id.into())
        .h(px(30.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(theme.radius.sm))
        .cursor_pointer()
        .bg(if is_selected {
            theme.primary.base
        } else {
            gpui::transparent_black()
        })
        .text_color(if is_selected {
            theme.neutral.card
        } else {
            theme.neutral.text_1
        })
        .hover(|s| {
            if is_selected {
                s.cursor_pointer().bg(theme.primary.hover)
            } else {
                s.cursor_pointer().bg(theme.neutral.card)
            }
        })
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            picker.update(cx, |picker, cx| {
                let next = build_value(picker.value, value);
                picker.select_time(next, window, cx);
            });
        })
        .child(
            div()
                .text_sm()
                .font_weight(if is_selected {
                    gpui::FontWeight::BOLD
                } else {
                    gpui::FontWeight::NORMAL
                })
                .child(format!("{:02}", value)),
        )
}

fn stepped_values(step: u32) -> Vec<u32> {
    let step = step.clamp(1, 60) as usize;
    (0..60).step_by(step).collect()
}

fn format_time_value(value: TimeValue, format: &str) -> String {
    format
        .replace("HH", &format!("{:02}", value.hour))
        .replace("H", &value.hour.to_string())
        .replace("mm", &format!("{:02}", value.minute))
        .replace("m", &value.minute.to_string())
        .replace("ss", &format!("{:02}", value.second))
        .replace("s", &value.second.to_string())
}

struct TimePickerBoundsCapturer {
    picker: Entity<TimePicker>,
}

impl IntoElement for TimePickerBoundsCapturer {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TimePickerBoundsCapturer {
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
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = gpui::Style::default();
        style.size.width = gpui::relative(1.0).into();
        style.size.height = gpui::relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _rl: &mut Self::RequestLayoutState,
        _window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.picker.update(cx, |picker, _| {
            picker.last_bounds = Some(bounds);
        });
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _rl: &mut Self::RequestLayoutState,
        _ps: &mut Self::PrepaintState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_picker_width_helpers_set_demo_widths() {
        assert_eq!(TimePicker::new().width_md().width, Some(px(240.0)));
        assert_eq!(TimePicker::new().width_lg().width, Some(px(280.0)));
    }
}
