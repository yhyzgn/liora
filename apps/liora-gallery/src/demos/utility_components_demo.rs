use gpui::{AnyElement, AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section, showcase_card, showcase_grid};
use liora_components::{
    FocusTrap, GroupBox, HoverCard, NativeMenu, NativeMenuItem, ScrollableMask, Space, Tag, Text,
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
            "轻量交互、基础设施 facade 和小型分组容器统一放在规整的 showcase card 中展示。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Interactive utilities",
                    "每个示例保持同一宽度、标题层级和内容密度，便于横向比较。",
                    showcase_grid(vec![
                        toggle_card(),
                        group_box_card(),
                        hover_card_card(),
                        scrollable_mask_card(),
                    ]),
                ))
                .child(section(
                    "Infrastructure facades",
                    "Clipboard、FocusTrap、NativeMenu 不是视觉重控件，Gallery 用摘要卡片展示集成方式。",
                    showcase_grid(vec![clipboard_card(), focus_trap_card(), native_menu_card()]),
                )),
        )
    }
}

fn toggle_card() -> AnyElement {
    showcase_card(
        "Toggle / ToggleGroup",
        "Toolbar state and single-choice display mode controls.",
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
    )
    .into_any_element()
}

fn group_box_card() -> AnyElement {
    showcase_card(
        "GroupBox",
        "A compact setting or form section with title and description.",
        GroupBox::new(
            "Editor",
            Space::new()
                .vertical()
                .gap_sm()
                .child(Text::new("Tab size: 4"))
                .child(Text::new("Soft tabs enabled")),
        )
        .description("Project-level editor preferences."),
    )
    .into_any_element()
}

fn hover_card_card() -> AnyElement {
    showcase_card(
        "HoverCard",
        "Popover-based profile, link, or context preview.",
        Space::new().vertical().gap_sm().child(
            HoverCard::new(Text::new("Open preview").underline()).content(|_, _| {
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(Text::new("Preview card").bold())
                    .child(Text::new("Use this for profile or link previews."))
            }),
        ),
    )
    .into_any_element()
}

fn scrollable_mask_card() -> AnyElement {
    showcase_card(
        "ScrollableMask",
        "Edge fades communicate scrollable overflow without visual noise.",
        ScrollableMask::new(
            Space::new()
                .vertical()
                .gap_sm()
                .children((1..=12).map(|i| Text::new(format!("Scrollable row {i}")))),
        )
        .height(px(156.0)),
    )
    .into_any_element()
}

fn clipboard_card() -> AnyElement {
    let item = clipboard_text("Liora");
    showcase_card(
        "Clipboard",
        "Native text clipboard helper shared by copy actions.",
        Space::new()
            .vertical()
            .gap_sm()
            .child(Tag::new("Helper").info())
            .child(Text::new(format!("Prepared item: {item:?}")).wrap()),
    )
    .into_any_element()
}

fn focus_trap_card() -> AnyElement {
    let policy = FocusTrap::new();
    showcase_card(
        "FocusTrap",
        "Reusable overlay focus policy for modal-like surfaces.",
        Space::new()
            .vertical()
            .gap_sm()
            .child(Tag::new("Overlay policy").info())
            .child(Text::new(format!(
                "restore_focus: {}",
                policy.restore_focus
            )))
            .child(Text::new(format!(
                "close_on_escape: {}",
                policy.close_on_escape
            ))),
    )
    .into_any_element()
}

fn native_menu_card() -> AnyElement {
    let menu = NativeMenu::new("File")
        .item(NativeMenuItem::new("open", "Open").shortcut("Ctrl+O"))
        .item(NativeMenuItem::new("save", "Save").shortcut("Ctrl+S"));

    showcase_card(
        "NativeMenu",
        "Platform-neutral app menu descriptor for host integration.",
        Space::new()
            .vertical()
            .gap_sm()
            .child(Tag::new(menu.title).success())
            .children(menu.items.into_iter().map(|item| {
                Text::new(format!(
                    "{} · {}",
                    item.label,
                    item.shortcut.unwrap_or_else(|| "No shortcut".into())
                ))
            })),
    )
    .into_any_element()
}
