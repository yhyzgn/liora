use aura_components::{Card, Space, Text, Transfer, TransferItem};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TransferDemo::new(cx)).into()
}

struct TransferDemo {
    basic: Entity<Transfer>,
    filtered: Entity<Transfer>,
    disabled: Entity<Transfer>,
}

impl TransferDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| {
                Transfer::new(city_items())
                    .titles("待选城市", "已选城市")
                    .target_keys(["shanghai"])
                    .checked_source_keys(["beijing", "shenzhen"])
            }),
            filtered: cx.new(|_| {
                Transfer::new(role_items())
                    .titles("全部角色", "已授权")
                    .filterable(true)
                    .source_filter("admin")
                    .target_filter("ops")
                    .target_keys(["ops"])
                    .checked_source_keys(["admin"])
                    .width_lg()
            }),
            disabled: cx.new(|_| {
                Transfer::new(city_items())
                    .titles("源列表", "目标列表")
                    .target_keys(["guangzhou", "disabled"])
                    .checked_target_keys(["guangzhou", "disabled"])
            }),
        }
    }
}

impl Render for TransferDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Transfer 穿梭框",
            "在两个列表之间移动条目，适合权限分配、人员选择等场景。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "示例预勾选两个源列表条目，点击右箭头移动到目标列表。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.basic.clone())
                            .child(Text::new("示例预勾选两个源列表条目，点击右箭头移动到目标列表。")),
                    ),
                ))
                .child(section(
                    "过滤展示",
                    "当前过滤文本通过 source_filter / target_filter 预置，业务可用自己的输入框驱动这些值。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.filtered.clone())
                            .child(Text::new("当前过滤文本通过 source_filter / target_filter 预置，业务可用自己的输入框驱动这些值。")),
                    ),
                ))
                .child(section(
                    "禁用项",
                    "禁用条目不可勾选或移动，已选目标中的禁用项也会保留。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.disabled.clone())
                            .child(Text::new("禁用条目不可勾选或移动，已选目标中的禁用项也会保留。路径测试覆盖了该行为。")),
                    ),
                )),
        )
    }
}

fn city_items() -> Vec<TransferItem> {
    vec![
        TransferItem::new("beijing", "北京").description("华北区域"),
        TransferItem::new("shanghai", "上海").description("华东区域"),
        TransferItem::new("shenzhen", "深圳").description("华南区域"),
        TransferItem::new("guangzhou", "广州").description("华南区域"),
        TransferItem::new("disabled", "成都（禁用）")
            .description("不可移动")
            .disabled(true),
    ]
}

fn role_items() -> Vec<TransferItem> {
    vec![
        TransferItem::new("admin", "Admin 管理员").description("admin / full access"),
        TransferItem::new("editor", "Editor 编辑").description("content write"),
        TransferItem::new("viewer", "Viewer 只读").description("read only"),
        TransferItem::new("ops", "Ops 运维").description("ops / deploy"),
        TransferItem::new("auditor", "Auditor 审计").description("compliance"),
    ]
}
