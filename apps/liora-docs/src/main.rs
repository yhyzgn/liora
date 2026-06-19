mod markdown;

use gpui::{App, AppContext, Application, Global, Window, WindowOptions, px, size};
use liora_components::{
    Button, Checkbox, Dialog, Paragraph, Space, WindowFrameMode, apply_window_frame_mode,
    init_liora,
};
use liora_core::{
    LinuxDesktopIdentity, attach_system_theme_observer, ensure_linux_desktop_identity,
    linux_desktop_entry, startup_maximized_window_bounds,
};
use liora_tray::{
    LioraTray, MouseButton, MouseButtonState, TrayCloseAction, TrayCommand, TrayConfig,
    TrayControlCenter, TrayIconEvent, default_liora_tray_menu, icon_from_png_bytes, solid_icon,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

struct DocsTrayState {
    tray: LioraTray,
    window: Option<gpui::AnyWindowHandle>,
    window_visible: bool,
    resident_enabled: bool,
    tray_visible: bool,
    auto_show: bool,
    close_dialog_open: bool,
    frame_mode: WindowFrameMode,
}

impl Global for DocsTrayState {}

fn run_docs() {
    Application::new().run(|cx: &mut App| {
        init_liora(cx);
        register_docs_desktop_identity();

        install_docs_tray(cx);
        if let Some(handle) = open_docs_window(cx) {
            if cx.has_global::<DocsTrayState>() {
                cx.global_mut::<DocsTrayState>().window = Some(handle);
            }
        }
    });
}

fn open_docs_window(cx: &mut App) -> Option<gpui::AnyWindowHandle> {
    let frame_mode = docs_frame_mode(cx);
    match cx.open_window(docs_window_options(cx, frame_mode), |window, cx| {
        attach_system_theme_observer(window, cx);

        let view = markdown::render_docs_shell(
            frame_mode,
            set_docs_frame_mode,
            request_docs_window_close,
            cx,
        );
        window.on_window_should_close(cx, |window, cx| handle_docs_window_should_close(window, cx));
        view
    }) {
        Ok(handle) => {
            let any_handle: gpui::AnyWindowHandle = handle.into();
            let _ = any_handle.update(cx, |_, window, _| window.activate_window());
            Some(any_handle)
        }
        Err(error) => {
            eprintln!("failed to open Liora Docs window: {error:?}");
            None
        }
    }
}

fn docs_window_options(cx: &App, frame_mode: WindowFrameMode) -> WindowOptions {
    apply_window_frame_mode(
        WindowOptions {
            show: false,
            window_bounds: Some(startup_maximized_window_bounds(
                cx,
                size(px(1680.0), px(1080.0)),
            )),
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Liora Docs — Native Main Window".into()),
                ..Default::default()
            }),
            app_id: Some("liora-docs".into()),
            ..Default::default()
        },
        frame_mode,
    )
}

fn register_docs_desktop_identity() {
    let desktop_entry = std::env::current_exe()
        .map(|executable| {
            linux_desktop_entry(
                "Liora Docs",
                "Documentation",
                "Native GPUI documentation app for Liora.",
                &executable,
                "liora-docs",
                "Development;Documentation;",
                "gpui;liora;docs;documentation;",
            )
        })
        .map(std::borrow::Cow::Owned)
        .unwrap_or_else(|_| {
            std::borrow::Cow::Borrowed(include_str!("../../../packaging/linux/liora-docs.desktop"))
        });

    if let Err(error) = ensure_linux_desktop_identity(LinuxDesktopIdentity {
        app_id: "liora-docs",
        desktop_entry,
        png_icon: include_bytes!("../assets/app-icons/liora-docs.png"),
        svg_icon: include_bytes!("../assets/app-icons/liora-docs.svg"),
    }) {
        eprintln!("failed to register Liora Docs desktop identity: {error}");
    }
}

fn install_docs_tray(cx: &mut App) {
    let (tx, rx) = mpsc::channel::<TrayCommand>();
    let menu_tx = tx.clone();
    liora_tray::MenuEvent::set_event_handler(Some(move |event: liora_tray::MenuEvent| {
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

    let mut config = TrayConfig::new("liora-docs")
        .tooltip("Liora Docs")
        .menu(default_liora_tray_menu());
    if let Some(icon) = docs_tray_icon("default") {
        config = config.icon(icon);
    }

    match LioraTray::install(config) {
        Ok(tray) => {
            cx.set_global(DocsTrayState {
                tray,
                window: None,
                window_visible: true,
                resident_enabled: true,
                tray_visible: true,
                auto_show: true,
                close_dialog_open: false,
                frame_mode: WindowFrameMode::System,
            });
            cx.set_global(TrayControlCenter::new(tx.clone()));
        }
        Err(error) => {
            eprintln!("failed to install Liora Docs tray icon: {error}");
            return;
        }
    }

    cx.spawn(async move |cx: &mut gpui::AsyncApp| {
        loop {
            liora_tray::pump_platform_events();
            while let Ok(command) = rx.try_recv() {
                let _ = cx.update(|cx| handle_docs_tray_command(command, cx));
            }
            cx.background_executor()
                .timer(Duration::from_millis(100))
                .await;
        }
    })
    .detach();
}

fn docs_frame_mode(cx: &App) -> WindowFrameMode {
    cx.has_global::<DocsTrayState>()
        .then(|| cx.global::<DocsTrayState>().frame_mode)
        .unwrap_or_default()
}

fn set_docs_frame_mode(mode: WindowFrameMode, window: &mut Window, cx: &mut App) {
    if cx.has_global::<DocsTrayState>() {
        let state = cx.global_mut::<DocsTrayState>();
        if state.frame_mode == mode {
            return;
        }
        state.frame_mode = mode;
        state.window = None;
        state.window_visible = true;
    }

    liora_components::toast_info!(
        "Docs window frame switched to {}",
        if mode.is_custom() { "custom" } else { "system" }
    );
    window.remove_window();
    cx.defer(move |cx| {
        if let Some(handle) = open_docs_window(cx)
            && cx.has_global::<DocsTrayState>()
        {
            let state = cx.global_mut::<DocsTrayState>();
            state.window = Some(handle);
            state.window_visible = true;
        }
    });
}

fn request_docs_window_close(window: &mut Window, cx: &mut App) {
    if !cx.has_global::<DocsTrayState>() || !cx.has_global::<TrayControlCenter>() {
        window.remove_window();
        return;
    }

    match cx
        .global::<TrayControlCenter>()
        .state
        .remembered_close_action
    {
        TrayCloseAction::ExitProcess => cx.quit(),
        TrayCloseAction::HideToTray => {
            prepare_docs_hide_to_tray(cx);
            window.remove_window();
        }
        TrayCloseAction::Ask => {
            if !cx.global::<DocsTrayState>().close_dialog_open {
                cx.global_mut::<DocsTrayState>().close_dialog_open = true;
                show_docs_close_confirm(cx);
            }
        }
    }
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
                if let Some(icon) = docs_tray_icon(&name) {
                    if let Err(error) = state.tray.set_icon(icon) {
                        eprintln!("failed to update Liora Docs tray icon: {error}");
                    }
                }
                let _ = state.tray.set_tooltip(Some(match name.as_str() {
                    "syncing" => "Liora Docs · Syncing",
                    "error" => "Liora Docs · Error",
                    _ => "Liora Docs",
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
            eprintln!("Liora Docs tray custom command: {name}");
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
        .title("关闭 Liora Docs？")
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
                        .child(Button::new("隐藏到托盘").on_click(move |_, window, cx| {
                            if remember_for_hide.load(Ordering::Relaxed)
                                && cx.has_global::<TrayControlCenter>()
                            {
                                cx.global_mut::<TrayControlCenter>()
                                    .set_remembered_close_action(TrayCloseAction::HideToTray);
                            }
                            if cx.has_global::<DocsTrayState>() {
                                cx.global_mut::<DocsTrayState>().close_dialog_open = false;
                            }
                            prepare_docs_hide_to_tray(cx);
                            Dialog::close(cx);
                            window.remove_window();
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

fn docs_tray_icon(name: &str) -> Option<liora_tray::TrayIconImage> {
    let bytes = match name {
        "syncing" => include_bytes!("../assets/tray-icons/syncing.png").as_slice(),
        "error" => include_bytes!("../assets/tray-icons/error.png").as_slice(),
        _ => include_bytes!("../assets/tray-icons/default.png").as_slice(),
    };
    match icon_from_png_bytes(bytes) {
        Ok(icon) => Some(icon),
        Err(error) => {
            eprintln!(
                "failed to load Liora Docs tray icon asset '{name}': {error}; using fallback icon"
            );
            match solid_icon([139, 92, 246, 255], 32) {
                Ok(icon) => Some(icon),
                Err(error) => {
                    eprintln!("failed to create fallback Liora Docs tray icon: {error}");
                    None
                }
            }
        }
    }
}

fn main() {
    run_docs();
}

#[cfg(test)]
mod shell_tests {
    #[test]
    fn docs_shell_registers_wayland_desktop_identity() {
        let source = include_str!("main.rs");

        assert!(source.contains("register_docs_desktop_identity();"));
        assert!(source.contains(r#"app_id: Some("liora-docs".into())"#));
        assert!(source.contains(r#"app_id: "liora-docs""#));
        assert!(source.contains("packaging/linux/liora-docs.desktop"));
        assert!(source.contains("../assets/app-icons/liora-docs.png"));
        assert!(source.contains("../assets/app-icons/liora-docs.svg"));
    }
}
