use gpui::{AnyView, App};
use liora_components::{Timer, TimerUnit};
use std::time::Duration;

#[path = "controls.rs"]
mod controls;

// Retain this view in the parent; do not recreate it during render.
pub fn timer_count_up(cx: &mut App) -> AnyView {
    controls::controlled_timer(
        cx,
        |elapsed| {
            Timer::count_up(elapsed)
                .title("Build elapsed")
                .display_unit(TimerUnit::Seconds)
        },
        Duration::ZERO,
    )
}
