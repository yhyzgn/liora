use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, Space, Timer};
use std::time::Duration;

// The host owns elapsed time; each run gets a fresh Timer identity.
pub struct ControlledTimer {
    make_timer: fn(Duration) -> Timer,
    timer: Timer,
    running: bool,
}

impl ControlledTimer {
    pub fn new(make_timer: fn(Duration) -> Timer, elapsed: Duration) -> Self {
        Self {
            make_timer,
            timer: make_timer(elapsed).start(),
            running: true,
        }
    }

    fn pause_at(&mut self, elapsed: Duration) {
        self.timer = (self.make_timer)(elapsed).paused();
        self.running = false;
    }

    fn pause(&mut self) {
        self.pause_at(self.timer.snapshot().elapsed);
    }

    fn resume(&mut self) {
        if !self.running && !self.timer.snapshot().finished {
            self.timer = (self.make_timer)(self.timer.snapshot().elapsed).start();
            self.running = true;
        }
    }

    fn reset(&mut self) {
        self.pause_at(Duration::ZERO);
    }
}

pub fn controlled_timer(
    cx: &mut App,
    make_timer: fn(Duration) -> Timer,
    elapsed: Duration,
) -> AnyView {
    cx.new(|_| ControlledTimer::new(make_timer, elapsed)).into()
}

impl Render for ControlledTimer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.timer.snapshot();
        let label = if self.running {
            "Pause"
        } else if snapshot.elapsed.is_zero() {
            "Start"
        } else {
            "Resume"
        };
        Space::new()
            .vertical()
            .gap_md()
            .child(self.timer.clone())
            .child(
                Space::new()
                    .gap_md()
                    .child(
                        Button::new(label)
                            .disabled(snapshot.finished)
                            .on_click(cx.listener(|this, _, _, cx| {
                                if this.running {
                                    this.pause();
                                } else {
                                    this.resume();
                                }
                                cx.notify();
                            })),
                    )
                    .child(Button::new("Reset").on_click(cx.listener(|this, _, _, cx| {
                        this.reset();
                        cx.notify();
                    }))),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pause_resume_reset_preserves_elapsed_and_countdown_duration() {
        let mut state = ControlledTimer::new(
            |elapsed| Timer::count_down(Duration::from_secs(30), elapsed),
            Duration::ZERO,
        );
        // Inject the pause snapshot without sleeps or a native window.
        state.pause_at(Duration::from_secs(12));
        assert_eq!(state.timer.snapshot().elapsed, Duration::from_secs(12));
        assert_eq!(
            state.timer.snapshot().remaining,
            Some(Duration::from_secs(18))
        );
        state.resume();
        assert!(state.running);
        assert_eq!(
            state.timer.clone().paused().snapshot().elapsed,
            Duration::from_secs(12)
        );
        assert!(state.timer.snapshot().elapsed >= Duration::from_secs(12));
        state.pause_at(Duration::from_secs(40));
        state.resume();
        assert!(!state.running);
        assert_eq!(state.timer.snapshot().remaining, Some(Duration::ZERO));
        state.reset();
        assert!(!state.running);
        assert_eq!(state.timer.snapshot().elapsed, Duration::ZERO);
        assert_eq!(
            state.timer.snapshot().remaining,
            Some(Duration::from_secs(30))
        );
        state.resume();
        assert!(state.running);
    }
}
