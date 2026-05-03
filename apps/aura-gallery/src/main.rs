mod category;
mod demos;

use gpui::{div, prelude::*, px, App, Bounds, Context, Render, Window, WindowBounds, WindowOptions, size};
use gpui_platform::application;
use aura_core::{init_aura, AuraContextExt};
use aura_theme::AuraTheme;
use category::Category;

pub struct Gallery;

fn run_gallery() {
    application().run(|cx: &mut App| {
        init_aura(cx, AuraTheme::light());
        cx.open_window(
            WindowOptions { window_bounds: Some(WindowBounds::Windowed(Bounds::centered(None, size(px(960.0), px(720.0)), cx))), ..Default::default() },
            |_, cx| cx.new(|_| Gallery),
        ).unwrap();
        cx.activate(true);
    });
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.aura();
        let registry = demos::registry();

        let header = div().flex().flex_col().gap_1().mb_4().pb_4().border_b_1().border_color(theme.neutral.border)
            .child(div().text_2xl().text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child("Aura UI"))
            .child(div().text_size(px(theme.font_size.md)).text_color(theme.neutral.text_3).child(format!("Native Component Library · {} components", registry.len())));

        let mut body = div().flex().flex_col().size_full().bg(theme.neutral.body).gap_4().p_8().child(header);

        for entry in &registry {
            let name = entry.name; let desc = entry.description;
            body = body.child(
                div().flex().flex_col().gap_4().p_4().border_1().border_color(theme.neutral.divider).rounded(px(theme.radius.lg)).bg(theme.neutral.card)
                    .child(div().flex().flex_col().gap_1()
                        .child(div().text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child(name))
                        .child(div().text_size(px(theme.font_size.sm)).text_color(theme.neutral.text_3).child(desc)))
                    .child((entry.render)(theme))
            );
        }
        body
    }
}

#[cfg(not(target_family = "wasm"))] fn main() { run_gallery(); }
#[cfg(target_family = "wasm")] #[wasm_bindgen::prelude::wasm_bindgen(start)] pub fn start() { gpui_platform::web_init(); run_gallery(); }
