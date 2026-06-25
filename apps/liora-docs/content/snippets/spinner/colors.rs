//! Spinner semantic color examples.

use gpui::{IntoElement, div, prelude::*, rgb};
use liora_components::{Space, Spinner, Text};

pub fn spinner_colors() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(spinner_color_row("Syncing", "同步远端配置中", 0x2563eb))
        .child(spinner_color_row("Verifying", "等待校验服务返回", 0x16a34a))
        .child(spinner_color_row(
            "Retrying",
            "网络不稳定，正在重试",
            0xf59e0b,
        ))
        .child(spinner_color_row(
            "Recovering",
            "错误恢复任务仍在运行",
            0xdc2626,
        ))
}

fn spinner_color_row(title: &'static str, detail: &'static str, color: u32) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_3()
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_3()
        .child(Spinner::new().large().color(rgb(color).into()))
        .child(
            Space::new()
                .vertical()
                .gap_xs()
                .child(Text::new(title).bold())
                .child(Text::new(detail).xs()),
        )
}
