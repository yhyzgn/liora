use gpui::{AnyView, App, Context, IntoElement, Render, Window, div, prelude::*, px, rgb};
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
                .child(Text::new("Application workspace").text_color(rgb(0x64748b).into())),
        )
        .child(status_bar)
}

impl Render for StatusBarDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        page(
            "StatusBar 状态栏",
            "桌面应用底部状态条，提供 left / center / right 三段区域，可组合连接状态、后台任务、版本、快捷键、当前上下文与自定义交互区域。",
            Space::new().vertical().gap_xl().child(section(
                "StatusBar showcase",
                "状态栏示例统一放入工作区预览卡片；每个示例展示不同扩展点，避免只覆盖最简单文本场景。",
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
                                .right_item(StatusBarItem::new("v0.1.20").pill()),
                        ),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "语义状态 + 分隔",
                        "每个 item 可以单独使用 tone、dot、detail、separator 组合成紧凑状态簇。",
                        shell_preview(
                            StatusBar::new()
                                .background(theme.neutral.hover.opacity(0.42))
                                .left_item(
                                    StatusBarItem::new("Connected")
                                        .success()
                                        .dot()
                                        .icon(IconName::Wifi)
                                        .detail("42ms")
                                        .pill(),
                                )
                                .left_item(StatusBarItem::separator())
                                .left_item(
                                    StatusBarItem::new("Queue")
                                        .warning()
                                        .dot()
                                        .icon(IconName::Clock3)
                                        .detail("3 jobs")
                                        .min_width(px(96.0)),
                                )
                                .center_item(
                                    StatusBarItem::new("Preview mode")
                                        .primary()
                                        .icon(IconName::Monitor),
                                )
                                .right_item(
                                    StatusBarItem::new("Offline cache")
                                        .danger()
                                        .dot()
                                        .icon(IconName::WifiOff)
                                        .pill(),
                                ),
                        ),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "自定义区域",
                        "StatusBarItem::custom 可以承载任意 Liora 控件，适合运行/构建/同步等操作区。",
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
                    showcase_card_wide(
                        "交互与品牌化",
                        "支持 item 级点击、最小宽度、自定义前景/背景、spacer 和 borderless 变体。",
                        shell_preview(
                            StatusBar::new()
                                .height(px(40.0))
                                .borderless()
                                .background(theme.primary.base.opacity(0.10))
                                .left_item(
                                    StatusBarItem::new("Deploy")
                                        .icon(IconName::Rocket)
                                        .dot()
                                        .min_width(px(108.0))
                                        .background(theme.primary.base.opacity(0.16))
                                        .text_color(theme.primary.base)
                                        .pill()
                                        .on_click(|_, _| {}),
                                )
                                .left_item(StatusBarItem::new("main").icon(IconName::GitBranch))
                                .center_item(StatusBarItem::spacer())
                                .right_item(
                                    StatusBarItem::new("Updates ready")
                                        .info()
                                        .icon(IconName::Download)
                                        .on_click(|_, _| {})
                                        .pill(),
                                )
                                .right_item(
                                    StatusBarItem::new("Open docs")
                                        .icon(IconName::ExternalLink)
                                        .compact()
                                        .on_click(|_, cx| cx.open_url("https://github.com/yhyzgn/liora")),
                                ),
                        ),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}
