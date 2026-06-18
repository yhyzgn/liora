use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*, rgb};
use liora_components::{Button, ButtonGroup, Space, Title};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> Entity<ButtonDemo> {
    cx.new(|_| ButtonDemo)
}

pub struct ButtonDemo;

impl Render for ButtonDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Space::new()
            .vertical()
            .gap_md()
            .child(hdr("Types 按钮类型"))
            .child(row(types()))
            .child(hdr("Text 幽灵按钮 / 文字按钮"))
            .child(row(text_buttons()))
            .child(hdr("Secondary 次要按钮"))
            .child(row(secondary()))
            .child(hdr("Secondary · no border"))
            .child(row(secondary_nb()))
            .child(hdr("Sizes 尺寸"))
            .child(row(sizes()))
            .child(hdr("Icons 图标"))
            .child(row(icons()))
            .child(hdr("Custom Icons 自定义图标 (AnyElement)"))
            .child(row(custom_icons()))
            .child(hdr("Button Group 按钮组"))
            .child(button_groups())
            .child(hdr("States 状态"))
            .child(row(states()))
            .child(row(loading_states()))
            .child(hdr("Rounded 圆角"))
            .child(row(rounded()))
            .child(hdr("Custom Colors 自定义颜色"))
            .child(row(custom_colors()))
            .child(hdr("Gradient 渐变按钮"))
            .child(row(gradients()))
    }
}

fn hdr(s: &str) -> impl IntoElement {
    Title::new(s.to_string()).h3()
}

fn row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().wrap().gap_sm().children(elements)
}

fn types() -> Vec<impl IntoElement> {
    vec![
        Button::new("Default"),
        Button::new("Tertiary").tertiary(),
        Button::new("Primary").primary(),
        Button::new("Info").info(),
        Button::new("Success").success(),
        Button::new("Warning").warning(),
        Button::new("Error").danger(),
    ]
}

fn text_buttons() -> Vec<impl IntoElement> {
    vec![
        Button::new("Default").text(),
        Button::new("Primary").text().primary(),
        Button::new("Info").text().info(),
        Button::new("Success").text().success(),
        Button::new("Warning").text().warning(),
        Button::new("Danger").text().danger(),
    ]
}

fn secondary() -> Vec<impl IntoElement> {
    vec![
        Button::new("Default").secondary(),
        Button::new("Tertiary").tertiary().secondary(),
        Button::new("Primary").primary().secondary(),
        Button::new("Info").info().secondary(),
        Button::new("Success").success().secondary(),
        Button::new("Warning").warning().secondary(),
        Button::new("Error").danger().secondary(),
    ]
}

fn secondary_nb() -> Vec<impl IntoElement> {
    vec![
        Button::new("Default").secondary().border(false),
        Button::new("Tertiary").tertiary().secondary().border(false),
        Button::new("Primary").primary().secondary().border(false),
        Button::new("Info").info().secondary().border(false),
        Button::new("Success").success().secondary().border(false),
        Button::new("Warning").warning().secondary().border(false),
        Button::new("Error").danger().secondary().border(false),
    ]
}

fn sizes() -> Vec<impl IntoElement> {
    vec![
        Button::new("Small").primary().small(),
        Button::new("Default").primary(),
        Button::new("Large").primary().large(),
    ]
}

fn icons() -> Vec<impl IntoElement> {
    vec![
        Button::new("Search").primary().icon_start(IconName::Search),
        Button::new("Settings")
            .primary()
            .icon_start(IconName::Settings),
        Button::new("Done").success().icon_end(IconName::Check),
        Button::new("Delete").danger().icon_end(IconName::X),
        Button::new("Home").tertiary().icon_start(IconName::House),
        Button::new("External")
            .tertiary()
            .icon_end(Icon::new(IconName::ArrowRight)),
        Button::new("Upload").info().icon_top(IconName::ArrowUp),
        Button::new("Download")
            .info()
            .icon_bottom(IconName::ArrowDown),
        Button::new("").primary().icon_only(IconName::Search),
        Button::new("").danger().icon_only(IconName::X),
        Button::new("").success().icon_only(IconName::Check),
    ]
}

fn custom_icons() -> Vec<impl IntoElement> {
    vec![
        Button::new("Custom Element")
            .primary()
            .icon_start(Icon::new(IconName::Star)),
        Button::new("Multiple Icons")
            .success()
            .icon_start(Icon::new(IconName::Check))
            .icon_end(Icon::new(IconName::Check)),
    ]
}

fn button_groups() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(
            ButtonGroup::new()
                .button(Button::new("Previous").icon_start(Icon::new(IconName::ArrowLeft)))
                .button(Button::new("Next").icon_end(Icon::new(IconName::ArrowRight))),
        )
        .child(
            ButtonGroup::new()
                .button(Button::new("Edit").icon_start(Icon::new(IconName::Pencil)))
                .button(Button::new("Share").icon_start(Icon::new(IconName::Share2)))
                .button(Button::new("Delete").icon_start(Icon::new(IconName::Trash2))),
        )
}

#[allow(dead_code)]
fn secondary_icons() -> Vec<impl IntoElement> {
    vec![
        Button::new("Default")
            .secondary()
            .icon_start(Icon::new(IconName::Search)),
        Button::new("Tertiary")
            .tertiary()
            .secondary()
            .icon_start(Icon::new(IconName::Search)),
        Button::new("Search")
            .primary()
            .secondary()
            .icon_start(Icon::new(IconName::Search)),
        Button::new("Info")
            .info()
            .secondary()
            .icon_start(Icon::new(IconName::Info)),
        Button::new("Success")
            .success()
            .secondary()
            .icon_start(Icon::new(IconName::Check)),
        Button::new("Warning")
            .warning()
            .secondary()
            .icon_start(Icon::new(IconName::Goal)),
        Button::new("Error")
            .danger()
            .secondary()
            .icon_start(Icon::new(IconName::X)),
    ]
}

fn states() -> Vec<impl IntoElement> {
    vec![
        Button::new("Disabled").primary().disabled(true),
        Button::new("Sec Disabled")
            .primary()
            .secondary()
            .disabled(true),
    ]
}

fn loading_states() -> Vec<impl IntoElement> {
    vec![
        Button::new("Loading").primary().loading(true),
        Button::new("Saving").success().loading(true),
        Button::new("Uploading").info().loading(true),
    ]
}

fn rounded() -> Vec<impl IntoElement> {
    vec![
        Button::new("4px").primary().rounded_sm(),
        Button::new("12px").primary().rounded_md(),
        Button::new("20px").primary().rounded_lg(),
        Button::new("Pill").primary().pill(),
    ]
}

fn custom_colors() -> Vec<impl IntoElement> {
    vec![
        Button::new("Violet")
            .custom_color(rgb(0x7c3aed).into(), gpui::white())
            .pill(),
        Button::new("Outline")
            .colors(liora_components::ButtonColors::outline(
                rgb(0x0891b2).into(),
                rgb(0x0f172a).into(),
                gpui::transparent_black(),
            ))
            .rounded_md(),
        Button::new("Disabled")
            .custom_color(rgb(0xdb2777).into(), gpui::white())
            .disabled(true),
    ]
}

fn gradients() -> Vec<impl IntoElement> {
    vec![
        Button::new("Aurora")
            .gradient(rgb(0x6366f1).into(), rgb(0x06b6d4).into())
            .pill(),
        Button::new("Sunset")
            .gradient_with_angle(110.0, rgb(0xf97316).into(), rgb(0xec4899).into())
            .large()
            .rounded_lg(),
        Button::new("Loading")
            .gradient(rgb(0x22c55e).into(), rgb(0x14b8a6).into())
            .loading(true),
        Button::new("Disabled")
            .gradient(rgb(0x8b5cf6).into(), rgb(0x3b82f6).into())
            .disabled(true),
    ]
}
