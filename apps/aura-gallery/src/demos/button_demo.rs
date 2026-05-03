use gpui::{div, prelude::*, px, AnyElement};
use aura_theme::AuraTheme;
use aura_components::AuraButton;

pub fn render(theme: &AuraTheme) -> AnyElement {
    div()
        .flex().flex_col().gap_3()
        .child(section_header(theme, "Types 按钮类型"))
        .child(demo_row(vec![
            AuraButton::new("Default").build(theme),
            AuraButton::new("Tertiary").tertiary().build(theme),
            AuraButton::new("Primary").primary().build(theme),
            AuraButton::new("Info").info().build(theme),
            AuraButton::new("Success").success().build(theme),
            AuraButton::new("Warning").warning().build(theme),
            AuraButton::new("Error").danger().build(theme),
        ]))
        .child(section_header(theme, "Secondary 次要按钮"))
        .child(demo_row(vec![
            AuraButton::new("Default").secondary().build(theme),
            AuraButton::new("Tertiary").tertiary().secondary().build(theme),
            AuraButton::new("Primary").primary().secondary().build(theme),
            AuraButton::new("Info").info().secondary().build(theme),
            AuraButton::new("Success").success().secondary().build(theme),
            AuraButton::new("Warning").warning().secondary().build(theme),
            AuraButton::new("Error").danger().secondary().build(theme),
        ]))
        .child(section_header(theme, "Secondary · no border"))
        .child(demo_row(vec![
            AuraButton::new("Default").secondary().border(false).build(theme),
            AuraButton::new("Primary").primary().secondary().border(false).build(theme),
            AuraButton::new("Info").info().secondary().border(false).build(theme),
            AuraButton::new("Success").success().secondary().border(false).build(theme),
            AuraButton::new("Warning").warning().secondary().border(false).build(theme),
            AuraButton::new("Error").danger().secondary().border(false).build(theme),
        ]))
        .child(section_header(theme, "Sizes 尺寸"))
        .child(demo_row(vec![
            AuraButton::new("Small").primary().small().build(theme),
            AuraButton::new("Default").primary().build(theme),
            AuraButton::new("Large").primary().large().build(theme),
        ]))
        .child(section_header(theme, "States 状态"))
        .child(demo_row(vec![
            AuraButton::new("Disabled").primary().disabled(true).build(theme),
            AuraButton::new("Loading").primary().loading(true).build(theme),
            AuraButton::new("Secondary Disabled").primary().secondary().disabled(true).build(theme),
        ]))
        .child(section_header(theme, "Rounded 圆角"))
        .child(demo_row(vec![
            AuraButton::new("4px").primary().rounded(4.0).build(theme),
            AuraButton::new("12px").primary().rounded(12.0).build(theme),
            AuraButton::new("20px").primary().rounded(20.0).build(theme),
            AuraButton::new("Pill").primary().rounded(9999.0).build(theme),
        ]))
        .into_any_element()
}

fn section_header(theme: &AuraTheme, label: impl IntoElement) -> impl IntoElement {
    div()
        .text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD).mt_2().child(label)
}

fn demo_row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    div().flex().flex_row().gap_2().items_center().flex_wrap().children(elements)
}
