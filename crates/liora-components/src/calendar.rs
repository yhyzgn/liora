//! Calendar module.
//!
//! This public module implements the Liora calendar surface for month grids and date event rendering. It keeps the reusable
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
use gpui::{
    App, ElementId, Hsla, IntoElement, RenderOnce, SharedString, ViewElement as Component, Window,
    div, prelude::*, px,
};
use liora_core::{Config, LocalesContext, locales, tr};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::{
    collections::{BTreeMap, HashSet},
    panic::Location,
    sync::Arc,
};

pub(crate) fn weekday_labels(cx: &impl LocalesContext) -> [SharedString; 7] {
    use locales::calendar::*;
    [monday(), tuesday(), wednesday(), thursday(), friday(), saturday(), sunday()]
        .map(|key| tr(cx, key))
}

pub(crate) fn month_heading(year: i32, month: u32, cx: &impl LocalesContext) -> String {
    tr(cx, locales::calendar::month_heading)
        .replace("{year}", &year.to_string())
        .replace("{month}", &format!("{month:02}"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Fluent native GPUI component for rendering Liora calendar date.
pub struct CalendarDate {
    /// Four-digit calendar year.
    pub year: i32,
    /// One-based calendar month.
    pub month: u32,
    /// One-based day within the month.
    pub day: u32,
}

impl CalendarDate {
    /// Creates `CalendarDate` initialized from the supplied year, month, and day.
    pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
            return None;
        }
        Some(Self { year, month, day })
    }

    /// Sets the today demo value used by the component.
    pub fn today_demo() -> Self {
        Self {
            year: 2026,
            month: 6,
            day: 16,
        }
    }
    /// Performs the format operation used by this component.
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[derive(Clone)]
/// Fluent native GPUI component for rendering Liora calendar event.
pub struct CalendarEvent {
    /// Calendar date associated with this value.
    pub date: CalendarDate,
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Color token or explicit color applied to the visual element.
    pub color: Option<Hsla>,
}

impl CalendarEvent {
    /// Creates `CalendarEvent` initialized from the supplied date, and label.
    pub fn new(date: CalendarDate, label: impl Into<SharedString>) -> Self {
        Self {
            date,
            label: label.into(),
            color: None,
        }
    }
    /// Applies an explicit color instead of the theme-derived default.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

/// Fluent native GPUI component for rendering Liora calendar.
pub struct Calendar {
    id: ElementId,
    year: i32,
    month: u32,
    selected: Option<CalendarDate>,
    range_start: Option<CalendarDate>,
    range_end: Option<CalendarDate>,
    disabled_dates: HashSet<CalendarDate>,
    events: Vec<CalendarEvent>,
    show_adjacent_months: bool,
    on_select: Option<Arc<dyn Fn(CalendarDate, &mut Window, &mut App) + 'static>>,
}

impl Calendar {
    /// Creates `Calendar` initialized from the supplied year, and month.
    #[track_caller]
    pub fn new(year: i32, month: u32) -> Self {
        let month = month.clamp(1, 12);
        Self {
            id: Location::caller().into(),
            year,
            month,
            selected: None,
            range_start: None,
            range_end: None,
            disabled_dates: HashSet::new(),
            events: Vec::new(),
            show_adjacent_months: true,
            on_select: None,
        }
    }

    /// Sets a stable identity for calendars built at the same call site in the same parent.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the current selected state.
    pub fn selected(mut self, date: CalendarDate) -> Self {
        self.selected = Some(date);
        self
    }
    /// Sets the range value used by the component.
    pub fn range(mut self, start: CalendarDate, end: CalendarDate) -> Self {
        let (a, b) = ordered_pair(start, end);
        self.range_start = Some(a);
        self.range_end = Some(b);
        self
    }
    /// Sets the disabled dates value used by the component.
    pub fn disabled_dates(mut self, dates: impl IntoIterator<Item = CalendarDate>) -> Self {
        self.disabled_dates = dates.into_iter().collect();
        self
    }
    /// Replaces the component events collection.
    pub fn events(mut self, events: impl IntoIterator<Item = CalendarEvent>) -> Self {
        self.events = events.into_iter().collect();
        self
    }
    /// Configures whether adjacent months is visible in the rendered component.
    pub fn show_adjacent_months(mut self, show: bool) -> Self {
        self.show_adjacent_months = show;
        self
    }
    /// Registers a callback that runs when select occurs.
    pub fn on_select(mut self, cb: impl Fn(CalendarDate, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Arc::new(cb));
        self
    }
    /// Performs the cells operation used by this component.
    pub fn cells(&self) -> Vec<CalendarDate> {
        calendar_cells(self.year, self.month)
    }
}

impl RenderOnce for Calendar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let events = group_events(&self.events);
        let weekdays = weekday_labels(cx);
        let cells = self.cells();
        let month_title = month_heading(self.year, self.month, cx);
        let on_select = self.on_select.clone();

        div()
            .id(self.id)
            .w_full()
            .rounded_lg()
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .p_4()
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
                            .gap_2()
                            .child(
                                Icon::new(IconName::CalendarDays)
                                    .size(px(18.0))
                                    .color(theme.primary.base),
                            )
                            .child(div().font_weight(gpui::FontWeight::BOLD).child(month_title)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.neutral.text_3)
                            .child(tr(cx, locales::calendar::month_view)),
                    ),
            )
            .child(
                div()
                    .grid()
                    .grid_cols(7)
                    .gap_2()
                    .children(weekdays.into_iter().map(|day| {
                        div()
                            .text_xs()
                            .text_color(theme.neutral.text_3)
                            .text_center()
                            .child(day)
                            .into_any_element()
                    })),
            )
            .child(
                div()
                    .grid()
                    .grid_cols(7)
                    .gap_2()
                    .children(cells.into_iter().map(|date| {
                        let in_month = date.month == self.month;
                        if !self.show_adjacent_months && !in_month {
                            return div().min_h(px(70.0)).into_any_element();
                        }
                        let selected = self.selected == Some(date);
                        let disabled = self.disabled_dates.contains(&date);
                        let in_range = is_between_or_edge(date, self.range_start, self.range_end);
                        let day_events = events.get(&date).cloned().unwrap_or_default();
                        let mut cell = div()
                            .id(element_id(format!("calendar-cell-{}", date.format())))
                            .debug_selector(move || format!("calendar-cell-{}", date.format()))
                            .min_h(px(70.0))
                            .rounded_md()
                            .border_1()
                            .border_color(if selected {
                                theme.primary.base
                            } else {
                                theme.neutral.border.opacity(0.5)
                            })
                            .bg(if selected {
                                theme.primary.base.opacity(0.14)
                            } else if in_range {
                                theme.primary.base.opacity(0.08)
                            } else {
                                gpui::transparent_black()
                            })
                            .p_2()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .text_color(if disabled {
                                theme.neutral.text_disabled
                            } else if in_month {
                                theme.neutral.text_1
                            } else {
                                theme.neutral.text_3.opacity(0.55)
                            })
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(if selected {
                                        gpui::FontWeight::BOLD
                                    } else {
                                        gpui::FontWeight::NORMAL
                                    })
                                    .debug_selector(move || {
                                        format!("calendar-day-{}", date.format())
                                    })
                                    .child(date.day.to_string()),
                            )
                            .children(day_events.into_iter().take(2).map(|event| {
                                let color = event.color.unwrap_or(theme.primary.base);
                                div()
                                    .truncate()
                                    .text_xs()
                                    .text_color(color)
                                    .child(format!("• {}", event.label))
                                    .into_any_element()
                            }));
                        if disabled {
                            cell = cell.cursor_not_allowed().opacity(0.55);
                        } else {
                            let select_handler = on_select.clone();
                            cell = cell
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.neutral.hover))
                                .on_click(move |_, window, cx| {
                                    if let Some(cb) = select_handler.clone() {
                                        cb(date, window, cx);
                                    }
                                });
                        }
                        cell.into_any_element()
                    })),
            )
    }
}

impl IntoElement for Calendar {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn group_events(events: &[CalendarEvent]) -> BTreeMap<CalendarDate, Vec<CalendarEvent>> {
    let mut grouped = BTreeMap::<CalendarDate, Vec<CalendarEvent>>::new();
    for event in events {
        grouped.entry(event.date).or_default().push(event.clone());
    }
    grouped
}

fn ordered_pair(a: CalendarDate, b: CalendarDate) -> (CalendarDate, CalendarDate) {
    if a <= b { (a, b) } else { (b, a) }
}
fn is_between_or_edge(
    value: CalendarDate,
    start: Option<CalendarDate>,
    end: Option<CalendarDate>,
) -> bool {
    matches!((start,end), (Some(start), Some(end)) if value >= start && value <= end)
}
fn calendar_cells(year: i32, month: u32) -> Vec<CalendarDate> {
    let first_weekday = weekday_monday_based(year, month, 1);
    let prev_month_index = year * 12 + month as i32 - 2;
    let prev_year = prev_month_index.div_euclid(12);
    let prev_month = prev_month_index.rem_euclid(12) as u32 + 1;
    let current_days = days_in_month(year, month);
    let prev_days = days_in_month(prev_year, prev_month);
    let mut cells = Vec::with_capacity(42);
    for i in (0..first_weekday).rev() {
        cells.push(CalendarDate {
            year: prev_year,
            month: prev_month,
            day: prev_days - i,
        });
    }
    for day in 1..=current_days {
        cells.push(CalendarDate { year, month, day });
    }
    let next_month_index = year * 12 + month as i32;
    let next_year = next_month_index.div_euclid(12);
    let next_month = next_month_index.rem_euclid(12) as u32 + 1;
    let mut next_day = 1;
    while cells.len() < 42 {
        cells.push(CalendarDate {
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
    fn calendar_labels_follow_locale_without_changing_monday_grid() {
        for (locale, monday, sunday, heading, view) in [
            ("en-US", "Mon", "Sun", "2040-02", "Month view"),
            ("zh-CN", "一", "日", "2040年 02月", "月视图"),
            ("en-US", "Mon", "Sun", "2040-02", "Month view"),
        ] {
            let cx = liora_core::LocalesConfig::system().with_locale(locale);
            let labels = weekday_labels(&cx);
            assert_eq!(labels[0].as_ref(), monday);
            assert_eq!(labels[6].as_ref(), sunday);
            assert_eq!(month_heading(2040, 2, &cx), heading);
            assert_eq!(tr(&cx, locales::calendar::month_view).as_ref(), view);
            let cells = Calendar::new(2040, 2).cells();
            assert_eq!(cells.len(), 42);
            assert_eq!(weekday_monday_based(cells[0].year, cells[0].month, cells[0].day), 0);
        }
    }
    #[test]
    fn calendar_month_grid_is_always_six_weeks() {
        let cells = Calendar::new(2026, 6).cells();
        assert_eq!(cells.len(), 42);
        assert_eq!(
            cells[0],
            CalendarDate {
                year: 2026,
                month: 6,
                day: 1
            }
        );
    }
    #[test]
    fn calendar_leap_year_and_range_work() {
        let feb = CalendarDate::new(2024, 2, 29).unwrap();
        assert_eq!(feb.day, 29);
        assert!(CalendarDate::new(2023, 2, 29).is_none());
        assert!(is_between_or_edge(
            feb,
            CalendarDate::new(2024, 2, 28),
            CalendarDate::new(2024, 3, 1)
        ));
    }

    struct CalendarInteractionTest {
        show_adjacent: bool,
        selections: std::rc::Rc<std::cell::RefCell<Vec<CalendarDate>>>,
    }

    impl gpui::Render for CalendarInteractionTest {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            let selections = self.selections.clone();
            div().child(
                Calendar::new(2026, 6)
                    .show_adjacent_months(self.show_adjacent)
                    .disabled_dates([CalendarDate {
                        year: 2026,
                        month: 7,
                        day: 2,
                    }])
                    .on_select(move |date, _, _| selections.borrow_mut().push(date)),
            )
        }
    }

    fn click(window: &mut gpui::VisualTestContext, position: gpui::Point<gpui::Pixels>) {
        window.update(|window, cx| {
            window.dispatch_event(
                gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                    position,
                    button: gpui::MouseButton::Left,
                    modifiers: gpui::Modifiers::none(),
                    click_count: 1,
                    first_mouse: false,
                }),
                cx,
            );
            window.draw(cx).clear(cx);
            window.dispatch_event(
                gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                    position,
                    button: gpui::MouseButton::Left,
                    modifiers: gpui::Modifiers::none(),
                    click_count: 1,
                }),
                cx,
            );
        });
    }

    #[test]
    fn hidden_adjacent_dates_stay_hidden_and_cannot_be_selected() -> Result<(), String> {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let selections = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let handle = app.add_window({
            let selections = selections.clone();
            move |_, _| CalendarInteractionTest {
                selections,
                show_adjacent: true,
            }
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));

        let visible = window
            .debug_bounds("calendar-cell-2026-06-01")
            .ok_or("missing visible June 1 cell")?;
        let hidden_enabled = window
            .debug_bounds("calendar-cell-2026-07-01")
            .ok_or("missing visible July 1 cell")?;
        let hidden_disabled = window
            .debug_bounds("calendar-cell-2026-07-02")
            .ok_or("missing visible July 2 cell")?;
        assert_eq!(hidden_enabled.size.height, visible.size.height);
        assert_eq!(hidden_disabled.size.height, visible.size.height);
        assert!(window.debug_bounds("calendar-day-2026-07-02").is_some());
        click(&mut window, hidden_disabled.center());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(selections.borrow().is_empty());
        click(&mut window, hidden_enabled.center());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            *selections.borrow(),
            [CalendarDate {
                year: 2026,
                month: 7,
                day: 1,
            }]
        );
        selections.borrow_mut().clear();

        handle
            .update(&mut app, |surface, _, cx| {
                surface.show_adjacent = false;
                cx.notify();
            })
            .map_err(|error| error.to_string())?;
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            window.debug_bounds("calendar-cell-2026-06-01"),
            Some(visible)
        );

        click(&mut window, hidden_enabled.center());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            selections.borrow().is_empty(),
            "hidden adjacent date selected"
        );

        click(&mut window, hidden_disabled.center());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            selections.borrow().is_empty(),
            "hidden disabled date selected"
        );
        assert!(window.debug_bounds("calendar-cell-2026-07-01").is_none());
        assert!(window.debug_bounds("calendar-cell-2026-07-02").is_none());
        assert!(window.debug_bounds("calendar-day-2026-07-01").is_none());
        assert!(window.debug_bounds("calendar-day-2026-07-02").is_none());

        click(&mut window, visible.center());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            *selections.borrow(),
            [CalendarDate {
                year: 2026,
                month: 6,
                day: 1,
            }]
        );
        app.quit();
        Ok(())
    }

    struct SiblingCalendarsTest {
        selections: std::rc::Rc<std::cell::RefCell<Vec<(usize, CalendarDate)>>>,
    }

    impl gpui::Render for SiblingCalendarsTest {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            let selections = self.selections.clone();
            let first = Calendar::new(2026, 6)
                .on_select(move |date, _, _| selections.borrow_mut().push((0, date)));
            let selections = self.selections.clone();
            let second = Calendar::new(2026, 6)
                .on_select(move |date, _, _| selections.borrow_mut().push((1, date)));
            // Debug selectors measure siblings without adding identity namespaces.
            div()
                .flex()
                .w(px(800.0))
                .child(
                    div()
                        .w(px(400.0))
                        .debug_selector(|| "calendar-first".into())
                        .child(first),
                )
                .child(
                    div()
                        .w(px(400.0))
                        .debug_selector(|| "calendar-second".into())
                        .child(second),
                )
        }
    }

    #[test]
    fn sibling_calendars_keep_selections_independent_across_redraw() -> Result<(), String> {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let selections = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let handle = app.add_window({
            let selections = selections.clone();
            move |_, _| SiblingCalendarsTest { selections }
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        let first = window
            .debug_bounds("calendar-first")
            .ok_or("missing first Calendar")?;
        let second = window
            .debug_bounds("calendar-second")
            .ok_or("missing second Calendar")?;
        assert_eq!(first.size, second.size);
        let cell = window
            .debug_bounds("calendar-cell-2026-06-01")
            .ok_or("missing second June 1")?;
        assert!(cell.center().x > second.origin.x);
        let first_position = gpui::point(
            cell.center().x + first.origin.x - second.origin.x,
            cell.center().y + first.origin.y - second.origin.y,
        );
        let date = CalendarDate {
            year: 2026,
            month: 6,
            day: 1,
        };
        click(&mut window, first_position);
        assert_eq!(*selections.borrow(), [(0, date)]);
        window.update(|window, cx| window.draw(cx).clear(cx));
        click(&mut window, cell.center());
        assert_eq!(*selections.borrow(), [(0, date), (1, date)]);
        app.quit();
        Ok(())
    }
}
