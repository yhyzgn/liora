//! Timer module.
//!
//! This public module implements the Liora timer/countdown display component. It keeps the reusable
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
    App, Component, ElementId, Global, IntoElement, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::Config;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control timer direction behavior.
pub enum TimerDirection {
    #[default]
    /// Measures elapsed time upward from the starting duration.
    CountUp,
    /// Counts down toward zero from the supplied duration.
    CountDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control timer unit behavior.
pub enum TimerUnit {
    /// Uses the `Milliseconds` option for `TimerUnit`.
    Milliseconds,
    #[default]
    /// Uses the `Seconds` option for `TimerUnit`.
    Seconds,
    /// Uses the `Minutes` option for `TimerUnit`.
    Minutes,
    /// Uses the `Hours` option for `TimerUnit`.
    Hours,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control timer format behavior.
pub enum TimerFormat {
    /// Uses the `Unit` option for `TimerFormat`.
    Unit,
    /// Uses the `Clock` option for `TimerFormat`.
    Clock,
}

impl Default for TimerFormat {
    fn default() -> Self {
        Self::Unit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Fluent native GPUI component for rendering Liora timer snapshot.
pub struct TimerSnapshot {
    /// Elapsed duration tracked by a timer.
    pub elapsed: Duration,
    /// Remaining duration before a timer completes.
    pub remaining: Option<Duration>,
    /// Whether a countdown timer has reached completion.
    pub finished: bool,
}

impl TimerSnapshot {
    /// Performs the elapsed as operation used by this component.
    pub fn elapsed_as(self, unit: TimerUnit) -> f64 {
        duration_as(self.elapsed, unit)
    }

    /// Performs the remaining as operation used by this component.
    pub fn remaining_as(self, unit: TimerUnit) -> Option<f64> {
        self.remaining.map(|remaining| duration_as(remaining, unit))
    }
}

#[derive(Clone)]
/// Fluent native GPUI component for rendering Liora timer.
pub struct Timer {
    id: SharedString,
    elapsed: Duration,
    duration: Option<Duration>,
    direction: TimerDirection,
    display_unit: TimerUnit,
    format: TimerFormat,
    show_unit: bool,
    title: Option<SharedString>,
    prefix: Option<SharedString>,
    suffix: Option<SharedString>,
    compact: bool,
    running: bool,
    started_at: Option<Instant>,
    tick_interval: Duration,
}

impl Timer {
    /// Sets the count up value used by the component.
    pub fn count_up(elapsed: Duration) -> Self {
        Self::new(TimerDirection::CountUp, elapsed, None)
    }

    /// Sets the count down value used by the component.
    pub fn count_down(duration: Duration, elapsed: Duration) -> Self {
        Self::new(TimerDirection::CountDown, elapsed, Some(duration))
    }

    /// Creates `Timer` initialized from the supplied direction, elapsed, and duration.
    pub fn new(direction: TimerDirection, elapsed: Duration, duration: Option<Duration>) -> Self {
        Self {
            id: liora_core::unique_id("timer"),
            elapsed,
            duration,
            direction,
            display_unit: TimerUnit::Seconds,
            format: TimerFormat::Unit,
            show_unit: true,
            title: None,
            prefix: None,
            suffix: None,
            compact: false,
            running: false,
            started_at: None,
            tick_interval: Duration::from_millis(250),
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the elapsed value used by the component.
    pub fn elapsed(mut self, elapsed: Duration) -> Self {
        self.elapsed = elapsed;
        self
    }

    /// Sets the duration value used by the component.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Selects the layout or animation direction.
    pub fn direction(mut self, direction: TimerDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the countup value used by the component.
    pub fn countup(mut self) -> Self {
        self.direction = TimerDirection::CountUp;
        self
    }

    /// Sets the countdown value used by the component.
    pub fn countdown(mut self) -> Self {
        self.direction = TimerDirection::CountDown;
        self
    }

    /// Sets the display unit value used by the component.
    pub fn display_unit(mut self, unit: TimerUnit) -> Self {
        self.display_unit = unit;
        self.format = TimerFormat::Unit;
        self
    }

    /// Sets the format displayed or consumed by the component.
    pub fn format(mut self, format: TimerFormat) -> Self {
        self.format = format;
        self
    }

    /// Sets the clock format value used by the component.
    pub fn clock_format(mut self) -> Self {
        self.format = TimerFormat::Clock;
        self
    }

    /// Configures whether unit is visible in the rendered component.
    pub fn show_unit(mut self, show: bool) -> Self {
        self.show_unit = show;
        self
    }

    /// Sets the primary title text displayed by the component.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the prefix value used by the component.
    pub fn prefix(mut self, prefix: impl Into<SharedString>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Sets the suffix value used by the component.
    pub fn suffix(mut self, suffix: impl Into<SharedString>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Sets the compact value used by the component.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// Sets the running value used by the component.
    pub fn running(mut self, running: bool) -> Self {
        self.running = running;
        if running && self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }
        self
    }

    /// Sets the start value used by the component.
    pub fn start(self) -> Self {
        self.running(true)
    }

    /// Sets the paused value used by the component.
    pub fn paused(self) -> Self {
        self.running(false)
    }

    /// Sets the tick interval value used by the component.
    pub fn tick_interval(mut self, interval: Duration) -> Self {
        self.tick_interval = interval.max(Duration::from_millis(16));
        self
    }

    fn effective_elapsed(&self) -> Duration {
        if self.running {
            self.started_at
                .map(|started_at| self.elapsed.saturating_add(started_at.elapsed()))
                .unwrap_or(self.elapsed)
        } else {
            self.elapsed
        }
    }

    /// Performs the snapshot operation used by this component.
    pub fn snapshot(&self) -> TimerSnapshot {
        let remaining = self
            .duration
            .map(|duration| duration.saturating_sub(self.effective_elapsed()));
        TimerSnapshot {
            elapsed: self.effective_elapsed(),
            remaining,
            finished: matches!(self.direction, TimerDirection::CountDown)
                && remaining.is_some_and(|remaining| remaining.is_zero()),
        }
    }

    /// Performs the elapsed as operation used by this component.
    pub fn elapsed_as(&self, unit: TimerUnit) -> f64 {
        self.snapshot().elapsed_as(unit)
    }

    /// Performs the remaining as operation used by this component.
    pub fn remaining_as(&self, unit: TimerUnit) -> Option<f64> {
        self.snapshot().remaining_as(unit)
    }

    fn display_duration(&self) -> Duration {
        match self.direction {
            TimerDirection::CountUp => self.effective_elapsed(),
            TimerDirection::CountDown => self
                .duration
                .map(|duration| duration.saturating_sub(self.effective_elapsed()))
                .unwrap_or_default(),
        }
    }

    fn format_value(&self) -> SharedString {
        match self.format {
            TimerFormat::Unit => {
                format_duration(self.display_duration(), self.display_unit, self.show_unit)
            }
            TimerFormat::Clock => format_clock(self.display_duration()),
        }
    }
}

impl RenderOnce for Timer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let mut timer = self;
        if timer.running {
            ensure_timer_runtime(cx);
            if let Some(runtime) = cx.try_global::<TimerRuntime>() {
                timer.started_at = Some(runtime.started_at(timer.id.clone()));
            }
        }
        if timer.running && !timer.snapshot().finished {
            if let Some(runtime) = cx.try_global::<TimerRuntime>() {
                runtime.register(window.window_handle(), timer.tick_interval);
            }
        }
        let value = timer.format_value();
        div()
            .id(ElementId::from(timer.id))
            .flex()
            .flex_col()
            .gap_1()
            .when(!timer.compact, |s| {
                s.p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.card)
            })
            .when_some(timer.title, |s, title| {
                s.child(
                    div()
                        .text_xs()
                        .text_color(theme.neutral.text_3)
                        .child(title),
                )
            })
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap_1()
                    .text_color(theme.neutral.text_1)
                    .when_some(timer.prefix, |s, prefix| {
                        s.child(
                            div()
                                .text_sm()
                                .text_color(theme.neutral.text_3)
                                .child(prefix),
                        )
                    })
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(value),
                    )
                    .when_some(timer.suffix, |s, suffix| {
                        s.child(
                            div()
                                .text_sm()
                                .text_color(theme.neutral.text_3)
                                .child(suffix),
                        )
                    }),
            )
    }
}

#[derive(Clone)]
struct TimerRuntime {
    windows: Arc<Mutex<HashSet<gpui::AnyWindowHandle>>>,
    starts: Arc<Mutex<HashMap<SharedString, Instant>>>,
}

impl Global for TimerRuntime {}

impl TimerRuntime {
    fn new(cx: &mut App) -> Self {
        let windows = Arc::new(Mutex::new(HashSet::new()));
        let runtime = Self {
            windows: windows.clone(),
            starts: Arc::new(Mutex::new(HashMap::new())),
        };
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx: &mut gpui::AsyncApp| {
            loop {
                executor.timer(Duration::from_millis(250)).await;
                let handles = lock_timer_windows(&windows)
                    .iter()
                    .copied()
                    .collect::<Vec<_>>();
                for handle in handles {
                    let _ = handle.update(cx, |_, window, _| window.refresh());
                }
            }
        })
        .detach();
        runtime
    }

    fn started_at(&self, id: SharedString) -> Instant {
        let mut starts = lock_timer_starts(&self.starts);
        *starts.entry(id).or_insert_with(Instant::now)
    }

    fn register(&self, window: gpui::AnyWindowHandle, _interval: Duration) {
        lock_timer_windows(&self.windows).insert(window);
    }
}

fn lock_timer_windows(
    windows: &Arc<Mutex<HashSet<gpui::AnyWindowHandle>>>,
) -> MutexGuard<'_, HashSet<gpui::AnyWindowHandle>> {
    windows
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn lock_timer_starts(
    starts: &Arc<Mutex<HashMap<SharedString, Instant>>>,
) -> MutexGuard<'_, HashMap<SharedString, Instant>> {
    starts
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn ensure_timer_runtime(cx: &mut App) {
    if !cx.has_global::<TimerRuntime>() {
        let runtime = TimerRuntime::new(cx);
        cx.set_global(runtime);
    }
}

impl IntoElement for Timer {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

/// Performs the duration as operation used by this component.
pub fn duration_as(duration: Duration, unit: TimerUnit) -> f64 {
    match unit {
        TimerUnit::Milliseconds => duration.as_secs_f64() * 1000.0,
        TimerUnit::Seconds => duration.as_secs_f64(),
        TimerUnit::Minutes => duration.as_secs_f64() / 60.0,
        TimerUnit::Hours => duration.as_secs_f64() / 3600.0,
    }
}

/// Formats duration for display.
pub fn format_duration(duration: Duration, unit: TimerUnit, show_unit: bool) -> SharedString {
    let value = duration_as(duration, unit);
    let text = match unit {
        TimerUnit::Milliseconds => format!("{value:.0}"),
        TimerUnit::Seconds => format!("{value:.1}"),
        TimerUnit::Minutes => format!("{value:.2}"),
        TimerUnit::Hours => format!("{value:.2}"),
    };
    if show_unit {
        format!("{} {}", text, unit_label(unit)).into()
    } else {
        text.into()
    }
}

/// Formats clock for display.
pub fn format_clock(duration: Duration) -> SharedString {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}").into()
}

fn unit_label(unit: TimerUnit) -> &'static str {
    match unit {
        TimerUnit::Milliseconds => "ms",
        TimerUnit::Seconds => "s",
        TimerUnit::Minutes => "min",
        TimerUnit::Hours => "h",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_snapshot_tracks_countdown_remaining() {
        let timer = Timer::count_down(Duration::from_secs(10), Duration::from_secs(4));
        let snapshot = timer.snapshot();
        assert_eq!(snapshot.elapsed, Duration::from_secs(4));
        assert_eq!(snapshot.remaining, Some(Duration::from_secs(6)));
        assert!(!snapshot.finished);
    }

    #[test]
    fn running_timer_includes_elapsed_since_start() {
        let timer = Timer::count_up(Duration::from_secs(2)).start();
        assert!(timer.effective_elapsed() >= Duration::from_secs(2));
        assert!(timer.running);
    }

    #[test]
    fn timer_countdown_saturates_at_zero() {
        let timer = Timer::count_down(Duration::from_secs(10), Duration::from_secs(12));
        let snapshot = timer.snapshot();
        assert_eq!(snapshot.remaining, Some(Duration::ZERO));
        assert!(snapshot.finished);
    }

    #[test]
    fn timer_formats_units() {
        assert_eq!(
            format_duration(Duration::from_millis(1500), TimerUnit::Milliseconds, true),
            SharedString::from("1500 ms")
        );
        assert_eq!(
            format_duration(Duration::from_secs(90), TimerUnit::Minutes, true),
            SharedString::from("1.50 min")
        );
        assert_eq!(
            Timer::count_up(Duration::from_secs(7200)).elapsed_as(TimerUnit::Hours),
            2.0
        );
    }

    #[test]
    fn timer_formats_clock() {
        assert_eq!(
            format_clock(Duration::from_secs(0)),
            SharedString::from("00:00:00")
        );
        assert_eq!(
            format_clock(Duration::from_secs(3661)),
            SharedString::from("01:01:01")
        );
        assert_eq!(
            Timer::count_up(Duration::from_secs(3661))
                .clock_format()
                .format_value(),
            SharedString::from("01:01:01")
        );
    }
}
