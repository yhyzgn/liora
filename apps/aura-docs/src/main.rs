mod markdown;

use aura_components::{
    Autocomplete, Button, Cascader, Checkbox, CodeBlock, ColorPicker, DatePicker, DateTimePicker,
    Dialog, Drawer, Input, MessageManager, Paragraph, Popover, Preview, Radio, RadioGroup, Select,
    Space, Switch, Text, TimePicker, Title,
};
use aura_core::init_aura;
use aura_theme::Theme;
use aura_tray::{
    AuraTray, BundledTrayIconSet, BundledTrayIconState, MouseButton, MouseButtonState,
    TrayCloseAction, TrayCommand, TrayConfig, TrayControlCenter, TrayIconEvent, bundled_tray_icon,
    default_aura_tray_menu,
};
use gpui::{App, AppContext, Bounds, Global, Window, WindowBounds, WindowOptions, px, size};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

struct DocsTrayState {
    tray: AuraTray,
    window: Option<gpui::AnyWindowHandle>,
    window_visible: bool,
    resident_enabled: bool,
    tray_visible: bool,
    auto_show: bool,
    close_dialog_open: bool,
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
    match cx.open_window(docs_window_options(), |window, cx| {
        let view = markdown::render_docs_shell(cx);
        window.on_window_should_close(cx, |window, cx| handle_docs_window_should_close(window, cx));
        view
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

    let tray_tx = tx.clone();
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
            let _ = tray_tx.send(TrayCommand::Toggle);
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
                tray_visible: true,
                auto_show: true,
                close_dialog_open: false,
            });
            cx.set_global(TrayControlCenter::new(tx.clone()));
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
            if cx.has_global::<TrayControlCenter>() {
                cx.global_mut::<TrayControlCenter>().set_active_icon(name);
            }
        }
        TrayCommand::Custom(name) if name == "auto-show" => {
            if cx.has_global::<DocsTrayState>() {
                let auto_show = {
                    let state = cx.global_mut::<DocsTrayState>();
                    state.auto_show = !state.auto_show;
                    let _ = state
                        .tray
                        .set_check_state(&TrayCommand::Custom("auto-show".into()), state.auto_show);
                    state.auto_show
                };
                if cx.has_global::<TrayControlCenter>() {
                    cx.global_mut::<TrayControlCenter>()
                        .set_auto_show(auto_show);
                }
            }
        }
        TrayCommand::Custom(name) if name == "resident-enabled" => {
            if cx.has_global::<DocsTrayState>() {
                let resident_enabled = {
                    let state = cx.global_mut::<DocsTrayState>();
                    state.resident_enabled = !state.resident_enabled;
                    state.tray_visible = state.resident_enabled;
                    let _ = state.tray.set_check_state(
                        &TrayCommand::Custom("resident-enabled".into()),
                        state.resident_enabled,
                    );
                    let _ = state.tray.set_visible(state.tray_visible);
                    state.resident_enabled
                };
                cx.set_quit_mode(if resident_enabled {
                    gpui::QuitMode::Explicit
                } else {
                    gpui::QuitMode::LastWindowClosed
                });
                if cx.has_global::<TrayControlCenter>() {
                    cx.global_mut::<TrayControlCenter>()
                        .set_resident_enabled(resident_enabled);
                }
            }
        }
        TrayCommand::Custom(name) if name == "tray-visible" => {
            let visible = cx
                .has_global::<DocsTrayState>()
                .then(|| !cx.global::<DocsTrayState>().tray_visible)
                .unwrap_or(true);
            set_docs_tray_visible(cx, visible);
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

fn prepare_docs_hide_to_tray(cx: &mut App) {
    if !cx.has_global::<DocsTrayState>() {
        return;
    }

    cx.set_quit_mode(gpui::QuitMode::Explicit);
    set_docs_tray_visible(cx, true);

    let state = cx.global_mut::<DocsTrayState>();
    state.window_visible = false;
    state.window = None;
}

fn hide_docs_window(cx: &mut App) {
    if !cx.has_global::<DocsTrayState>() {
        return;
    }

    let existing = cx.global::<DocsTrayState>().window;
    prepare_docs_hide_to_tray(cx);
    if let Some(handle) = existing {
        let _ = handle.update(cx, |_, window: &mut Window, _| window.remove_window());
    }
}

fn set_docs_tray_visible(cx: &mut App, visible: bool) {
    if cx.has_global::<DocsTrayState>() {
        let state = cx.global_mut::<DocsTrayState>();
        state.tray_visible = visible;
        if visible {
            state.resident_enabled = true;
        }
        let _ = state.tray.set_visible(visible);
    }
    if cx.has_global::<TrayControlCenter>() {
        cx.global_mut::<TrayControlCenter>()
            .set_tray_visible(visible);
    }
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

fn handle_docs_window_should_close(_window: &mut Window, cx: &mut App) -> bool {
    if !cx.has_global::<DocsTrayState>() || !cx.has_global::<TrayControlCenter>() {
        return true;
    }

    match cx
        .global::<TrayControlCenter>()
        .state
        .remembered_close_action
    {
        TrayCloseAction::ExitProcess => {
            cx.quit();
            false
        }
        TrayCloseAction::HideToTray => {
            prepare_docs_hide_to_tray(cx);
            true
        }
        TrayCloseAction::Ask => {
            if !cx.global::<DocsTrayState>().close_dialog_open {
                cx.global_mut::<DocsTrayState>().close_dialog_open = true;
                show_docs_close_confirm(cx);
            }
            false
        }
    }
}

fn show_docs_close_confirm(cx: &mut App) {
    let remember = Arc::new(AtomicBool::new(false));

    Dialog::new()
        .id("docs-close-confirm")
        .title("关闭 Aura Docs？")
        .close_on_click_outside(false)
        .close_on_escape(false)
        .content(move |_window, cx| {
            let remember_for_checkbox = remember.clone();
            let remember_for_exit = remember.clone();
            let remember_for_hide = remember.clone();
            let checkbox = cx.new(move |cx| {
                Checkbox::new(false, cx).label("记住本次选择").on_change({
                    let remember = remember_for_checkbox.clone();
                    move |checked, _, _| remember.store(checked, Ordering::Relaxed)
                })
            });

            Space::new()
                .vertical()
                .gap_lg()
                .child(Paragraph::with_text(
                    "你可以直接退出进程，或者关闭主窗口并让文档应用继续驻留在系统托盘。",
                ))
                .child(checkbox)
                .child(
                    Space::new()
                        .gap_md()
                        .child(Button::new("隐藏到托盘").on_click(move |_, _, cx| {
                            if remember_for_hide.load(Ordering::Relaxed)
                                && cx.has_global::<TrayControlCenter>()
                            {
                                cx.global_mut::<TrayControlCenter>()
                                    .set_remembered_close_action(TrayCloseAction::HideToTray);
                            }
                            if cx.has_global::<DocsTrayState>() {
                                cx.global_mut::<DocsTrayState>().close_dialog_open = false;
                            }
                            Dialog::close(cx);
                            hide_docs_window(cx);
                        }))
                        .child(Button::new("关闭进程").danger().on_click(move |_, _, cx| {
                            if remember_for_exit.load(Ordering::Relaxed)
                                && cx.has_global::<TrayControlCenter>()
                            {
                                cx.global_mut::<TrayControlCenter>()
                                    .set_remembered_close_action(TrayCloseAction::ExitProcess);
                            }
                            if cx.has_global::<DocsTrayState>() {
                                cx.global_mut::<DocsTrayState>().close_dialog_open = false;
                            }
                            Dialog::close(cx);
                            cx.quit();
                        })),
                )
        })
        .show(cx);
}

fn docs_tray_icon(name: &str) -> aura_tray::TrayIconImage {
    bundled_tray_icon(
        BundledTrayIconSet::Docs,
        BundledTrayIconState::from_name(name),
    )
    .expect("bundled docs tray icon should be valid")
}

fn main() {
    run_docs();
}
