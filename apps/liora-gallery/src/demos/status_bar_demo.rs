use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{Button, Space, StatusBar, StatusBarItem, Text};
use liora_core::Config;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| StatusBarDemo).into()
}

struct StatusBarDemo;

fn shell_preview(status_bar: StatusBar) -> impl IntoElement {
    div()
        .w_full()
        .rounded_lg()
        .border_1()
        .overflow_hidden()
        .child(
            div()
                .min_h(px(150.0))
                .flex()
                .items_center()
                .justify_center()
                .child(Text::new("Application workspace").text_color(gpui::rgb(0x64748b).into())),
        )
        .child(status_bar)
}

impl Render for StatusBarDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        page(
            "StatusBar 状态栏",
            "桌面应用底部状态条，提供 left / center / right 三段区域，用于连接状态、后台任务、版本、快捷键与当前上下文。",
            Space::new().vertical().gap_xl().child(section(
                "StatusBar showcase",
                "状态栏示例统一放入工作区预览卡片，避免不同长度状态条在页面中散乱跳动。",
                showcase_stack(vec![
                    showcase_card_wide(
                        "应用壳状态",
                        "最常见的 IDE/工具型应用状态条布局。",
                        shell_preview(
                            StatusBar::new()
                                .left_item(
                                    StatusBarItem::new("Ready")
                                        .success()
                                        .icon(IconName::CircleCheck)
                                        .pill(),
                                )
                                .left_item(StatusBarItem::new("Syncing").loading(true).info())
                                .center_item(
                                    StatusBarItem::new("src/main.rs")
                                        .icon(IconName::FileCode)
                                        .primary(),
                                )
                                .right_item(StatusBarItem::new("UTF-8").compact())
                                .right_item(StatusBarItem::new("Ln 42, Col 7").compact())
                                .right_item(StatusBarItem::new("v0.1.12").pill()),
                        ),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "语义状态",
                        "每个 item 可以单独使用 success/warning/danger/info/primary tone。",
                        shell_preview(
                            StatusBar::new()
                                .background(theme.neutral.hover.opacity(0.42))
                                .left_item(
                                    StatusBarItem::new("Connected")
                                        .success()
                                        .icon(IconName::Wifi)
                                        .detail("42ms")
                                        .pill(),
                                )
                                .left_item(
                                    StatusBarItem::new("Queue")
                                        .warning()
                                        .icon(IconName::Clock3)
                                        .detail("3 jobs"),
                                )
                                .center_item(
                                    StatusBarItem::new("Preview mode")
                                        .primary()
                                        .icon(IconName::Monitor),
                                )
                                .right_item(
                                    StatusBarItem::new("Offline cache")
                                        .danger()
                                        .icon(IconName::WifiOff)
                                        .pill(),
                                ),
                        ),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "自定义区域",
                        "StatusBarItem::custom 可以承载任意 Liora 控件。",
                        shell_preview(
                            StatusBar::new()
                                .height(px(38.0))
                                .left_item(
                                    StatusBarItem::new("Workspace: Liora")
                                        .icon(IconName::FolderOpen),
                                )
                                .center_item(StatusBarItem::custom(
                                    Space::new()
                                        .gap_sm()
                                        .child(
                                            Button::new("Run")
                                                .small()
                                                .primary()
                                                .icon_start(IconName::Play),
                                        )
                                        .child(
                                            Button::new("Build")
                                                .small()
                                                .icon_start(IconName::Hammer),
                                        ),
                                ))
                                .right_item(StatusBarItem::new("Native GPUI").info().pill()),
                        ),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}
