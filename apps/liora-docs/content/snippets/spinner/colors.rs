//! Spinner semantic color examples.

use gpui::{AnyElement, IntoElement, div, prelude::*, px, rgb};
use liora_components::{Space, Spinner, Text};

pub fn spinner_colors() -> impl IntoElement {
    spinner_snippet_grid(vec![
        spinner_color_row("Syncing", "同步远端配置中", 0x2563eb),
        spinner_color_row("Verifying", "等待校验服务返回", 0x16a34a),
        spinner_color_row("Retrying", "网络不稳定，正在重试", 0xf59e0b),
        spinner_color_row("Recovering", "错误恢复任务仍在运行", 0xdc2626),
    ])
}

fn spinner_snippet_grid(children: Vec<AnyElement>) -> impl IntoElement {
    div().flex().flex_wrap().gap_3().children(children)
}

fn spinner_text(title: &'static str, detail: &'static str) -> impl IntoElement {
    div().flex_1().min_w(px(0.0)).child(
        Space::new()
            .vertical()
            .gap_xs()
            .child(Text::new(title).bold().nowrap())
            .child(Text::new(detail).xs().wrap()),
    )
}

fn spinner_color_row(title: &'static str, detail: &'static str, color: u32) -> AnyElement {
    div()
        .w(px(320.0))
        .min_h(px(84.0))
        .flex()
        .items_center()
        .gap_4()
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_4()
        .child(
            div()
                .flex_none()
                .child(Spinner::new().large().color(rgb(color).into())),
        )
        .child(spinner_text(title, detail))
        .into_any_element()
}
