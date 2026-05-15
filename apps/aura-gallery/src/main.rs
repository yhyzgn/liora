use aura_components::{
    Autocomplete, Button, Card, Cascader, Checkbox, CodeBlock, ColorPicker, Container, DatePicker,
    DateTimePicker, Dialog, Drawer, Input, Menu, MenuMode, MessageManager, Paragraph, Popover,
    Preview, Radio, RadioGroup, Select, Space, Switch, Text, TimePicker, Title,
};
use aura_core::{PassivePortal, Portal, init_aura};
use aura_gallery::demos;
use aura_theme::Theme;
use aura_tray::{
    AuraTray, BundledTrayIconSet, BundledTrayIconState, MouseButton, MouseButtonState,
    TrayCloseAction, TrayCommand, TrayConfig, TrayControlCenter, TrayIconEvent, bundled_tray_icon,
    default_aura_tray_menu,
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
    nav_menu: Option<gpui::Entity<aura_components::Menu>>,
}

struct GalleryTrayState {
    tray: AuraTray,
    window: Option<gpui::AnyWindowHandle>,
    window_visible: bool,
    resident_enabled: bool,
    tray_visible: bool,
    auto_show: bool,
    close_dialog_open: bool,
}

impl Global for GalleryTrayState {}

fn run_gallery() {
    gpui_platform::application()
        .with_quit_mode(gpui::QuitMode::Explicit)
        .run(|cx: &mut App| {
            init_aura(cx, Theme::light());
            MessageManager::init(cx);

            // Register all key bindings
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

            install_gallery_tray(cx);
            if let Some(handle) = open_gallery_window(cx) {
                if cx.has_global::<GalleryTrayState>() {
                    cx.global_mut::<GalleryTrayState>().window = Some(handle);
                }
            }
        });
}

fn open_gallery_window(cx: &mut App) -> Option<gpui::AnyWindowHandle> {
    match cx.open_window(gallery_window_options(), |window, cx| {
        let entries = demos::registry();
        let demos = entries.iter().map(|entry| (entry.render)(cx)).collect();
        let view = cx.new(|_| Gallery {
            entries,
            demos,
            selected: 0,
            nav_menu: None,
        });
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

fn gallery_window_options() -> WindowOptions {
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
    }
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

    match AuraTray::install(
        TrayConfig::new("aura-gallery")
            .tooltip("Aura Gallery")
            .icon(gallery_tray_icon("default"))
            .menu(default_aura_tray_menu()),
    ) {
        Ok(tray) => {
            cx.set_global(GalleryTrayState {
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

fn handle_gallery_tray_command(command: TrayCommand, cx: &mut App) {
    match command {
        TrayCommand::Show => show_gallery_window(cx),
        TrayCommand::Hide => hide_gallery_window(cx),
        TrayCommand::Toggle => toggle_gallery_window(cx),
        TrayCommand::Quit => cx.quit(),
        TrayCommand::SetIcon(name) => {
            if cx.has_global::<GalleryTrayState>() {
                let state = cx.global_mut::<GalleryTrayState>();
                if let Err(error) = state.tray.set_icon(gallery_tray_icon(&name)) {
                    eprintln!("failed to update Aura Gallery tray icon: {error}");
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

fn hide_gallery_window(cx: &mut App) {
    if !cx.has_global::<GalleryTrayState>() {
        return;
    }

    cx.set_quit_mode(gpui::QuitMode::Explicit);
    set_gallery_tray_visible(cx, true);

    let existing = cx.global::<GalleryTrayState>().window;
    if let Some(handle) = existing {
        let _ = handle.update(cx, |_, window, _| window.remove_window());
    }
    let state = cx.global_mut::<GalleryTrayState>();
    state.window_visible = false;
    state.window = None;
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
            hide_gallery_window(cx);
            false
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
                        .child(Button::new("隐藏到托盘").on_click(move |_, _, cx| {
                            if remember_for_hide.load(Ordering::Relaxed)
                                && cx.has_global::<TrayControlCenter>()
                            {
                                cx.global_mut::<TrayControlCenter>()
                                    .set_remembered_close_action(TrayCloseAction::HideToTray);
                            }
                            if cx.has_global::<GalleryTrayState>() {
                                cx.global_mut::<GalleryTrayState>().close_dialog_open = false;
                            }
                            Dialog::close(cx);
                            hide_gallery_window(cx);
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

fn gallery_tray_icon(name: &str) -> aura_tray::TrayIconImage {
    bundled_tray_icon(
        BundledTrayIconSet::Gallery,
        BundledTrayIconState::from_name(name),
    )
    .expect("bundled gallery tray icon should be valid")
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
        assert!(!source.contains(&format!("gallery-demo{}nav-", "-")));
    }
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected.min(self.entries.len().saturating_sub(1));
        self.selected = selected;

        let nav_menu = self.gallery_nav_menu(selected, cx);

        let selected_entry = &self.entries[selected];
        let selected_demo = self.demos[selected].clone();

        let header = Space::new()
            .vertical()
            .gap_xs()
            .child(Title::new("Aura UI").h2())
            .child(Text::new(format!(
                "Native Component Library · {} demos · rendering one demo at a time",
                self.entries.len()
            )));

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

        Container::new()
            .header(header)
            .header_height_lg()
            .aside(nav_menu)
            .aside_width_lg()
            .aside_scroll()
            .main_scroll()
            .main_padding_xl()
            .child(content)
            .overlay(PortalLayer)
    }
}

impl Gallery {
    fn gallery_nav_menu(&mut self, selected: usize, cx: &mut Context<Self>) -> gpui::Entity<Menu> {
        if let Some(nav_menu) = &self.nav_menu {
            return nav_menu.clone();
        }

        let gallery = cx.entity().downgrade();
        let items = self
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| (index, entry.name))
            .collect::<Vec<_>>();
        let nav_menu = cx.new(move |_| build_gallery_menu(items, selected, gallery));
        self.nav_menu = Some(nav_menu.clone());
        nav_menu
    }
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
