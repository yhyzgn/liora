use aura_components::{Timer, TimerFormat};
use gpui::IntoElement;
use std::time::Duration;

pub fn timer_clock() -> impl IntoElement {
    // clock_format 等价于 format(TimerFormat::Clock)，输出 HH:MM:SS。
    Timer::count_up(Duration::from_secs(3661))
        .id("docs-running-clock")
        .start()
        .title("Elapsed clock")
        .format(TimerFormat::Clock)
}
