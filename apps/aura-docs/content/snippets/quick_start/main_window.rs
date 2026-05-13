//! Minimal GPUI + Aura window bootstrap.

use aura_components::{
    Autocomplete, Cascader, Checkbox, CodeBlock, ColorPicker, DatePicker, DateTimePicker, Dialog,
    Drawer, Input, MessageManager, Paragraph, Popover, Preview, Radio, RadioGroup, Select, Switch,
    Text, TimePicker, Title,
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
        Autocomplete::register_key_bindings(cx);
        Cascader::register_key_bindings(cx);
        ColorPicker::register_key_bindings(cx);
        DatePicker::register_key_bindings(cx);
        DateTimePicker::register_key_bindings(cx);
        Popover::register_key_bindings(cx);
        Select::register_key_bindings(cx);
        TimePicker::register_key_bindings(cx);
        Text::register_key_bindings(cx);
        Paragraph::register_key_bindings(cx);
        Title::register_key_bindings(cx);

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
