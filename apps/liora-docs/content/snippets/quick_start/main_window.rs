//! Minimal GPUI + Liora window bootstrap.

use gpui::{
    App, AppContext, Application, Bounds, Context, Render, Window, WindowBounds, WindowOptions, px,
    size,
};
use liora_components::{ThemeMode, init_liora, init_liora_with_mode};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        liora_components::Title::new("Liora Native Demo").h2()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // 1. Initialize Liora core/theme state, component services, and key bindings.
        init_liora(cx);

        // Optional: choose an explicit startup mode instead.
        // init_liora_with_mode(cx, ThemeMode::Dark);

        // 2. Open the native window and mount a root GPUI View.
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: gpui::Point::default(),
                    size: size(px(1100.0), px(760.0)),
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Liora Native Demo".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| RootView),
        );
    });
}
