use gpui::{AnyElement, AnyView, App, Context, Render, Window, div, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Label, Space, Spinner, Text};
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SpinnerDemo).into()
}

struct SpinnerDemo;

fn spinner_card(title: &'static str, detail: &'static str, spinner: Spinner) -> AnyElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_4()
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xf8fafc))
        .p_3()
        .child(
            Space::new()
                .vertical()
                .gap_xs()
                .child(Text::new(title).bold())
                .child(Text::new(detail).xs()),
        )
        .child(spinner)
        .into_any_element()
}

fn spinner_status_card(title: &'static str, detail: &'static str, color: u32) -> AnyElement {
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
        .into_any_element()
}

impl Render for SpinnerDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Spinner 旋转加载",
            "独立的细粒度加载图标，适合按钮、状态栏、工具栏、列表行和卡片局部刷新。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "尺寸与图标",
                    "每个 Spinner 都放在真实状态行里展示，方便看清尺寸、旋转图标和文案关系。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(spinner_card(
                            "Small / inline",
                            "用于按钮、菜单项、状态栏短文本。",
                            Spinner::new().small(),
                        ))
                        .child(spinner_card(
                            "Default / row",
                            "用于列表行、工具栏后台任务。",
                            Spinner::new(),
                        ))
                        .child(spinner_card(
                            "Large / panel",
                            "用于卡片局部刷新或较强状态提示。",
                            Spinner::new().large(),
                        ))
                        .child(spinner_card(
                            "Custom icon",
                            "RefreshCw 保持同一 spin motion。",
                            Spinner::new().icon(IconName::RefreshCw).size(px(22.0)),
                        )),
                ))
                .child(section(
                    "语义颜色",
                    "用业务色配合标题/说明，而不是只摆几个孤立的彩色图标。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(spinner_status_card("Syncing", "同步远端配置中", 0x2563eb))
                        .child(spinner_status_card(
                            "Verifying",
                            "等待校验服务返回",
                            0x16a34a,
                        ))
                        .child(spinner_status_card(
                            "Retrying",
                            "网络不稳定，正在重试",
                            0xf59e0b,
                        ))
                        .child(spinner_status_card(
                            "Recovering",
                            "错误恢复任务仍在运行",
                            0xdc2626,
                        )),
                ))
                .child(section(
                    "组合场景",
                    "Spinner 是独立控件，不需要 Loading 遮罩；可以嵌入任何 Liora 组合。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    Button::new("Syncing")
                                        .primary()
                                        .icon_start(Spinner::new().small().into_any_element()),
                                )
                                .child(
                                    Button::new("Exporting")
                                        .icon_start(Spinner::new().small().into_any_element()),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_4()
                                .rounded_lg()
                                .border_1()
                                .border_color(rgb(0xe2e8f0))
                                .bg(rgb(0xf8fafc))
                                .p_3()
                                .child(
                                    Label::new("Fetching metrics")
                                        .custom_icon(Spinner::new().small()),
                                )
                                .child(Text::new("12 jobs queued").xs()),
                        )
                        .child(spinner_card(
                            "Background export",
                            "Exporting reports.zip · 42%",
                            Spinner::new().icon(IconName::LoaderCircle).large(),
                        )),
                )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn spinner_demo_is_dedicated_and_rich() {
        let source = include_str!("spinner_demo.rs");
        assert!(source.contains("Spinner 旋转加载"));
        assert!(source.contains("spinner_card"));
        assert!(source.contains("spinner_status_card"));
        assert!(source.contains("Small / inline"));
        assert!(source.contains("Exporting reports.zip"));
        assert!(source.contains("语义颜色"));
        assert!(source.contains("组合场景"));
        assert!(source.contains("use liora_components::{Button, Label, Space, Spinner, Text};"));
    }
}
