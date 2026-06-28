use gpui::{AnyElement, AnyView, App, Context, Render, Window, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section, showcase_card, showcase_grid};
use liora_components::{Button, Label, Space, Spinner, Text};
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SpinnerDemo).into()
}

struct SpinnerDemo;

impl Render for SpinnerDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Spinner 旋转加载",
            "独立的细粒度加载图标，适合按钮、状态栏、工具栏、列表行和卡片局部刷新。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Spinner showcase",
                    "每个 Spinner 都放在同一规格的状态卡片里展示尺寸、语义色和组合关系。",
                    showcase_grid(vec![
                        spinner_card(
                            "Small / inline",
                            "按钮、菜单项、状态栏短文本。",
                            Spinner::new().small(),
                        ),
                        spinner_card("Default / row", "列表行、工具栏后台任务。", Spinner::new()),
                        spinner_card(
                            "Large / panel",
                            "卡片局部刷新或较强状态提示。",
                            Spinner::new().large(),
                        ),
                        spinner_card(
                            "Custom icon",
                            "RefreshCw 保持同一 spin motion。",
                            Spinner::new().icon(IconName::RefreshCw).size(px(22.0)),
                        ),
                        spinner_card(
                            "Syncing",
                            "同步远端配置中。",
                            Spinner::new().large().color(rgb(0x2563eb).into()),
                        ),
                        spinner_card(
                            "Verifying",
                            "等待校验服务返回。",
                            Spinner::new().large().color(rgb(0x16a34a).into()),
                        ),
                        spinner_card(
                            "Retrying",
                            "网络不稳定，正在重试。",
                            Spinner::new().large().color(rgb(0xf59e0b).into()),
                        ),
                        spinner_card(
                            "Recovering",
                            "错误恢复任务仍在运行。",
                            Spinner::new().large().color(rgb(0xdc2626).into()),
                        ),
                    ]),
                ))
                .child(section(
                    "组合场景",
                    "Spinner 是独立控件，不需要 Loading 遮罩；可以嵌入任何 Liora 组合。",
                    showcase_grid(vec![
                        showcase_card(
                            "Buttons",
                            "把 Spinner 放入按钮 icon slot。",
                            Space::new()
                                .wrap()
                                .gap_md()
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
                        .into_any_element(),
                        showcase_card(
                            "Inline status",
                            "与 Label/Text 组成局部任务状态。",
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(
                                    Label::new("Fetching metrics")
                                        .custom_icon(Spinner::new().small()),
                                )
                                .child(Text::new("12 jobs queued").xs()),
                        )
                        .into_any_element(),
                        spinner_card(
                            "Background export",
                            "Exporting reports.zip · 42%",
                            Spinner::new().icon(IconName::LoaderCircle).large(),
                        ),
                    ]),
                )),
        )
    }
}

fn spinner_card(title: &'static str, detail: &'static str, spinner: Spinner) -> AnyElement {
    showcase_card(
        title,
        detail,
        Space::new()
            .gap_md()
            .align_center()
            .child(spinner)
            .child(Text::new(detail).xs().wrap()),
    )
    .into_any_element()
}

#[cfg(test)]
mod tests {
    #[test]
    fn spinner_demo_is_dedicated_and_rich() {
        let source = include_str!("spinner_demo.rs");
        assert!(source.contains("Spinner 旋转加载"));
        assert!(source.contains("Spinner showcase"));
        assert!(source.contains("组合场景"));
        assert!(source.contains("custom_icon(Spinner::new().small())"));
    }
}
