use aura_components::{
    AppWindowFrame, Autocomplete, Button, Card, Cascader, Checkbox, CodeBlock, CodeEditor,
    ColorPicker, Container, DatePicker, DateTimePicker, Dialog, Drawer, Input, Menu, MenuMode,
    MessageManager, Paragraph, Popover, Preview, Radio, RadioGroup, Segmented, SegmentedOption,
    Select, Space, Switch, Tag, Text, TimePicker, Title, Tour, WindowFrameMode,
    apply_window_frame_mode, frame_mode_switch_row, toast_info, toast_success,
};
use aura_core::{
    Config, PassivePortal, Portal, ThemeMode, apply_theme_mode, init_aura_with_mode,
    sync_system_theme,
};
use aura_gallery::demos;
use aura_tray::{
    AuraTray, BundledTrayIconSet, BundledTrayIconState, MouseButton, MouseButtonState,
    TrayCloseAction, TrayCommand, TrayConfig, TrayControlCenter, TrayIconEvent, bundled_tray_icon,
    default_aura_tray_menu, solid_icon,
};
use gpui::{
    AnyView, App, Bounds, Component, Context, Global, Render, WeakEntity, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

pub struct Gallery {
    entries: Vec<demos::DemoEntry>,
    demos: Vec<AnyView>,
    selected: usize,
    nav_filter: gpui::Entity<Input>,
    nav_menu: Option<gpui::Entity<aura_components::Menu>>,
    nav_query: String,
    theme_mode: ThemeMode,
    theme_mode_segmented: gpui::Entity<Segmented>,
    frame_mode: WindowFrameMode,
    frame_mode_switch: gpui::Entity<Switch>,
    refresh_revision: u32,
}

struct GalleryTrayState {
    tray: AuraTray,
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
        .with_quit_mode(gpui::QuitMode::Explicit)
        .run(|cx: &mut App| {
            init_aura_with_mode(cx, ThemeMode::System);
            MessageManager::init(cx);

            // Register all key bindings
            Input::register_key_bindings(cx);
            CodeBlock::register_key_bindings(cx);
            CodeEditor::register_key_bindings(cx);
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
            Tour::register_key_bindings(cx);

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
    match cx.open_window(gallery_window_options(frame_mode), |window, cx| {
        let entries = demos::registry();
        let demos = entries.iter().map(|entry| (entry.render)(cx)).collect();
        let view = cx.new(|cx| {
            let theme_mode = cx.global::<Config>().theme_mode;
            Gallery {
                entries,
                demos,
                selected: 0,
                nav_filter: cx.new(|cx| Input::new("", cx).placeholder("搜索组件 / Search demos")),
                nav_menu: None,
                nav_query: String::new(),
                theme_mode,
                theme_mode_segmented: cx.new(move |_| theme_mode_segmented(theme_mode)),
                frame_mode,
                frame_mode_switch: cx.new(|cx| Switch::new(frame_mode.is_custom(), cx)),
                refresh_revision: 0,
            }
        });
        let _ = window.observe_window_appearance(|window, cx| sync_system_theme(window, cx));
        window.on_window_should_close(cx, |window, cx| {
            handle_gallery_window_should_close(window, cx)
        });
        view
    }) {
        Ok(handle) => Some(handle.into()),
        Err(error) => {
            eprintln!("failed to open Aura Gallery window: {error:?}");
            None
        }
    }
}

fn gallery_window_options(frame_mode: WindowFrameMode) -> WindowOptions {
    apply_window_frame_mode(
        WindowOptions {
            window_bounds: Some(WindowBounds::Maximized(Bounds {
                origin: gpui::Point::default(),
                size: size(px(1920.0), px(1080.0)),
            })),
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Aura UI Gallery".into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        frame_mode,
    )
}

fn install_gallery_tray(cx: &mut App) {
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

    let mut config = TrayConfig::new("aura-gallery")
        .tooltip("Aura Gallery")
        .menu(default_aura_tray_menu());
    if let Some(icon) = gallery_tray_icon("default") {
        config = config.icon(icon);
    }

    match AuraTray::install(config) {
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
            eprintln!("failed to install Aura Gallery tray icon: {error}");
            cx.set_quit_mode(gpui::QuitMode::LastWindowClosed);
            return;
        }
    }

    cx.spawn(async move |cx: &mut gpui::AsyncApp| {
        loop {
            aura_tray::pump_platform_events();
            while let Ok(command) = rx.try_recv() {
                cx.update(|cx| handle_gallery_tray_command(command, cx));
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
                show_gallery_close_confirm(cx);
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
                        eprintln!("failed to update Aura Gallery tray icon: {error}");
                    }
                }
                let _ = state.tray.set_tooltip(Some(match name.as_str() {
                    "syncing" => "Aura Gallery · Syncing",
                    "error" => "Aura Gallery · Error",
                    _ => "Aura Gallery",
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
                .has_global::<GalleryTrayState>()
                .then(|| !cx.global::<GalleryTrayState>().tray_visible)
                .unwrap_or(true);
            set_gallery_tray_visible(cx, visible);
        }
        TrayCommand::Custom(name) => {
            eprintln!("Aura Gallery tray custom command: {name}");
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

    cx.set_quit_mode(gpui::QuitMode::Explicit);
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

fn handle_gallery_window_should_close(_window: &mut Window, cx: &mut App) -> bool {
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
                show_gallery_close_confirm(cx);
            }
            false
        }
    }
}

fn show_gallery_close_confirm(cx: &mut App) {
    let remember = Arc::new(AtomicBool::new(false));

    Dialog::new()
        .id("gallery-close-confirm")
        .title("关闭 Aura Gallery？")
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
                    "你可以直接退出进程，或者关闭主窗口并让应用继续驻留在系统托盘。",
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
                            if cx.has_global::<GalleryTrayState>() {
                                cx.global_mut::<GalleryTrayState>().close_dialog_open = false;
                            }
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
                            if cx.has_global::<GalleryTrayState>() {
                                cx.global_mut::<GalleryTrayState>().close_dialog_open = false;
                            }
                            Dialog::close(cx);
                            cx.quit();
                        })),
                )
        })
        .show(cx);
}

fn gallery_tray_icon(name: &str) -> Option<aura_tray::TrayIconImage> {
    match bundled_tray_icon(
        BundledTrayIconSet::Gallery,
        BundledTrayIconState::from_name(name),
    ) {
        Ok(icon) => Some(icon),
        Err(error) => {
            eprintln!(
                "failed to load bundled Aura Gallery tray icon '{name}': {error}; using fallback icon"
            );
            match solid_icon([32, 96, 255, 255], 32) {
                Ok(icon) => Some(icon),
                Err(error) => {
                    eprintln!("failed to create fallback Aura Gallery tray icon: {error}");
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
        assert!(source.contains("Preview::register_key_bindings(cx)"));
        assert!(source.contains("nav_filter"));
        assert!(source.contains("nav_menu: Option"));
        assert!(source.contains("self.nav_menu = Some"));
        assert!(source.contains("frame_mode_switch"));
        assert!(source.contains("AppWindowFrame::new"));
        assert!(source.contains("theme_mode_segmented"));
        assert!(source.contains("ThemeMode::System"));
        assert!(source.contains("observe_window_appearance"));
        assert!(source.contains("Gallery theme switched"));
    }
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected.min(self.entries.len().saturating_sub(1));
        self.selected = selected;

        self.wire_shell_controls(cx);
        let nav_menu = self.gallery_nav_menu(selected, cx);

        let selected_entry = &self.entries[selected];
        let selected_demo = self.demos[selected].clone();

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .gap_4()
            .child(
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(Title::new("Aura UI").h2())
                    .child(Text::new(format!(
                        "Native Component Library · {} demos · rendering one demo at a time",
                        self.entries.len()
                    ))),
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
                            gallery.update(cx, |gallery, cx| {
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

        let content = Card::new(
            Space::new()
                .vertical()
                .gap_lg()
                .child(
                    Space::new()
                        .vertical()
                        .gap_xs()
                        .child(Title::new(selected_entry.name).h3())
                        .child(Paragraph::with_text(selected_entry.description)),
                )
                .child(selected_demo),
        )
        .no_shadow()
        .no_shrink();

        aura_components::message::render_messages(cx);
        aura_components::notification::render_notifications(cx);
        aura_components::image::render_image_preview(_window, cx);
        aura_core::render_active_tooltip_in_window(_window, cx);
        aura_core::render_active_popover_in_window(_window, cx);
        aura_core::render_active_modal_in_window(_window, cx);
        aura_core::render_active_drawer_in_window(_window, cx);

        let shell = Container::new()
            .header(header)
            .header_height_lg()
            .aside(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(self.nav_filter.clone())
                    .child(nav_menu),
            )
            .aside_width_lg()
            .aside_scroll()
            .main_scroll()
            .main_padding_xl()
            .child(content)
            .overlay(PortalLayer);

        AppWindowFrame::new("Aura UI Gallery", shell)
            .subtitle("Native component gallery")
            .mode(self.frame_mode)
            .on_close(request_gallery_window_close)
    }
}

impl Gallery {
    fn gallery_nav_menu(&mut self, selected: usize, cx: &mut Context<Self>) -> gpui::Entity<Menu> {
        let query = self
            .nav_filter
            .read(cx)
            .value()
            .to_string()
            .trim()
            .to_lowercase();

        if self.nav_query == query {
            if let Some(nav_menu) = &self.nav_menu {
                return nav_menu.clone();
            }
        }

        let items = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                query.is_empty()
                    || entry.name.to_lowercase().contains(&query)
                    || entry.description.to_lowercase().contains(&query)
            })
            .map(|(index, entry)| (index, entry.name))
            .collect::<Vec<_>>();

        let gallery = cx.entity().downgrade();
        let nav_menu = cx.new(move |_| build_gallery_menu(items, selected, gallery));
        self.nav_query = query;
        self.nav_menu = Some(nav_menu.clone());
        nav_menu
    }

    fn wire_shell_controls(&self, cx: &mut Context<Self>) {
        let gallery = cx.entity().clone();
        cx.update_entity(&self.nav_filter, |input, _cx| {
            input.set_on_change({
                let gallery = gallery.clone();
                move |_, cx| {
                    let _ = gallery.update(cx, |_gallery, cx| {
                        cx.notify();
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

fn theme_mode_segmented(mode: ThemeMode) -> Segmented {
    Segmented::new(vec![
        SegmentedOption::new("System", ThemeMode::System.value()),
        SegmentedOption::new("Light", ThemeMode::Light.value()),
        SegmentedOption::new("Dark", ThemeMode::Dark.value()),
    ])
    .id("gallery-theme-mode")
    .value(mode.value())
}

fn build_gallery_menu(
    items: Vec<(usize, &'static str)>,
    selected: usize,
    gallery: WeakEntity<Gallery>,
) -> Menu {
    let mut menu = Menu::new()
        .id("gallery-menu")
        .mode(MenuMode::Vertical)
        .default_active(selected.to_string())
        .on_select(move |id, _, cx| {
            let Ok(index) = id.parse::<usize>() else {
                return;
            };
            let _ = gallery.update(cx, |gallery, cx| {
                gallery.selected = index;
                cx.notify();
            });
        });

    if items.is_empty() {
        return menu.item("empty", "无匹配组件", None);
    }

    for (index, name) in items {
        menu = menu.item(index.to_string(), name, None);
    }

    menu
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

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_gallery();
}
#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_gallery();
}
