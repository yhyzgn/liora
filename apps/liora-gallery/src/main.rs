use gpui::{
    AnyView, App, Application, Component, Context, Global, Render, RenderImage, ScrollHandle,
    SharedString, Window, WindowOptions, div, img, prelude::*, px, size,
};
use liora_components::{
    AppWindowFrame, Button, Card, Checkbox, Container, Dialog, Input, Menu, MenuMode, MenuNode,
    Paragraph, Segmented, SegmentedOption, Space, Spinner, Switch, Tag, Text, Title,
    WindowFrameMode, apply_window_frame_mode, frame_mode_switch_row, init_liora, toast_info,
    toast_success,
};
use liora_core::{
    Config, LinuxDesktopIdentity, LinuxDesktopPngIcon, PassivePortal, Portal, ThemeMode,
    apply_theme_mode, attach_system_theme_observer, ensure_linux_desktop_identity,
    linux_desktop_entry, linux_desktop_png_icon_path, startup_maximized_window_bounds,
};
use liora_gallery::demos;
use liora_tray::{
    LioraTray, MouseButton, MouseButtonState, TrayCloseAction, TrayCommand, TrayConfig,
    TrayControlCenter, TrayIconEvent, default_liora_tray_menu, icon_from_png_bytes, solid_icon,
};
use liora_updater::{
    AssetKind, InstallAction, InstallPlan, LioraApp, Platform, UpdateRequest, Updater,
    liora_asset_selector,
};
use std::{
    process::Command,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

pub struct Gallery {
    entries: Vec<demos::DemoEntry>,
    active_demo_index: Option<usize>,
    active_demo: Option<AnyView>,
    pending_demo_index: Option<usize>,
    nav_index: Vec<GalleryNavEntry>,
    selected: usize,
    nav_filter: gpui::Entity<Input>,
    nav_menu: Option<gpui::Entity<Menu>>,
    nav_scroll: ScrollHandle,
    nav_query: String,
    nav_refresh_pending: bool,
    theme_mode: ThemeMode,
    theme_mode_segmented: gpui::Entity<Segmented>,
    frame_mode: WindowFrameMode,
    frame_mode_switch: gpui::Entity<Switch>,
    refresh_revision: u32,
    updater_status: UpdatePanelStatus,
    install_plan: Option<InstallPlan>,
}

#[derive(Debug, Clone)]
struct GalleryNavEntry {
    label: SharedString,
    search_text: String,
}

enum UpdatePanelStatus {
    Idle,
    Checking,
    UpToDate(String),
    Available(String),
    Downloading(String),
    Downloaded(String),
    Installing(String),
    Error(String),
}

impl UpdatePanelStatus {
    fn label(&self) -> String {
        match self {
            Self::Idle => "未检查更新 / Not checked".into(),
            Self::Checking => "正在检查 GitHub Release…".into(),
            Self::UpToDate(version) => format!("已是最新版本 {version}"),
            Self::Available(version) => format!("发现新版本 {version}"),
            Self::Downloading(version) => format!("正在下载并校验 {version}…"),
            Self::Downloaded(version) => format!("已下载并通过 SHA-256 校验：{version}"),
            Self::Installing(detail) => format!("已启动安装流程：{detail}"),
            Self::Error(error) => format!("更新失败：{error}"),
        }
    }

    fn status_bar_icon(&self) -> &'static str {
        match self {
            Self::Checking | Self::Downloading(_) | Self::Installing(_) => "syncing",
            Self::Available(_) | Self::Downloaded(_) => "update",
            Self::Error(_) => "error",
            Self::Idle | Self::UpToDate(_) => "ready",
        }
    }
}

struct GalleryTrayState {
    tray: LioraTray,
    window: Option<gpui::AnyWindowHandle>,
    window_visible: bool,
    resident_enabled: bool,
    tray_visible: bool,
    auto_show: bool,
    close_dialog_open: bool,
    frame_mode: WindowFrameMode,
}

impl Global for GalleryTrayState {}

fn run_gallery() {
    Application::new()
        .with_assets(liora_icons::IconAssetSource)
        .run(|cx: &mut App| {
            init_liora(cx);
            register_gallery_desktop_identity();

            install_gallery_tray(cx);
            if let Some(handle) = open_gallery_window(cx) {
                if cx.has_global::<GalleryTrayState>() {
                    cx.global_mut::<GalleryTrayState>().window = Some(handle);
                }
            }
        });
}

fn open_gallery_window(cx: &mut App) -> Option<gpui::AnyWindowHandle> {
    let frame_mode = gallery_frame_mode(cx);
    match cx.open_window(gallery_window_options(cx, frame_mode), |window, cx| {
        attach_system_theme_observer(window, cx);

        let entries = demos::registry();
        let nav_index = gallery_nav_index(&entries);
        let view = cx.new(|cx| {
            let theme_mode = cx.global::<Config>().theme_mode;
            let gallery = Gallery {
                entries,
                active_demo_index: None,
                active_demo: None,
                pending_demo_index: None,
                nav_index,
                selected: 0,
                nav_filter: cx.new(|cx| Input::new("", cx).placeholder("搜索组件 / Search demos")),
                nav_menu: None,
                nav_scroll: ScrollHandle::new(),
                nav_query: String::new(),
                nav_refresh_pending: false,
                theme_mode,
                theme_mode_segmented: cx.new(move |_| theme_mode_segmented(theme_mode)),
                frame_mode,
                frame_mode_switch: cx.new(|cx| Switch::new(frame_mode.is_custom(), cx)),
                refresh_revision: 0,
                updater_status: UpdatePanelStatus::Idle,
                install_plan: None,
            };
            gallery.wire_shell_controls(cx);
            gallery
        });
        window.on_window_should_close(cx, |window, cx| {
            handle_gallery_window_should_close(window, cx)
        });
        view
    }) {
        Ok(handle) => {
            let any_handle: gpui::AnyWindowHandle = handle.into();
            let _ = any_handle.update(cx, |_, window, _| window.activate_window());
            Some(any_handle)
        }
        Err(error) => {
            eprintln!("failed to open Liora Gallery window: {error:?}");
            None
        }
    }
}

fn gallery_window_options(cx: &App, frame_mode: WindowFrameMode) -> WindowOptions {
    apply_window_frame_mode(
        WindowOptions {
            show: false,
            window_bounds: Some(startup_maximized_window_bounds(
                cx,
                size(px(1920.0), px(1080.0)),
            )),
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Liora UI Gallery".into()),
                ..Default::default()
            }),
            app_id: Some("liora-gallery".into()),
            ..Default::default()
        },
        frame_mode,
    )
}

fn register_gallery_desktop_identity() {
    let desktop_icon = linux_desktop_png_icon_path("liora-gallery", 512)
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|| "liora-gallery".to_string());

    let desktop_entry = std::env::current_exe()
        .map(|executable| {
            linux_desktop_entry(
                "Liora Gallery",
                "Component Gallery",
                "Native GPUI component gallery for Liora.",
                &executable,
                &desktop_icon,
                "Development;Utility;",
                "gpui;liora;components;gallery;",
            )
        })
        .map(std::borrow::Cow::Owned)
        .unwrap_or_else(|_| {
            std::borrow::Cow::Borrowed(include_str!(
                "../../../packaging/linux/liora-gallery.desktop"
            ))
        });

    if let Err(error) = ensure_linux_desktop_identity(LinuxDesktopIdentity {
        app_id: "liora-gallery",
        desktop_entry,
        png_icons: &[
            LinuxDesktopPngIcon {
                size: 16,
                bytes: include_bytes!("../assets/app-icons/liora-gallery-16.png"),
            },
            LinuxDesktopPngIcon {
                size: 24,
                bytes: include_bytes!("../assets/app-icons/liora-gallery-24.png"),
            },
            LinuxDesktopPngIcon {
                size: 32,
                bytes: include_bytes!("../assets/app-icons/liora-gallery-32.png"),
            },
            LinuxDesktopPngIcon {
                size: 48,
                bytes: include_bytes!("../assets/app-icons/liora-gallery-48.png"),
            },
            LinuxDesktopPngIcon {
                size: 64,
                bytes: include_bytes!("../assets/app-icons/liora-gallery-64.png"),
            },
            LinuxDesktopPngIcon {
                size: 128,
                bytes: include_bytes!("../assets/app-icons/liora-gallery-128.png"),
            },
            LinuxDesktopPngIcon {
                size: 256,
                bytes: include_bytes!("../assets/app-icons/liora-gallery-256.png"),
            },
            LinuxDesktopPngIcon {
                size: 512,
                bytes: include_bytes!("../assets/app-icons/liora-gallery.png"),
            },
        ],
        svg_icon: include_bytes!("../assets/app-icons/liora-gallery.svg"),
    }) {
        eprintln!("failed to register Liora Gallery desktop identity: {error}");
    }
}

fn install_gallery_tray(cx: &mut App) {
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

    let mut config = TrayConfig::new("liora-gallery")
        .tooltip("Liora Gallery")
        .menu(default_liora_tray_menu());
    if let Some(icon) = gallery_tray_icon("default") {
        config = config.icon(icon);
    }

    match LioraTray::install(config) {
        Ok(tray) => {
            cx.set_global(GalleryTrayState {
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
            eprintln!("failed to install Liora Gallery tray icon: {error}");
            return;
        }
    }

    cx.spawn(async move |cx: &mut gpui::AsyncApp| {
        loop {
            liora_tray::pump_platform_events();
            while let Ok(command) = rx.try_recv() {
                let _ = cx.update(|cx| handle_gallery_tray_command(command, cx));
            }
            cx.background_executor()
                .timer(Duration::from_millis(100))
                .await;
        }
    })
    .detach();
}

fn gallery_frame_mode(cx: &App) -> WindowFrameMode {
    cx.has_global::<GalleryTrayState>()
        .then(|| cx.global::<GalleryTrayState>().frame_mode)
        .unwrap_or_default()
}

fn set_gallery_frame_mode(mode: WindowFrameMode, window: &mut Window, cx: &mut App) {
    if cx.has_global::<GalleryTrayState>() {
        let state = cx.global_mut::<GalleryTrayState>();
        if state.frame_mode == mode {
            return;
        }
        state.frame_mode = mode;
        state.window = None;
        state.window_visible = true;
    }

    toast_info!(
        "Gallery window frame switched to {}",
        if mode.is_custom() { "custom" } else { "system" }
    );
    window.remove_window();
    cx.defer(move |cx| {
        if let Some(handle) = open_gallery_window(cx)
            && cx.has_global::<GalleryTrayState>()
        {
            let state = cx.global_mut::<GalleryTrayState>();
            state.window = Some(handle);
            state.window_visible = true;
        }
    });
}

fn request_gallery_window_close(window: &mut Window, cx: &mut App) {
    if !cx.has_global::<GalleryTrayState>() || !cx.has_global::<TrayControlCenter>() {
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
            prepare_gallery_hide_to_tray(cx);
            window.remove_window();
        }
        TrayCloseAction::Ask => {
            if !cx.global::<GalleryTrayState>().close_dialog_open {
                cx.global_mut::<GalleryTrayState>().close_dialog_open = true;
                show_gallery_close_confirm(window, cx);
            }
        }
    }
}

fn handle_gallery_tray_command(command: TrayCommand, cx: &mut App) {
    match command {
        TrayCommand::Show => show_gallery_window(cx),
        TrayCommand::Hide => hide_gallery_window(cx),
        TrayCommand::Toggle => toggle_gallery_window(cx),
        TrayCommand::Quit => cx.quit(),
        TrayCommand::SetIcon(name) => {
            if cx.has_global::<GalleryTrayState>() {
                let state = cx.global_mut::<GalleryTrayState>();
                if let Some(icon) = gallery_tray_icon(&name) {
                    if let Err(error) = state.tray.set_icon(icon) {
                        eprintln!("failed to update Liora Gallery tray icon: {error}");
                    }
                }
                let _ = state.tray.set_tooltip(Some(match name.as_str() {
                    "syncing" => "Liora Gallery · Syncing",
                    "error" => "Liora Gallery · Error",
                    _ => "Liora Gallery",
                }));
            }
            if cx.has_global::<TrayControlCenter>() {
                cx.global_mut::<TrayControlCenter>().set_active_icon(name);
            }
        }
        TrayCommand::Custom(name) if name == "auto-show" => {
            if cx.has_global::<GalleryTrayState>() {
                let auto_show = {
                    let state = cx.global_mut::<GalleryTrayState>();
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
            if cx.has_global::<GalleryTrayState>() {
                let resident_enabled = {
                    let state = cx.global_mut::<GalleryTrayState>();
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
                .has_global::<GalleryTrayState>()
                .then(|| !cx.global::<GalleryTrayState>().tray_visible)
                .unwrap_or(true);
            set_gallery_tray_visible(cx, visible);
        }
        TrayCommand::Custom(name) => {
            eprintln!("Liora Gallery tray custom command: {name}");
        }
    }
}

fn show_gallery_window(cx: &mut App) {
    if !cx.has_global::<GalleryTrayState>() {
        return;
    }

    let existing = cx.global::<GalleryTrayState>().window;
    if let Some(handle) = existing {
        if handle
            .update(cx, |_, window, _| window.activate_window())
            .is_ok()
        {
            let state = cx.global_mut::<GalleryTrayState>();
            state.window_visible = true;
            return;
        }
    }

    if let Some(handle) = open_gallery_window(cx) {
        let state = cx.global_mut::<GalleryTrayState>();
        state.window = Some(handle);
        state.window_visible = true;
    }
}

fn prepare_gallery_hide_to_tray(cx: &mut App) {
    if !cx.has_global::<GalleryTrayState>() {
        return;
    }

    set_gallery_tray_visible(cx, true);

    let state = cx.global_mut::<GalleryTrayState>();
    state.window_visible = false;
    state.window = None;
}

fn hide_gallery_window(cx: &mut App) {
    if !cx.has_global::<GalleryTrayState>() {
        return;
    }

    let existing = cx.global::<GalleryTrayState>().window;
    prepare_gallery_hide_to_tray(cx);
    if let Some(handle) = existing {
        let _ = handle.update(cx, |_, window, _| window.remove_window());
    }
}

fn set_gallery_tray_visible(cx: &mut App, visible: bool) {
    if cx.has_global::<GalleryTrayState>() {
        let state = cx.global_mut::<GalleryTrayState>();
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

fn toggle_gallery_window(cx: &mut App) {
    let should_hide = cx
        .has_global::<GalleryTrayState>()
        .then(|| cx.global::<GalleryTrayState>().window_visible)
        .unwrap_or(false);

    if should_hide {
        hide_gallery_window(cx);
    } else {
        show_gallery_window(cx);
    }
}

fn handle_gallery_window_should_close(window: &mut Window, cx: &mut App) -> bool {
    if !cx.has_global::<GalleryTrayState>() || !cx.has_global::<TrayControlCenter>() {
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
            prepare_gallery_hide_to_tray(cx);
            true
        }
        TrayCloseAction::Ask => {
            if !cx.global::<GalleryTrayState>().close_dialog_open {
                cx.global_mut::<GalleryTrayState>().close_dialog_open = true;
                show_gallery_close_confirm(window, cx);
            }
            false
        }
    }
}

fn show_gallery_close_confirm(window: &mut Window, cx: &mut App) {
    let remember = Arc::new(AtomicBool::new(false));

    Dialog::new()
        .id("gallery-close-confirm")
        .title("关闭 Liora Gallery？")
        .immediate()
        .close_on_click_outside(false)
        .close_on_escape(true)
        .on_close(|_, cx| reset_gallery_close_confirm(cx))
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
                .shrink()
                .child(Paragraph::with_text(
                    "你可以直接退出进程，或者关闭主窗口并让应用继续驻留在系统托盘。",
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
                            reset_gallery_close_confirm(cx);
                            prepare_gallery_hide_to_tray(cx);
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
                            reset_gallery_close_confirm(cx);
                            Dialog::close(cx);
                            cx.quit();
                        })),
                )
        })
        .show_in_window(window, cx);
}

fn reset_gallery_close_confirm(cx: &mut App) {
    if cx.has_global::<GalleryTrayState>() {
        cx.global_mut::<GalleryTrayState>().close_dialog_open = false;
    }
}

fn gallery_tray_icon(name: &str) -> Option<liora_tray::TrayIconImage> {
    let bytes = match name {
        "syncing" => include_bytes!("../assets/tray-icons/syncing.png").as_slice(),
        "error" => include_bytes!("../assets/tray-icons/error.png").as_slice(),
        _ => include_bytes!("../assets/tray-icons/default.png").as_slice(),
    };
    match icon_from_png_bytes(bytes) {
        Ok(icon) => Some(icon),
        Err(error) => {
            eprintln!(
                "failed to load Liora Gallery tray icon asset '{name}': {error}; using fallback icon"
            );
            match solid_icon([32, 96, 255, 255], 32) {
                Ok(icon) => Some(icon),
                Err(error) => {
                    eprintln!("failed to create fallback Liora Gallery tray icon: {error}");
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod shell_tests {
    #[test]
    fn gallery_shell_uses_container_and_menu() {
        let source = include_str!("main.rs");

        assert!(source.contains("Container::new()"));
        assert!(source.contains("Menu::new()"));
        assert!(source.contains(".no_shrink()"));
        assert!(source.contains("init_liora(cx)"));
        assert!(source.contains("with_assets(liora_icons::IconAssetSource)"));
        assert!(source.contains("nav_filter"));
        assert!(source.contains("nav_menu: Option"));
        assert!(source.contains("self.nav_menu = Some"));
        assert!(source.contains("frame_mode_switch"));
        assert!(source.contains("AppWindowFrame::new"));
        assert!(source.contains("theme_mode_segmented"));
        assert!(source.contains("ThemeMode::System"));
        let attach_call = format!("{}(window, cx);", "attach_system_theme_observer");
        let open_window = &source[source
            .find("match cx.open_window")
            .expect("Gallery should open a GPUI window")..];
        assert!(source.contains("show: false,"));
        assert!(source.contains("startup_maximized_window_bounds"));
        assert!(source.contains("app_id: Some(\"liora-gallery\".into())"));
        assert!(source.contains("register_gallery_desktop_identity();"));
        assert!(source.contains("app_id: \"liora-gallery\""));
        assert!(source.contains("packaging/linux/liora-gallery.desktop"));
        assert!(source.contains("LinuxDesktopPngIcon"));
        assert!(source.contains("../assets/app-icons/liora-gallery-16.png"));
        assert!(source.contains("../assets/app-icons/liora-gallery-48.png"));
        assert!(source.contains("../assets/app-icons/liora-gallery-128.png"));
        assert!(source.contains("../assets/app-icons/liora-gallery.png"));
        assert!(source.contains("../assets/app-icons/liora-gallery.svg"));
        assert!(source.contains("gallery_status_icon"));
        assert!(source.contains("../assets/status-icons/status.png"));
        assert!(source.contains("gallery_status_bar_icon"));
        assert!(source.contains("../assets/status-bar-icons/ready.png"));
        assert!(source.contains("../assets/tray-icons/default.png"));
        let attach_index = open_window
            .find(&attach_call)
            .expect("Gallery should attach System theme before first render");
        let entries_index = open_window
            .find("let entries = demos::registry();")
            .expect("Gallery should build demos after theme attach");
        assert!(
            attach_index < entries_index,
            "System theme must sync from the real window before demo/view creation to avoid first-frame theme flash"
        );
        let ok_branch = &source[source
            .find("Ok(handle) =>")
            .expect("Gallery should handle opened window")..];
        assert!(ok_branch.contains("window.activate_window()"));
        assert!(source.contains("Gallery theme switched"));
        assert!(source.contains("About / 关于"));
        assert!(source.contains("check_gallery_update"));
    }
}

fn gallery_status_icon() -> Arc<RenderImage> {
    static ICON: OnceLock<Arc<RenderImage>> = OnceLock::new();
    ICON.get_or_init(|| {
        let image = ::image::load_from_memory(include_bytes!("../assets/status-icons/status.png"))
            .expect("Gallery status icon asset must be a valid PNG")
            .into_rgba8();
        Arc::new(RenderImage::new([::image::Frame::new(image)]))
    })
    .clone()
}

fn gallery_status_bar_icon(name: &str) -> Arc<RenderImage> {
    static READY: OnceLock<Arc<RenderImage>> = OnceLock::new();
    static SYNCING: OnceLock<Arc<RenderImage>> = OnceLock::new();
    static UPDATE: OnceLock<Arc<RenderImage>> = OnceLock::new();
    static ERROR: OnceLock<Arc<RenderImage>> = OnceLock::new();

    let (slot, bytes, label) = match name {
        "syncing" => (
            &SYNCING,
            include_bytes!("../assets/status-bar-icons/syncing.png").as_slice(),
            "Gallery syncing status-bar icon",
        ),
        "update" => (
            &UPDATE,
            include_bytes!("../assets/status-bar-icons/update.png").as_slice(),
            "Gallery update status-bar icon",
        ),
        "error" => (
            &ERROR,
            include_bytes!("../assets/status-bar-icons/error.png").as_slice(),
            "Gallery error status-bar icon",
        ),
        _ => (
            &READY,
            include_bytes!("../assets/status-bar-icons/ready.png").as_slice(),
            "Gallery ready status-bar icon",
        ),
    };

    slot.get_or_init(|| {
        let image = ::image::load_from_memory(bytes)
            .unwrap_or_else(|error| panic!("{label} asset must be a valid PNG: {error}"))
            .into_rgba8();
        Arc::new(RenderImage::new([::image::Frame::new(image)]))
    })
    .clone()
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected.min(self.entries.len());
        self.selected = selected;

        let nav_menu = self.gallery_nav_menu(selected, cx);

        let selected_entry = self.entries.get(selected).copied();
        let selected_demo = self.selected_demo(selected, cx);
        let selected_label = selected_entry
            .map(|entry| entry.name.to_string())
            .unwrap_or_else(|| "About".into());

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .gap_4()
            .child(
                Space::new()
                    .gap_md()
                    .child(
                        div()
                            .size(px(42.0))
                            .rounded(px(13.0))
                            .overflow_hidden()
                            .child(img(gallery_status_icon()).size(px(42.0))),
                    )
                    .child(
                        Space::new()
                            .vertical()
                            .gap_xs()
                            .child(Title::new("Liora UI").h2())
                            .child(Text::new(format!(
                                "Native Component Library · {} demos · rendering one demo at a time",
                                self.entries.len()
                            ))),
                    ),
            )
            .child(
                Space::new()
                    .gap_md()
                    .wrap()
                    .child(
                        Tag::new(format!("rev {}", self.refresh_revision))
                            .success()
                            .round(true),
                    )
                    .child(Button::new("Refresh").primary().on_click({
                        let gallery = cx.entity().clone();
                        move |_, _window, cx| {
                            let _ = gallery.update(cx, |gallery, cx| {
                                gallery.refresh_revision += 1;
                                cx.notify();
                            });
                            toast_success!("Gallery refreshed");
                        }
                    }))
                    .child(
                        Space::new()
                            .gap_sm()
                            .child(Text::new("Theme"))
                            .child(self.theme_mode_segmented.clone()),
                    )
                    .child(frame_mode_switch_row(
                        self.frame_mode_switch.clone(),
                        self.frame_mode,
                    )),
            );

        let content_body = if selected == self.entries.len() {
            self.render_about_panel(cx).into_any_element()
        } else if let Some(selected_entry) = selected_entry {
            let header = Space::new()
                .vertical()
                .gap_xs()
                .child(Title::new(selected_entry.name).h3())
                .child(Paragraph::with_text(selected_entry.description));

            let mut panel = Space::new().vertical().gap_lg().child(header);
            if let Some(selected_demo) = selected_demo {
                panel = panel.child(selected_demo);
            } else {
                panel = panel.child(self.render_demo_loading_panel(selected_entry.name, cx));
            }

            panel.into_any_element()
        } else {
            Paragraph::with_text("No gallery entry selected.").into_any_element()
        };

        let content = Card::new(content_body).no_shadow().no_shrink();

        liora_components::message::render_messages(cx);
        liora_components::notification::render_notifications(cx);
        liora_components::image::render_image_preview(_window, cx);
        liora_core::render_active_tooltip_in_window(_window, cx);
        liora_core::render_active_popover_in_window(_window, cx);
        liora_core::render_active_modal_in_window(_window, cx);
        liora_core::render_active_drawer_in_window(_window, cx);

        let shell = Container::new()
            .header(header)
            .header_height_lg()
            .aside(
                div()
                    .flex()
                    .flex_col()
                    .h_full()
                    .gap_2()
                    .p_2()
                    .child(div().flex_none().child(self.nav_filter.clone()))
                    .child(
                        div()
                            .id("gallery-nav-scroll")
                            .flex_1()
                            .min_h_0()
                            .w_full()
                            .overflow_y_scroll()
                            .track_scroll(&self.nav_scroll)
                            .child(div().flex_none().w_full().child(nav_menu)),
                    ),
            )
            .aside_width_lg()
            .main_scroll()
            .main_padding_xl()
            .child(content)
            .footer(self.render_status_bar(selected_label, cx))
            .footer_height(px(38.0))
            .overlay(PortalLayer);

        AppWindowFrame::new("Liora UI Gallery", shell)
            .subtitle("Native component gallery")
            .mode(self.frame_mode)
            .on_close(request_gallery_window_close)
    }
}

impl Gallery {
    fn render_status_bar(
        &self,
        selected_label: String,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .gap_4()
            .child(
                Space::new()
                    .gap_sm()
                    .child(
                        div()
                            .size(px(22.0))
                            .rounded(px(7.0))
                            .overflow_hidden()
                            .child(
                                img(gallery_status_bar_icon(
                                    self.updater_status.status_bar_icon(),
                                ))
                                .size(px(22.0)),
                            ),
                    )
                    .child(Text::new(self.updater_status.label()).sm().nowrap()),
            )
            .child(
                Space::new()
                    .gap_sm()
                    .child(
                        Text::new(selected_label)
                            .sm()
                            .text_color(theme.neutral.text_3)
                            .nowrap(),
                    )
                    .child(
                        Tag::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .small()
                            .round(true),
                    ),
            )
    }

    fn gallery_nav_menu(&mut self, selected: usize, cx: &mut Context<Self>) -> gpui::Entity<Menu> {
        let active_id = gallery_nav_item_id(selected);
        let items = gallery_nav_menu_items(&self.nav_index, &self.nav_query);

        if let Some(nav_menu) = &self.nav_menu {
            cx.update_entity(nav_menu, |menu, cx| {
                menu.set_items(items, cx);
                menu.set_active_index(active_id, cx);
            });
            return nav_menu.clone();
        }

        let gallery = cx.entity().downgrade();
        let nav_menu = cx.new(|_cx| {
            Menu::new()
                .id("gallery-nav-menu")
                .mode(MenuMode::Vertical)
                .default_active(active_id)
                .with_items(items)
                .on_select(move |id, _window, cx| {
                    let Some(entry_index) = gallery_nav_index_from_id(&id) else {
                        return;
                    };
                    let _ = gallery.update(cx, |gallery, cx| {
                        if gallery.selected != entry_index {
                            gallery.selected = entry_index;
                            cx.notify();
                        }
                    });
                })
        });
        self.nav_menu = Some(nav_menu.clone());
        nav_menu
    }

    fn selected_demo(&mut self, selected: usize, cx: &mut Context<Self>) -> Option<AnyView> {
        let Some(entry) = self.entries.get(selected).copied() else {
            self.clear_active_demo();
            return None;
        };
        if self.active_demo_index == Some(selected) {
            self.pending_demo_index = None;
            return self.active_demo.clone();
        }

        if self.pending_demo_index != Some(selected) {
            self.active_demo_index = None;
            self.active_demo = None;
            self.pending_demo_index = Some(selected);
            let gallery = cx.entity().clone();
            cx.spawn(async move |_gallery, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(1))
                    .await;
                let _ = gallery.update(cx, |gallery, cx| {
                    if gallery.selected != selected || gallery.pending_demo_index != Some(selected)
                    {
                        return;
                    }
                    gallery.active_demo = Some((entry.render)(cx));
                    gallery.active_demo_index = Some(selected);
                    gallery.pending_demo_index = None;
                    cx.notify();
                });
            })
            .detach();
        }

        None
    }

    fn clear_active_demo(&mut self) {
        self.active_demo_index = None;
        self.active_demo = None;
        self.pending_demo_index = None;
    }

    fn render_demo_loading_panel(
        &self,
        selected_name: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .min_h(px(180.0))
            .gap_3()
            .text_color(theme.neutral.text_3)
            .child(Spinner::new().small().color(theme.primary.base))
            .child(Text::new(format!("正在加载 {selected_name}…")).sm())
    }

    fn set_nav_query_deferred(&mut self, query: String, cx: &mut Context<Self>) {
        if self.nav_query == query {
            return;
        }
        self.nav_query = query;

        if self.nav_refresh_pending {
            return;
        }
        self.nav_refresh_pending = true;
        cx.spawn(async move |gallery, cx| {
            let _ = gallery.update(cx, |gallery, cx| {
                gallery.nav_refresh_pending = false;
                gallery.refresh_nav_menu_for_current_query(cx);
            });
        })
        .detach();
    }

    fn refresh_nav_menu_for_current_query(&mut self, cx: &mut Context<Self>) {
        let items = gallery_nav_menu_items(&self.nav_index, &self.nav_query);
        let active_id = gallery_nav_item_id(self.selected);
        if let Some(nav_menu) = &self.nav_menu {
            cx.update_entity(nav_menu, |menu, cx| {
                menu.set_items(items, cx);
                menu.set_active_index(active_id, cx);
            });
        } else {
            cx.notify();
        }
    }

    fn render_about_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let gallery = cx.entity().clone();
        let can_install = self.install_plan.is_some();

        Space::new()
            .vertical()
            .gap_lg()
            .child(
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(Title::new("About Liora Gallery").h3())
                    .child(Paragraph::with_text(
                        "Liora Gallery 是 Liora 原生 GPUI 组件库的 dogfooding 展示应用，用真实桌面应用壳验证主题、托盘、弹层、代码与数据组件。",
                    )),
            )
            .child(
                Space::new()
                    .gap_sm()
                    .wrap()
                    .child(Tag::new(format!("Version {}", env!("CARGO_PKG_VERSION"))).success().round(true))
                    .child(Tag::new("Pure Rust + official Zed GPUI").round(true))
                    .child(Tag::new(current_platform_label()).round(true)),
            )
            .child(Paragraph::with_text(
                "更新通道使用 GitHub Releases：Gallery 优先下载当前平台的安装器（AppImage/DMG/NSIS/MSI 等），下载后会校验 SHA256SUMS.txt。需要系统权限的包管理器安装会生成明确计划，不会静默提权。",
            ))
            .child(
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Text::new(self.updater_status.label()).text_color(theme.primary.base).bold())
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(Button::new("检查更新 / Check").primary().on_click({
                                    let gallery = gallery.clone();
                                    move |_, _window, cx| check_gallery_update(gallery.clone(), cx)
                                }))
                                .child(Button::new("下载更新 / Download").on_click({
                                    let gallery = gallery.clone();
                                    move |_, _window, cx| download_gallery_update(gallery.clone(), cx)
                                }))
                                .child(Button::new("安装 / Install").disabled(!can_install).on_click({
                                    let gallery = gallery.clone();
                                    move |_, _window, cx| install_gallery_update(gallery.clone(), cx)
                                })),
                        ),
                )
                .no_shadow(),
            )
    }

    fn wire_shell_controls(&self, cx: &mut Context<Self>) {
        let gallery = cx.entity().clone();
        cx.update_entity(&self.nav_filter, |input, _cx| {
            input.set_on_change({
                let gallery = gallery.clone();
                move |value, cx| {
                    let query = value.trim().to_lowercase();
                    let _ = gallery.update(cx, |gallery, cx| {
                        gallery.set_nav_query_deferred(query, cx);
                    });
                }
            });
        });

        let gallery = cx.entity().clone();
        cx.update_entity(&self.theme_mode_segmented, |segmented, _cx| {
            segmented.set_on_change(move |value, window, cx| {
                let Some(mode) = ThemeMode::from_value(value.as_ref()) else {
                    return;
                };
                apply_theme_mode(window, cx, mode);
                let _ = gallery.update(cx, |gallery, cx| {
                    gallery.theme_mode = mode;
                    cx.notify();
                });
                toast_info!("Gallery theme switched to {}", mode.label());
            });
        });

        cx.update_entity(&self.frame_mode_switch, |switch, _cx| {
            switch.set_on_change(|enabled, window, cx| {
                set_gallery_frame_mode(WindowFrameMode::from_custom(enabled), window, cx);
            });
        });
    }
}

fn check_gallery_update(gallery: gpui::Entity<Gallery>, cx: &mut App) {
    let _ = gallery.update(cx, |gallery, cx| {
        gallery.updater_status = UpdatePanelStatus::Checking;
        gallery.install_plan = None;
        cx.notify();
    });
    let async_cx = cx.to_async();
    let executor = cx.background_executor().clone();
    cx.foreground_executor()
        .spawn(async move {
            let result = executor
                .spawn(async move {
                    Updater::default()
                        .update_available(&format!("v{}", env!("CARGO_PKG_VERSION")), false)
                })
                .await;
            let _ = async_cx.update(move |cx| {
                let _ = gallery.update(cx, |gallery, cx| {
                    gallery.updater_status = match result {
                        Ok(None) => UpdatePanelStatus::UpToDate(env!("CARGO_PKG_VERSION").into()),
                        Ok(Some(release)) => UpdatePanelStatus::Available(release.tag),
                        Err(error) => UpdatePanelStatus::Error(error.to_string()),
                    };
                    cx.notify();
                });
            });
        })
        .detach();
}

fn download_gallery_update(gallery: gpui::Entity<Gallery>, cx: &mut App) {
    let _ = gallery.update(cx, |gallery, cx| {
        gallery.updater_status = UpdatePanelStatus::Downloading("latest".into());
        cx.notify();
    });
    let async_cx = cx.to_async();
    let executor = cx.background_executor().clone();
    cx.foreground_executor()
        .spawn(async move {
            let result = executor
                .spawn(async move { download_gallery_update_sync() })
                .await;
            let _ = async_cx.update(move |cx| {
                let _ = gallery.update(cx, |gallery, cx| {
                    match result {
                        Ok(Some((version, plan))) => {
                            gallery.install_plan = Some(plan);
                            gallery.updater_status = UpdatePanelStatus::Downloaded(version);
                        }
                        Ok(None) => {
                            gallery.updater_status =
                                UpdatePanelStatus::UpToDate(env!("CARGO_PKG_VERSION").into());
                        }
                        Err(error) => {
                            gallery.updater_status = UpdatePanelStatus::Error(error.to_string())
                        }
                    }
                    cx.notify();
                });
            });
        })
        .detach();
}

fn install_gallery_update(gallery: gpui::Entity<Gallery>, cx: &mut App) {
    let _ = gallery.update(cx, |gallery, cx| {
        let Some(plan) = &gallery.install_plan else {
            gallery.updater_status = UpdatePanelStatus::Error("请先下载并校验更新".into());
            cx.notify();
            return;
        };
        let description = install_plan_description(plan);
        match &plan.action {
            InstallAction::RunExecutable { program, args } => {
                match Command::new(program).args(args).spawn() {
                    Ok(_) => gallery.updater_status = UpdatePanelStatus::Installing(description),
                    Err(error) => {
                        gallery.updater_status = UpdatePanelStatus::Error(error.to_string())
                    }
                }
            }
            InstallAction::OpenWithSystem { program, args } => {
                match Command::new(program).args(args).spawn() {
                    Ok(_) => gallery.updater_status = UpdatePanelStatus::Installing(description),
                    Err(error) => {
                        gallery.updater_status = UpdatePanelStatus::Error(error.to_string())
                    }
                }
            }
            InstallAction::Manual { .. } => {
                gallery.updater_status = UpdatePanelStatus::Installing(description);
            }
        }
        cx.notify();
    });
}

fn current_platform_label() -> &'static str {
    match Platform::current() {
        Some(Platform::LinuxX64) => "Linux x64",
        Some(Platform::MacosArm64) => "macOS arm64",
        Some(Platform::WindowsX64) => "Windows x64",
        None => "Unsupported platform",
    }
}

fn update_cache_dir(app: LioraApp) -> std::path::PathBuf {
    std::env::var_os("LIORA_UPDATE_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("liora-updates"))
        .join(app.release_name())
        .join(env!("CARGO_PKG_VERSION"))
}

fn download_gallery_update_sync()
-> Result<Option<(String, InstallPlan)>, liora_updater::UpdaterError> {
    let Some(platform) = Platform::current() else {
        return Ok(None);
    };
    let request = UpdateRequest::new(
        LioraApp::Gallery,
        format!("v{}", env!("CARGO_PKG_VERSION")),
        platform,
        update_cache_dir(LioraApp::Gallery),
    )
    .selector(liora_asset_selector(
        LioraApp::Gallery,
        platform,
        AssetKind::Installer,
    ));
    let Some(update) = Updater::default().prepare_update(&request)? else {
        return Ok(None);
    };
    Ok(Some((update.release.tag, update.install_plan)))
}

fn install_plan_description(plan: &InstallPlan) -> String {
    match &plan.action {
        InstallAction::RunExecutable { program, args } if args.is_empty() => {
            format!("Run {}", program.display())
        }
        InstallAction::RunExecutable { program, args } => {
            format!("Run {} {}", program.display(), args.join(" "))
        }
        InstallAction::OpenWithSystem { program, args } => {
            format!("Run {} {}", program, args.join(" "))
        }
        InstallAction::Manual { description } => description.clone(),
    }
}

fn theme_mode_segmented(mode: ThemeMode) -> Segmented {
    Segmented::new(vec![
        SegmentedOption::new("System", ThemeMode::System.value()),
        SegmentedOption::new("Light", ThemeMode::Light.value()),
        SegmentedOption::new("Dark", ThemeMode::Dark.value()),
    ])
    .id("gallery-theme-mode")
    .value(mode.value())
}

fn gallery_nav_index(entries: &[demos::DemoEntry]) -> Vec<GalleryNavEntry> {
    entries
        .iter()
        .enumerate()
        .map(|(_index, entry)| GalleryNavEntry {
            label: entry.name.into(),
            search_text: format!("{} {}", entry.name, entry.description).to_lowercase(),
        })
        .chain(std::iter::once(GalleryNavEntry {
            label: "About / 关于".into(),
            search_text: "about 关于 updates 更新".into(),
        }))
        .collect()
}

fn gallery_nav_visible_indices(index: &[GalleryNavEntry], query: &str) -> Vec<usize> {
    index
        .iter()
        .enumerate()
        .filter_map(|(idx, entry)| {
            (query.is_empty() || entry.search_text.contains(query)).then_some(idx)
        })
        .collect()
}

fn gallery_nav_item_id(index: usize) -> SharedString {
    format!("gallery-nav-{index}").into()
}

fn gallery_nav_index_from_id(id: &str) -> Option<usize> {
    id.strip_prefix("gallery-nav-")?.parse().ok()
}

fn gallery_nav_menu_items(index: &[GalleryNavEntry], query: &str) -> Vec<MenuNode> {
    let visible = gallery_nav_visible_indices(index, query);
    if visible.is_empty() {
        return vec![MenuNode::Item(liora_components::MenuItem {
            id: "gallery-nav-empty".into(),
            label: "无匹配组件".into(),
            icon: None,
        })];
    }

    visible
        .into_iter()
        .filter_map(|entry_index| {
            let entry = index.get(entry_index)?;
            Some(MenuNode::Item(liora_components::MenuItem {
                id: gallery_nav_item_id(entry_index),
                label: entry.label.clone(),
                icon: None,
            }))
        })
        .collect()
}

struct PortalLayer;

impl IntoElement for PortalLayer {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl gpui::RenderOnce for PortalLayer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let passive_portals = if cx.has_global::<PassivePortal>() {
            std::mem::take(&mut cx.global_mut::<PassivePortal>().entries)
        } else {
            Vec::new()
        };
        let portals = if cx.has_global::<Portal>() {
            std::mem::take(&mut cx.global_mut::<Portal>().entries)
        } else {
            Vec::new()
        };

        if passive_portals.is_empty() && portals.is_empty() {
            return div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .into_any_element();
        }

        let mut container = div().absolute().top_0().left_0().size_full();

        if !passive_portals.is_empty() {
            let mut passive_container = div()
                .id("passive-portal-layer")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .bg(gpui::transparent_black());

            for entry in passive_portals {
                passive_container = passive_container.child((entry.render)(window, cx));
            }

            container = container.child(passive_container);
        }

        if !portals.is_empty() {
            let mut active_container = div()
                .id("portal-layer")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .cursor_default()
                .occlude()
                .bg(gpui::transparent_black())
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_move(|_, _, cx| {
                    cx.stop_propagation();
                });

            for entry in portals {
                active_container = active_container.child((entry.render)(window, cx));
            }

            container = container.child(active_container);
        }

        container.into_any_element()
    }
}

#[cfg(test)]
mod shell_regression_tests {
    use super::{
        GalleryNavEntry, gallery_nav_index_from_id, gallery_nav_item_id, gallery_nav_menu_items,
        gallery_nav_visible_indices,
    };

    #[test]
    fn close_confirm_body_allows_content_to_shrink_and_wrap() {
        let source = include_str!("main.rs")
            .split("mod shell_regression_tests")
            .next()
            .unwrap();

        assert!(source.contains("fn show_gallery_close_confirm"));
        assert!(source.contains(".shrink()"));
        assert!(source.contains("Paragraph::with_text"));
    }

    #[test]
    fn close_confirm_dismissal_resets_flag_and_escape_is_enabled() {
        let source = include_str!("main.rs")
            .split("mod shell_regression_tests")
            .next()
            .unwrap();

        let confirm = source
            .split("fn show_gallery_close_confirm")
            .nth(1)
            .expect("Gallery close confirmation should exist")
            .split("fn gallery_tray_icon")
            .next()
            .expect("Gallery close confirmation should appear before tray icon helper");

        assert!(confirm.contains(".close_on_escape(true)"));
        assert!(!confirm.contains(".close_on_escape(false)"));
        assert!(confirm.contains(".immediate()"));
        assert!(confirm.contains(".show_in_window(window, cx)"));
        assert!(confirm.contains(".on_close(|_, cx| reset_gallery_close_confirm(cx))"));
        assert_eq!(
            confirm.matches("reset_gallery_close_confirm(cx);").count(),
            2
        );
        assert!(source.contains("fn reset_gallery_close_confirm(cx: &mut App)"));
    }

    fn sample_nav_entries() -> Vec<GalleryNavEntry> {
        vec![
            GalleryNavEntry {
                label: "Button".into(),
                search_text: "button clickable".into(),
            },
            GalleryNavEntry {
                label: "Input".into(),
                search_text: "input search form".into(),
            },
            GalleryNavEntry {
                label: "About / 关于".into(),
                search_text: "about 关于 updates 更新".into(),
            },
        ]
    }

    #[test]
    fn gallery_nav_visible_indices_preserve_original_selection_ids() {
        let entries = sample_nav_entries();

        assert_eq!(gallery_nav_visible_indices(&entries, ""), vec![0, 1, 2]);
        assert_eq!(gallery_nav_visible_indices(&entries, "input"), vec![1]);
        assert_eq!(gallery_nav_visible_indices(&entries, "关于"), vec![2]);
        assert!(gallery_nav_visible_indices(&entries, "missing").is_empty());
        assert_eq!(gallery_nav_item_id(12).as_ref(), "gallery-nav-12");
        assert_eq!(gallery_nav_index_from_id("gallery-nav-12"), Some(12));
        assert_eq!(gallery_nav_index_from_id("gallery-nav-empty"), None);
    }

    #[test]
    fn gallery_nav_menu_items_use_original_indices_as_menu_ids() {
        let entries = sample_nav_entries();
        let items = gallery_nav_menu_items(&entries, "input");

        assert_eq!(items.len(), 1);
        let liora_components::MenuNode::Item(item) = &items[0] else {
            panic!("Gallery nav should build flat Menu items");
        };
        assert_eq!(item.id.as_ref(), "gallery-nav-1");
        assert_eq!(item.label.as_ref(), "Input");

        let empty_items = gallery_nav_menu_items(&entries, "missing");
        let liora_components::MenuNode::Item(empty_item) = &empty_items[0] else {
            panic!("Gallery nav empty state should still be a Menu item");
        };
        assert_eq!(empty_item.id.as_ref(), "gallery-nav-empty");
    }

    #[test]
    fn gallery_search_refreshes_menu_without_full_gallery_notify() {
        let source = include_str!("main.rs")
            .split("mod shell_regression_tests")
            .next()
            .unwrap();

        assert!(source.contains("fn set_nav_query_deferred"));
        assert!(source.contains("nav_refresh_pending"));
        assert!(source.contains("fn refresh_nav_menu_for_current_query"));
        assert!(!source.contains("fn current_nav_query"));
        assert!(!source.contains(
            "self.nav_filter
            .read(cx)"
        ));
        assert!(source.contains("fn gallery_nav_index"));
        assert!(source.contains("fn gallery_nav_visible_indices"));
        assert!(source.contains("fn gallery_nav_menu_items"));
        assert!(source.contains("fn gallery_nav_item_id"));
        assert!(source.contains("fn gallery_nav_index_from_id"));
        assert!(source.contains("nav_menu: Option<gpui::Entity<Menu>>"));
        assert!(source.contains("nav_scroll: ScrollHandle"));
        assert!(source.contains("ScrollHandle::new()"));
        assert!(source.contains(r#".id("gallery-nav-menu")"#));
        assert!(source.contains(".mode(MenuMode::Vertical)"));
        assert!(source.contains(".with_items(items)"));
        assert!(source.contains(".on_select(move |id"));
        assert!(source.contains("menu.set_items(items, cx);"));
        assert!(source.contains("menu.set_active_index(active_id, cx);"));
        assert!(source.contains("if gallery.selected != entry_index"));
        assert!(!source.contains("struct GalleryNavMenu"));
        assert!(!source.contains("UniformListScrollHandle"));
        assert!(!source.contains("uniform_list("));
        assert!(!source.contains(".track_scroll(self.scroll_handle.clone())"));
        assert!(!source.contains("ListState::new"));
        assert!(!source.contains("list(self.list_state.clone()"));
        assert!(!source.contains("list_state.reset"));
        assert!(!source.contains("timer(Duration::from_millis(24))"));
        let shell = source
            .split("let shell = Container::new()")
            .nth(1)
            .expect("Gallery shell should exist")
            .split("AppWindowFrame::new")
            .next()
            .expect("shell should end before app frame");
        assert!(shell.contains(r#".id("gallery-nav-scroll")"#));
        assert!(shell.contains(".overflow_y_scroll()"));
        assert!(shell.contains(".track_scroll(&self.nav_scroll)"));
        assert!(!source.contains("gallery.refresh_nav_menu_for_query(query, cx);"));
        assert!(!source.contains(
            "move |_, cx| {
                    let _ = gallery.update(cx, |_gallery, cx| {
                        cx.notify();"
        ));
    }

    #[test]
    fn gallery_nav_does_not_eagerly_build_every_demo_or_rebind_controls_on_render() {
        let source = include_str!("main.rs")
            .split("mod shell_regression_tests")
            .next()
            .unwrap();
        let open_window = source
            .split("fn open_gallery_window")
            .nth(1)
            .expect("Gallery window bootstrap should exist")
            .split("fn gallery_window_options")
            .next()
            .expect("bootstrap should appear before window options");
        let render = source
            .split("impl Render for Gallery")
            .nth(1)
            .expect("Gallery render impl should exist")
            .split("impl Gallery")
            .next()
            .expect("Gallery render impl should appear before methods");

        assert!(source.contains("active_demo_index: Option<usize>"));
        assert!(source.contains("active_demo: Option<AnyView>"));
        assert!(source.contains("pending_demo_index: Option<usize>"));
        assert!(source.contains("fn selected_demo(&mut self"));
        assert!(source.contains("cx.spawn(async move |_gallery, cx|"));
        assert!(source.contains("timer(Duration::from_millis(1))"));
        assert!(source.contains("self.render_demo_loading_panel"));
        assert!(source.contains("self.active_demo = None;"));
        assert!(source.contains("gallery.active_demo = Some((entry.render)(cx));"));
        assert!(source.contains("fn clear_active_demo(&mut self)"));
        assert!(!open_window.contains("let demo_cache = vec![None; entries.len()];"));
        assert!(!open_window.contains("entries.iter().map(|entry| (entry.render)(cx)).collect()"));
        assert!(!source.contains("demo_cache: Vec<Option<AnyView>>"));
        assert!(!source.contains("demos: Vec<AnyView>"));
        assert!(!source.contains("self.demos.get(selected)"));
        assert!(!render.contains("self.active_demo = Some((entry.render)(cx));"));
        assert!(!render.contains("self.wire_shell_controls(cx);"));
    }

    #[test]
    fn gallery_startup_does_not_auto_download_updates_on_ui_thread() {
        let source = include_str!("main.rs")
            .split("mod shell_regression_tests")
            .next()
            .unwrap();
        let open_window = source
            .split("fn open_gallery_window")
            .nth(1)
            .expect("Gallery window bootstrap should exist")
            .split("fn gallery_window_options")
            .next()
            .expect("bootstrap should appear before window options");

        assert!(!open_window.contains("download_gallery_update("));
        assert!(!open_window.contains("auto_update_view"));
        assert!(source.contains("Button::new(\"检查更新 / Check\")"));
        assert!(source.contains("Button::new(\"下载更新 / Download\")"));
    }
}

fn main() {
    run_gallery();
}
