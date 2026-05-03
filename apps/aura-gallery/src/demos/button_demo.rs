use gpui::{div, prelude::*, px, AnyElement};
use aura_theme::AuraTheme;
use aura_components::AuraButton;

pub fn render(theme: &AuraTheme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(section_header(theme, "Variants 按钮变体"))
        .child(demo_row(
            theme,
            vec![
                AuraButton::new("Primary").primary().build(theme),
                AuraButton::new("Success").success().build(theme),
                AuraButton::new("Warning").warning().build(theme),
                AuraButton::new("Danger").danger().build(theme),
                AuraButton::new("Info").info().build(theme),
            ],
        ))
        .child(section_header(theme, "Sizes 尺寸"))
        .child(demo_row(
            theme,
            vec![
                AuraButton::new("Small").primary().small().build(theme),
                AuraButton::new("Default").primary().build(theme),
                AuraButton::new("Large").primary().large().build(theme),
            ],
        ))
        .child(section_header(theme, "States 状态"))
        .child(demo_row(
            theme,
            vec![
                AuraButton::new("Disabled").primary().disabled(true).build(theme),
                AuraButton::new("Loading").primary().loading(true).build(theme),
            ],
        ))
        .child(section_header(theme, "Plain 朴素按钮"))
        .child(demo_row(
            theme,
            vec![
                AuraButton::new("Default").build(theme),
                AuraButton::new("Plain Primary").primary().build(theme),
            ],
        ))
        .into_any_element()
}

fn section_header(theme: &AuraTheme, label: impl IntoElement) -> impl IntoElement {
    div()
        .text_size(px(theme.font_size.lg))
        .text_color(theme.text_primary)
        .font_weight(gpui::FontWeight::BOLD)
        .mt_2()
        .child(label)
}

fn demo_row(_theme: &AuraTheme, elements: Vec<impl IntoElement>) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .gap_2()
        .items_center()
        .flex_wrap()
        .children(elements)
}
