use aura_core::{Config, push_portal};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, Bounds, Context, Element, ElementId, Entity, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, MouseButton, Pixels, Render, SharedString, Window, div, prelude::*, px,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateValue {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

pub struct DatePicker {
    id: SharedString,
    value: Option<DateValue>,
    view_year: i32,
    view_month: u32,
    is_open: bool,
    placeholder: SharedString,
    width: Option<Pixels>,
    disabled: bool,
    last_bounds: Option<Bounds<Pixels>>,
    on_change: Option<Box<dyn Fn(Option<DateValue>, &mut Window, &mut App) + 'static>>,
}

impl DateValue {
    pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
            return None;
        }
        Some(Self { year, month, day })
    }

    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl DatePicker {
    #[track_caller]
    pub fn new() -> Self {
        let caller = std::panic::Location::caller();
        Self {
            id: format!("date-picker-{}", caller).into(),
            value: None,
            view_year: 2026,
            view_month: 5,
            is_open: false,
            placeholder: "请选择日期".into(),
            width: None,
            disabled: false,
            last_bounds: None,
            on_change: None,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn value(mut self, value: DateValue) -> Self {
        self.view_year = value.year;
        self.view_month = value.month;
        self.value = Some(value);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        f: impl Fn(Option<DateValue>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn set_on_change(
        &mut self,
        f: impl Fn(Option<DateValue>, &mut Window, &mut App) + 'static,
        _cx: &mut Context<Self>,
    ) {
        self.on_change = Some(Box::new(f));
    }

    pub fn set_value(&mut self, value: Option<DateValue>, cx: &mut Context<Self>) {
        self.value = value;
        if let Some(value) = value {
            self.view_year = value.year;
            self.view_month = value.month;
        }
        cx.notify();
    }

    pub fn value_ref(&self) -> Option<DateValue> {
        self.value
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

    fn select_date(&mut self, date: DateValue, window: &mut Window, cx: &mut Context<Self>) {
        self.value = Some(date);
        self.view_year = date.year;
        self.view_month = date.month;
        self.is_open = false;
        if let Some(ref on_change) = self.on_change {
            on_change(Some(date), window, cx);
        }
        cx.notify();
    }

    fn shift_month(&mut self, delta: i32, cx: &mut Context<Self>) {
        let month_index = self.view_year * 12 + self.view_month as i32 - 1 + delta;
        self.view_year = month_index.div_euclid(12);
        self.view_month = month_index.rem_euclid(12) as u32 + 1;
        cx.notify();
    }
}

impl Render for DatePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.entity().clone();
        let display = self
            .value
            .map(|value| value.format())
            .unwrap_or_else(|| self.placeholder.to_string());
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
            push_portal(
                move |_window, _cx| {
                    let (top, left, width) = if let Some(bounds) = bounds {
                        (bounds.bottom() + px(4.0), bounds.left(), bounds.size.width)
                    } else {
                        (px(100.0), px(100.0), px(240.0))
                    };

                    div()
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(width.max(px(280.0)))
                        .child(CalendarPanel {
                            id: picker_id,
                            picker: entity,
                        })
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
                    .text_size(px(theme.font_size.md))
                    .text_color(if has_value {
                        theme.neutral.text_1
                    } else {
                        theme.neutral.placeholder
                    })
                    .child(display),
            )
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
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.close(cx);
            }))
    }
}

struct CalendarPanel {
    id: SharedString,
    picker: Entity<DatePicker>,
}

impl IntoElement for CalendarPanel {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for CalendarPanel {
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
        let mut panel = self.render_panel(window, cx);
        (panel.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _rl: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let mut panel = self.render_panel(window, cx);
        panel.prepaint_at(bounds.origin, window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _rl: &mut Self::RequestLayoutState,
        _ps: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mut panel = self.render_panel(window, cx);
        panel.paint(window, cx);
    }
}

impl CalendarPanel {
    fn render_panel(&self, _window: &mut Window, cx: &mut App) -> gpui::AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        let (view_year, view_month, selected) = self.picker.update(cx, |picker, _| {
            (picker.view_year, picker.view_month, picker.value)
        });
        let days = calendar_cells(view_year, view_month);
        let picker_prev = self.picker.clone();
        let picker_next = self.picker.clone();
        let weekdays = ["一", "二", "三", "四", "五", "六", "日"];

        div()
            .id(format!("{}-panel", self.id))
            .cursor_default()
            .occlude()
            .flex()
            .flex_col()
            .p_3()
            .gap_3()
            .bg(theme.neutral.card)
            .border_1()
            .border_color(theme.neutral.border)
            .rounded(px(theme.radius.md))
            .shadow_lg()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .id(format!("{}-prev", self.id))
                            .cursor_pointer()
                            .p_1()
                            .rounded(px(theme.radius.sm))
                            .hover(|s| s.cursor_pointer().bg(theme.neutral.hover))
                            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                picker_prev.update(cx, |picker, cx| picker.shift_month(-1, cx));
                            })
                            .child(
                                Icon::new(IconName::ChevronLeft)
                                    .size(px(18.0))
                                    .color(theme.neutral.icon),
                            ),
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
                            .id(format!("{}-next", self.id))
                            .cursor_pointer()
                            .p_1()
                            .rounded(px(theme.radius.sm))
                            .hover(|s| s.cursor_pointer().bg(theme.neutral.hover))
                            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                picker_next.update(cx, |picker, cx| picker.shift_month(1, cx));
                            })
                            .child(
                                Icon::new(IconName::ChevronRight)
                                    .size(px(18.0))
                                    .color(theme.neutral.icon),
                            ),
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
                        div()
                            .flex()
                            .flex_row()
                            .children(week.iter().enumerate().map(|(day_idx, cell)| {
                                let is_current_month = cell.month == view_month;
                                let is_selected = selected == Some(*cell);
                                let picker = self.picker.clone();
                                let date = *cell;
                                div()
                                    .id(format!("{}-day-{}-{}", self.id, week_idx, day_idx))
                                    .flex_1()
                                    .h(px(34.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .rounded(px(theme.radius.sm))
                                    .bg(if is_selected {
                                        theme.primary.base
                                    } else {
                                        theme.neutral.card
                                    })
                                    .text_color(if is_selected {
                                        theme.neutral.card
                                    } else if is_current_month {
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
                                        picker.update(cx, |picker, cx| {
                                            picker.select_date(date, window, cx);
                                        });
                                    })
                                    .child(div().text_sm().child(cell.day.to_string()))
                            }))
                    })),
            )
            .into_any_element()
    }
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
