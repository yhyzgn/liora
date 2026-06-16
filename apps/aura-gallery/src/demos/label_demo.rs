use aura_components::layout_helpers::{page, row_md, section};
use aura_components::{Label, Space};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px, rgb};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| LabelDemo).into()
}

struct LabelDemo;

impl Render for LabelDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Label 标签组合",
            "Icon + Text 的轻量组合控件，适合指标名、状态说明、列表前缀和操作行标题。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础图标标签",
                    "使用内置 Lucide 图标快速组合图标与文本。",
                    row_md(vec![
                        Label::new("CPU")
                            .icon(IconName::Cpu)
                            .size(px(14.0))
                            .into_any_element(),
                        Label::new("Network")
                            .icon(IconName::Wifi)
                            .gap(px(10.0))
                            .into_any_element(),
                        Label::new("Storage")
                            .icon(IconName::HardDrive)
                            .size(px(15.0))
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "状态颜色",
                    "可单独指定图标与文字颜色，用于展示状态或语义。",
                    row_md(vec![
                        Label::new("Build passed")
                            .icon(IconName::CircleCheck)
                            .color(rgb(0x16a34a).into())
                            .into_any_element(),
                        Label::new("Deploy warning")
                            .icon(IconName::TriangleAlert)
                            .color(rgb(0xf59e0b).into())
                            .into_any_element(),
                        Label::new("Sync failed")
                            .icon(IconName::CircleX)
                            .color(rgb(0xef4444).into())
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "自定义图标元素",
                    "custom_icon 可以传入任意 GPUI 元素，方便接入品牌图标或复杂视觉。",
                    row_md(vec![
                        Label::new("Aura")
                            .custom_icon(
                                div()
                                    .size(px(18.0))
                                    .rounded_full()
                                    .bg(rgb(0x6366f1))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(IconName::Sparkles)
                                            .size(px(12.0))
                                            .color(gpui::white()),
                                    ),
                            )
                            .gap(px(8.0))
                            .color(rgb(0x4338ca).into())
                            .into_any_element(),
                    ]),
                )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn label_demo_covers_core_label_configurations() {
        let source = include_str!("label_demo.rs");
        assert!(source.contains("Label::new"));
        assert!(source.contains(".icon("));
        assert!(source.contains(".custom_icon("));
        assert!(source.contains(".gap("));
        assert!(source.contains(".color("));
    }
}
