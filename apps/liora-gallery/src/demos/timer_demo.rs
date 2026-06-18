use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, row_md, section};
use liora_components::{Space, Timer, TimerFormat, TimerUnit};
use std::time::Duration;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TimerDemo).into()
}

struct TimerDemo;

impl Render for TimerDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Timer 计时器",
            "展示正向计时、倒计时、单位换算和计时结果读取 API。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "正向计时",
                    "count_up 用于展示已耗时，可选择显示单位。",
                    row_md(vec![
                        Timer::count_up(Duration::ZERO)
                            .id("timer-demo-running-count-up")
                            .start()
                            .title("Build elapsed")
                            .display_unit(TimerUnit::Seconds)
                            .into_any_element(),
                        Timer::count_up(Duration::from_secs(7320))
                            .title("Session")
                            .display_unit(TimerUnit::Hours)
                            .suffix("total")
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "倒计时",
                    "count_down 接收总时长和已用时，remaining 会自动饱和到 0。",
                    row_md(vec![
                        Timer::count_down(Duration::from_secs(300), Duration::from_secs(84))
                            .id("timer-demo-running-count-down")
                            .start()
                            .title("Deploy window")
                            .display_unit(TimerUnit::Minutes)
                            .prefix("剩余")
                            .into_any_element(),
                        Timer::count_down(Duration::from_secs(30), Duration::from_secs(36))
                            .title("Finished")
                            .display_unit(TimerUnit::Seconds)
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "时钟格式",
                    "支持 00:00:00 形式的 HH:MM:SS clock format。",
                    row_md(vec![
                        Timer::count_up(Duration::from_secs(3661))
                            .id("timer-demo-running-clock")
                            .start()
                            .title("Elapsed clock")
                            .clock_format()
                            .into_any_element(),
                        Timer::count_down(Duration::from_secs(7200), Duration::from_secs(139))
                            .title("Remaining clock")
                            .format(TimerFormat::Clock)
                            .prefix("剩余")
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "单位与紧凑模式",
                    "支持毫秒、秒、分钟、小时，也可隐藏单位用于自定义布局。",
                    row_md(vec![
                        Timer::count_up(Duration::from_millis(1532))
                            .title("Latency")
                            .display_unit(TimerUnit::Milliseconds)
                            .compact()
                            .into_any_element(),
                        Timer::count_up(Duration::from_secs(64))
                            .display_unit(TimerUnit::Seconds)
                            .show_unit(false)
                            .prefix("T+")
                            .suffix("seconds")
                            .compact()
                            .into_any_element(),
                    ]),
                )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn timer_demo_uses_timer_api() {
        let source = include_str!("timer_demo.rs");
        assert!(source.contains("Timer::count_up"));
        assert!(source.contains("Timer::count_down"));
        assert!(source.contains("TimerUnit"));
        assert!(source.contains("clock_format"));
        assert!(source.contains("TimerFormat::Clock"));
        assert!(source.contains(".start()"));
    }
}
