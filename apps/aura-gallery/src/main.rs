mod category;
mod demos;

use gpui::{
    div, prelude::*, px, App, Bounds, Context, Render, Window,
    WindowBounds, WindowOptions, size,
};
use gpui_platform::application;
use aura_core::{init_aura, AuraContextExt};
use aura_theme::AuraTheme;
use category::Category;

struct Gallery;

fn category_section(theme: &AuraTheme, category: Category, entries: &[&demos::DemoEntry]) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .mb_8()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .mb_4()
                .pb_2()
                .border_b_1()
                .border_color(theme.border)
                .child(
                    div()
                        .text_xl()
                        .text_color(theme.color.primary)
                        .child(format!("{} {}", category.icon(), category.name())),
                )
                .child(
                    div()
                        .text_size(px(theme.font_size.sm))
                        .text_color(theme.text_secondary)
                        .child(format!("{} components", entries.len())),
                ),
        )
        .children(entries.iter().map(|entry| component_card(theme, entry)))
}

fn component_card(theme: &AuraTheme, entry: &demos::DemoEntry) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .p_4()
        .border_1()
        .border_color(theme.border_light)
        .rounded(px(theme.radius.lg))
        .bg(theme.background)
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_size(px(theme.font_size.lg))
                        .text_color(theme.text_primary)
                        .font_weight(gpui::FontWeight::BOLD)
                        .child(entry.name),
                )
                .child(
                    div()
                        .text_size(px(theme.font_size.sm))
                        .text_color(theme.text_secondary)
                        .child(entry.description),
                ),
        )
        .child(
            div()
                .child((entry.render)(theme))
        )
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = _cx.aura();
        let registry = demos::registry();

        let grouped: Vec<(Category, Vec<&demos::DemoEntry>)> = Category::ALL
            .iter()
            .map(|&cat| {
                let entries: Vec<_> = registry
                    .iter()
                    .filter(|e| e.category == cat)
                    .collect();
                (cat, entries)
            })
            .filter(|(_, entries)| !entries.is_empty())
            .collect();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .gap_4()
            .p_8()

            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .mb_4()
                    .pb_4()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_2xl()
                            .text_color(theme.text_primary)
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Aura UI"),
                    )
                    .child(
                        div()
                            .text_size(px(theme.font_size.md))
                            .text_color(theme.text_secondary)
                            .child(format!(
                                "Native Component Library — {} components",
                                registry.len()
                            )),
                    ),
            )
            .children(grouped.iter().map(|(cat, entries)| {
                category_section(theme, *cat, entries)
            }))
    }
}

fn run_gallery() {
    application().run(|cx: &mut App| {
        init_aura(cx, AuraTheme::light());

        let bounds = Bounds::centered(None, size(px(960.0), px(720.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| Gallery)
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_gallery();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_gallery();
}
