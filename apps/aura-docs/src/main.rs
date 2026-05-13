mod markdown;

use aura_components::{
    Autocomplete, Cascader, Checkbox, CodeBlock, ColorPicker, DatePicker, DateTimePicker, Dialog,
    Drawer, Input, MessageManager, Paragraph, Popover, Preview, Radio, RadioGroup, Select, Switch,
    Text, TimePicker,
};
use aura_core::init_aura;
use aura_theme::Theme;
use gpui::{App, Bounds, WindowBounds, WindowOptions, px, size};

fn run_docs() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());
        MessageManager::init(cx);
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

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(Bounds {
                    origin: gpui::Point::default(),
                    size: size(px(1680.0), px(1080.0)),
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Aura Docs — Native Main Window".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| markdown::render_docs_shell(cx),
        );
    });
}

fn main() {
    run_docs();
}
