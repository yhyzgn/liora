use aura_components::{Timer, TimerUnit};
use gpui::IntoElement;
use std::time::Duration;

pub fn timer_count_down() -> impl IntoElement {
    Timer::count_down(Duration::from_secs(300), Duration::from_secs(84))
        .id("docs-running-count-down")
        .start()
        .title("Deploy window")
        .display_unit(TimerUnit::Minutes)
        .prefix("剩余")
}
