use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::{Segmented, SegmentedOption, Space};

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| SegmentedDemo {
        basic: cx.new(|_| {
            Segmented::new(vec![
                SegmentedOption::new("Daily", "daily"),
                SegmentedOption::new("Weekly", "weekly"),
                SegmentedOption::new("Monthly", "monthly"),
                SegmentedOption::new("Quarterly", "quarterly"),
                SegmentedOption::new("Yearly", "yearly"),
            ])
            .id("segmented-demo-basic")
            .on_change(|val, _, _| println!("Selected: {}", val))
        }),
        disabled: cx.new(|_| {
            Segmented::new(vec![
                SegmentedOption::new("Map", "Map"),
                SegmentedOption::new("Transit", "Transit"),
                SegmentedOption::new("Satellite", "Satellite").disabled(true),
            ])
            .id("segmented-demo-disabled")
            .value("Map")
            .on_change(|val, _, _| println!("Selected: {}", val))
        }),
        block: cx.new(|_| {
            Segmented::new(vec![
                SegmentedOption::new("123", "123"),
                SegmentedOption::new("456", "456"),
                SegmentedOption::new("long-text-option", "long"),
            ])
            .id("segmented-demo-block")
            .block(true)
            .on_change(|val, _, _| println!("Selected: {}", val))
        }),
    })
    .into()
}

struct SegmentedDemo {
    basic: Entity<Segmented>,
    disabled: Entity<Segmented>,
    block: Entity<Segmented>,
}

impl Render for SegmentedDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Segmented 分段控制器",
            "用于展示多个选项并允许用户选择其中单个选项。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section("基础用法", "基础分段选择。", self.basic.clone()))
                .child(section(
                    "不可用状态",
                    "禁用指定选项。",
                    self.disabled.clone(),
                ))
                .child(section(
                    "Block 模式",
                    "撑满父容器宽度。",
                    self.block.clone(),
                )),
        )
    }
}
