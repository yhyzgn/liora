use aura_components::{Button, ButtonGroup};
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use aura_theme::Theme;
use gpui::{App, Context, Entity, IntoElement, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> Entity<ButtonDemo> {
    cx.new(|_| ButtonDemo)
}

pub struct ButtonDemo;

impl Render for ButtonDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(hdr(theme, "Types 按钮类型"))
            .child(row(types()))
            .child(hdr(theme, "Text 幽灵按钮 / 文字按钮"))
            .child(row(text_buttons()))
            .child(hdr(theme, "Secondary 次要按钮"))
            .child(row(secondary()))
            .child(hdr(theme, "Secondary · no border"))
            .child(row(secondary_nb()))
            .child(hdr(theme, "Sizes 尺寸"))
            .child(row(sizes()))
            .child(hdr(theme, "Icons 图标"))
            .child(row(icons()))
            .child(hdr(theme, "Custom Icons 自定义图标 (AnyElement)"))
            .child(row(custom_icons()))
            .child(hdr(theme, "Button Group 按钮组"))
            .child(button_groups())
            .child(hdr(theme, "States 状态"))
            .child(row(states()))
            .child(row(loading_states()))
            .child(hdr(theme, "Rounded 圆角"))
            .child(row(rounded()))
    }
}

fn hdr(theme: &Theme, s: &str) -> impl IntoElement {
    div()
        .text_size(px(theme.font_size.lg))
        .text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD)
        .mt_2()
        .child(s.to_string())
}

fn row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .gap_2()
        .items_center()
        .flex_wrap()
        .children(elements)
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
        Button::new("Tertiary")
            .tertiary()
            .secondary()
            .border(false),
        Button::new("Primary")
            .primary()
            .secondary()
            .border(false),
        Button::new("Info").info().secondary().border(false),
        Button::new("Success")
            .success()
            .secondary()
            .border(false),
        Button::new("Warning")
            .warning()
            .secondary()
            .border(false),
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
        Button::new("Search")
            .primary()
            .icon_start(IconName::Search),
        Button::new("Settings")
            .primary()
            .icon_start(IconName::Settings),
        Button::new("Done").success().icon_end(IconName::Check),
        Button::new("Delete").danger().icon_end(IconName::X),
        Button::new("Home")
            .tertiary()
            .icon_start(IconName::House),
        Button::new("External")
            .tertiary()
            .icon_end(Icon::new(IconName::ArrowRight)),
        Button::new("Upload").info().icon_top(IconName::ArrowUp),
        Button::new("Download").info().icon_bottom(IconName::ArrowDown),
        Button::new("").primary().icon_only(IconName::Search),
        Button::new("").danger().icon_only(IconName::X),
        Button::new("").success().icon_only(IconName::Check),
    ]
}

fn custom_icons() -> Vec<impl IntoElement> {
    vec![
        Button::new("Custom Element").primary().icon_start(
            div()
                .size(px(12.0))
                .bg(gpui::red())
                .rounded_full()
                .into_any_element()
        ),
        Button::new("Multiple Icons")
            .success()
            .icon_start(Icon::new(IconName::Check))
            .icon_end(Icon::new(IconName::Check)),
    ]
}

fn button_groups() -> impl IntoElement {
    div().flex().flex_col().gap_2().child(
        ButtonGroup::new()
            .button(Button::new("Previous").icon_start(Icon::new(IconName::ArrowLeft)))
            .button(Button::new("Next").icon_end(Icon::new(IconName::ArrowRight)))
    ).child(
        ButtonGroup::new()
            .button(Button::new("Edit").icon_start(Icon::new(IconName::Pencil)))
            .button(Button::new("Share").icon_start(Icon::new(IconName::Share2)))
            .button(Button::new("Delete").icon_start(Icon::new(IconName::Trash2)))
    )
}

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
        Button::new("4px").primary().rounded(px(4.0)),
        Button::new("12px").primary().rounded(px(12.0)),
        Button::new("20px").primary().rounded(px(20.0)),
        Button::new("Pill").primary().rounded(px(9999.0)),
    ]
}
