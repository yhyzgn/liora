//! Minimal GPUI + Aura window bootstrap.

use aura_components::{
    Checkbox, CodeBlock, Dialog, Drawer, Input, MessageManager, Preview, Radio, RadioGroup, Switch,
};
use aura_core::init_aura;
use aura_theme::Theme;
use gpui::{
    App, AppContext, Bounds, Context, Render, Window, WindowBounds, WindowOptions, px, size,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        aura_components::Title::new("Aura Native Demo").h2()
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        // 1. Register Aura theme/config globally.
        init_aura(cx, Theme::light());

        // 2. Initialize global services used by components.
        MessageManager::init(cx);

        // 3. Register keyboard behavior for interactive components.
        Input::register_key_bindings(cx);
        CodeBlock::register_key_bindings(cx);
        Checkbox::register_key_bindings(cx);
        Radio::register_key_bindings(cx);
        RadioGroup::register_key_bindings(cx);
        Switch::register_key_bindings(cx);
        Dialog::register_key_bindings(cx);
        Drawer::register_key_bindings(cx);
        Preview::register_key_bindings(cx);

        // 4. Open the native window and mount a root GPUI View.
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: gpui::Point::default(),
                    size: size(px(1100.0), px(760.0)),
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Aura Native Demo".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| RootView),
        );
    });
}
