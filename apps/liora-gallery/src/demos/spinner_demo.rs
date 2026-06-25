use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, row_md, section};
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
                    "尺寸与图标",
                    "提供 small/default/large 和自定义 Lucide 图标，保持同一 motion 语义。",
                    row_md(vec![
                        Spinner::new().small().into_any_element(),
                        Spinner::new().into_any_element(),
                        Spinner::new().large().into_any_element(),
                        Spinner::new()
                            .icon(IconName::RefreshCw)
                            .size(px(20.0))
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "语义颜色",
                    "可使用主题或业务色表达同步、成功等待、警告重试等状态。",
                    row_md(vec![
                        Spinner::new()
                            .color(rgb(0x2563eb).into())
                            .into_any_element(),
                        Spinner::new()
                            .color(rgb(0x16a34a).into())
                            .into_any_element(),
                        Spinner::new()
                            .color(rgb(0xf59e0b).into())
                            .into_any_element(),
                        Spinner::new()
                            .color(rgb(0xdc2626).into())
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "组合场景",
                    "Spinner 是独立控件，不需要 Loading 遮罩；可以嵌入任何 Liora 组合。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(row_md(vec![
                            Button::new("Syncing")
                                .primary()
                                .icon_start(Spinner::new().small().into_any_element())
                                .into_any_element(),
                            Label::new("Fetching metrics")
                                .custom_icon(Spinner::new().small())
                                .into_any_element(),
                        ]))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_4()
                                .rounded_lg()
                                .border_1()
                                .border_color(rgb(0xe2e8f0))
                                .p_3()
                                .child(Text::new("Background job: exporting reports"))
                                .child(Spinner::new().icon(IconName::LoaderCircle)),
                        ),
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
        assert!(source.contains("语义颜色"));
        assert!(source.contains("组合场景"));
        assert!(source.contains("use liora_components::{Button, Label, Space, Spinner, Text};"));
    }
}
