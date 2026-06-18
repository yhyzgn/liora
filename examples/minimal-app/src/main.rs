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
    WindowControlArea, WindowDecorations, WindowOptions, div, prelude::*, px, size,
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

    fn titlebar(&self, window: &mut Window, theme: &Theme) -> impl IntoElement {
        div()
            .id("minimal-titlebar")
            .window_control_area(WindowControlArea::Drag)
            .h(px(40.0))
            .w_full()
            .px_3()
            .bg(theme.neutral.card)
            .border_b_1()
            .border_color(theme.neutral.border)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .w(px(10.0))
                            .h(px(10.0))
                            .rounded_full()
                            .bg(theme.primary.base),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(theme.neutral.text_1)
                            .child("Aura Minimal"),
                    ),
            )
            .child(
                div()
                    .window_control_area(WindowControlArea::Drag)
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(window_button(
                        "minimal-window-min",
                        "−",
                        theme,
                        WindowControlArea::Min,
                        |window, _cx| {
                            window.minimize_window();
                        },
                    ))
                    .child(window_button(
                        "minimal-window-max",
                        if window.is_maximized() { "↙" } else { "□" },
                        theme,
                        WindowControlArea::Max,
                        |window, _cx| {
                            window.zoom_window();
                        },
                    ))
                    .child(window_button(
                        "minimal-window-close",
                        "×",
                        theme,
                        WindowControlArea::Close,
                        |window, cx| {
                            window.remove_window();
                            cx.quit();
                        },
                    )),
            )
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .size_full()
            .bg(theme.neutral.body)
            .flex()
            .flex_col()
            .child(self.titlebar(window, &theme))
            .child(
                div().flex_1().min_h_0().p_4().child(
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
                    .width(px(680.0)),
                ),
            )
    }
}

fn window_button(
    id: &'static str,
    label: &'static str,
    theme: &Theme,
    area: WindowControlArea,
    action: fn(&mut Window, &mut App),
) -> impl IntoElement {
    div()
        .id(id)
        .window_control_area(area)
        .w(px(32.0))
        .h(px(28.0))
        .rounded(px(theme.radius.sm))
        .flex()
        .items_center()
        .justify_center()
        .text_sm()
        .text_color(theme.neutral.text_2)
        .hover(|style| style.bg(theme.neutral.hover).cursor_pointer())
        .on_click(move |_, window, cx| action(window, cx))
        .child(label)
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
                    size: size(px(960.0), px(600.0)),
                })),
                titlebar: None,
                window_decorations: Some(WindowDecorations::Client),
                window_min_size: Some(size(px(640.0), px(420.0))),
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
