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
use aura_core::{Config, init_aura};
use aura_theme::Theme;
use gpui::{
    App, AppContext, Bounds, Context, Entity, IntoElement, Render, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .size_full()
            .bg(theme.neutral.body)
            .flex()
            .items_center()
            .justify_center()
            .p_6()
            .child(
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_lg()
                        .child(
                            Space::new()
                                .vertical()
                                .gap_xs()
                                .child(Title::new("Hello Aura").h2())
                                .child(Text::new(
                                    "A minimal pure Rust + GPUI native app using Aura components.",
                                )),
                        )
                        .child(
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Text::new("Project name"))
                                .child(self.name.clone()),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_4()
                                .child(
                                    Space::new()
                                        .vertical()
                                        .gap_xs()
                                        .child(Text::new("Enable native controls"))
                                        .child(Text::new(
                                            "Entity-backed state survives re-rendering.",
                                        )),
                                )
                                .child(self.enabled.clone()),
                        )
                        .child(
                            Button::new("Save")
                                .primary()
                                .on_click(|_, _, _| toast_success!("Saved from {}", "minimal app")),
                        ),
                )
                .width(px(520.0)),
            )
    }
}

fn main() {
    if std::env::var_os("DISPLAY").is_none() && std::env::var_os("WAYLAND_DISPLAY").is_none() {
        eprintln!("failed to start Aura Minimal App: neither DISPLAY nor WAYLAND_DISPLAY is set");
        eprintln!(
            "hint: run from a graphical Linux session, or export WAYLAND_DISPLAY/DISPLAY before launching."
        );
        std::process::exit(1);
    }

    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());
        MessageManager::init(cx);
        Input::register_key_bindings(cx);
        Switch::register_key_bindings(cx);
        Text::register_key_bindings(cx);
        Title::register_key_bindings(cx);

        eprintln!("starting Aura Minimal App...");
        match cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: gpui::Point::default(),
                    size: size(px(720.0), px(480.0)),
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Aura Minimal App".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                window.activate_window();
                cx.new(AppView::new)
            },
        ) {
            Ok(_) => eprintln!("Aura Minimal App window opened."),
            Err(error) => {
                eprintln!("failed to open Aura Minimal App window: {error:?}");
                eprintln!(
                    "hint: run from a graphical session with DISPLAY or WAYLAND_DISPLAY set."
                );
                cx.quit();
            }
        }
    });
}
