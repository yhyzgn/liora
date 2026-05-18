use aura_components::{Timer, TimerUnit};
use gpui::IntoElement;
use std::time::Duration;

pub fn timer_count_up() -> impl IntoElement {
    Timer::count_up(Duration::ZERO)
        .id("docs-running-count-up")
        .start()
        .title("Build elapsed")
        .display_unit(TimerUnit::Seconds)
}
