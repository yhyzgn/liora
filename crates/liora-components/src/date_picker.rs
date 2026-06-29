//! Date Picker module.
//!
//! This public module implements the Liora date picker popup for single date and date range selection. It keeps the reusable
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
    date_picker,
    [
        #[doc = "Keyboard action that closes the active date picker popup."]
        DatePickerClose
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Fluent native GPUI component for rendering Liora date value.
pub struct DateValue {
    /// Four-digit calendar year.
    pub year: i32,
    /// One-based calendar month.
    pub month: u32,
    /// One-based day within the month.
    pub day: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control date picker type behavior.
pub enum DatePickerType {
    #[default]
    /// Selects a single calendar date.
    Date,
    /// Selects a start and end calendar date.
    DateRange,
    /// Selects a single month.
    Month,
    /// Selects a start and end month.
    MonthRange,
    /// Selects a single year.
    Year,
    /// Selects a start and end year.
    YearRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control date picker selection behavior.
pub enum DatePickerSelection {
    /// Stores a single selection value.
    Single(Option<DateValue>),
    /// Stores a range selection value.
    Range {
        /// Inclusive start date for the selected range.
        start: Option<DateValue>,
        /// Inclusive end date for the selected range.
        end: Option<DateValue>,
    },
}

/// Fluent native GPUI component for rendering Liora date picker.
pub struct DatePicker {
    id: SharedString,
    picker_type: DatePickerType,
    value: Option<DateValue>,
    range_start: Option<DateValue>,
    range_end: Option<DateValue>,
    view_year: i32,
    view_month: u32,
    is_open: bool,
    placeholder: SharedString,
    display_format: Option<SharedString>,
    range_separator: SharedString,
    width: Option<Pixels>,
    disabled: bool,
    last_bounds: Option<Bounds<Pixels>>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    on_change: Option<Box<dyn Fn(Option<DateValue>, &mut Window, &mut App) + 'static>>,
    on_range_change:
        Option<Box<dyn Fn(Option<DateValue>, Option<DateValue>, &mut Window, &mut App) + 'static>>,
    on_selection_change: Option<Box<dyn Fn(DatePickerSelection, &mut Window, &mut App) + 'static>>,
}

impl DateValue {
    /// Creates `DateValue` initialized from the supplied year, month, and day.
    pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
            return None;
        }
        Some(Self { year, month, day })
    }

    /// Performs the format operation used by this component.
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl DatePicker {
    /// Creates `DatePicker` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            id: liora_core::unique_id("date-picker"),
            picker_type: DatePickerType::Date,
            value: None,
            range_start: None,
            range_end: None,
            view_year: 2026,
            view_month: 5,
            is_open: false,
            placeholder: "请选择日期".into(),
            display_format: None,
            range_separator: " 至 ".into(),
            width: None,
            disabled: false,
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
    pub fn picker_type(mut self, picker_type: DatePickerType) -> Self {
        self.picker_type = picker_type;
        if self.placeholder == SharedString::from("请选择日期") {
            self.placeholder = default_placeholder(picker_type).into();
        }
        self
    }

    /// Sets the date value used by the component.
    pub fn date(self) -> Self {
        self.picker_type(DatePickerType::Date)
    }

    /// Sets the date range value used by the component.
    pub fn date_range(self) -> Self {
        self.picker_type(DatePickerType::DateRange)
    }

    /// Sets the month value used by the component.
    pub fn month(self) -> Self {
        self.picker_type(DatePickerType::Month)
    }

    /// Sets the month range value used by the component.
    pub fn month_range(self) -> Self {
        self.picker_type(DatePickerType::MonthRange)
    }

    /// Sets the year value used by the component.
    pub fn year(self) -> Self {
        self.picker_type(DatePickerType::Year)
    }

    /// Sets the year range value used by the component.
    pub fn year_range(self) -> Self {
        self.picker_type(DatePickerType::YearRange)
    }

    /// Returns the serialized value used by forms, configuration, or persistence.
    pub fn value(mut self, value: DateValue) -> Self {
        self.view_year = value.year;
        self.view_month = value.month;
        self.value = Some(normalize_value(value, self.picker_type));
        self
    }

    /// Sets the range value used by the component.
    pub fn range(mut self, start: DateValue, end: DateValue) -> Self {
        let (start, end) = ordered_pair(
            normalize_value(start, self.picker_type),
            normalize_value(end, self.picker_type),
        );
        self.view_year = start.year;
        self.view_month = start.month;
        self.range_start = Some(start);
        self.range_end = Some(end);
        self
    }

    /// Uses the supplied placeholder text when the value is empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the format displayed or consumed by the component.
    pub fn format(mut self, format: impl Into<SharedString>) -> Self {
        self.display_format = Some(format.into());
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
        self.width(px(260.0))
    }

    /// Applies the predefined width lg sizing preset.
    pub fn width_lg(self) -> Self {
        self.width(px(320.0))
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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
        cx.bind_keys([gpui::KeyBinding::new("escape", DatePickerClose, None)]);
    }

    fn close_on_escape_action(
        &mut self,
        _: &DatePickerClose,
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
        f: impl Fn(Option<DateValue>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// Registers a callback that runs when range change occurs.
    pub fn on_range_change(
        mut self,
        f: impl Fn(Option<DateValue>, Option<DateValue>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_range_change = Some(Box::new(f));
        self
    }

    /// Registers a callback that runs when selection change occurs.
    pub fn on_selection_change(
        mut self,
        f: impl Fn(DatePickerSelection, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_selection_change = Some(Box::new(f));
        self
    }

    /// Updates the stored on change value and keeps the existing component identity.
    pub fn set_on_change(
        &mut self,
        f: impl Fn(Option<DateValue>, &mut Window, &mut App) + 'static,
        _cx: &mut Context<Self>,
    ) {
        self.on_change = Some(Box::new(f));
    }

    /// Updates the stored on range change value and keeps the existing component identity.
    pub fn set_on_range_change(
        &mut self,
        f: impl Fn(Option<DateValue>, Option<DateValue>, &mut Window, &mut App) + 'static,
        _cx: &mut Context<Self>,
    ) {
        self.on_range_change = Some(Box::new(f));
    }

    /// Updates the stored on selection change value and keeps the existing component identity.
    pub fn set_on_selection_change(
        &mut self,
        f: impl Fn(DatePickerSelection, &mut Window, &mut App) + 'static,
        _cx: &mut Context<Self>,
    ) {
        self.on_selection_change = Some(Box::new(f));
    }

    /// Updates the stored value value and keeps the existing component identity.
    pub fn set_value(&mut self, value: Option<DateValue>, cx: &mut Context<Self>) {
        self.value = value.map(|value| normalize_value(value, self.picker_type));
        if let Some(value) = self.value {
            self.view_year = value.year;
            self.view_month = value.month;
        }
        cx.notify();
    }

    /// Updates the stored range value and keeps the existing component identity.
    pub fn set_range(
        &mut self,
        start: Option<DateValue>,
        end: Option<DateValue>,
        cx: &mut Context<Self>,
    ) {
        match (start, end) {
            (Some(start), Some(end)) => {
                let (start, end) = ordered_pair(
                    normalize_value(start, self.picker_type),
                    normalize_value(end, self.picker_type),
                );
                self.range_start = Some(start);
                self.range_end = Some(end);
                self.view_year = start.year;
                self.view_month = start.month;
            }
            (start, end) => {
                self.range_start = start.map(|value| normalize_value(value, self.picker_type));
                self.range_end = end.map(|value| normalize_value(value, self.picker_type));
            }
        }
        cx.notify();
    }

    /// Performs the value ref operation used by this component.
    pub fn value_ref(&self) -> Option<DateValue> {
        self.value
    }

    /// Performs the range ref operation used by this component.
    pub fn range_ref(&self) -> (Option<DateValue>, Option<DateValue>) {
        (self.range_start, self.range_end)
    }

    fn display_text(&self) -> String {
        if self.picker_type.is_range() {
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

    fn format_value(&self, value: DateValue) -> String {
        let format = self
            .display_format
            .as_ref()
            .map(|format| format.as_ref())
            .unwrap_or_else(|| default_format(self.picker_type));
        format_date_value(value, format)
    }

    fn has_display_value(&self) -> bool {
        if self.picker_type.is_range() {
            self.range_start.is_some()
        } else {
            self.value.is_some()
        }
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

    fn select_value(&mut self, value: DateValue, window: &mut Window, cx: &mut Context<Self>) {
        let value = normalize_value(value, self.picker_type);
        if self.picker_type.is_range() {
            match (self.range_start, self.range_end) {
                (None, _) | (Some(_), Some(_)) => {
                    self.range_start = Some(value);
                    self.range_end = None;
                }
                (Some(start), None) => {
                    let (start, end) = ordered_pair(start, value);
                    self.range_start = Some(start);
                    self.range_end = Some(end);
                    self.is_open = false;
                }
            }
        } else {
            self.value = Some(value);
            self.is_open = false;
        }

        self.view_year = value.year;
        self.view_month = value.month;
        self.emit_change(window, cx);
        cx.notify();
    }

    fn emit_change(&self, window: &mut Window, cx: &mut App) {
        if self.picker_type.is_range() {
            if let Some(ref on_range_change) = self.on_range_change {
                on_range_change(self.range_start, self.range_end, window, cx);
            }
            if let Some(ref on_selection_change) = self.on_selection_change {
                on_selection_change(
                    DatePickerSelection::Range {
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
                on_selection_change(DatePickerSelection::Single(self.value), window, cx);
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

impl DatePickerType {
    fn is_range(self) -> bool {
        matches!(
            self,
            DatePickerType::DateRange | DatePickerType::MonthRange | DatePickerType::YearRange
        )
    }
}

impl Render for DatePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.entity().clone();
        let display = self.display_text();
        let range_start_text = self.range_start.map(|value| self.format_value(value));
        let range_end_text = self.range_end.map(|value| self.format_value(value));
        let range_separator = self.range_separator.clone();
        let is_range = self.picker_type.is_range();
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
                        (px(100.0), px(100.0), px(240.0))
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
                                .w(width.max(px(300.0)))
                                .child(render_picker_panel(picker_id, entity, _cx)),
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
                    .child(DatePickerBoundsCapturer { picker: entity }),
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

fn render_picker_panel(
    id: SharedString,
    picker: Entity<DatePicker>,
    cx: &mut App,
) -> gpui::AnyElement {
    let picker_type = picker.update(cx, |picker, _| picker.picker_type);
    match picker_type {
        DatePickerType::Date | DatePickerType::DateRange => render_date_panel(id, picker, cx),
        DatePickerType::Month | DatePickerType::MonthRange => render_month_panel(id, picker, cx),
        DatePickerType::Year | DatePickerType::YearRange => render_year_panel(id, picker, cx),
    }
}

fn panel_shell(id: &SharedString, theme: &liora_theme::Theme) -> gpui::Stateful<gpui::Div> {
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
}

fn render_date_panel(
    id: SharedString,
    picker: Entity<DatePicker>,
    cx: &mut App,
) -> gpui::AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let (view_year, view_month, selected, range_start, range_end) =
        picker.update(cx, |picker, _| {
            (
                picker.view_year,
                picker.view_month,
                picker.value,
                picker.range_start,
                picker.range_end,
            )
        });
    let days = calendar_cells(view_year, view_month);
    let picker_prev_year = picker.clone();
    let picker_prev_month = picker.clone();
    let picker_next_month = picker.clone();
    let picker_next_year = picker.clone();
    let weekdays = ["一", "二", "三", "四", "五", "六", "日"];

    panel_shell(&id, &theme)
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
        .child(
            div()
                .flex()
                .flex_row()
                .children(weekdays.into_iter().map(|day| {
                    div()
                        .flex_1()
                        .h(px(28.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_xs()
                        .text_color(theme.neutral.text_3)
                        .child(day)
                })),
        )
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
                        .flex_row()
                        .children(week.iter().enumerate().map(move |(day_idx, cell)| {
                            let is_current_month = cell.month == view_month;
                            let is_selected = selected == Some(*cell)
                                || range_start == Some(*cell)
                                || range_end == Some(*cell);
                            let in_range = is_between(*cell, range_start, range_end);
                            let picker = week_picker.clone();
                            let date = *cell;
                            selectable_cell(
                                format!("{}-day-{}-{}", id, week_idx, day_idx),
                                cell.day.to_string(),
                                is_selected,
                                in_range,
                                is_current_month,
                                week_theme.clone(),
                                picker,
                                move |picker, window, cx| picker.select_value(date, window, cx),
                            )
                        }))
                })),
        )
        .into_any_element()
}

fn render_month_panel(
    id: SharedString,
    picker: Entity<DatePicker>,
    cx: &mut App,
) -> gpui::AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let (view_year, selected, range_start, range_end) = picker.update(cx, |picker, _| {
        (
            picker.view_year,
            picker.value,
            picker.range_start,
            picker.range_end,
        )
    });
    let picker_prev_year = picker.clone();
    let picker_next_year = picker.clone();
    let labels = [
        "一月",
        "二月",
        "三月",
        "四月",
        "五月",
        "六月",
        "七月",
        "八月",
        "九月",
        "十月",
        "十一月",
        "十二月",
    ];

    panel_shell(&id, &theme)
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(nav_button(
                    format!("{}-prev-year", id),
                    IconName::ChevronsLeft,
                    theme.neutral.icon,
                    theme.neutral.hover,
                    picker_prev_year,
                    |picker, cx| picker.shift_year(-1, cx),
                ))
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(theme.neutral.text_1)
                        .child(format!("{} 年", view_year)),
                )
                .child(nav_button(
                    format!("{}-next-year", id),
                    IconName::ChevronsRight,
                    theme.neutral.icon,
                    theme.neutral.hover,
                    picker_next_year,
                    |picker, cx| picker.shift_year(1, cx),
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(labels.chunks(3).enumerate().map(|(row_idx, row)| {
                    let id = id.clone();
                    let row_picker = picker.clone();
                    let row_theme = theme.clone();
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .children(row.iter().enumerate().map(move |(col_idx, label)| {
                            let month = (row_idx * 3 + col_idx + 1) as u32;
                            let value = DateValue {
                                year: view_year,
                                month,
                                day: 1,
                            };
                            let is_selected = selected == Some(value)
                                || range_start == Some(value)
                                || range_end == Some(value);
                            let in_range = is_between(value, range_start, range_end);
                            let picker = row_picker.clone();
                            selectable_cell(
                                format!("{}-month-{}", id, month),
                                *label,
                                is_selected,
                                in_range,
                                true,
                                row_theme.clone(),
                                picker,
                                move |picker, window, cx| picker.select_value(value, window, cx),
                            )
                        }))
                })),
        )
        .into_any_element()
}

fn render_year_panel(
    id: SharedString,
    picker: Entity<DatePicker>,
    cx: &mut App,
) -> gpui::AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let (view_year, selected, range_start, range_end) = picker.update(cx, |picker, _| {
        (
            picker.view_year,
            picker.value,
            picker.range_start,
            picker.range_end,
        )
    });
    let start_year = view_year.div_euclid(12) * 12;
    let picker_prev = picker.clone();
    let picker_next = picker.clone();
    let years: Vec<i32> = (start_year..start_year + 12).collect();

    panel_shell(&id, &theme)
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(nav_button(
                    format!("{}-prev-years", id),
                    IconName::ChevronsLeft,
                    theme.neutral.icon,
                    theme.neutral.hover,
                    picker_prev,
                    |picker, cx| picker.shift_year(-12, cx),
                ))
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(theme.neutral.text_1)
                        .child(format!("{} - {}", start_year, start_year + 11)),
                )
                .child(nav_button(
                    format!("{}-next-years", id),
                    IconName::ChevronsRight,
                    theme.neutral.icon,
                    theme.neutral.hover,
                    picker_next,
                    |picker, cx| picker.shift_year(12, cx),
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(years.chunks(4).enumerate().map(|(row_idx, row)| {
                    let id = id.clone();
                    let row_picker = picker.clone();
                    let row_theme = theme.clone();
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .children(row.iter().enumerate().map(move |(col_idx, year)| {
                            let value = DateValue {
                                year: *year,
                                month: 1,
                                day: 1,
                            };
                            let is_selected = selected == Some(value)
                                || range_start == Some(value)
                                || range_end == Some(value);
                            let in_range = is_between(value, range_start, range_end);
                            let picker = row_picker.clone();
                            selectable_cell(
                                format!("{}-year-{}-{}", id, row_idx, col_idx),
                                year.to_string(),
                                is_selected,
                                in_range,
                                true,
                                row_theme.clone(),
                                picker,
                                move |picker, window, cx| picker.select_value(value, window, cx),
                            )
                        }))
                })),
        )
        .into_any_element()
}

fn selectable_cell(
    id: impl Into<SharedString>,
    label: impl Into<SharedString>,
    is_selected: bool,
    in_range: bool,
    is_current_scope: bool,
    theme: liora_theme::Theme,
    picker: Entity<DatePicker>,
    action: impl Fn(&mut DatePicker, &mut Window, &mut Context<DatePicker>) + 'static,
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
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            picker.update(cx, |picker, cx| action(picker, window, cx));
        })
        .child(div().text_sm().child(label.into()))
}

fn nav_button(
    id: impl Into<SharedString>,
    icon: IconName,
    icon_color: Hsla,
    hover_bg: Hsla,
    picker: Entity<DatePicker>,
    action: impl Fn(&mut DatePicker, &mut Context<DatePicker>) + 'static,
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

struct DatePickerBoundsCapturer {
    picker: Entity<DatePicker>,
}

impl IntoElement for DatePickerBoundsCapturer {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for DatePickerBoundsCapturer {
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

fn default_placeholder(picker_type: DatePickerType) -> &'static str {
    match picker_type {
        DatePickerType::Date => "请选择日期",
        DatePickerType::DateRange => "请选择日期范围",
        DatePickerType::Month => "请选择月份",
        DatePickerType::MonthRange => "请选择月份范围",
        DatePickerType::Year => "请选择年份",
        DatePickerType::YearRange => "请选择年份范围",
    }
}

fn default_format(picker_type: DatePickerType) -> &'static str {
    match picker_type {
        DatePickerType::Date | DatePickerType::DateRange => "YYYY-MM-DD",
        DatePickerType::Month | DatePickerType::MonthRange => "YYYY-MM",
        DatePickerType::Year | DatePickerType::YearRange => "YYYY",
    }
}

fn format_date_value(value: DateValue, format: &str) -> String {
    format
        .replace("YYYY", &format!("{:04}", value.year))
        .replace("YY", &format!("{:02}", value.year.rem_euclid(100)))
        .replace("MM", &format!("{:02}", value.month))
        .replace("M", &value.month.to_string())
        .replace("DD", &format!("{:02}", value.day))
        .replace("D", &value.day.to_string())
}

fn normalize_value(value: DateValue, picker_type: DatePickerType) -> DateValue {
    match picker_type {
        DatePickerType::Date | DatePickerType::DateRange => value,
        DatePickerType::Month | DatePickerType::MonthRange => DateValue {
            year: value.year,
            month: value.month,
            day: 1,
        },
        DatePickerType::Year | DatePickerType::YearRange => DateValue {
            year: value.year,
            month: 1,
            day: 1,
        },
    }
}

fn ordered_pair(a: DateValue, b: DateValue) -> (DateValue, DateValue) {
    if a <= b { (a, b) } else { (b, a) }
}

fn is_between(value: DateValue, start: Option<DateValue>, end: Option<DateValue>) -> bool {
    matches!((start, end), (Some(start), Some(end)) if value > start && value < end)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_picker_width_helpers_set_demo_widths() {
        assert_eq!(DatePicker::new().width_md().width, Some(px(260.0)));
        assert_eq!(DatePicker::new().width_lg().width, Some(px(320.0)));
    }
}
