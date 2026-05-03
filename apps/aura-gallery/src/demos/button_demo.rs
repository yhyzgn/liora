use aura_components::AuraButton;
use aura_theme::AuraTheme;
use gpui::{AnyElement, div, prelude::*, px};

pub fn render(theme: &AuraTheme) -> AnyElement {
    let mut page = div().flex().flex_col().gap_3();
    page = page.child(hdr(theme, "Types 按钮类型"));
    page = page.child(row(types(theme)));
    page = page.child(hdr(theme, "Secondary 次要按钮"));
    page = page.child(row(secondary(theme)));
    page = page.child(hdr(theme, "Secondary · no border"));
    page = page.child(row(secondary_nb(theme)));
    page = page.child(hdr(theme, "Sizes 尺寸"));
    page = page.child(row(sizes(theme)));
    page = page.child(hdr(theme, "States 状态"));
    page = page.child(row(states(theme)));
    page = page.child(hdr(theme, "Rounded 圆角"));
    page = page.child(row(rounded(theme)));
    page.into_any_element()
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

fn types(theme: &AuraTheme) -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Default").build(theme),
        AuraButton::new("Tertiary").tertiary().build(theme),
        AuraButton::new("Primary").primary().build(theme),
        AuraButton::new("Info").info().build(theme),
        AuraButton::new("Success").success().build(theme),
        AuraButton::new("Warning").warning().build(theme),
        AuraButton::new("Error").danger().build(theme),
    ]
}

fn secondary(theme: &AuraTheme) -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Default").secondary().build(theme),
        AuraButton::new("Tertiary")
            .tertiary()
            .secondary()
            .build(theme),
        AuraButton::new("Primary")
            .primary()
            .secondary()
            .build(theme),
        AuraButton::new("Info").info().secondary().build(theme),
        AuraButton::new("Success")
            .success()
            .secondary()
            .build(theme),
        AuraButton::new("Warning")
            .warning()
            .secondary()
            .build(theme),
        AuraButton::new("Error").danger().secondary().build(theme),
    ]
}

fn secondary_nb(theme: &AuraTheme) -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Default")
            .secondary()
            .border(false)
            .build(theme),
        AuraButton::new("Primary")
            .primary()
            .secondary()
            .border(false)
            .build(theme),
        AuraButton::new("Info")
            .info()
            .secondary()
            .border(false)
            .build(theme),
        AuraButton::new("Success")
            .success()
            .secondary()
            .border(false)
            .build(theme),
        AuraButton::new("Warning")
            .warning()
            .secondary()
            .border(false)
            .build(theme),
        AuraButton::new("Error")
            .danger()
            .secondary()
            .border(false)
            .build(theme),
    ]
}

fn sizes(theme: &AuraTheme) -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Small").primary().small().build(theme),
        AuraButton::new("Default").primary().build(theme),
        AuraButton::new("Large").primary().large().build(theme),
    ]
}

fn states(theme: &AuraTheme) -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Disabled")
            .primary()
            .disabled(true)
            .build(theme),
        AuraButton::new("Loading")
            .primary()
            .loading(true)
            .build(theme),
        AuraButton::new("Sec Disabled")
            .primary()
            .secondary()
            .disabled(true)
            .build(theme),
    ]
}

fn rounded(theme: &AuraTheme) -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("4px").primary().rounded(4.0).build(theme),
        AuraButton::new("12px").primary().rounded(12.0).build(theme),
        AuraButton::new("20px").primary().rounded(20.0).build(theme),
        AuraButton::new("Pill")
            .primary()
            .rounded(9999.0)
            .build(theme),
    ]
}
