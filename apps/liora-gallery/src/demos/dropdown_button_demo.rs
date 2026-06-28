use gpui::{AnyElement, AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card, showcase_grid};
use liora_components::{
    DropdownButton, DropdownButtonItem, Space, Text, toast_info, toast_success,
};
use liora_core::Placement;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DropdownButtonDemo).into()
}

struct DropdownButtonDemo;

impl Render for DropdownButtonDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "DropdownButton 下拉按钮",
            "把主操作和一组相关命令合并到同一个按钮入口，支持普通按钮菜单和 split button。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Button menu showcase",
                    "所有下拉按钮示例使用一致的卡片、说明和双列按钮布局展示。",
                    showcase_grid(vec![
                        basic_menu_card(),
                        split_button_card(),
                        size_tone_card(),
                        item_state_card(),
                        close_policy_card(),
                    ]),
                ))
                .child(Text::new("提示：split 模式下点击左侧主按钮不会打开菜单，只执行 on_click；点击右侧箭头才展开菜单。")),
        )
    }
}

fn button_pair(left: impl IntoElement, right: impl IntoElement) -> impl IntoElement {
    Space::new().wrap().gap_md().child(left).child(right)
}

fn basic_menu_card() -> AnyElement {
    showcase_card(
        "基础菜单",
        "整颗按钮作为触发器，点击后展开一组命令。",
        button_pair(
            DropdownButton::new("Actions")
                .id("dropdown-button-demo-actions")
                .primary()
                .item("Create task", |_, _| toast_success!("Create task"))
                .item("Duplicate", |_, _| toast_info!("Duplicate"))
                .item("Archive", |_, _| toast_info!("Archive")),
            DropdownButton::new("Export")
                .id("dropdown-button-demo-export")
                .icon_start(IconName::Download)
                .item("Export CSV", |_, _| toast_info!("Export CSV"))
                .item("Export JSON", |_, _| toast_info!("Export JSON"))
                .item("Copy link", |_, _| toast_info!("Copy link")),
        ),
    )
    .into_any_element()
}

fn split_button_card() -> AnyElement {
    showcase_card(
        "Split button",
        "左侧执行主操作，右侧 Chevron 单独展开更多操作。",
        button_pair(
            DropdownButton::new("Deploy")
                .id("dropdown-button-demo-split-deploy")
                .primary()
                .split(true)
                .icon_start(IconName::Rocket)
                .on_click(|_, _| toast_success!("Deploy clicked"))
                .menu_item(
                    DropdownButtonItem::new("Preview deployment", |_, _| {
                        toast_info!("Preview deployment")
                    })
                    .icon(IconName::Eye),
                )
                .menu_item(
                    DropdownButtonItem::new("Rollback", |_, _| toast_info!("Rollback"))
                        .icon(IconName::Undo2),
                )
                .danger_item("Delete release", |_, _| toast_info!("Delete release")),
            DropdownButton::new("Save")
                .id("dropdown-button-demo-split-save")
                .success()
                .split(true)
                .on_click(|_, _| toast_success!("Saved"))
                .item("Save as draft", |_, _| toast_info!("Save as draft"))
                .item("Save template", |_, _| toast_info!("Save template"))
                .disabled_item("Locked by reviewer"),
        ),
    )
    .into_any_element()
}

fn size_tone_card() -> AnyElement {
    showcase_card(
        "尺寸与语义",
        "复用 Button 的尺寸、语义色和 secondary 风格。",
        Space::new()
            .wrap()
            .gap_md()
            .child(
                DropdownButton::new("Small")
                    .id("dropdown-button-demo-small")
                    .small()
                    .item("Action", |_, _| toast_info!("Small action")),
            )
            .child(
                DropdownButton::new("Default")
                    .id("dropdown-button-demo-default")
                    .info()
                    .item("Action", |_, _| toast_info!("Default action")),
            )
            .child(
                DropdownButton::new("Large")
                    .id("dropdown-button-demo-large")
                    .large()
                    .warning()
                    .secondary()
                    .item("Action", |_, _| toast_info!("Large action")),
            )
            .child(
                DropdownButton::new("Danger")
                    .id("dropdown-button-demo-danger")
                    .danger()
                    .item("Inspect", |_, _| toast_info!("Inspect"))
                    .danger_item("Remove", |_, _| toast_info!("Remove")),
            ),
    )
    .into_any_element()
}

fn item_state_card() -> AnyElement {
    showcase_card(
        "菜单项状态与位置",
        "菜单项支持图标、禁用、危险项；placement 沿用 Popover 定位。",
        button_pair(
            DropdownButton::new("Item states")
                .id("dropdown-button-demo-item-states")
                .placement(Placement::BottomStart)
                .menu_item(
                    DropdownButtonItem::new("Rename", |_, _| toast_info!("Rename"))
                        .icon(IconName::Pencil),
                )
                .menu_item(
                    DropdownButtonItem::new("Move", |_, _| toast_info!("Move"))
                        .icon(IconName::FolderInput),
                )
                .disabled_item("No permission")
                .danger_item("Delete permanently", |_, _| toast_info!("Delete")),
            DropdownButton::new("Top end")
                .id("dropdown-button-demo-top-end")
                .placement(Placement::TopEnd)
                .item("Pin", |_, _| toast_info!("Pin"))
                .item("Unpin", |_, _| toast_info!("Unpin")),
        ),
    )
    .into_any_element()
}

fn close_policy_card() -> AnyElement {
    showcase_card(
        "禁用与关闭策略",
        "禁用按钮不会打开菜单，也可以禁止外部点击或 ESC 自动关闭。",
        button_pair(
            DropdownButton::new("Disabled")
                .id("dropdown-button-demo-disabled")
                .disabled(true)
                .item("Hidden action", |_, _| toast_info!("Hidden action")),
            DropdownButton::new("Manual close")
                .id("dropdown-button-demo-manual-close")
                .close_on_click_outside(false)
                .close_on_escape(false)
                .item("Still closes on item click", |_, _| {
                    toast_info!("Item click")
                })
                .item("Duplicate", |_, _| toast_info!("Duplicate")),
        ),
    )
    .into_any_element()
}

#[cfg(test)]
mod tests {
    #[test]
    fn dropdown_button_demo_is_dedicated_and_rich() {
        let source = include_str!("dropdown_button_demo.rs");

        assert!(source.contains("DropdownButton 下拉按钮"));
        assert!(source.contains("Split button"));
        assert!(source.contains("菜单项状态与位置"));
        assert!(source.contains("禁用与关闭策略"));
        assert!(source.contains("DropdownButtonItem::new"));
        assert!(source.contains(".split(true)"));
        assert!(source.contains(".danger_item("));
        assert!(source.contains(".disabled_item("));
    }
}
