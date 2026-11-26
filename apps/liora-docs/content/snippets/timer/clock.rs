use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Timer, TimerFormat};
use std::time::Duration;

struct Clock {
    timer: Timer,
}

// Create once in the parent, then clone the returned view during render.
pub fn timer_clock(cx: &mut App) -> AnyView {
    cx.new(|_| Clock {
        timer: Timer::count_up(Duration::from_secs(3661))
            .start()
            .title("Elapsed clock")
            .format(TimerFormat::Clock),
    })
    .into()
}

impl Render for Clock {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.timer.clone()
    }
}
