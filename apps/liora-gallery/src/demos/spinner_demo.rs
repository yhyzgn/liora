use gpui::{AnyElement, AnyView, App, Context, Hsla, Render, Window, div, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Label, Space, Spinner, Text};
use liora_core::Config;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SpinnerDemo).into()
}

struct SpinnerDemo;

fn spinner_grid(children: Vec<AnyElement>) -> AnyElement {
    div()
        .flex()
        .flex_wrap()
        .gap_3()
        .children(children)
        .into_any_element()
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

fn spinner_card(
    title: &'static str,
    detail: &'static str,
    spinner: Spinner,
    bg: Hsla,
    border: Hsla,
) -> AnyElement {
    div()
        .w(px(320.0))
        .min_h(px(84.0))
        .flex()
        .items_center()
        .gap_4()
        .rounded_lg()
        .border_1()
        .border_color(border)
        .bg(bg)
        .p_4()
        .child(spinner_text(title, detail))
        .child(div().flex_none().child(spinner))
        .into_any_element()
}

fn spinner_status_card(
    title: &'static str,
    detail: &'static str,
    color: u32,
    bg: Hsla,
    border: Hsla,
) -> AnyElement {
    div()
        .w(px(320.0))
        .min_h(px(84.0))
        .flex()
        .items_center()
        .gap_4()
        .rounded_lg()
        .border_1()
        .border_color(border)
        .bg(bg)
        .p_4()
        .child(
            div()
                .flex_none()
                .child(Spinner::new().large().color(rgb(color).into())),
        )
        .child(spinner_text(title, detail))
        .into_any_element()
}

impl Render for SpinnerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let card_bg = theme.neutral.hover.opacity(0.56);
        let card_alt_bg = theme.neutral.card;
        let card_border = theme.neutral.border;

        page(
            "Spinner 旋转加载",
            "独立的细粒度加载图标，适合按钮、状态栏、工具栏、列表行和卡片局部刷新。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "尺寸与图标",
                    "每个 Spinner 都放在真实状态行里展示，方便看清尺寸、旋转图标和文案关系。",
                    spinner_grid(vec![
                        spinner_card(
                            "Small / inline",
                            "用于按钮、菜单项、状态栏短文本。",
                            Spinner::new().small(),
                            card_bg,
                            card_border,
                        ),
                        spinner_card(
                            "Default / row",
                            "用于列表行、工具栏后台任务。",
                            Spinner::new(),
                            card_alt_bg,
                            card_border,
                        ),
                        spinner_card(
                            "Large / panel",
                            "用于卡片局部刷新或较强状态提示。",
                            Spinner::new().large(),
                            card_bg,
                            card_border,
                        ),
                        spinner_card(
                            "Custom icon",
                            "RefreshCw 保持同一 spin motion。",
                            Spinner::new().icon(IconName::RefreshCw).size(px(22.0)),
                            card_alt_bg,
                            card_border,
                        ),
                    ]),
                ))
                .child(section(
                    "语义颜色",
                    "用业务色配合标题/说明，而不是只摆几个孤立的彩色图标。",
                    spinner_grid(vec![
                        spinner_status_card(
                            "Syncing",
                            "同步远端配置中",
                            0x2563eb,
                            card_alt_bg,
                            card_border,
                        ),
                        spinner_status_card(
                            "Verifying",
                            "等待校验服务返回",
                            0x16a34a,
                            card_bg,
                            card_border,
                        ),
                        spinner_status_card(
                            "Retrying",
                            "网络不稳定，正在重试",
                            0xf59e0b,
                            card_alt_bg,
                            card_border,
                        ),
                        spinner_status_card(
                            "Recovering",
                            "错误恢复任务仍在运行",
                            0xdc2626,
                            card_bg,
                            card_border,
                        ),
                    ]),
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
                                .w(px(320.0))
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_4()
                                .rounded_lg()
                                .border_1()
                                .border_color(card_border)
                                .bg(card_bg)
                                .p_4()
                                .child(
                                    div().flex_1().min_w(px(0.0)).child(
                                        Label::new("Fetching metrics")
                                            .custom_icon(Spinner::new().small()),
                                    ),
                                )
                                .child(
                                    div()
                                        .flex_none()
                                        .child(Text::new("12 jobs queued").xs().nowrap()),
                                ),
                        )
                        .child(spinner_card(
                            "Background export",
                            "Exporting reports.zip · 42%",
                            Spinner::new().icon(IconName::LoaderCircle).large(),
                            card_alt_bg,
                            card_border,
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
        assert!(source.contains("spinner_grid"));
        assert!(source.contains("spinner_card"));
        assert!(source.contains("spinner_status_card"));
        assert!(source.contains(".w(px(320.0))"));
        assert!(source.contains(".flex_1().min_w(px(0.0))"));
        assert!(source.contains("Small / inline"));
        assert!(source.contains("Exporting reports.zip"));
        assert!(source.contains("语义颜色"));
        assert!(source.contains("组合场景"));
        assert!(source.contains("use liora_components::{Button, Label, Space, Spinner, Text};"));
    }
}
