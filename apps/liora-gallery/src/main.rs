#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::{
    AnyView, App, Component, Context, FontWeight, Global, Render, RenderImage, SharedString,
    Window, WindowOptions, div, img, prelude::*, px, size,
};
use liora_components::{
    AppWindowFrame, Button, Card, Checkbox, Container, Dialog, Input, Menu, MenuBar, MenuItem,
    NavigationMenu, NavigationMenuMode, NavigationMenuNode, Paragraph, Segmented, SegmentedOption,
    Sidebar, Space, Spinner, Switch, Tag, Text, Title, TitleBar, WindowFrameMode,
    apply_window_frame_mode, frame_mode_switch_row, init_liora_with_options,
    request_window_frame_mode, toast_info, toast_success,
};
use liora_core::{
    Config, FontConfig, FontLoadMode, FontLoadOptions, LinuxDesktopIdentity, LinuxDesktopPngIcon,
    Options, PassivePortal, Portal, ThemeMode, apply_locale, apply_theme_mode,
    attach_system_theme_observer, current_locale, ensure_linux_desktop_identity,
    linux_desktop_entry, linux_desktop_png_icon_path, load_app_fonts,
    startup_maximized_window_bounds, tr,
};
use liora_gallery::{category, demos, tray_menu::gallery_tray_menu};
use liora_tray::{
    MouseButton, MouseButtonState, Tray, TrayCloseAction, TrayCommand, TrayConfig,
    TrayControlCenter, TrayIconEvent, icon_from_png_bytes, solid_icon,
};
use liora_updater::{
    AssetKind, AssetSelector, InstallAction, InstallPlan, Platform, UpdateRequest, Updater,
};

pub mod locales {
    include!(concat!(env!("OUT_DIR"), "/locales_keys.rs"));
}

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
    nav_menu: Option<gpui::Entity<NavigationMenu>>,
    nav_query: String,
    nav_refresh_pending: bool,
    theme_mode: ThemeMode,
    theme_mode_segmented: gpui::Entity<Segmented>,
    locale_segmented: gpui::Entity<Segmented>,
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
    fn label(&self, cx: &impl liora_core::LocalesContext) -> String {
        match self {
            Self::Idle => tr(cx, locales::update_status::idle).to_string(),
            Self::Checking => tr(cx, locales::update_status::checking).to_string(),
            Self::UpToDate(version) => locale_template(cx, locales::update_status::up_to_date)
                .replace("{version}", version),
            Self::Available(version) => {
                locale_template(cx, locales::update_status::available).replace("{version}", version)
            }
            Self::Downloading(version) => locale_template(cx, locales::update_status::downloading)
                .replace("{version}", version),
            Self::Downloaded(version) => locale_template(cx, locales::update_status::downloaded)
                .replace("{version}", version),
            Self::Installing(detail) => {
                locale_template(cx, locales::update_status::installing).replace("{detail}", detail)
            }
            Self::Error(error) => {
                locale_template(cx, locales::update_status::error).replace("{error}", error)
            }
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

fn locale_template(
    cx: &impl liora_core::LocalesContext,
    key: impl liora_core::IntoLocalesKey,
) -> String {
    tr(cx, key).to_string()
}

fn theme_mode_label(cx: &impl liora_core::LocalesContext, mode: ThemeMode) -> String {
    match mode {
        ThemeMode::System => tr(cx, locales::theme_mode::system).to_string(),
        ThemeMode::Light => tr(cx, locales::theme_mode::light).to_string(),
        ThemeMode::Dark => tr(cx, locales::theme_mode::dark).to_string(),
    }
}

fn localized_platform_label(cx: &impl liora_core::LocalesContext) -> String {
    match Platform::current() {
        Some(Platform::LinuxX64) => tr(cx, locales::platform::linux_x64).to_string(),
        Some(Platform::MacosArm64) => tr(cx, locales::platform::macos_arm64).to_string(),
        Some(Platform::WindowsX64) => tr(cx, locales::platform::windows_x64).to_string(),
        None => tr(cx, locales::platform::unsupported).to_string(),
    }
}

fn localized_tray_tooltip(cx: &impl liora_core::LocalesContext, name: &str) -> String {
    match name {
        "syncing" => tr(cx, locales::tray::tooltip_syncing).to_string(),
        "error" => tr(cx, locales::tray::tooltip_error).to_string(),
        _ => tr(cx, locales::tray::tooltip_default).to_string(),
    }
}

struct GalleryTrayState {
    tray: Tray,
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
    gpui_platform::application()
        .with_assets(liora_icons::IconAssetSource)
        .run(|cx: &mut App| {
            install_gallery_fonts(cx);
            init_liora_with_options(cx, gallery_liora_options());
            register_gallery_desktop_identity();
            register_gallery_system_menus(cx);

            install_gallery_tray(cx);
            if let Some(handle) = open_gallery_window(cx) {
                if cx.has_global::<GalleryTrayState>() {
                    cx.global_mut::<GalleryTrayState>().window = Some(handle);
                }
            }
        });
}

/// Registers the gallery example menu through GPUI's official application menu API.
fn register_gallery_system_menus(cx: &mut App) {
    Menu::register(
        cx,
        [
            Menu::new(locales::menu::file)
                .item(MenuItem::new_window())
                .item(MenuItem::open_file())
                .item(MenuItem::open_folder())
                .item(MenuItem::separator())
                .item(MenuItem::save())
                .item(MenuItem::quit()),
            Menu::new(locales::menu::edit)
                .item(MenuItem::undo())
                .item(MenuItem::redo())
                .item(MenuItem::separator())
                .item(MenuItem::cut())
                .item(MenuItem::copy())
                .item(MenuItem::paste())
                .item(MenuItem::separator())
                .item(MenuItem::select_all()),
            Menu::new(locales::menu::view)
                .item(MenuItem::command_palette())
                .item(MenuItem::toggle_sidebar())
                .item(MenuItem::toggle_statusbar())
                .item(MenuItem::separator())
                .item(MenuItem::action(
                    liora_components::MenuAction::ZoomIn,
                    tr(cx, locales::menu::zoom_in),
                ))
                .item(MenuItem::action(
                    liora_components::MenuAction::ZoomOut,
                    tr(cx, locales::menu::zoom_out),
                ))
                .item(MenuItem::action(
                    liora_components::MenuAction::ZoomReset,
                    tr(cx, locales::menu::reset_zoom),
                )),
            Menu::new(locales::menu::help)
                .item(MenuItem::open_url(
                    tr(cx, locales::menu::github),
                    "https://github.com/yhyzgn/liora",
                ))
                .item(MenuItem::new(
                    "about-gallery",
                    tr(cx, locales::menu::about_gallery),
                )),
        ],
    );
}

// This app uses init_liora_with_options instead of init_liora(cx) because it sets app fonts.
fn gallery_liora_options() -> Options {
    let options = Options::system()
        .with_locale("zh-CN")
        .with_fallback_locale("en-US")
        .with_fonts(
            FontConfig::system()
                .with_ui_families(["MiSans", "Segoe UI", "Arial"])
                .with_ui_weight(FontWeight::MEDIUM)
                .with_code_families(["Consolas", "JetBrains Mono", "SF Mono", "Monospace"]),
        );

    match options.clone().try_with_locales_dir(app_locales_dir()) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("Liora Gallery locales loading report: {error}");
            options
        }
    }
}

fn install_gallery_fonts(cx: &mut App) {
    let options = FontLoadOptions::new(app_font_load_mode()).require_family("MiSans");

    let mut options = add_embedded_app_fonts(options);

    for dir in app_font_dirs("liora-gallery") {
        options = options.external_dir(dir);
    }

    let report = load_app_fonts(cx, options);
    if !report.failures.is_empty() || !report.required_families_available() {
        eprintln!("Liora Gallery font loading report: {report:?}");
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

fn app_locales_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/locales")
}

fn open_gallery_window(cx: &mut App) -> Option<gpui::AnyWindowHandle> {
    let frame_mode = gallery_frame_mode(cx);
    match cx.open_window(gallery_window_options(cx, frame_mode), |window, cx| {
        attach_system_theme_observer(window, cx);

        let entries = demos::registry();
        let nav_index = gallery_nav_index(&entries, cx);
        let view = cx.new(|cx| {
            let theme_mode = cx.global::<Config>().theme_mode;
            let selected = default_gallery_selection(&entries);
            let gallery = Gallery {
                entries,
                active_demo_index: None,
                active_demo: None,
                pending_demo_index: None,
                nav_index,
                selected,
                nav_filter: cx
                    .new(|cx| Input::new("", cx).placeholder(locales::gallery::search_placeholder)),
                nav_menu: None,
                nav_query: String::new(),
                nav_refresh_pending: false,
                theme_mode,
                theme_mode_segmented: cx.new(move |cx| theme_mode_segmented(theme_mode, cx)),
                locale_segmented: cx.new(|cx| locale_segmented(cx)),
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
                title: Some(tr(cx, locales::gallery::window_title)),
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
        .tooltip(tr(cx, locales::tray::tooltip_default).to_string())
        .menu(gallery_tray_menu());
    if let Some(icon) = gallery_tray_icon("default") {
        config = config.icon(icon);
    }

    match Tray::install(config) {
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
        state.window_visible = true;
    }

    match request_window_frame_mode(window, mode) {
        liora_components::WindowFrameChange::AppliedLive => {
            toast_info!(
                "{}",
                locale_template(cx, locales::window_frame::gallery_switched).replace(
                    "{mode}",
                    &if mode.is_custom() {
                        tr(cx, locales::window_frame::custom).to_string()
                    } else {
                        tr(cx, locales::window_frame::system).to_string()
                    },
                )
            );
        }
        liora_components::WindowFrameChange::RequiresWindowReopen => {
            toast_info!(
                "{}",
                locale_template(cx, locales::window_frame::gallery_reopen).replace(
                    "{mode}",
                    &if mode.is_custom() {
                        tr(cx, locales::window_frame::custom_frame).to_string()
                    } else {
                        tr(cx, locales::window_frame::system_frame).to_string()
                    },
                )
            );
            window.defer(cx, |window, cx| {
                window.remove_window();
                if let Some(handle) = open_gallery_window(cx)
                    && cx.has_global::<GalleryTrayState>()
                {
                    cx.global_mut::<GalleryTrayState>().window = Some(handle);
                }
            });
        }
    }
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
                let tooltip = localized_tray_tooltip(cx, name.as_str());
                let state = cx.global_mut::<GalleryTrayState>();
                if let Some(icon) = gallery_tray_icon(&name) {
                    if let Err(error) = state.tray.set_icon(icon) {
                        eprintln!("failed to update Liora Gallery tray icon: {error}");
                    }
                }
                let _ = state.tray.set_tooltip(Some(&tooltip));
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
        .title(tr(cx, locales::close_confirm::gallery_title))
        .immediate()
        .close_on_click_outside(false)
        .close_on_escape(true)
        .on_close(|_, cx| reset_gallery_close_confirm(cx))
        .content(move |_window, cx| {
            let remember_for_checkbox = remember.clone();
            let remember_for_exit = remember.clone();
            let remember_for_hide = remember.clone();
            let checkbox = cx.new(move |cx| {
                Checkbox::new(false, cx)
                    .label(locales::close_confirm::remember)
                    .on_change({
                        let remember = remember_for_checkbox.clone();
                        move |checked, _, _| remember.store(checked, Ordering::Relaxed)
                    })
            });

            Space::new()
                .vertical()
                .gap_lg()
                .grow()
                .shrink()
                .child(Paragraph::with_text(tr(
                    cx,
                    locales::close_confirm::gallery_body,
                )))
                .child(checkbox)
                .child(
                    Space::new()
                        .gap_md()
                        .wrap()
                        .child(Button::new(locales::close_confirm::hide_to_tray).on_click(
                            move |_, window, cx| {
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
                            },
                        ))
                        .child(
                            Button::new(locales::close_confirm::exit_process)
                                .danger()
                                .on_click(move |_, _, cx| {
                                    if remember_for_exit.load(Ordering::Relaxed)
                                        && cx.has_global::<TrayControlCenter>()
                                    {
                                        cx.global_mut::<TrayControlCenter>()
                                            .set_remembered_close_action(
                                                TrayCloseAction::ExitProcess,
                                            );
                                    }
                                    reset_gallery_close_confirm(cx);
                                    Dialog::close(cx);
                                    cx.quit();
                                }),
                        ),
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
        assert!(source.contains("Sidebar::new()"));
        assert!(source.contains("NavigationMenu::new()"));
        assert!(source.contains(".no_shrink()"));
        assert!(source.contains("init_liora(cx)"));
        assert!(source.contains("gpui_platform::application()"));
        assert!(source.contains("with_assets(liora_icons::IconAssetSource)"));
        assert!(source.contains("nav_filter"));
        assert!(source.contains("nav_menu: Option"));
        assert!(source.contains(r#".id("gallery-sidebar")"#));
        assert!(source.contains(".aside_passthrough()"));
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
        let production_shell = source
            .rsplit("let shell = Container::new()")
            .next()
            .expect("Gallery production shell should exist")
            .split("AppWindowFrame::new")
            .next()
            .expect("Gallery shell should end before app frame");
        assert!(!production_shell.contains(".aside_width_lg()"));
        assert!(source.contains("check_gallery_update"));
        assert!(source.contains("MiSans-Medium.ttf"));
        assert!(source.contains("require_family(\"MiSans\")"));
        assert!(source.contains(".with_ui_weight(FontWeight::MEDIUM)"));
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
            .unwrap_or_else(|| tr(cx, locales::gallery::about_label).to_string());

        let header_main = div()
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
                            .child(Title::new(locales::gallery::title).h2())
                            .child(Text::new(
                                locale_template(cx, locales::gallery::subtitle)
                                    .replace("{count}", &self.entries.len().to_string()),
                            )),
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
                    .child(Button::new(locales::gallery::refresh).primary().on_click({
                        let gallery = cx.entity().clone();
                        move |_, _window, cx| {
                            let _ = gallery.update(cx, |gallery, cx| {
                                gallery.refresh_revision += 1;
                                cx.notify();
                            });
                            toast_success!("{}", tr(cx, locales::gallery::refreshed));
                        }
                    }))
                    .child(
                        Space::new()
                            .gap_sm()
                            .child(Text::new(locales::gallery::theme))
                            .child(self.theme_mode_segmented.clone()),
                    )
                    .child(
                        Space::new()
                            .gap_sm()
                            .child(Text::new(locales::language::label))
                            .child(self.locale_segmented.clone()),
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
            Paragraph::with_text(locales::gallery::no_entry).into_any_element()
        };

        let content = Card::new(content_body).no_shadow().no_shrink();

        liora_components::message::render_messages(cx);
        liora_components::notification::render_notifications(cx);
        liora_components::image::render_image_preview(_window, cx);
        liora_core::render_active_tooltip_in_window(_window, cx);
        liora_core::render_active_popover_in_window(_window, cx);
        liora_core::render_active_modal_in_window(_window, cx);
        liora_core::render_active_drawer_in_window(_window, cx);

        let header = div()
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(gallery_fallback_menu_bar(cx))
            .child(header_main);

        let shell = Container::new()
            .header(header)
            .header_height(px(112.0))
            .aside(
                Sidebar::new()
                    .id("gallery-sidebar")
                    .expanded_width(px(280.0))
                    .scrollable()
                    .header(div().w_full().p_2().child(self.nav_filter.clone()))
                    .child(nav_menu),
            )
            .aside_passthrough()
            .main_scroll()
            .main_padding_xl()
            .child(content)
            .footer(self.render_status_bar(selected_label, cx))
            .footer_height(px(38.0))
            .overlay(PortalLayer);

        AppWindowFrame::new(tr(cx, locales::gallery::window_title), shell)
            .titlebar(gallery_titlebar())
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
                    .child(Text::new(self.updater_status.label(cx)).sm().nowrap()),
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

    fn gallery_nav_menu(
        &mut self,
        selected: usize,
        cx: &mut Context<Self>,
    ) -> gpui::Entity<NavigationMenu> {
        let active_id = gallery_nav_item_id(selected);

        if let Some(nav_menu) = &self.nav_menu {
            cx.update_entity(nav_menu, |menu, cx| {
                menu.set_active_index(active_id, cx);
            });
            return nav_menu.clone();
        }

        let items = gallery_nav_menu_items(&self.nav_index, &self.nav_query, cx);
        let gallery = cx.entity().downgrade();
        let nav_menu = cx.new(|_cx| {
            NavigationMenu::new()
                .id("gallery-nav-menu")
                .mode(NavigationMenuMode::Vertical)
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
            .child(
                Text::new(
                    locale_template(cx, locales::gallery::loading_demo)
                        .replace("{name}", selected_name),
                )
                .sm(),
            )
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
        let items = gallery_nav_menu_items(&self.nav_index, &self.nav_query, cx);
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
                    .child(Title::new(locales::gallery::about_title).h3())
                    .child(Paragraph::with_text(locales::gallery::about_body)),
            )
            .child(
                Space::new()
                    .gap_sm()
                    .wrap()
                    .child(
                        Tag::new(
                            locale_template(cx, locales::gallery::version)
                                .replace("{version}", env!("CARGO_PKG_VERSION")),
                        )
                        .success()
                        .round(true),
                    )
                    .child(Tag::new(locales::gallery::pure_rust_gpui).round(true))
                    .child(Tag::new(localized_platform_label(cx)).round(true)),
            )
            .child(Paragraph::with_text(tr(
                cx,
                locales::gallery::update_channel,
            )))
            .child(
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Title::new(locales::gallery::validates_title).h4())
                        .child(Paragraph::with_text(tr(
                            cx,
                            locales::gallery::validates_body,
                        )))
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(Tag::new(locales::gallery::tag_dogfooding).round(true))
                                .child(Tag::new(locales::gallery::tag_theme_parity).round(true))
                                .child(Tag::new(locales::gallery::tag_installer_assets).round(true))
                                .child(Tag::new(locales::gallery::tag_no_webview).round(true)),
                        ),
                )
                .no_shadow(),
            )
            .child(
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Title::new(locales::gallery::contract_title).h4())
                        .child(Paragraph::with_text(tr(
                            cx,
                            locales::gallery::contract_body,
                        )))
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(Tag::new(locales::gallery::tag_components_first).round(true))
                                .child(Tag::new(locales::gallery::tag_official_gpui).round(true))
                                .child(Tag::new(locales::gallery::tag_readme_sync).round(true)),
                        ),
                )
                .no_shadow(),
            )
            .child(
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(
                            Text::new(self.updater_status.label(cx))
                                .text_color(theme.primary.base)
                                .bold(),
                        )
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(
                                    Button::new(locales::gallery::check_updates)
                                        .primary()
                                        .on_click({
                                            let gallery = gallery.clone();
                                            move |_, _window, cx| {
                                                check_gallery_update(gallery.clone(), cx)
                                            }
                                        }),
                                )
                                .child(Button::new(locales::gallery::download_update).on_click({
                                    let gallery = gallery.clone();
                                    move |_, _window, cx| {
                                        download_gallery_update(gallery.clone(), cx)
                                    }
                                }))
                                .child(
                                    Button::new(locales::gallery::install_update)
                                        .disabled(!can_install)
                                        .on_click({
                                            let gallery = gallery.clone();
                                            move |_, _window, cx| {
                                                install_gallery_update(gallery.clone(), cx)
                                            }
                                        }),
                                ),
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
                toast_info!(
                    "{}",
                    locale_template(cx, locales::gallery::theme_switched)
                        .replace("{mode}", &theme_mode_label(cx, mode))
                );
            });
        });

        cx.update_entity(&self.locale_segmented, |segmented, cx| {
            segmented.set_options(locale_segmented_options(cx));
        });

        let gallery = cx.entity().clone();
        cx.update_entity(&self.locale_segmented, |segmented, _cx| {
            segmented.set_on_change(move |value, window, cx| {
                if apply_locale(window, cx, value.as_ref()).is_ok() {
                    let gallery = gallery.clone();
                    window.defer(cx, move |_window, cx| {
                        let _ = gallery.update(cx, |gallery, cx| {
                            cx.update_entity(&gallery.locale_segmented, |segmented, cx| {
                                segmented.set_options(locale_segmented_options(cx));
                            });
                            cx.update_entity(&gallery.theme_mode_segmented, |segmented, cx| {
                                segmented.set_options(theme_mode_segmented_options(cx));
                            });
                            cx.update_entity(&gallery.nav_filter, |input, cx| {
                                input.set_placeholder(
                                    tr(cx, locales::gallery::search_placeholder),
                                    cx,
                                );
                            });
                            register_gallery_system_menus(cx);
                            if cx.has_global::<GalleryTrayState>() {
                                let active_icon = cx
                                    .has_global::<TrayControlCenter>()
                                    .then(|| {
                                        cx.global::<TrayControlCenter>().state.active_icon.clone()
                                    })
                                    .unwrap_or_else(|| "default".into());
                                let tooltip = localized_tray_tooltip(cx, &active_icon);
                                let _ = cx
                                    .global_mut::<GalleryTrayState>()
                                    .tray
                                    .set_tooltip(Some(&tooltip));
                            }
                            cx.notify();
                        });
                        toast_info!("{}", tr(cx, locales::language::switched));
                    });
                }
            });
        });

        let gallery = cx.entity().clone();
        cx.update_entity(&self.frame_mode_switch, |switch, _cx| {
            switch.set_on_change(move |enabled, window, cx| {
                let mode = WindowFrameMode::from_custom(enabled);
                set_gallery_frame_mode(mode, window, cx);
                let _ = gallery.update(cx, |gallery, cx| {
                    gallery.frame_mode = mode;
                    cx.notify();
                });
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
                    liora_updater()
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
            gallery.updater_status = UpdatePanelStatus::Error(
                tr(cx, locales::update_status::download_first).to_string(),
            );
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

const GALLERY_UPDATE_APP: &str = "liora-gallery";

fn liora_updater() -> Updater {
    Updater::new("yhyzgn", "liora")
}

fn gallery_asset_selector(platform: Platform) -> AssetSelector {
    let priority = match platform {
        Platform::LinuxX64 => [
            AssetKind::Installer,
            AssetKind::PortableArchive,
            AssetKind::RawExecutable,
        ]
        .as_slice()
        .to_vec(),
        Platform::MacosArm64 | Platform::WindowsX64 => {
            [AssetKind::Installer, AssetKind::RawExecutable]
                .as_slice()
                .to_vec()
        }
    };
    AssetSelector::for_platform(platform)
        .matching_prefix(GALLERY_UPDATE_APP)
        .kind_priority(priority)
}

fn update_cache_dir(app: &str) -> std::path::PathBuf {
    std::env::var_os("LIORA_UPDATE_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("liora-updates"))
        .join(app)
        .join(env!("CARGO_PKG_VERSION"))
}

fn download_gallery_update_sync()
-> Result<Option<(String, InstallPlan)>, liora_updater::UpdaterError> {
    let Some(platform) = Platform::current() else {
        return Ok(None);
    };
    let request = UpdateRequest::new(
        GALLERY_UPDATE_APP,
        format!("v{}", env!("CARGO_PKG_VERSION")),
        platform,
        update_cache_dir(GALLERY_UPDATE_APP),
    )
    .selector(gallery_asset_selector(platform));
    let Some(update) = liora_updater().prepare_update(&request)? else {
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

fn locale_segmented(cx: &App) -> Segmented {
    Segmented::new(locale_segmented_options(cx))
        .id("gallery-locale")
        .value(current_locale(cx).as_str())
}

fn locale_segmented_options(_cx: &App) -> Vec<SegmentedOption> {
    vec![
        SegmentedOption::new(locales::language::zh_cn, "zh-CN"),
        SegmentedOption::new(locales::language::en_us, "en-US"),
    ]
}

fn theme_mode_segmented(mode: ThemeMode, cx: &App) -> Segmented {
    Segmented::new(theme_mode_segmented_options(cx))
        .id("gallery-theme-mode")
        .value(mode.value())
}

fn theme_mode_segmented_options(cx: &impl liora_core::LocalesContext) -> Vec<SegmentedOption> {
    vec![
        SegmentedOption::new(
            tr(cx, locales::theme_mode::system),
            ThemeMode::System.value(),
        ),
        SegmentedOption::new(locales::theme_mode::light, ThemeMode::Light.value()),
        SegmentedOption::new(locales::theme_mode::dark, ThemeMode::Dark.value()),
    ]
}

fn gallery_nav_index(
    entries: &[demos::DemoEntry],
    cx: &impl liora_core::LocalesContext,
) -> Vec<GalleryNavEntry> {
    entries
        .iter()
        .enumerate()
        .map(|(_index, entry)| GalleryNavEntry {
            label: entry.name.into(),
            search_text: format!("{} {}", entry.name, entry.description).to_lowercase(),
        })
        .chain(std::iter::once(GalleryNavEntry {
            label: tr(cx, locales::gallery::about_nav),
            search_text: tr(cx, locales::gallery::about_search).to_string(),
        }))
        .collect()
}

/// Builds the visible in-window menu bar used by the custom Gallery titlebar.
fn gallery_titlebar() -> TitleBar {
    TitleBar::new()
        .title("Liora UI Gallery")
        .subtitle("Native component gallery")
}

/// Builds the in-window fallback menu bar that mirrors the registered GPUI platform menu.
///
/// `Menu::register` delegates to GPUI/OS integration, but that platform menu
/// is not guaranteed to be visible inside a Linux/Windows application window.
/// Gallery renders this fallback in the content header so both system-frame and
/// custom-frame modes demonstrate the same command structure.
fn gallery_fallback_menu_bar(cx: &impl liora_core::LocalesContext) -> MenuBar {
    MenuBar::new([
        Menu::new(locales::menu::file)
            .perform_builtin_actions(false)
            .item(MenuItem::new_window())
            .item(MenuItem::open_file())
            .item(MenuItem::open_folder())
            .item(MenuItem::separator())
            .item(MenuItem::save())
            .item(MenuItem::quit()),
        Menu::new(locales::menu::edit)
            .perform_builtin_actions(false)
            .item(MenuItem::undo())
            .item(MenuItem::redo())
            .item(MenuItem::separator())
            .item(MenuItem::cut())
            .item(MenuItem::copy())
            .item(MenuItem::paste())
            .item(MenuItem::separator())
            .item(MenuItem::select_all()),
        Menu::new(locales::menu::view)
            .perform_builtin_actions(false)
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
        Menu::new(locales::menu::help)
            .perform_builtin_actions(false)
            .item(MenuItem::open_url(
                tr(cx, locales::menu::github),
                "https://github.com/yhyzgn/liora",
            ))
            .item(MenuItem::new(
                "about-gallery",
                tr(cx, locales::menu::about_gallery),
            )),
    ])
}

fn default_gallery_selection(entries: &[demos::DemoEntry]) -> usize {
    entries.len()
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

fn gallery_category_name(
    category: category::Category,
    cx: &impl liora_core::LocalesContext,
) -> String {
    match category {
        category::Category::About => tr(cx, locales::category::about).to_string(),
        category::Category::IconLibrary => tr(cx, locales::category::icon_library).to_string(),
        category::Category::WindowLayout => tr(cx, locales::category::window_layout).to_string(),
        category::Category::Control => tr(cx, locales::category::control).to_string(),
    }
}

fn gallery_nav_menu_items(
    index: &[GalleryNavEntry],
    query: &str,
    cx: &impl liora_core::LocalesContext,
) -> Vec<NavigationMenuNode> {
    let mut visible = gallery_nav_visible_indices(index, query);
    if visible.is_empty() {
        return vec![NavigationMenuNode::Item(
            liora_components::NavigationMenuItem {
                id: "gallery-nav-empty".into(),
                label: tr(cx, locales::gallery::no_matches),
                icon: None,
            },
        )];
    }

    visible.sort_by(|left, right| {
        let left_entry = &index[*left];
        let right_entry = &index[*right];
        let left_category = category::category_for(left_entry.label.as_ref());
        let right_category = category::category_for(right_entry.label.as_ref());
        left_category
            .order()
            .cmp(&right_category.order())
            .then_with(|| {
                category::component_key(left_entry.label.as_ref())
                    .cmp(category::component_key(right_entry.label.as_ref()))
            })
            .then_with(|| left.cmp(right))
    });

    let mut groups: Vec<NavigationMenuNode> = Vec::new();
    for group_category in category::Category::ALL {
        let children = visible
            .iter()
            .filter_map(|entry_index| {
                let entry = index.get(*entry_index)?;
                (category::category_for(entry.label.as_ref()) == *group_category).then(|| {
                    NavigationMenuNode::Item(liora_components::NavigationMenuItem {
                        id: gallery_nav_item_id(*entry_index),
                        label: entry.label.clone(),
                        icon: None,
                    })
                })
            })
            .collect::<Vec<_>>();

        if !children.is_empty() {
            groups.push(NavigationMenuNode::Group(
                liora_components::NavigationMenuGroup {
                    title: gallery_category_name(*group_category, cx).into(),
                    children,
                },
            ));
        }
    }

    groups
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

    fn gallery_test_locales() -> liora_core::LocalesConfig {
        liora_core::LocalesConfig::default()
            .try_with_locales_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/locales"))
            .expect("gallery test locales should load")
    }

    fn sample_nav_entries() -> Vec<GalleryNavEntry> {
        vec![
            GalleryNavEntry {
                label: "Shell".into(),
                search_text: "shell frame window".into(),
            },
            GalleryNavEntry {
                label: "Input".into(),
                search_text: "input search form".into(),
            },
            GalleryNavEntry {
                label: "About / 关于".into(),
                search_text: "about 关于 gallery".into(),
            },
            GalleryNavEntry {
                label: "Dialog".into(),
                search_text: "dialog frame window".into(),
            },
        ]
    }

    fn menu_group_labels(group: &liora_components::NavigationMenuGroup) -> Vec<&str> {
        group
            .children
            .iter()
            .map(|node| match node {
                liora_components::NavigationMenuNode::Item(item) => item.label.as_ref(),
                _ => panic!("Gallery nav groups should contain leaf items"),
            })
            .collect()
    }

    fn menu_group_ids(group: &liora_components::NavigationMenuGroup) -> Vec<&str> {
        group
            .children
            .iter()
            .map(|node| match node {
                liora_components::NavigationMenuNode::Item(item) => item.id.as_ref(),
                _ => panic!("Gallery nav groups should contain leaf items"),
            })
            .collect()
    }

    #[test]
    fn gallery_nav_visible_indices_preserve_original_selection_ids() {
        let entries = sample_nav_entries();

        assert_eq!(gallery_nav_visible_indices(&entries, ""), vec![0, 1, 2, 3]);
        assert_eq!(gallery_nav_visible_indices(&entries, "input"), vec![1]);
        assert_eq!(gallery_nav_visible_indices(&entries, "关于"), vec![2]);
        assert!(gallery_nav_visible_indices(&entries, "missing").is_empty());
        assert_eq!(gallery_nav_item_id(12).as_ref(), "gallery-nav-12");
        assert_eq!(gallery_nav_index_from_id("gallery-nav-12"), Some(12));
        assert_eq!(gallery_nav_index_from_id("gallery-nav-empty"), None);
    }

    #[test]
    fn gallery_nav_menu_items_group_by_component_category_then_label() {
        let entries = sample_nav_entries();
        let locales = gallery_test_locales();
        let items = gallery_nav_menu_items(&entries, "", &locales);

        assert_eq!(items.len(), 3);

        let liora_components::NavigationMenuNode::Group(about_group) = &items[0] else {
            panic!("About should be the first standalone Gallery nav group");
        };
        assert_eq!(about_group.title.as_ref(), "About");
        assert_eq!(menu_group_labels(about_group), vec!["About / 关于"]);

        let liora_components::NavigationMenuNode::Group(window_group) = &items[1] else {
            panic!("Gallery nav should group visible menu items by component category");
        };
        assert_eq!(window_group.title.as_ref(), "Window / Layout");
        assert_eq!(menu_group_labels(window_group), vec!["Dialog", "Shell"]);

        let liora_components::NavigationMenuNode::Group(control_group) = &items[2] else {
            panic!("Controls should be grouped after window layout entries");
        };
        assert_eq!(control_group.title.as_ref(), "Controls");
        assert_eq!(menu_group_labels(control_group), vec!["Input"]);
        assert_eq!(menu_group_ids(control_group), vec!["gallery-nav-1"]);

        let empty_items = gallery_nav_menu_items(&entries, "missing", &locales);
        let liora_components::NavigationMenuNode::Item(empty_item) = &empty_items[0] else {
            panic!("Gallery nav empty state should still be a Menu item");
        };
        assert_eq!(empty_item.id.as_ref(), "gallery-nav-empty");
    }

    #[test]
    fn gallery_nav_filter_keeps_matching_groups_only() {
        let entries = sample_nav_entries();
        let locales = gallery_test_locales();
        let items = gallery_nav_menu_items(&entries, "input", &locales);

        assert_eq!(items.len(), 1);
        let liora_components::NavigationMenuNode::Group(control_group) = &items[0] else {
            panic!("Filtered Gallery nav should keep category headings for matching items");
        };
        assert_eq!(control_group.title.as_ref(), "Controls");
        assert_eq!(menu_group_labels(control_group), vec!["Input"]);
        assert_eq!(menu_group_ids(control_group), vec!["gallery-nav-1"]);
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
        assert!(source.contains("nav_menu: Option<gpui::Entity<NavigationMenu>>"));
        assert!(source.contains(r#".id("gallery-nav-menu")"#));
        assert!(source.contains(".mode(NavigationMenuMode::Vertical)"));
        assert!(source.contains(".with_items(items)"));
        assert!(source.contains(".on_select(move |id"));
        assert!(source.contains("menu.set_items(items, cx);"));
        assert!(source.contains("fn refresh_nav_menu_for_current_query"));
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
            .rsplit("let shell = Container::new()")
            .next()
            .expect("Gallery shell should exist")
            .split("AppWindowFrame::new")
            .next()
            .expect("shell should end before app frame");
        assert!(shell.contains("Sidebar::new()"));
        assert!(shell.contains(r#".id("gallery-sidebar")"#));
        assert!(shell.contains(".expanded_width(px(280.0))"));
        assert!(shell.contains(".scrollable()"));
        assert!(shell.contains(".child(nav_menu)"));
        assert!(shell.contains(".aside_passthrough()"));
        assert!(!shell.contains(r#".id("gallery-nav-scroll")"#));
        assert!(!shell.contains(".track_scroll(&self.nav_scroll)"));
        assert!(!source.contains("gallery.refresh_nav_menu_for_query(query, cx);"));
        assert!(!source.contains(
            "move |_, cx| {
                    let _ = gallery.update(cx, |_gallery, cx| {
                        cx.notify();"
        ));
    }

    #[test]
    fn gallery_frame_mode_switch_reopens_when_gpui_requires_creation_time_titlebar_options() {
        let source = include_str!("main.rs")
            .split("mod shell_regression_tests")
            .next()
            .unwrap();
        let handler = source
            .split("fn set_gallery_frame_mode")
            .nth(1)
            .expect("Gallery frame mode handler should exist")
            .split("fn request_gallery_window_close")
            .next()
            .expect("Gallery frame mode handler should end before close handler");

        assert!(handler.contains("request_window_frame_mode"));
        assert!(handler.contains("WindowFrameChange::RequiresWindowReopen"));
        assert!(handler.contains("window.defer(cx"));
        assert!(handler.contains("window.remove_window()"));
        assert!(handler.contains("open_gallery_window(cx)"));
    }

    #[test]
    fn gallery_nav_menu_does_not_rebuild_filtered_items_during_parent_render() {
        let source = include_str!("main.rs");
        let render_update = source
            .split("if let Some(nav_menu) = &self.nav_menu")
            .nth(1)
            .and_then(|part| part.split("return nav_menu.clone();").next())
            .unwrap();

        assert!(!render_update.contains("gallery_nav_menu_items"));
        assert!(render_update.contains("menu.set_active_index(active_id, cx);"));
    }

    #[test]
    fn gallery_nav_menu_uses_real_layout_tree_for_scrollable_menu() {
        let source = include_str!("main.rs");
        let render = source
            .split("impl Render for Gallery")
            .nth(1)
            .expect("Gallery render impl should exist")
            .split("impl Gallery")
            .next()
            .expect("Gallery render impl should end before inherent impl");

        assert!(render.contains("let nav_menu = self.gallery_nav_menu(selected, cx);"));
        assert!(!render.contains("AnyView::from(self.gallery_nav_menu(selected, cx))"));
        assert!(!render.contains(".cached("));
    }

    #[test]
    fn gallery_defaults_to_component_overview_instead_of_button_demo() {
        let source = include_str!("main.rs")
            .split("mod shell_regression_tests")
            .next()
            .unwrap();

        assert!(
            source.contains("fn default_gallery_selection"),
            "Gallery should centralize its startup selection instead of relying on registry order"
        );
        assert!(
            source.contains("let selected = default_gallery_selection(&entries);")
                && source.contains("selected,"),
            "Gallery startup should use the explicit default selection"
        );
        assert!(
            !source.contains("selected: 0,"),
            "Gallery should not default to the first registry item because that currently selects Button"
        );
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
    fn gallery_header_embeds_fallback_menu_bar_for_all_frame_modes() {
        let source = include_str!("main.rs")
            .split("mod shell_regression_tests")
            .next()
            .unwrap();

        assert!(source.contains(".header(header)"));
        assert!(source.contains(".child(gallery_fallback_menu_bar(cx))"));
        assert!(source.contains(
            "fn gallery_fallback_menu_bar(cx: &impl liora_core::LocalesContext) -> MenuBar"
        ));
        assert!(source.contains("MenuBar::new(["));
        assert!(source.contains(".titlebar(gallery_titlebar())"));
        assert!(!source.contains(".leading(gallery_window_menu_bar())"));
    }

    #[test]
    fn gallery_registers_gpui_system_menus_on_startup() {
        let source = include_str!("main.rs")
            .split("mod shell_regression_tests")
            .next()
            .unwrap();

        assert!(source.contains("register_gallery_system_menus(cx);"));
        assert!(source.contains("fn register_gallery_system_menus(cx: &mut App)"));
        assert!(source.contains("Menu::register("));
        assert!(source.contains("MenuItem::open_file()"));
        assert!(source.contains("MenuItem::open_folder()"));
        assert!(source.contains("MenuItem::select_all()"));
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
        assert!(source.contains("Button::new(locales::gallery::check_updates)"));
        assert!(source.contains("Button::new(locales::gallery::download_update)"));
    }
}

fn main() {
    run_gallery();
}
