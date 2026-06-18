//! Minimal external-style GPUI application using Aura.
//!
//! Run with:
//!
//! ```bash
//! cargo run -p aura-minimal-app
//! ```

use aura_components::{
    Button, Card, Input, MessageManager, Space, Switch, Text, Title, toast_success,
};
use aura_core::init_aura;
use aura_theme::Theme;
use gpui::{
    App, AppContext, Bounds, Context, Entity, Render, Window, WindowBounds, WindowOptions, px, size,
};

struct AppView {
    name: Entity<Input>,
    enabled: Entity<Switch>,
}

impl AppView {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            name: cx.new(|cx| Input::new("Aura", cx).placeholder("Project name")),
            enabled: cx.new(|cx| Switch::new(true, cx)),
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Card::new(
            Space::new()
                .vertical()
                .gap_lg()
                .child(Title::new("Hello Aura").h2())
                .child(Text::new("This is a pure Rust + GPUI native Aura app."))
                .child(self.name.clone())
                .child(self.enabled.clone())
                .child(
                    Button::new("Save")
                        .primary()
                        .on_click(|_, _, _| toast_success!("Saved from {}", "minimal app")),
                ),
        )
        .no_shadow()
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());
        MessageManager::init(cx);
        Input::register_key_bindings(cx);
        Switch::register_key_bindings(cx);
        Text::register_key_bindings(cx);
        Title::register_key_bindings(cx);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: gpui::Point::default(),
                    size: size(px(720.0), px(420.0)),
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Aura Minimal App".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(AppView::new),
        );
    });
}
