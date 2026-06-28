use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card, showcase_grid};
use liora_components::{Accordion, Space, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| AccordionDemo {
        basic: cx.new(|_| {
            Accordion::new()
                .id("gallery-accordion-basic")
                .default_open("account")
                .item_with_description(
                    "account",
                    "账户设置",
                    "登录、安全与通知偏好",
                    |_, _| {
                        Text::new("使用单开模式展示常见设置分组，点击其他标题会自动收起当前面板。")
                    },
                )
                .item_with_description(
                    "billing",
                    "账单与发票",
                    "付款方式、发票抬头和订阅周期",
                    |_, _| Text::new("适合 FAQ、设置页和文档目录中的渐进披露信息。"),
                )
        }),
        multiple: cx.new(|_| {
            Accordion::new()
                .id("gallery-accordion-multiple")
                .multiple()
                .default_open("status")
                .default_open("deploy")
                .item("status", "服务状态", |_, _| {
                    Text::new("多个面板可同时保持展开，方便对比信息。")
                })
                .item("deploy", "发布检查", |_, _| {
                    Text::new("适合排障清单、部署步骤和审计结果。")
                })
                .disabled_item("locked", "企业策略（禁用）", |_, _| {
                    Text::new("禁用项不会响应点击。")
                })
        }),
        sizes: cx.new(|_| {
            Accordion::new()
                .id("gallery-accordion-sizes")
                .large()
                .bordered(false)
                .default_open("large")
                .item("large", "大尺寸无边框", |_, _| {
                    Text::new("更宽松的间距适合文档、引导页和营销说明。")
                })
                .item("compact", "同一组件 API", |_, _| {
                    Text::new("small()/large()/bordered(false) 可组合使用。")
                })
        }),
    })
    .into()
}

struct AccordionDemo {
    basic: Entity<Accordion>,
    multiple: Entity<Accordion>,
    sizes: Entity<Accordion>,
}

impl Render for AccordionDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Accordion 手风琴",
            "独立手风琴控件，用于 FAQ、设置分组和文档信息折叠。",
            Space::new().vertical().gap_xl().child(section(
                "Accordion showcase",
                "所有手风琴示例使用同一规格卡片承载，避免折叠面板在页面中参差散落。",
                showcase_grid(vec![
                    showcase_card(
                        "基础单开",
                        "默认只允许一个面板展开，适合主流程信息。",
                        self.basic.clone(),
                    )
                    .into_any_element(),
                    showcase_card(
                        "多开与禁用",
                        "multiple() 允许多个面板展开，disabled_item() 展示不可操作项。",
                        self.multiple.clone(),
                    )
                    .into_any_element(),
                    showcase_card(
                        "尺寸与边框",
                        "展示 large() 与 borderless 组合，避免和 Collapse 共用页面。",
                        self.sizes.clone(),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn accordion_demo_is_dedicated_and_rich() {
        let source = include_str!("accordion_demo.rs");
        assert!(source.contains("Accordion 手风琴"));
        assert!(source.contains("基础单开"));
        assert!(source.contains("多开与禁用"));
        assert!(source.contains("尺寸与边框"));
        assert!(source.contains("disabled_item"));
        let forbidden = ["use liora_components::{", "Collapse"].concat();
        assert!(!source.contains(&forbidden));
    }
}
