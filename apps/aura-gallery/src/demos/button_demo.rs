use aura_components::AuraButton;
use aura_core::AuraConfig;
use aura_icons_lucide::IconName;
use aura_theme::AuraTheme;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement {
    Component::new(ButtonDemo).into_any_element()
}

struct ButtonDemo;

impl RenderOnce for ButtonDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<AuraConfig>().theme;
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(hdr(theme, "Types 按钮类型"))
            .child(row(types()))
            .child(hdr(theme, "Secondary 次要按钮"))
            .child(row(secondary()))
            .child(hdr(theme, "Secondary · no border"))
            .child(row(secondary_nb()))
            .child(hdr(theme, "Sizes 尺寸"))
            .child(row(sizes()))
            .child(hdr(theme, "Icons 图标"))
            .child(row(icons()))
            .child(hdr(theme, "Icons 图标的次要按钮"))
            .child(row(secondary_icons()))
            .child(hdr(theme, "States 状态"))
            .child(row(states()))
            .child(row(loading_states()))
            .child(hdr(theme, "Rounded 圆角"))
            .child(row(rounded()))
    }
}

fn hdr(theme: &AuraTheme, s: &str) -> impl IntoElement {
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
        AuraButton::new("Default"),
        AuraButton::new("Tertiary").tertiary(),
        AuraButton::new("Primary").primary(),
        AuraButton::new("Info").info(),
        AuraButton::new("Success").success(),
        AuraButton::new("Warning").warning(),
        AuraButton::new("Error").danger(),
    ]
}

fn secondary() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Default").secondary(),
        AuraButton::new("Tertiary").tertiary().secondary(),
        AuraButton::new("Primary").primary().secondary(),
        AuraButton::new("Info").info().secondary(),
        AuraButton::new("Success").success().secondary(),
        AuraButton::new("Warning").warning().secondary(),
        AuraButton::new("Error").danger().secondary(),
    ]
}

fn secondary_nb() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Default").secondary().border(false),
        AuraButton::new("Tertiary")
            .tertiary()
            .secondary()
            .border(false),
        AuraButton::new("Primary")
            .primary()
            .secondary()
            .border(false),
        AuraButton::new("Info").info().secondary().border(false),
        AuraButton::new("Success")
            .success()
            .secondary()
            .border(false),
        AuraButton::new("Warning")
            .warning()
            .secondary()
            .border(false),
        AuraButton::new("Error").danger().secondary().border(false),
    ]
}

fn sizes() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Small").primary().small(),
        AuraButton::new("Default").primary(),
        AuraButton::new("Large").primary().large(),
    ]
}

fn icons() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Search").primary().icon_start(IconName::Search),
        AuraButton::new("Settings").primary().icon_start(IconName::Settings),
        AuraButton::new("Done").success().icon_end(IconName::Check),
        AuraButton::new("Delete").danger().icon_end(IconName::X),
        AuraButton::new("Home").tertiary().icon_start(IconName::House),
        AuraButton::new("External").tertiary().icon_end(IconName::ArrowRight),
        AuraButton::new("Upload").info().icon_top(IconName::ArrowUp),
        AuraButton::new("Download").info().icon_bottom(IconName::ArrowDown),
        AuraButton::new("").primary().icon_only(IconName::Search),
        AuraButton::new("").danger().icon_only(IconName::X),
        AuraButton::new("").success().icon_only(IconName::Check),
    ]
}

fn secondary_icons() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Default")
            .secondary()
            .icon_start(IconName::Search),
        AuraButton::new("Tertiary")
            .tertiary()
            .secondary()
            .icon_start(IconName::Search),
        AuraButton::new("Search")
            .primary()
            .secondary()
            .icon_start(IconName::Search),
        AuraButton::new("Info")
            .info()
            .secondary()
            .icon_start(IconName::Info),
        AuraButton::new("Success")
            .success()
            .secondary()
            .icon_start(IconName::Check),
        AuraButton::new("Warning")
            .warning()
            .secondary()
            .icon_start(IconName::Goal),
        AuraButton::new("Error")
            .danger()
            .secondary()
            .icon_start(IconName::X),
    ]
}

fn states() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Disabled").primary().disabled(true),
        AuraButton::new("Sec Disabled")
            .primary()
            .secondary()
            .disabled(true),
    ]
}

fn loading_states() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Loading").primary().loading(true),
        AuraButton::new("Saving").success().loading(true),
        AuraButton::new("Uploading").info().loading(true),
    ]
}

fn rounded() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("4px").primary().rounded(4.0),
        AuraButton::new("12px").primary().rounded(12.0),
        AuraButton::new("20px").primary().rounded(20.0),
        AuraButton::new("Pill").primary().rounded(9999.0),
    ]
}
