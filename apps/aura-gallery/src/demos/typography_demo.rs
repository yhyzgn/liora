use aura_components::{Link, Paragraph, Text, Title, Divider};
use aura_core::Config;
use aura_theme::Theme;
use gpui::{App, Context, Entity, IntoElement, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> Entity<TypographyDemo> {
    cx.new(|_| TypographyDemo)
}

pub struct TypographyDemo;

impl Render for TypographyDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(hdr(theme, "Title 标题"))
            .child(div().flex().flex_col().gap_2().children(vec![
                Title::new("h1. Aura UI Title").h1().into_any_element(),
                Title::new("h2. Aura UI Title").h2().into_any_element(),
                Title::new("h3. Aura UI Title").h3().into_any_element(),
                Title::new("h4. Aura UI Title").h4().into_any_element(),
                Title::new("h5. Aura UI Title").h5().into_any_element(),
                Title::new("h6. Aura UI Title").h6().into_any_element(),
            ]))
            .child(Divider::new())
            .child(hdr(theme, "Text 文本"))
            .child(div().flex().flex_col().gap_2().children(vec![
                Text::new("Default text color and size"),
                Text::new("Primary color text").color(theme.primary.base),
                Text::new("Success color text").color(theme.success.base),
                Text::new("Warning color text").color(theme.warning.base),
                Text::new("Danger color text").color(theme.danger.base),
                Text::new("Small text").size(px(theme.font_size.sm)),
                Text::new("Large text").size(px(theme.font_size.lg)),
            ]))
            .child(Divider::new())
            .child(hdr(theme, "Link 链接"))
            .child(div().flex().flex_row().gap_4().children(vec![
                Link::new("Default Link"),
                Link::new("Primary Link").primary(),
                Link::new("Success Link").success(),
                Link::new("Warning Link").warning(),
                Link::new("Danger Link").danger(),
                Link::new("No Underline").underline(false),
                Link::new("Disabled").disabled(true),
            ]))
            .child(Divider::new())
            .child(hdr(theme, "Paragraph 段落"))
            .child(div().w_80().child(
                Paragraph::new("Aura UI is a professional desktop UI library for Rust, built on top of GPUI. It provides a comprehensive set of components inspired by Element Plus, designed to help developers build beautiful and performant native applications.")
            ))
    }
}

fn hdr(theme: &Theme, s: &str) -> impl IntoElement {
    div()
        .text_size(px(theme.font_size.lg))
        .text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD)
        .child(s.to_string())
}
