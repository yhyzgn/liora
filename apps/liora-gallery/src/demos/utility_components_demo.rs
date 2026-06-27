use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    FocusTrap, GroupBox, HoverCard, NativeMenu, NativeMenuItem, ScrollableMask, Space, Text,
    Toggle, ToggleGroup, ToggleOption, clipboard_text,
};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| UtilityComponentsDemo).into()
}

struct UtilityComponentsDemo;

impl Render for UtilityComponentsDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Utility Components 工具控件",
            "Toggle、GroupBox、HoverCard、ScrollableMask、Clipboard、FocusTrap 和 NativeMenu facade。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Toggle / ToggleGroup",
                    "适合工具栏状态按钮和单选模式切换。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Toggle::new("Bold", true))
                        .child(
                            ToggleGroup::new([
                                ToggleOption::new("preview", "Preview"),
                                ToggleOption::new("code", "Code"),
                                ToggleOption::new("split", "Split"),
                            ])
                            .selected("preview"),
                        ),
                ))
                .child(section(
                    "GroupBox",
                    "带标题和说明的表单/设置分组容器。",
                    GroupBox::new(
                        "Editor",
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .child(Text::new("Tab size: 4"))
                            .child(Text::new("Soft tabs enabled")),
                    )
                    .description("Project-level editor preferences."),
                ))
                .child(section(
                    "HoverCard",
                    "基于 Popover 的 hover/click preview facade。",
                    HoverCard::new(Text::new("Hover target").underline()).content(|_, _| {
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .child(Text::new("Preview card").bold())
                            .child(Text::new("Use this for profile or link previews."))
                    }),
                ))
                .child(section(
                    "ScrollableMask",
                    "滚动内容边缘渐隐提示。",
                    ScrollableMask::new(
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .children((1..=16).map(|i| Text::new(format!("Scrollable row {i}")))),
                    )
                    .height(px(160.0)),
                ))
                .child(section(
                    "Infrastructure facades",
                    "Clipboard、FocusTrap、NativeMenu 是给应用和 overlay 复用的轻量基础设施。",
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(Text::new(format!(
                            "Clipboard item: {:?}",
                            clipboard_text("Liora")
                        )))
                        .child(Text::new(format!(
                            "FocusTrap restore_focus: {}",
                            FocusTrap::new().restore_focus
                        )))
                        .child(Text::new(format!(
                            "NativeMenu items: {}",
                            NativeMenu::new("File")
                                .item(NativeMenuItem::new("open", "Open").shortcut("Ctrl+O"))
                                .items
                                .len()
                        ))),
                )),
        )
    }
}
