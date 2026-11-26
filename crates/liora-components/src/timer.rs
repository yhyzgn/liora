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
    App, ElementId, Global, IntoElement, RenderOnce, SharedString, Task, ViewElement as Component,
    Window, div, prelude::*, px,
};
use liora_core::Config;
use std::{
    collections::HashMap,
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

    /// Sets the window refresh interval, clamped to at least 16ms; faster sibling timers may refresh it sooner.
    /// Intervals beyond the platform clock range do not schedule a wake.
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
        let elapsed = self.effective_elapsed();
        let remaining = self
            .duration
            .map(|duration| duration.saturating_sub(elapsed));
        TimerSnapshot {
            elapsed,
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
                let started_at = timer.started_at.unwrap_or_else(Instant::now);
                timer.started_at = Some(runtime.started_at(timer.id.clone(), started_at));
            }
        }
        if timer.running && !timer.snapshot().finished {
            if let Some(runtime) = cx.try_global::<TimerRuntime>().cloned() {
                runtime.register(window.window_handle(), timer.tick_interval, cx);
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
    windows: Arc<Mutex<HashMap<gpui::AnyWindowHandle, TimerWindow>>>,
    // shortcut: stable-id render builders retain epochs for the app lifetime until an explicit owner/reset contract exists.
    starts: Arc<Mutex<HashMap<SharedString, Instant>>>,
}

struct TimerWindow {
    deadline: Instant,
    task: Task<()>,
}

impl Global for TimerRuntime {}

impl TimerRuntime {
    fn new() -> Self {
        Self {
            windows: Arc::new(Mutex::new(HashMap::new())),
            starts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn started_at(&self, id: SharedString, started_at: Instant) -> Instant {
        let mut starts = lock_timer_starts(&self.starts);
        *starts.entry(id).or_insert(started_at)
    }

    fn register(&self, window: gpui::AnyWindowHandle, interval: Duration, cx: &mut App) {
        let executor = cx.background_executor();
        let Some(deadline) = executor.now().checked_add(interval) else {
            return;
        };
        let mut windows = lock_timer_windows(&self.windows);
        if windows
            .get(&window)
            .is_some_and(|scheduled| scheduled.deadline <= deadline)
        {
            return;
        }

        let timer = executor.timer(interval);
        let pending = Arc::downgrade(&self.windows);
        let task = cx.spawn(async move |cx| {
            timer.await;
            let Some(pending) = pending.upgrade() else {
                return;
            };
            // Retire before refreshing so only timers still rendered schedule the next wake.
            if let Some(scheduled) = lock_timer_windows(&pending).remove(&window) {
                scheduled.task.detach();
            }
            let _ = window.update(cx, |_, window, _| window.refresh());
        });
        // Replacing the retained task cancels a later wake for this window.
        windows.insert(window, TimerWindow { deadline, task });
    }
}

fn lock_timer_windows(
    windows: &Arc<Mutex<HashMap<gpui::AnyWindowHandle, TimerWindow>>>,
) -> MutexGuard<'_, HashMap<gpui::AnyWindowHandle, TimerWindow>> {
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
        let runtime = TimerRuntime::new();
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
    use std::{cell::Cell, rc::Rc};

    struct TimerSurface {
        intervals: Vec<Duration>,
        running: Vec<bool>,
        visible: Vec<bool>,
        renders: Rc<Cell<usize>>,
    }

    impl gpui::Render for TimerSurface {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div().children(
                self.intervals
                    .iter()
                    .enumerate()
                    .filter_map(|(index, interval)| {
                        self.visible[index].then(|| {
                            Timer::count_up(Duration::ZERO)
                                .id(format!("mounted-timer-{index}"))
                                .running(self.running[index])
                                .tick_interval(*interval)
                        })
                    }),
            )
        }
    }

    struct TimerRoot<T: gpui::Render>(gpui::Entity<T>);

    impl<T: gpui::Render> gpui::Render for TimerRoot<T> {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            self.0.clone().cached(Default::default())
        }
    }

    fn serve_headless_frame(window: &mut gpui::VisualTestContext) {
        // Serve the public headless frame path; do not invalidate the cached root ourselves.
        let settled = (0..100).any(|_| !window.executor().tick());
        assert!(settled, "timer tasks must settle without a polling loop");
        window.update(|window, cx| window.draw(cx).clear(cx));
    }

    #[test]
    fn retained_countdown_finished_before_first_render_does_not_restart() {
        struct RetainedCountdown {
            timer: Timer,
            renders: Rc<Cell<usize>>,
        }

        impl gpui::Render for RetainedCountdown {
            fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
                self.renders.set(self.renders.get() + 1);
                self.timer.clone()
            }
        }

        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let renders = Rc::new(Cell::new(0));
        let mut timer = Timer::count_down(Duration::from_secs(3600), Duration::ZERO).start();
        timer.started_at = Some(Instant::now() - Duration::from_secs(7200));
        assert!(timer.snapshot().finished);
        let handle = app.add_window(|_, cx| {
            TimerRoot(cx.new(|_| RetainedCountdown {
                timer,
                renders: renders.clone(),
            }))
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        serve_headless_frame(&mut window);
        let initial = renders.get();
        assert!(initial > 0);
        app.executor().advance_clock(Duration::from_secs(1));
        serve_headless_frame(&mut window);
        assert_eq!(
            renders.get(),
            initial,
            "a finished retained timer must not restart on first render"
        );
        app.quit();
    }

    #[test]
    fn mounted_timer_refreshes_cached_root_at_requested_interval_and_rearms() {
        for millis in [50, 250, 500] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let renders = Rc::new(Cell::new(0));
            let handle = app.add_window(|_, cx| {
                TimerRoot(cx.new(|_| TimerSurface {
                    intervals: vec![Duration::from_millis(millis)],
                    running: vec![true],
                    visible: vec![true],
                    renders: renders.clone(),
                }))
            });
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
            serve_headless_frame(&mut window);
            let initial = renders.get();
            assert!(initial > 0);
            for tick in 1..=3 {
                app.executor()
                    .advance_clock(Duration::from_millis(millis - 1));
                serve_headless_frame(&mut window);
                assert_eq!(
                    renders.get(),
                    initial + tick - 1,
                    "no early {millis}ms redraw"
                );
                app.executor().advance_clock(Duration::from_millis(1));
                serve_headless_frame(&mut window);
                assert_eq!(
                    renders.get(),
                    initial + tick,
                    "{millis}ms wake must invalidate the mounted root and rearm on render"
                );
            }
            app.quit();
        }
    }

    #[test]
    fn unrepresentable_interval_stays_idle_without_displacing_sibling_wakes() {
        let fast = Duration::from_millis(50);
        for intervals in [
            vec![Duration::MAX],
            vec![Duration::MAX, fast],
            vec![fast, Duration::MAX],
        ] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let renders = Rc::new(Cell::new(0));
            let has_sibling = intervals.contains(&fast);
            let handle = app.add_window(|_, cx| {
                TimerRoot(cx.new(|_| TimerSurface {
                    running: vec![true; intervals.len()],
                    visible: vec![true; intervals.len()],
                    intervals,
                    renders: renders.clone(),
                }))
            });
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
            serve_headless_frame(&mut window);
            let initial = renders.get();
            assert!(initial > 0);
            for tick in 1..=3 {
                app.executor().advance_clock(fast);
                serve_headless_frame(&mut window);
                assert_eq!(renders.get(), initial + if has_sibling { tick } else { 0 });
            }
            app.quit();
        }
    }

    #[test]
    fn mounted_timer_pause_unmount_and_close_stop_renewal_without_stopping_siblings() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let renders = Rc::new(Cell::new(0));
        let handle = app.add_window(|_, cx| {
            TimerRoot(cx.new(|_| TimerSurface {
                intervals: vec![Duration::from_millis(500), Duration::from_millis(50)],
                running: vec![true, true],
                visible: vec![true, true],
                renders: renders.clone(),
            }))
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        serve_headless_frame(&mut window);
        let initial = renders.get();
        app.executor().advance_clock(Duration::from_millis(50));
        serve_headless_frame(&mut window);
        assert_eq!(renders.get(), initial + 1);

        // Change host state without notify: the outstanding wake must perform the redraw.
        assert!(
            handle
                .update(&mut app, |root, _, cx| {
                    root.0.update(cx, |surface, _| surface.running[1] = false)
                })
                .is_ok()
        );
        app.executor().advance_clock(Duration::from_millis(50));
        serve_headless_frame(&mut window);
        let paused = renders.get();
        assert_eq!(paused, initial + 2);
        app.executor().advance_clock(Duration::from_millis(499));
        serve_headless_frame(&mut window);
        assert_eq!(renders.get(), paused);
        app.executor().advance_clock(Duration::from_millis(1));
        serve_headless_frame(&mut window);
        assert_eq!(
            renders.get(),
            paused + 1,
            "remaining sibling keeps its 500ms wake"
        );

        assert!(
            handle
                .update(&mut app, |root, _, cx| {
                    root.0.update(cx, |surface, _| surface.visible[0] = false)
                })
                .is_ok()
        );
        app.executor().advance_clock(Duration::from_millis(500));
        serve_headless_frame(&mut window);
        let unmounted = renders.get();
        assert_eq!(
            unmounted,
            paused + 2,
            "one pending wake may retire after unmount"
        );
        app.executor().advance_clock(Duration::from_secs(2));
        serve_headless_frame(&mut window);
        assert_eq!(renders.get(), unmounted);

        assert!(
            handle
                .update(&mut app, |_, window, _| window.remove_window())
                .is_ok()
        );
        app.quit();
    }

    #[test]
    fn closing_mounted_timer_window_retires_its_pending_wake() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let renders = Rc::new(Cell::new(0));
        let handle = app.add_window(|_, cx| {
            TimerRoot(cx.new(|_| TimerSurface {
                intervals: vec![Duration::from_millis(50)],
                running: vec![true],
                visible: vec![true],
                renders: renders.clone(),
            }))
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        serve_headless_frame(&mut window);
        let closing = renders.get();
        assert!(
            handle
                .update(&mut app, |_, window, _| window.remove_window())
                .is_ok()
        );
        app.executor().advance_clock(Duration::from_millis(50));
        for _ in 0..100 {
            if !app.executor().tick() {
                break;
            }
        }
        assert_eq!(renders.get(), closing);
        assert!(!app.windows().contains(&handle.into()));
        app.executor().advance_clock(Duration::from_secs(1));
        assert!(
            !app.executor().tick(),
            "closed window must not renew its timer"
        );
        app.quit();
    }

    #[test]
    fn timer_registration_expires_at_requested_interval() {
        for millis in [50, 250, 500] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let executor = app.executor();
            let window = app.add_empty_window();
            window.update(|window, cx| {
                let _ = Timer::count_up(Duration::ZERO)
                    .start()
                    .tick_interval(Duration::from_millis(millis))
                    .render(window, cx);
            });
            while executor.tick() {}
            executor.advance_clock(Duration::from_millis(millis - 1));
            while executor.tick() {}
            window.update(|_, cx| {
                assert_eq!(
                    lock_timer_windows(&cx.global::<TimerRuntime>().windows).len(),
                    1
                );
            });
            executor.advance_clock(Duration::from_millis(1));
            while executor.tick() {}
            window.update(|window, cx| {
                let _ = Timer::count_up(Duration::ZERO).paused().render(window, cx);
                let _ = Timer::count_down(Duration::from_secs(1), Duration::from_secs(1))
                    .start()
                    .render(window, cx);
                assert!(
                    lock_timer_windows(&cx.global::<TimerRuntime>().windows).is_empty(),
                    "registration should wake and expire after {millis}ms"
                );
            });
            app.quit();
        }
    }

    #[test]
    fn timer_registration_keeps_earliest_deadline_and_rearms_after_waking() {
        let mut app = gpui::TestAppContext::single();
        let executor = app.executor();
        let handle = app
            .add_empty_window()
            .update(|window, _| window.window_handle());
        app.update(|cx| {
            ensure_timer_runtime(cx);
            let runtime = cx.global::<TimerRuntime>().clone();
            for millis in [500, 250, 50, 500] {
                runtime.register(handle, Duration::from_millis(millis), cx);
            }
            let windows = lock_timer_windows(&runtime.windows);
            assert_eq!(windows.len(), 1);
            assert_eq!(
                windows[&handle].deadline,
                executor.now() + Duration::from_millis(50)
            );
        });
        while executor.tick() {}
        executor.advance_clock(Duration::from_millis(50));
        while executor.tick() {}
        app.update(|cx| {
            let runtime = cx.global::<TimerRuntime>().clone();
            assert!(lock_timer_windows(&runtime.windows).is_empty());
            runtime.register(handle, Duration::from_millis(250), cx);
            assert_eq!(
                lock_timer_windows(&runtime.windows)[&handle].deadline,
                executor.now() + Duration::from_millis(250)
            );
        });
        while executor.tick() {}
        executor.advance_clock(Duration::from_millis(249));
        while executor.tick() {}
        app.update(|cx| {
            assert_eq!(
                lock_timer_windows(&cx.global::<TimerRuntime>().windows).len(),
                1
            );
        });
        executor.advance_clock(Duration::from_millis(1));
        while executor.tick() {}
        app.update(|cx| {
            assert!(lock_timer_windows(&cx.global::<TimerRuntime>().windows).is_empty());
        });
        app.quit();
    }

    #[test]
    fn timer_windows_wake_independently_and_closed_windows_are_retired() {
        let mut app = gpui::TestAppContext::single();
        let executor = app.executor();
        let first = app
            .add_empty_window()
            .update(|window, _| window.window_handle());
        let second = app
            .add_empty_window()
            .update(|window, _| window.window_handle());
        app.update(|cx| {
            ensure_timer_runtime(cx);
            let runtime = cx.global::<TimerRuntime>().clone();
            runtime.register(first, Duration::from_millis(50), cx);
            runtime.register(second, Duration::from_millis(500), cx);
            second
                .update(cx, |_, window, _| window.remove_window())
                .unwrap();
        });
        while executor.tick() {}
        executor.advance_clock(Duration::from_millis(50));
        while executor.tick() {}
        app.update(|cx| {
            let windows = lock_timer_windows(&cx.global::<TimerRuntime>().windows);
            assert!(!windows.contains_key(&first));
            assert!(windows.contains_key(&second));
        });
        executor.advance_clock(Duration::from_millis(450));
        while executor.tick() {}
        app.update(|cx| {
            assert!(lock_timer_windows(&cx.global::<TimerRuntime>().windows).is_empty());
        });
        app.quit();
    }

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
