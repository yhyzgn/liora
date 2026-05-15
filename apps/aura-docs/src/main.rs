mod markdown;

use aura_components::{
    Autocomplete, Cascader, Checkbox, CodeBlock, ColorPicker, DatePicker, DateTimePicker, Dialog,
    Drawer, Input, MessageManager, Paragraph, Popover, Preview, Radio, RadioGroup, Select, Switch,
    Text, TimePicker, Title,
};
use aura_core::init_aura;
use aura_theme::Theme;
use aura_tray::{
    AuraTray, MouseButton, MouseButtonState, TrayCommand, TrayConfig, TrayIconEvent,
    default_aura_tray_menu, solid_icon,
};
use gpui::{App, Bounds, Global, Window, WindowBounds, WindowOptions, px, size};
use std::{sync::mpsc, time::Duration};

struct DocsTrayState {
    tray: AuraTray,
    window: Option<gpui::AnyWindowHandle>,
    window_visible: bool,
    resident_enabled: bool,
    auto_show: bool,
}

impl Global for DocsTrayState {}

fn run_docs() {
    gpui_platform::application()
        .with_quit_mode(gpui::QuitMode::Explicit)
        .run(|cx: &mut App| {
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
            Title::register_key_bindings(cx);

            install_docs_tray(cx);
            if let Some(handle) = open_docs_window(cx) {
                if cx.has_global::<DocsTrayState>() {
                    cx.global_mut::<DocsTrayState>().window = Some(handle);
                }
            }
        });
}

fn open_docs_window(cx: &mut App) -> Option<gpui::AnyWindowHandle> {
    match cx.open_window(docs_window_options(), |_, cx| {
        markdown::render_docs_shell(cx)
    }) {
        Ok(handle) => Some(handle.into()),
        Err(error) => {
            eprintln!("failed to open Aura Docs window: {error:?}");
            None
        }
    }
}

fn docs_window_options() -> WindowOptions {
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
    }
}

fn install_docs_tray(cx: &mut App) {
    let (tx, rx) = mpsc::channel::<TrayCommand>();
    let menu_tx = tx.clone();
    aura_tray::MenuEvent::set_event_handler(Some(move |event: aura_tray::MenuEvent| {
        if let Some(command) = TrayCommand::from_id(event.id().as_ref()) {
            let _ = menu_tx.send(command);
        }
    }));

    TrayIconEvent::set_event_handler(Some(move |event| {
        if matches!(
            event,
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } | TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            }
        ) {
            let _ = tx.send(TrayCommand::Toggle);
        }
    }));

    match AuraTray::install(
        TrayConfig::new("aura-docs")
            .tooltip("Aura Docs")
            .icon(docs_tray_icon("default"))
            .menu(default_aura_tray_menu()),
    ) {
        Ok(tray) => {
            cx.set_global(DocsTrayState {
                tray,
                window: None,
                window_visible: true,
                resident_enabled: true,
                auto_show: true,
            });
        }
        Err(error) => {
            eprintln!("failed to install Aura Docs tray icon: {error}");
            cx.set_quit_mode(gpui::QuitMode::LastWindowClosed);
            return;
        }
    }

    cx.spawn(async move |cx: &mut gpui::AsyncApp| {
        loop {
            aura_tray::pump_platform_events();
            while let Ok(command) = rx.try_recv() {
                cx.update(|cx| handle_docs_tray_command(command, cx));
            }
            cx.background_executor()
                .timer(Duration::from_millis(100))
                .await;
        }
    })
    .detach();
}

fn handle_docs_tray_command(command: TrayCommand, cx: &mut App) {
    match command {
        TrayCommand::Show => show_docs_window(cx),
        TrayCommand::Hide => hide_docs_window(cx),
        TrayCommand::Toggle => toggle_docs_window(cx),
        TrayCommand::Quit => cx.quit(),
        TrayCommand::SetIcon(name) => {
            if cx.has_global::<DocsTrayState>() {
                let state = cx.global_mut::<DocsTrayState>();
                if let Err(error) = state.tray.set_icon(docs_tray_icon(&name)) {
                    eprintln!("failed to update Aura Docs tray icon: {error}");
                }
                let _ = state.tray.set_tooltip(Some(match name.as_str() {
                    "syncing" => "Aura Docs · Syncing",
                    "error" => "Aura Docs · Error",
                    _ => "Aura Docs",
                }));
            }
        }
        TrayCommand::Custom(name) if name == "auto-show" => {
            if cx.has_global::<DocsTrayState>() {
                let state = cx.global_mut::<DocsTrayState>();
                state.auto_show = !state.auto_show;
                let _ = state
                    .tray
                    .set_check_state(&TrayCommand::Custom("auto-show".into()), state.auto_show);
            }
        }
        TrayCommand::Custom(name) if name == "resident-enabled" => {
            if cx.has_global::<DocsTrayState>() {
                let resident_enabled = {
                    let state = cx.global_mut::<DocsTrayState>();
                    state.resident_enabled = !state.resident_enabled;
                    let _ = state.tray.set_check_state(
                        &TrayCommand::Custom("resident-enabled".into()),
                        state.resident_enabled,
                    );
                    let _ = state.tray.set_visible(state.resident_enabled);
                    state.resident_enabled
                };
                cx.set_quit_mode(if resident_enabled {
                    gpui::QuitMode::Explicit
                } else {
                    gpui::QuitMode::LastWindowClosed
                });
            }
        }
        TrayCommand::Custom(name) => {
            eprintln!("Aura Docs tray custom command: {name}");
        }
    }
}

fn show_docs_window(cx: &mut App) {
    if !cx.has_global::<DocsTrayState>() {
        return;
    }

    let existing = cx.global::<DocsTrayState>().window;
    if let Some(handle) = existing {
        if handle
            .update(cx, |_, window: &mut Window, _| window.activate_window())
            .is_ok()
        {
            cx.global_mut::<DocsTrayState>().window_visible = true;
            return;
        }
    }

    if let Some(handle) = open_docs_window(cx) {
        let state = cx.global_mut::<DocsTrayState>();
        state.window = Some(handle);
        state.window_visible = true;
    }
}

fn hide_docs_window(cx: &mut App) {
    if !cx.has_global::<DocsTrayState>() {
        return;
    }

    let existing = cx.global::<DocsTrayState>().window;
    if let Some(handle) = existing {
        let _ = handle.update(cx, |_, window: &mut Window, _| window.minimize_window());
    }
    cx.global_mut::<DocsTrayState>().window_visible = false;
}

fn toggle_docs_window(cx: &mut App) {
    let should_hide = cx
        .has_global::<DocsTrayState>()
        .then(|| cx.global::<DocsTrayState>().window_visible)
        .unwrap_or(false);

    if should_hide {
        hide_docs_window(cx);
    } else {
        show_docs_window(cx);
    }
}

fn docs_tray_icon(name: &str) -> aura_tray::TrayIconImage {
    let color = match name {
        "syncing" => [103, 194, 58, 255],
        "error" => [245, 108, 108, 255],
        _ => [103, 58, 183, 255],
    };
    solid_icon(color, 32).expect("solid 32px RGBA icon should be valid")
}

fn main() {
    run_docs();
}
