mod category;
mod demos;

use aura_core::{ContextExt, init_aura, Portal};
use aura_theme::Theme;
use aura_components::{Input, Checkbox, Radio, RadioGroup, Switch, Dialog, Drawer};
use gpui::{
    AnyView, App, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div, prelude::*, px, size, Component
};

pub struct Gallery {
    demos: Vec<AnyView>,
}

fn run_gallery() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());

        // Register all key bindings
        Input::register_key_bindings(cx);
        Checkbox::register_key_bindings(cx);
        Radio::register_key_bindings(cx);
        RadioGroup::register_key_bindings(cx);
        Switch::register_key_bindings(cx);
        Dialog::register_key_bindings(cx);
        Drawer::register_key_bindings(cx);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(Bounds {
                    origin: gpui::Point::default(),
                    size: size(px(1920.0), px(1080.0)),
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Aura UI Gallery".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| {
                let demos = demos::registry()
                    .into_iter()
                    .map(|entry| (entry.render)(cx))
                    .collect();
                cx.new(|_| Gallery { demos })
            },
        );
    });
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let registry = demos::registry();

        let header = {
            let theme = cx.aura();
            div().flex().flex_col().gap_1().mb_4().pb_4().border_b_1().border_color(theme.neutral.border)
                .child(div().text_2xl().text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child("Aura UI"))
                .child(div().text_size(px(theme.font_size.md)).text_color(theme.neutral.text_3).child(format!("Native Component Library · {} chapters", registry.len())))
        };

        let mut body = div().flex().flex_col().size_full().bg(cx.aura().neutral.body).gap_8().p_8()
            .id("gallery-body").overflow_y_scroll().child(header);

        for (i, entry) in registry.into_iter().enumerate() {
            body = body.child(
                div().flex().flex_col().gap_4().p_4().border_1().border_color(cx.aura().neutral.divider).rounded(px(cx.aura().radius.lg)).bg(cx.aura().neutral.card)
                    .child(div().flex().flex_col().gap_1()
                        .child(div().text_size(px(cx.aura().font_size.lg)).text_color(cx.aura().neutral.text_1).font_weight(gpui::FontWeight::BOLD).child(entry.name))
                        .child(div().text_size(px(cx.aura().font_size.sm)).text_color(cx.aura().neutral.text_3).child(entry.description)))
                    .child(self.demos[i].clone())
            );
        }

        let container = div().size_full().relative()
            .child(body);
        
        aura_components::message::render_messages(cx);
        aura_components::notification::render_notifications(cx);
        aura_core::render_active_tooltip_in_window(_window, cx);
        aura_core::render_active_popover_in_window(_window, cx);
        aura_core::render_active_modal_in_window(_window, cx);
        aura_core::render_active_drawer_in_window(_window, cx);

        container.child(PortalLayer)
    }
}

struct PortalLayer;

impl IntoElement for PortalLayer {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}

impl gpui::RenderOnce for PortalLayer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut container = div().absolute().top_0().left_0().size_full();
        
        if cx.has_global::<Portal>() {
            let portals = std::mem::take(&mut cx.global_mut::<Portal>().entries);
            for entry in portals {
                container = container.child((entry.render)(window, cx));
            }
        }
        
        container
    }
}

#[cfg(not(target_family = "wasm"))] fn main() { run_gallery(); }
#[cfg(target_family = "wasm")] #[wasm_bindgen::prelude::wasm_bindgen(start)] pub fn start() { gpui_platform::web_init(); run_gallery(); }
