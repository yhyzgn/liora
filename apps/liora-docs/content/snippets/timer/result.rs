use liora_components::{Timer, TimerUnit};
use std::time::Duration;

pub fn read_timer_result() -> (f64, Option<f64>, bool) {
    let timer = Timer::count_down(Duration::from_secs(300), Duration::from_secs(84));
    let snapshot = timer.snapshot();

    (
        snapshot.elapsed_as(TimerUnit::Seconds),
        snapshot.remaining_as(TimerUnit::Minutes),
        snapshot.finished,
    )
}
