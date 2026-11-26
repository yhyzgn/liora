use gpui::{AnyView, App};
use liora_components::{Timer, TimerUnit};
use std::time::Duration;

#[path = "controls.rs"]
mod controls;

// Retain this view in the parent; Resume preserves this countdown's duration.
pub fn timer_count_down(cx: &mut App) -> AnyView {
    controls::controlled_timer(
        cx,
        |elapsed| {
            Timer::count_down(Duration::from_secs(300), elapsed)
                .title("Deploy window")
                .display_unit(TimerUnit::Minutes)
                .prefix("剩余")
        },
        Duration::from_secs(84),
    )
}
