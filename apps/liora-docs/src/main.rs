#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod markdown;

use gpui::{App, AppContext, FontWeight, Global, Window, WindowOptions, px, size};
use liora_components::{
    Button, Checkbox, Dialog, Menu, MenuItem, Paragraph, Space, WindowFrameMode,
    apply_window_frame_mode, init_liora_with_options, request_window_frame_mode,
};
use liora_core::{
    FontConfig, FontLoadMode, FontLoadOptions, LinuxDesktopIdentity, LinuxDesktopPngIcon,
    LioraOptions, attach_system_theme_observer, ensure_linux_desktop_identity, linux_desktop_entry,
    linux_desktop_png_icon_path, load_app_fonts, startup_maximized_window_bounds,
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
    gpui_platform::application()
        .with_assets(liora_icons::IconAssetSource)
        .run(|cx: &mut App| {
            install_docs_fonts(cx);
            init_liora_with_options(cx, docs_liora_options());
            register_docs_desktop_identity();
            register_docs_system_menus(cx);

            install_docs_tray(cx);
            if let Some(handle) = open_docs_window(cx) {
                if cx.has_global::<DocsTrayState>() {
                    cx.global_mut::<DocsTrayState>().window = Some(handle);
                }
            }
        });
}

/// Registers the docs example menu through GPUI's official application menu API.
fn register_docs_system_menus(cx: &mut App) {
    Menu::register(
        cx,
        [
            Menu::new("File")
                .item(MenuItem::open_file())
                .item(MenuItem::open_folder())
                .item(MenuItem::separator())
                .item(MenuItem::save())
                .item(MenuItem::quit()),
            Menu::new("Edit")
                .item(MenuItem::undo())
                .item(MenuItem::redo())
                .item(MenuItem::separator())
                .item(MenuItem::cut())
                .item(MenuItem::copy())
                .item(MenuItem::paste())
                .item(MenuItem::separator())
                .item(MenuItem::select_all()),
            Menu::new("View")
                .item(MenuItem::command_palette())
                .item(MenuItem::toggle_sidebar())
                .item(MenuItem::toggle_statusbar())
                .item(MenuItem::separator())
                .item(MenuItem::action(
                    liora_components::MenuAction::ZoomIn,
                    "Zoom In",
                ))
                .item(MenuItem::action(
                    liora_components::MenuAction::ZoomOut,
                    "Zoom Out",
                ))
                .item(MenuItem::action(
                    liora_components::MenuAction::ZoomReset,
                    "Reset Zoom",
                )),
            Menu::new("Help")
                .item(MenuItem::open_url(
                    "Liora on GitHub",
                    "https://github.com/yhyzgn/liora",
                ))
                .item(MenuItem::new("about-docs", "About Liora Docs")),
        ],
    );
}

// This app uses init_liora_with_options instead of init_liora(cx) because it sets app fonts.
fn docs_liora_options() -> LioraOptions {
    LioraOptions::system().with_fonts(
        FontConfig::system()
            .with_ui_families(["MiSans", "Segoe UI", "Arial"])
            .with_ui_weight(FontWeight::MEDIUM)
            .with_code_families(["Consolas", "JetBrains Mono", "SF Mono", "Monospace"]),
    )
}

fn install_docs_fonts(cx: &mut App) {
    let options = FontLoadOptions::new(app_font_load_mode()).require_family("MiSans");

    let mut options = add_embedded_app_fonts(options);

    for dir in app_font_dirs("liora-docs") {
        options = options.external_dir(dir);
    }

    let report = load_app_fonts(cx, options);
    if !report.failures.is_empty() || !report.required_families_available() {
        eprintln!("Liora Docs font loading report: {report:?}");
    }
}

fn app_font_load_mode() -> FontLoadMode {
    if cfg!(feature = "embedded-fonts") {
        FontLoadMode::ExternalThenEmbedded
    } else {
        FontLoadMode::External
    }
}

#[cfg(feature = "embedded-fonts")]
fn add_embedded_app_fonts(options: FontLoadOptions) -> FontLoadOptions {
    options.embedded(
        "MiSans-Medium.ttf",
        std::borrow::Cow::Borrowed(
            include_bytes!("../assets/fonts/MiSans/MiSans-Medium.ttf").as_slice(),
        ),
    )
}

#[cfg(not(feature = "embedded-fonts"))]
fn add_embedded_app_fonts(options: FontLoadOptions) -> FontLoadOptions {
    options
}

fn app_font_dirs(binary: &str) -> Vec<std::path::PathBuf> {
    let mut dirs = Vec::new();
    dirs.push(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/fonts"));

    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            dirs.push(exe_dir.join("assets/fonts"));
            dirs.push(exe_dir.join("..").join("assets/fonts"));
            dirs.push(exe_dir.join("..").join("Resources").join("assets/fonts"));
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    dirs.push(
        std::path::PathBuf::from("/usr/lib")
            .join(binary)
            .join("assets/fonts"),
    );

    dirs
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
    let desktop_icon = linux_desktop_png_icon_path("liora-docs", 512)
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|| "liora-docs".to_string());

    let desktop_entry = std::env::current_exe()
        .map(|executable| {
            linux_desktop_entry(
                "Liora Docs",
                "Documentation",
                "Native GPUI documentation app for Liora.",
                &executable,
                &desktop_icon,
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
        png_icons: &[
            LinuxDesktopPngIcon {
                size: 16,
                bytes: include_bytes!("../assets/app-icons/liora-docs-16.png"),
            },
            LinuxDesktopPngIcon {
                size: 24,
                bytes: include_bytes!("../assets/app-icons/liora-docs-24.png"),
            },
            LinuxDesktopPngIcon {
                size: 32,
                bytes: include_bytes!("../assets/app-icons/liora-docs-32.png"),
            },
            LinuxDesktopPngIcon {
                size: 48,
                bytes: include_bytes!("../assets/app-icons/liora-docs-48.png"),
            },
            LinuxDesktopPngIcon {
                size: 64,
                bytes: include_bytes!("../assets/app-icons/liora-docs-64.png"),
            },
            LinuxDesktopPngIcon {
                size: 128,
                bytes: include_bytes!("../assets/app-icons/liora-docs-128.png"),
            },
            LinuxDesktopPngIcon {
                size: 256,
                bytes: include_bytes!("../assets/app-icons/liora-docs-256.png"),
            },
            LinuxDesktopPngIcon {
                size: 512,
                bytes: include_bytes!("../assets/app-icons/liora-docs.png"),
            },
        ],
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
        state.window_visible = true;
    }

    match request_window_frame_mode(window, mode) {
        liora_components::WindowFrameChange::AppliedLive => {
            liora_components::toast_info!(
                "Docs window frame switched to {}",
                if mode.is_custom() { "custom" } else { "system" }
            );
        }
        liora_components::WindowFrameChange::RequiresWindowReopen => {
            liora_components::toast_info!(
                "Docs window frame will reopen to apply {}",
                if mode.is_custom() {
                    "custom frame"
                } else {
                    "system frame"
                }
            );
            window.defer(cx, |window, cx| {
                window.remove_window();
                if let Some(handle) = open_docs_window(cx)
                    && cx.has_global::<DocsTrayState>()
                {
                    cx.global_mut::<DocsTrayState>().window = Some(handle);
                }
            });
        }
    }
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
                show_docs_close_confirm(window, cx);
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

fn handle_docs_window_should_close(window: &mut Window, cx: &mut App) -> bool {
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
                show_docs_close_confirm(window, cx);
            }
            false
        }
    }
}

fn show_docs_close_confirm(window: &mut Window, cx: &mut App) {
    let remember = Arc::new(AtomicBool::new(false));

    Dialog::new()
        .id("docs-close-confirm")
        .title("关闭 Liora Docs？")
        .immediate()
        .close_on_click_outside(false)
        .close_on_escape(true)
        .on_close(|_, cx| reset_docs_close_confirm(cx))
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
                .grow()
                .child(Paragraph::with_text(
                    "你可以直接退出进程，或者关闭主窗口并让文档应用继续驻留在系统托盘。",
                ))
                .child(checkbox)
                .child(
                    Space::new()
                        .gap_md()
                        .wrap()
                        .child(Button::new("隐藏到托盘").on_click(move |_, window, cx| {
                            if remember_for_hide.load(Ordering::Relaxed)
                                && cx.has_global::<TrayControlCenter>()
                            {
                                cx.global_mut::<TrayControlCenter>()
                                    .set_remembered_close_action(TrayCloseAction::HideToTray);
                            }
                            reset_docs_close_confirm(cx);
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
                            reset_docs_close_confirm(cx);
                            Dialog::close(cx);
                            cx.quit();
                        })),
                )
        })
        .show_in_window(window, cx);
}

fn reset_docs_close_confirm(cx: &mut App) {
    if cx.has_global::<DocsTrayState>() {
        cx.global_mut::<DocsTrayState>().close_dialog_open = false;
    }
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
        assert!(source.contains("gpui_platform::application()"));
        assert!(source.contains("with_assets(liora_icons::IconAssetSource)"));
        assert!(source.contains(r#"app_id: Some("liora-docs".into())"#));
        assert!(source.contains(r#"app_id: "liora-docs""#));
        assert!(source.contains("packaging/linux/liora-docs.desktop"));
        assert!(source.contains("LinuxDesktopPngIcon"));
        assert!(source.contains("../assets/app-icons/liora-docs-16.png"));
        assert!(source.contains("../assets/app-icons/liora-docs-48.png"));
        assert!(source.contains("../assets/app-icons/liora-docs-128.png"));
        assert!(source.contains("../assets/app-icons/liora-docs.png"));
        assert!(source.contains("../assets/app-icons/liora-docs.svg"));
    }

    #[test]
    fn docs_registers_gpui_system_menus_on_startup() {
        let source = include_str!("main.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("docs app should have production source before tests");

        assert!(source.contains("register_docs_system_menus(cx);"));
        assert!(source.contains("fn register_docs_system_menus(cx: &mut App)"));
        assert!(source.contains("Menu::register("));
        assert!(source.contains("MenuItem::open_file()"));
        assert!(source.contains("MenuItem::open_folder()"));
        assert!(source.contains("MenuItem::select_all()"));
    }

    #[test]
    fn docs_frame_mode_switch_reopens_when_gpui_requires_creation_time_titlebar_options() {
        let source = include_str!("main.rs");
        let handler = source
            .split("fn set_docs_frame_mode")
            .nth(1)
            .expect("Docs frame mode handler should exist")
            .split("fn request_docs_window_close")
            .next()
            .expect("Docs frame mode handler should end before close handler");

        assert!(handler.contains("request_window_frame_mode"));
        assert!(handler.contains("WindowFrameChange::RequiresWindowReopen"));
        assert!(handler.contains("window.defer(cx"));
        assert!(handler.contains("window.remove_window()"));
        assert!(handler.contains("open_docs_window(cx)"));
    }

    #[test]
    fn close_confirm_dismissal_resets_flag_and_escape_is_enabled() {
        let source = include_str!("main.rs");

        let confirm = source
            .split("fn show_docs_close_confirm")
            .nth(1)
            .expect("Docs close confirmation should exist")
            .split("fn docs_tray_icon")
            .next()
            .expect("Docs close confirmation should appear before tray icon helper");

        assert!(confirm.contains(".close_on_escape(true)"));
        assert!(!confirm.contains(".close_on_escape(false)"));
        assert!(confirm.contains(".immediate()"));
        assert!(confirm.contains(".show_in_window(window, cx)"));
        assert!(confirm.contains(".on_close(|_, cx| reset_docs_close_confirm(cx))"));
        assert_eq!(confirm.matches("reset_docs_close_confirm(cx);").count(), 2);
        assert!(source.contains("fn reset_docs_close_confirm(cx: &mut App)"));
    }
}

#[cfg(test)]
mod font_loading_tests {
    #[test]
    fn docs_font_loading_requires_misans_family() {
        let source = include_str!("main.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("docs app should have production source before tests");

        assert!(source.contains("MiSans-Medium.ttf"));
        assert!(source.contains("require_family(\"MiSans\")"));
        assert!(!source.contains("MiSans-Medium.woff2"));
    }
}
