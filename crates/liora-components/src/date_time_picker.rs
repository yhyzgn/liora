//! Date Time Picker module.
//!
//! This public module implements the Liora date-time picker popup for combined calendar and time selection. It keeps the reusable
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

use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{
    App, Bounds, Context, Element, ElementId, Entity, GlobalElementId, Hsla, InspectorElementId,
    IntoElement, LayoutId, MouseButton, Pixels, Render, SharedString, Window, actions, div,
    prelude::*, px,
};
use liora_core::{Config, push_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

actions!(
    date_time_picker,
    [
        #[doc = "Keyboard action that closes the active date-time picker popup."]
        DateTimePickerClose
    ]
);

use crate::{DateValue, TimeValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Fluent native GPUI component for rendering Liora date time value.
pub struct DateTimeValue {
    /// Calendar date associated with this value.
    pub date: DateValue,
    /// Clock time associated with this value.
    pub time: TimeValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control date time picker type behavior.
pub enum DateTimePickerType {
    #[default]
    /// Selects a single date and time value.
    DateTime,
    /// Selects a start and end date-time value.
    DateTimeRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control date time picker selection behavior.
pub enum DateTimePickerSelection {
    /// Stores a single selection value.
    Single(Option<DateTimeValue>),
    /// Stores a range selection value.
    Range {
        /// Inclusive start date-time for the selected range.
        start: Option<DateTimeValue>,
        /// Inclusive end date-time for the selected range.
        end: Option<DateTimeValue>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RangeEndpoint {
    Start,
    End,
}

/// Fluent native GPUI component for rendering Liora date time picker.
pub struct DateTimePicker {
    id: SharedString,
    picker_type: DateTimePickerType,
    value: Option<DateTimeValue>,
    range_start: Option<DateTimeValue>,
    range_end: Option<DateTimeValue>,
    editing_endpoint: RangeEndpoint,
    view_year: i32,
    view_month: u32,
    draft_date: DateValue,
    draft_time: TimeValue,
    is_open: bool,
    placeholder: SharedString,
    display_format: SharedString,
    range_separator: SharedString,
    width: Option<Pixels>,
    disabled: bool,
    minute_step: u32,
    second_step: u32,
    show_seconds: bool,
    last_bounds: Option<Bounds<Pixels>>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    on_change: Option<Box<dyn Fn(Option<DateTimeValue>, &mut Window, &mut App) + 'static>>,
    on_range_change: Option<
        Box<dyn Fn(Option<DateTimeValue>, Option<DateTimeValue>, &mut Window, &mut App) + 'static>,
    >,
    on_selection_change:
        Option<Box<dyn Fn(DateTimePickerSelection, &mut Window, &mut App) + 'static>>,
}

impl DateTimeValue {
    /// Creates `DateTimeValue` with default theme-driven styling and no optional callbacks attached.
    pub fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Option<Self> {
        Some(Self {
            date: DateValue::new(year, month, day)?,
            time: TimeValue::new(hour, minute, second)?,
        })
    }

    /// Creates this value from parts.
    pub fn from_parts(date: DateValue, time: TimeValue) -> Self {
        Self { date, time }
    }

    /// Performs the format operation used by this component.
    pub fn format(&self) -> String {
        format!("{} {}", self.date.format(), self.time.format())
    }
}

impl DateTimePicker {
    /// Creates `DateTimePicker` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        let default_date = DateValue {
            year: 2026,
            month: 5,
            day: 8,
        };
        let default_time = TimeValue {
            hour: 0,
            minute: 0,
            second: 0,
        };
        Self {
            id: liora_core::unique_id("date-time-picker"),
            picker_type: DateTimePickerType::DateTime,
            value: None,
            range_start: None,
            range_end: None,
            editing_endpoint: RangeEndpoint::Start,
            view_year: default_date.year,
            view_month: default_date.month,
            draft_date: default_date,
            draft_time: default_time,
            is_open: false,
            placeholder: "请选择日期时间".into(),
            display_format: "YYYY-MM-DD HH:mm:ss".into(),
            range_separator: " 至 ".into(),
            width: None,
            disabled: false,
            minute_step: 1,
            second_step: 1,
            show_seconds: true,
            last_bounds: None,
            close_on_click_outside: true,
            close_on_escape: true,
            on_change: None,
            on_range_change: None,
            on_selection_change: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Selects single-date or range picking behavior.
    pub fn picker_type(mut self, picker_type: DateTimePickerType) -> Self {
        self.picker_type = picker_type;
        if picker_type.is_range() && self.placeholder == SharedString::from("请选择日期时间")
        {
            self.placeholder = "请选择日期时间范围".into();
        }
        self
    }

    /// Sets the date time value used by the component.
    pub fn date_time(self) -> Self {
        self.picker_type(DateTimePickerType::DateTime)
    }

    /// Sets the date time range value used by the component.
    pub fn date_time_range(self) -> Self {
        self.picker_type(DateTimePickerType::DateTimeRange)
    }

    /// Returns the serialized value used by forms, configuration, or persistence.
    pub fn value(mut self, value: DateTimeValue) -> Self {
        self.view_year = value.date.year;
        self.view_month = value.date.month;
        self.draft_date = value.date;
        self.draft_time = value.time;
        self.value = Some(value);
        self
    }

    /// Sets the range value used by the component.
    pub fn range(mut self, start: DateTimeValue, end: DateTimeValue) -> Self {
        let (start, end) = ordered_pair(start, end);
        self.view_year = start.date.year;
        self.view_month = start.date.month;
        self.draft_date = start.date;
        self.draft_time = start.time;
        self.range_start = Some(start);
        self.range_end = Some(end);
        self.editing_endpoint = RangeEndpoint::End;
        self
    }

    /// Uses the supplied placeholder text when the value is empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the format displayed or consumed by the component.
    pub fn format(mut self, format: impl Into<SharedString>) -> Self {
        self.display_format = format.into();
        self
    }

    /// Sets the text displayed between range endpoints.
    pub fn range_separator(mut self, separator: impl Into<SharedString>) -> Self {
        self.range_separator = separator.into();
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Applies the predefined width md sizing preset.
    pub fn width_md(self) -> Self {
        self.width(px(280.0))
    }

    /// Applies the predefined width lg sizing preset.
    pub fn width_lg(self) -> Self {
        self.width(px(460.0))
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets minute increments shown by the picker.
    pub fn minute_step(mut self, step: u32) -> Self {
        self.minute_step = step.clamp(1, 60);
        self
    }

    /// Sets second increments shown by the picker.
    pub fn second_step(mut self, step: u32) -> Self {
        self.second_step = step.clamp(1, 60);
        self
    }

    /// Hides seconds from the time selection UI.
    pub fn without_seconds(mut self) -> Self {
        self.show_seconds = false;
        self.display_format = "YYYY-MM-DD HH:mm".into();
        self
    }

    /// Toggles whether the popup closes when escape occurs.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Toggles whether the popup closes when click outside occurs.
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([gpui::KeyBinding::new("escape", DateTimePickerClose, None)]);
    }

    fn close_on_escape_action(
        &mut self,
        _: &DateTimePickerClose,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.close_on_escape && self.is_open {
            self.close(cx);
        }
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(
        mut self,
        f: impl Fn(Option<DateTimeValue>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// Registers a callback that runs when range change occurs.
    pub fn on_range_change(
        mut self,
        f: impl Fn(Option<DateTimeValue>, Option<DateTimeValue>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_range_change = Some(Box::new(f));
        self
    }

    /// Registers a callback that runs when selection change occurs.
    pub fn on_selection_change(
        mut self,
        f: impl Fn(DateTimePickerSelection, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_selection_change = Some(Box::new(f));
        self
    }

    /// Performs the value ref operation used by this component.
    pub fn value_ref(&self) -> Option<DateTimeValue> {
        self.value
    }

    /// Performs the range ref operation used by this component.
    pub fn range_ref(&self) -> (Option<DateTimeValue>, Option<DateTimeValue>) {
        (self.range_start, self.range_end)
    }

    fn is_range(&self) -> bool {
        self.picker_type.is_range()
    }

    fn has_display_value(&self) -> bool {
        if self.is_range() {
            self.range_start.is_some()
        } else {
            self.value.is_some()
        }
    }

    fn display_text(&self) -> String {
        if self.is_range() {
            match (self.range_start, self.range_end) {
                (Some(start), Some(end)) => format!(
                    "{}{}{}",
                    self.format_value(start),
                    self.range_separator,
                    self.format_value(end)
                ),
                (Some(start), None) => {
                    format!("{}{}", self.format_value(start), self.range_separator)
                }
                _ => self.placeholder.to_string(),
            }
        } else {
            self.value
                .map(|value| self.format_value(value))
                .unwrap_or_else(|| self.placeholder.to_string())
        }
    }

    fn format_value(&self, value: DateTimeValue) -> String {
        format_date_time_value(value, self.display_format.as_ref())
    }

    fn toggle_open(&mut self, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.is_open = !self.is_open;
        if self.is_open {
            self.sync_draft_from_value();
        }
        cx.notify();
    }

    fn close(&mut self, cx: &mut Context<Self>) {
        if self.is_open {
            self.is_open = false;
            cx.notify();
        }
    }

    fn sync_draft_from_value(&mut self) {
        let seed = if self.is_range() {
            match self.editing_endpoint {
                RangeEndpoint::Start => self.range_start.or(self.range_end),
                RangeEndpoint::End => self.range_end.or(self.range_start),
            }
        } else {
            self.value
        };
        if let Some(value) = seed {
            self.draft_date = value.date;
            self.draft_time = value.time;
            self.view_year = value.date.year;
            self.view_month = value.date.month;
        }
    }

    fn select_date(&mut self, date: DateValue, cx: &mut Context<Self>) {
        self.draft_date = date;
        self.view_year = date.year;
        self.view_month = date.month;
        if self.is_range() {
            let next = DateTimeValue::from_parts(date, self.draft_time);
            match (self.range_start, self.range_end) {
                (None, _) | (Some(_), Some(_)) => {
                    self.range_start = Some(next);
                    self.range_end = None;
                    self.editing_endpoint = RangeEndpoint::Start;
                }
                (Some(start), None) => {
                    let (start, end) = ordered_pair(start, next);
                    self.range_start = Some(start);
                    self.range_end = Some(end);
                    self.editing_endpoint = RangeEndpoint::End;
                    self.draft_date = end.date;
                    self.draft_time = end.time;
                }
            }
        }
        cx.notify();
    }

    fn select_time(&mut self, time: TimeValue, cx: &mut Context<Self>) {
        self.draft_time = time;
        if self.is_range() {
            let next = DateTimeValue::from_parts(self.draft_date, time);
            match self.editing_endpoint {
                RangeEndpoint::Start => self.range_start = Some(next),
                RangeEndpoint::End => self.range_end = Some(next),
            }
            if let (Some(start), Some(end)) = (self.range_start, self.range_end) {
                let (start, end) = ordered_pair(start, end);
                self.range_start = Some(start);
                self.range_end = Some(end);
            }
        }
        cx.notify();
    }

    fn edit_endpoint(&mut self, endpoint: RangeEndpoint, cx: &mut Context<Self>) {
        self.editing_endpoint = endpoint;
        if let Some(value) = match endpoint {
            RangeEndpoint::Start => self.range_start,
            RangeEndpoint::End => self.range_end,
        } {
            self.draft_date = value.date;
            self.draft_time = value.time;
            self.view_year = value.date.year;
            self.view_month = value.date.month;
        }
        cx.notify();
    }

    fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_range() {
            if let (Some(start), Some(end)) = (self.range_start, self.range_end) {
                let (start, end) = ordered_pair(start, end);
                self.range_start = Some(start);
                self.range_end = Some(end);
            }
        } else {
            self.value = Some(DateTimeValue::from_parts(self.draft_date, self.draft_time));
        }
        self.is_open = false;
        self.emit_change(window, cx);
        cx.notify();
    }

    fn emit_change(&self, window: &mut Window, cx: &mut App) {
        if self.is_range() {
            if let Some(ref on_range_change) = self.on_range_change {
                on_range_change(self.range_start, self.range_end, window, cx);
            }
            if let Some(ref on_selection_change) = self.on_selection_change {
                on_selection_change(
                    DateTimePickerSelection::Range {
                        start: self.range_start,
                        end: self.range_end,
                    },
                    window,
                    cx,
                );
            }
        } else {
            if let Some(ref on_change) = self.on_change {
                on_change(self.value, window, cx);
            }
            if let Some(ref on_selection_change) = self.on_selection_change {
                on_selection_change(DateTimePickerSelection::Single(self.value), window, cx);
            }
        }
    }

    fn shift_month(&mut self, delta: i32, cx: &mut Context<Self>) {
        let month_index = self.view_year * 12 + self.view_month as i32 - 1 + delta;
        self.view_year = month_index.div_euclid(12);
        self.view_month = month_index.rem_euclid(12) as u32 + 1;
        cx.notify();
    }

    fn shift_year(&mut self, delta: i32, cx: &mut Context<Self>) {
        self.view_year += delta;
        cx.notify();
    }
}

impl DateTimePickerType {
    fn is_range(self) -> bool {
        matches!(self, DateTimePickerType::DateTimeRange)
    }
}

impl Render for DateTimePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.entity().clone();
        let display = self.display_text();
        let range_start_text = self.range_start.map(|value| self.format_value(value));
        let range_end_text = self.range_end.map(|value| self.format_value(value));
        let range_separator = self.range_separator.clone();
        let is_range = self.is_range();
        let has_value = self.has_display_value();
        let border_color = if self.is_open {
            theme.primary.base
        } else {
            theme.neutral.border
        };

        if self.is_open {
            let entity = entity.clone();
            let picker_id = self.id.clone();
            let bounds = self.last_bounds;
            let close_on_click_outside = self.close_on_click_outside;
            push_portal(
                move |_window, _cx| {
                    let (top, left, width) = if let Some(bounds) = bounds {
                        (bounds.bottom() + px(4.0), bounds.left(), bounds.size.width)
                    } else {
                        (px(100.0), px(100.0), px(520.0))
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
                            element_id(format!("{}-panel-motion", picker_id)),
                            div()
                                .absolute()
                                .top(top)
                                .left(left)
                                .w(width.max(px(620.0)))
                                .child(render_date_time_panel(picker_id, entity, _cx)),
                        ))
                        .into_any_element()
                },
                cx,
            );
        }

        div()
            .relative()
            .when_some(self.width, |s, width| s.w(width))
            .when(self.width.is_none(), |s| s.w(px(260.0)))
            .h(px(34.0))
            .id(element_id(format!("{}-trigger", self.id)))
            .flex()
            .items_center()
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
            .child(div().flex_1().min_w(px(0.0)).child(if is_range {
                render_range_trigger_text(
                    range_start_text,
                    range_end_text,
                    range_separator,
                    self.placeholder.clone(),
                    has_value,
                    &theme,
                )
            } else {
                div()
                    .text_size(px(theme.font_size.md))
                    .text_color(if has_value {
                        theme.neutral.text_1
                    } else {
                        theme.neutral.placeholder
                    })
                    .child(display)
                    .into_any_element()
            }))
            .child(
                Icon::new(IconName::CalendarDays)
                    .size(px(16.0))
                    .color(theme.neutral.icon),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(DateTimePickerBoundsCapturer { picker: entity }),
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

fn render_range_trigger_text(
    start: Option<String>,
    end: Option<String>,
    separator: SharedString,
    placeholder: SharedString,
    has_value: bool,
    theme: &liora_theme::Theme,
) -> gpui::AnyElement {
    if !has_value {
        return div()
            .text_size(px(theme.font_size.md))
            .text_color(theme.neutral.placeholder)
            .child(placeholder)
            .into_any_element();
    }

    let start = start.unwrap_or_default();
    let end = end.unwrap_or_else(|| "请选择结束".to_string());

    div()
        .flex()
        .items_center()
        .gap_2()
        .w_full()
        .child(range_value_text(start, true, theme))
        .child(
            div()
                .flex_shrink_0()
                .px_2()
                .py_1()
                .rounded(px(theme.radius.sm))
                .bg(theme.neutral.hover)
                .text_xs()
                .text_color(theme.neutral.text_3)
                .child(separator),
        )
        .child(range_value_text(end, false, theme))
        .into_any_element()
}

fn range_value_text(
    text: impl Into<SharedString>,
    filled: bool,
    theme: &liora_theme::Theme,
) -> impl IntoElement {
    div()
        .flex_1()
        .min_w(px(0.0))
        .px_1()
        .text_size(px(theme.font_size.md))
        .text_color(if filled {
            theme.neutral.text_1
        } else {
            theme.neutral.text_3
        })
        .child(text.into())
}

fn render_date_time_panel(
    id: SharedString,
    picker: Entity<DateTimePicker>,
    cx: &mut App,
) -> gpui::AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let is_range = picker.update(cx, |picker, _| picker.is_range());

    div()
        .id(element_id(format!("{}-panel", id)))
        .cursor_default()
        .occlude()
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
        .flex()
        .flex_col()
        .p_3()
        .gap_3()
        .bg(theme.neutral.card)
        .border_1()
        .border_color(theme.neutral.border)
        .rounded(px(theme.radius.md))
        .shadow_lg()
        .child(if is_range {
            render_range_summary(id.clone(), picker.clone(), cx, &theme).into_any_element()
        } else {
            div().into_any_element()
        })
        .child(
            div()
                .flex()
                .gap_3()
                .child(render_calendar_panel(id.clone(), picker.clone(), cx))
                .child(render_time_panel(id.clone(), picker.clone(), cx)),
        )
        .child(render_footer(id, picker, &theme))
        .into_any_element()
}

fn render_range_summary(
    id: SharedString,
    picker: Entity<DateTimePicker>,
    cx: &mut App,
    theme: &liora_theme::Theme,
) -> impl IntoElement {
    let (start, end, editing, format, separator) = picker.update(cx, |picker, _| {
        (
            picker.range_start,
            picker.range_end,
            picker.editing_endpoint,
            picker.display_format.clone(),
            picker.range_separator.clone(),
        )
    });
    let start_picker = picker.clone();
    let end_picker = picker.clone();

    div()
        .flex()
        .items_center()
        .gap_2()
        .child(endpoint_chip(
            format!("{}-start-chip", id),
            "开始",
            start
                .map(|v| format_date_time_value(v, format.as_ref()))
                .unwrap_or_else(|| "未选择".to_string()),
            editing == RangeEndpoint::Start,
            theme.clone(),
            start_picker,
            RangeEndpoint::Start,
        ))
        .child(
            div()
                .px_2()
                .py_1()
                .rounded(px(theme.radius.sm))
                .bg(theme.neutral.hover)
                .text_xs()
                .text_color(theme.neutral.text_3)
                .child(separator),
        )
        .child(endpoint_chip(
            format!("{}-end-chip", id),
            "结束",
            end.map(|v| format_date_time_value(v, format.as_ref()))
                .unwrap_or_else(|| "未选择".to_string()),
            editing == RangeEndpoint::End,
            theme.clone(),
            end_picker,
            RangeEndpoint::End,
        ))
}

fn endpoint_chip(
    id: impl Into<SharedString>,
    label: &'static str,
    value: String,
    active: bool,
    theme: liora_theme::Theme,
    picker: Entity<DateTimePicker>,
    endpoint: RangeEndpoint,
) -> impl IntoElement {
    div()
        .id(id.into())
        .flex_1()
        .min_w(px(0.0))
        .cursor_pointer()
        .rounded(px(theme.radius.md))
        .border_1()
        .border_color(if active {
            theme.primary.base
        } else {
            theme.neutral.border
        })
        .bg(if active {
            theme.primary.light_9
        } else {
            theme.neutral.body
        })
        .px_3()
        .py_2()
        .hover(|s| s.cursor_pointer().border_color(theme.primary.base))
        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
            picker.update(cx, |picker, cx| picker.edit_endpoint(endpoint, cx));
        })
        .child(
            div()
                .text_xs()
                .text_color(theme.neutral.text_3)
                .child(label),
        )
        .child(
            div()
                .text_sm()
                .text_color(theme.neutral.text_1)
                .child(value),
        )
}

fn render_calendar_panel(
    id: SharedString,
    picker: Entity<DateTimePicker>,
    cx: &mut App,
) -> gpui::AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let (view_year, view_month, selected, range_start, range_end) =
        picker.update(cx, |picker, _| {
            (
                picker.view_year,
                picker.view_month,
                picker.value.map(|value| value.date),
                picker.range_start.map(|value| value.date),
                picker.range_end.map(|value| value.date),
            )
        });
    let days = calendar_cells(view_year, view_month);
    let picker_prev_year = picker.clone();
    let picker_prev_month = picker.clone();
    let picker_next_month = picker.clone();
    let picker_next_year = picker.clone();
    let weekdays = ["一", "二", "三", "四", "五", "六", "日"];

    div()
        .w(px(292.0))
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(nav_button(
                            format!("{}-prev-year", id),
                            IconName::ChevronsLeft,
                            theme.neutral.icon,
                            theme.neutral.hover,
                            picker_prev_year,
                            |picker, cx| picker.shift_year(-1, cx),
                        ))
                        .child(nav_button(
                            format!("{}-prev-month", id),
                            IconName::ChevronLeft,
                            theme.neutral.icon,
                            theme.neutral.hover,
                            picker_prev_month,
                            |picker, cx| picker.shift_month(-1, cx),
                        )),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(theme.neutral.text_1)
                        .child(format!("{} 年 {:02} 月", view_year, view_month)),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(nav_button(
                            format!("{}-next-month", id),
                            IconName::ChevronRight,
                            theme.neutral.icon,
                            theme.neutral.hover,
                            picker_next_month,
                            |picker, cx| picker.shift_month(1, cx),
                        ))
                        .child(nav_button(
                            format!("{}-next-year", id),
                            IconName::ChevronsRight,
                            theme.neutral.icon,
                            theme.neutral.hover,
                            picker_next_year,
                            |picker, cx| picker.shift_year(1, cx),
                        )),
                ),
        )
        .child(div().flex().children(weekdays.into_iter().map(|day| {
            div()
                .flex_1()
                .h(px(28.0))
                .flex()
                .items_center()
                .justify_center()
                .text_xs()
                .text_color(theme.neutral.text_3)
                .child(day)
        })))
        .child(
            div()
                .flex()
                .flex_col()
                .children(days.chunks(7).enumerate().map(|(week_idx, week)| {
                    let id = id.clone();
                    let week_picker = picker.clone();
                    let week_theme = theme.clone();
                    div()
                        .flex()
                        .children(week.iter().enumerate().map(move |(day_idx, cell)| {
                            let is_current_month = cell.month == view_month;
                            let is_selected = selected == Some(*cell)
                                || range_start == Some(*cell)
                                || range_end == Some(*cell);
                            let in_range = is_between(*cell, range_start, range_end);
                            let picker = week_picker.clone();
                            let date = *cell;
                            selectable_date_cell(
                                format!("{}-day-{}-{}", id, week_idx, day_idx),
                                cell.day.to_string(),
                                is_selected,
                                in_range,
                                is_current_month,
                                week_theme.clone(),
                                picker,
                                date,
                            )
                        }))
                })),
        )
        .into_any_element()
}

fn render_time_panel(
    id: SharedString,
    picker: Entity<DateTimePicker>,
    cx: &mut App,
) -> gpui::AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let (selected, minute_step, second_step, show_seconds) = picker.update(cx, |picker, _| {
        (
            picker.draft_time,
            picker.minute_step,
            picker.second_step,
            picker.show_seconds,
        )
    });
    let hours: Vec<u32> = (0..24).collect();
    let minutes = stepped_values(minute_step);
    let seconds = stepped_values(second_step);
    let preview = if show_seconds {
        format!(
            "{:02}:{:02}:{:02}",
            selected.hour, selected.minute, selected.second
        )
    } else {
        format!("{:02}:{:02}", selected.hour, selected.minute)
    };

    div()
        .w(px(if show_seconds { 276.0 } else { 208.0 }))
        .flex()
        .flex_col()
        .gap_2()
        .p_2()
        .rounded(px(theme.radius.lg))
        .border_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.body)
        .child(
            div()
                .h(px(34.0))
                .flex()
                .items_center()
                .justify_between()
                .px_1()
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
                .bg(theme.neutral.card)
                .child(time_column(
                    format!("{}-hour", id),
                    "时",
                    hours,
                    selected.hour,
                    &theme,
                    picker.clone(),
                    move |current, hour| TimeValue { hour, ..current },
                ))
                .child(time_column(
                    format!("{}-minute", id),
                    "分",
                    minutes,
                    selected.minute,
                    &theme,
                    picker.clone(),
                    move |current, minute| TimeValue { minute, ..current },
                ))
                .when(show_seconds, |s| {
                    s.child(time_column(
                        format!("{}-second", id),
                        "秒",
                        seconds,
                        selected.second,
                        &theme,
                        picker.clone(),
                        move |current, second| TimeValue { second, ..current },
                    ))
                }),
        )
        .into_any_element()
}

fn render_footer(
    id: SharedString,
    picker: Entity<DateTimePicker>,
    theme: &liora_theme::Theme,
) -> impl IntoElement {
    let confirm_picker = picker.clone();
    let cancel_picker = picker.clone();
    div()
        .flex()
        .items_center()
        .justify_end()
        .gap_2()
        .pt_2()
        .child(
            div()
                .id(element_id(format!("{}-cancel", id)))
                .cursor_pointer()
                .px_3()
                .py_1()
                .rounded(px(theme.radius.sm))
                .text_sm()
                .text_color(theme.neutral.text_2)
                .hover(|s| s.cursor_pointer().bg(theme.neutral.hover))
                .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                    cancel_picker.update(cx, |picker, cx| picker.close(cx));
                })
                .child("取消"),
        )
        .child(
            div()
                .id(element_id(format!("{}-confirm", id)))
                .cursor_pointer()
                .px_3()
                .py_1()
                .rounded(px(theme.radius.sm))
                .bg(theme.primary.base)
                .text_sm()
                .text_color(theme.neutral.card)
                .hover(|s| s.cursor_pointer().bg(theme.primary.hover))
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    confirm_picker.update(cx, |picker, cx| picker.confirm(window, cx));
                })
                .child("确定"),
        )
}

fn selectable_date_cell(
    id: impl Into<SharedString>,
    label: impl Into<SharedString>,
    is_selected: bool,
    in_range: bool,
    is_current_scope: bool,
    theme: liora_theme::Theme,
    picker: Entity<DateTimePicker>,
    date: DateValue,
) -> impl IntoElement {
    div()
        .id(id.into())
        .flex_1()
        .h(px(34.0))
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .rounded(px(theme.radius.sm))
        .bg(if is_selected {
            theme.primary.base
        } else if in_range {
            theme.primary.light_9
        } else {
            theme.neutral.card
        })
        .text_color(if is_selected {
            theme.neutral.card
        } else if is_current_scope {
            theme.neutral.text_1
        } else {
            theme.neutral.text_3.opacity(0.55)
        })
        .hover(|s| {
            if is_selected {
                s.cursor_pointer()
            } else {
                s.cursor_pointer().bg(theme.neutral.hover)
            }
        })
        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
            picker.update(cx, |picker, cx| picker.select_date(date, cx));
        })
        .child(div().text_sm().child(label.into()))
}

fn nav_button(
    id: impl Into<SharedString>,
    icon: IconName,
    icon_color: Hsla,
    hover_bg: Hsla,
    picker: Entity<DateTimePicker>,
    action: impl Fn(&mut DateTimePicker, &mut Context<DateTimePicker>) + 'static,
) -> impl IntoElement {
    div()
        .id(id.into())
        .cursor_pointer()
        .p_1()
        .rounded(px(4.0))
        .hover(move |s| s.cursor_pointer().bg(hover_bg))
        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
            picker.update(cx, |picker, cx| action(picker, cx));
        })
        .child(Icon::new(icon).size(px(18.0)).color(icon_color))
}

fn time_column(
    id: impl Into<SharedString>,
    title: &'static str,
    values: Vec<u32>,
    selected: u32,
    theme: &liora_theme::Theme,
    picker: Entity<DateTimePicker>,
    build_value: impl Fn(TimeValue, u32) -> TimeValue + Clone + 'static,
) -> impl IntoElement {
    let id = id.into();
    div()
        .flex_1()
        .min_w(px(58.0))
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
                .id(element_id(format!("{}-scroll", id)))
                .max_h(px(210.0))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .gap_1()
                .children(values.into_iter().map(move |value| {
                    let is_selected = selected == value;
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
    theme: liora_theme::Theme,
    picker: Entity<DateTimePicker>,
    build_value: impl Fn(TimeValue, u32) -> TimeValue + 'static,
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
                s.cursor_pointer().bg(theme.neutral.hover)
            }
        })
        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
            picker.update(cx, |picker, cx| {
                let next = build_value(picker.draft_time, value);
                picker.select_time(next, cx);
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

fn ordered_pair(a: DateTimeValue, b: DateTimeValue) -> (DateTimeValue, DateTimeValue) {
    if a <= b { (a, b) } else { (b, a) }
}

fn is_between(value: DateValue, start: Option<DateValue>, end: Option<DateValue>) -> bool {
    matches!((start, end), (Some(start), Some(end)) if value > start && value < end)
}

fn format_date_time_value(value: DateTimeValue, format: &str) -> String {
    format
        .replace("YYYY", &format!("{:04}", value.date.year))
        .replace("YY", &format!("{:02}", value.date.year.rem_euclid(100)))
        .replace("MM", &format!("{:02}", value.date.month))
        .replace("M", &value.date.month.to_string())
        .replace("DD", &format!("{:02}", value.date.day))
        .replace("D", &value.date.day.to_string())
        .replace("HH", &format!("{:02}", value.time.hour))
        .replace("H", &value.time.hour.to_string())
        .replace("mm", &format!("{:02}", value.time.minute))
        .replace("m", &value.time.minute.to_string())
        .replace("ss", &format!("{:02}", value.time.second))
        .replace("s", &value.time.second.to_string())
}

fn calendar_cells(year: i32, month: u32) -> Vec<DateValue> {
    let first_weekday = weekday_monday_based(year, month, 1);
    let prev_month_index = year * 12 + month as i32 - 2;
    let prev_year = prev_month_index.div_euclid(12);
    let prev_month = prev_month_index.rem_euclid(12) as u32 + 1;
    let current_days = days_in_month(year, month);
    let prev_days = days_in_month(prev_year, prev_month);
    let mut cells = Vec::with_capacity(42);

    for i in (0..first_weekday).rev() {
        cells.push(DateValue {
            year: prev_year,
            month: prev_month,
            day: prev_days - i,
        });
    }

    for day in 1..=current_days {
        cells.push(DateValue { year, month, day });
    }

    let next_month_index = year * 12 + month as i32;
    let next_year = next_month_index.div_euclid(12);
    let next_month = next_month_index.rem_euclid(12) as u32 + 1;
    let mut next_day = 1;
    while cells.len() < 42 {
        cells.push(DateValue {
            year: next_year,
            month: next_month,
            day: next_day,
        });
        next_day += 1;
    }

    cells
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn weekday_monday_based(year: i32, month: u32, day: u32) -> u32 {
    let mut y = year;
    let mut m = month as i32;
    if m < 3 {
        m += 12;
        y -= 1;
    }
    let k = y % 100;
    let j = y / 100;
    let h = (day as i32 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j).rem_euclid(7);
    ((h + 5) % 7) as u32
}

struct DateTimePickerBoundsCapturer {
    picker: Entity<DateTimePicker>,
}

impl IntoElement for DateTimePickerBoundsCapturer {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for DateTimePickerBoundsCapturer {
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
    fn date_time_picker_width_helpers_set_demo_widths() {
        assert_eq!(DateTimePicker::new().width_md().width, Some(px(280.0)));
        assert_eq!(DateTimePicker::new().width_lg().width, Some(px(460.0)));
    }
}
