use gpui::{
    Animation, AnimationExt, App, Bounds, Context, Global, Hsla, Pixels, TextRun, Window,
    WindowAppearance, WindowBounds, prelude::*, px,
};

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static NEXT_UNIQUE_ID: AtomicU64 = AtomicU64::new(1);

/// Generate a process-wide unique, monotonically increasing numeric id.
pub fn next_unique_id() -> u64 {
    NEXT_UNIQUE_ID.fetch_add(1, Ordering::Relaxed)
}

/// Generate a process-wide unique id string with a stable component prefix.
///
/// Important: GPUI interactive state is keyed by `ElementId`, so call this only
/// when constructing a persistent component/entity instance. Do not call it from
/// a per-frame `render` path for a `RenderOnce` component, because that would
/// assign a new ID every frame and break hover/click/portal state.
pub fn unique_id(prefix: &str) -> gpui::SharedString {
    format!("{}-{}", prefix, next_unique_id()).into()
}

/// Return a stable process-wide unique id for the current element path.
///
/// This is safe inside render paths because GPUI stores the generated value in
/// keyed element state and reuses it for the same element across frames. The
/// `key` must itself be stable for the visual element being rendered.
pub fn stable_unique_id(
    key: impl Into<gpui::SharedString>,
    prefix: &str,
    window: &mut Window,
    cx: &mut App,
) -> gpui::SharedString {
    let prefix = prefix.to_string();
    let key = gpui::ElementId::from(key.into());
    window
        .use_keyed_state(key, cx, move |_, _| unique_id(&prefix))
        .read(cx)
        .clone()
}

pub mod popper;

pub use popper::*;

pub use liora_theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemeMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }

    pub fn value(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn from_value(value: &str) -> Option<Self> {
        match value {
            "system" => Some(Self::System),
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }

    pub fn resolve(self, appearance: WindowAppearance) -> Theme {
        match self {
            Self::System => theme_for_window_appearance(appearance),
            Self::Light => Theme::light(),
            Self::Dark => Theme::dark(),
        }
    }

    pub fn from_theme(theme: &Theme) -> Self {
        match theme.name.as_str() {
            "dark" => Self::Dark,
            _ => Self::Light,
        }
    }
}

/// Return startup bounds for a window that should request GPUI maximized state.
///
/// This helper is intentionally limited to stable crates.io GPUI APIs. It
/// preserves the caller's `WindowBounds::Maximized` intent and uses the primary
/// display bounds as the restore/fallback geometry; exact first-frame behavior
/// is decided by the GPUI backend selected by the application root.
pub fn startup_maximized_window_bounds(
    cx: &App,
    fallback_size: gpui::Size<Pixels>,
) -> WindowBounds {
    let bounds = cx
        .primary_display()
        .map(|display| display.bounds())
        .unwrap_or(Bounds {
            origin: gpui::Point::default(),
            size: fallback_size,
        });
    WindowBounds::Maximized(bounds)
}

fn startup_system_appearance(cx: &App) -> WindowAppearance {
    platform_system_appearance().unwrap_or_else(|| cx.window_appearance())
}

fn current_system_appearance(window: &Window, _cx: &App) -> WindowAppearance {
    platform_system_appearance().unwrap_or_else(|| window.appearance())
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn platform_system_appearance() -> Option<WindowAppearance> {
    gtk_theme_env_appearance()
        .or_else(gtk_settings_appearance)
        .or_else(gsettings_color_scheme_appearance)
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn platform_system_appearance() -> Option<WindowAppearance> {
    None
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn gtk_theme_env_appearance() -> Option<WindowAppearance> {
    std::env::var("GTK_THEME")
        .ok()
        .and_then(|theme| appearance_from_theme_name(&theme))
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn gtk_settings_appearance() -> Option<WindowAppearance> {
    ["gtk-4.0", "gtk-3.0"]
        .into_iter()
        .filter_map(|version| {
            std::env::var_os("HOME").map(|home| {
                std::path::PathBuf::from(home)
                    .join(".config")
                    .join(version)
                    .join("settings.ini")
            })
        })
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .find_map(|settings| appearance_from_gtk_settings(&settings))
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn gsettings_color_scheme_appearance() -> Option<WindowAppearance> {
    let output = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8_lossy(&output.stdout);
    appearance_from_color_scheme(&value)
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn appearance_from_color_scheme(value: &str) -> Option<WindowAppearance> {
    let value = value
        .trim()
        .trim_matches('\'')
        .trim_matches('"')
        .to_ascii_lowercase();
    if value.contains("prefer-dark") {
        Some(WindowAppearance::Dark)
    } else if value.contains("prefer-light") || value == "default" {
        Some(WindowAppearance::Light)
    } else {
        None
    }
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn appearance_from_gtk_settings(settings: &str) -> Option<WindowAppearance> {
    for line in settings.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.starts_with(';') || line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key == "gtk-application-prefer-dark-theme" {
            return match value.to_ascii_lowercase().as_str() {
                "true" | "1" => Some(WindowAppearance::Dark),
                "false" | "0" => Some(WindowAppearance::Light),
                _ => None,
            };
        }
        if key == "gtk-theme-name"
            && let Some(appearance) = appearance_from_theme_name(value)
        {
            return Some(appearance);
        }
    }
    None
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn appearance_from_theme_name(theme: &str) -> Option<WindowAppearance> {
    let theme = theme.to_ascii_lowercase();
    if theme.contains("dark") {
        Some(WindowAppearance::Dark)
    } else if theme.contains("light") {
        Some(WindowAppearance::Light)
    } else {
        None
    }
}

pub fn theme_for_window_appearance(appearance: WindowAppearance) -> Theme {
    match appearance {
        WindowAppearance::Light | WindowAppearance::VibrantLight => Theme::light(),
        WindowAppearance::Dark | WindowAppearance::VibrantDark => Theme::dark(),
    }
}

pub struct Config {
    pub theme: Theme,
    pub theme_mode: ThemeMode,
    pub z_index_base: u32,
}

impl Global for Config {}

impl Config {
    pub fn set_theme_mode(&mut self, mode: ThemeMode, appearance: WindowAppearance) {
        self.theme_mode = mode;
        self.theme = mode.resolve(appearance);
    }

    pub fn sync_system_theme(&mut self, appearance: WindowAppearance) -> bool {
        if self.theme_mode != ThemeMode::System {
            return false;
        }
        let theme = ThemeMode::System.resolve(appearance);
        let changed = self.theme.name != theme.name;
        self.theme = theme;
        changed
    }
}

pub fn init_liora(cx: &mut App, theme: Theme) {
    let theme_mode = ThemeMode::from_theme(&theme);
    cx.set_global(Config {
        theme,
        theme_mode,
        z_index_base: 1000,
    });
    cx.set_global(crate::popper::ZIndexStack::default());
    cx.set_global(crate::popper::ActiveTooltip(Vec::new()));
    cx.set_global(crate::popper::ActivePopover(Vec::new()));
    cx.set_global(crate::popper::ActiveModal(Vec::new()));
    cx.set_global(crate::popper::ActiveDrawer(Vec::new()));
}

pub fn init_liora_with_mode(cx: &mut App, mode: ThemeMode) {
    let appearance = startup_system_appearance(cx);
    cx.set_global(Config {
        theme: mode.resolve(appearance),
        theme_mode: mode,
        z_index_base: 1000,
    });
    cx.set_global(crate::popper::ZIndexStack::default());
    cx.set_global(crate::popper::ActiveTooltip(Vec::new()));
    cx.set_global(crate::popper::ActivePopover(Vec::new()));
    cx.set_global(crate::popper::ActiveModal(Vec::new()));
    cx.set_global(crate::popper::ActiveDrawer(Vec::new()));
}

pub fn apply_theme_mode(window: &mut Window, cx: &mut App, mode: ThemeMode) {
    let appearance = current_system_appearance(window, cx);
    cx.global_mut::<Config>().set_theme_mode(mode, appearance);
    window.refresh();
}

pub fn sync_system_theme(window: &mut Window, cx: &mut App) {
    let appearance = current_system_appearance(window, cx);
    if cx.global_mut::<Config>().sync_system_theme(appearance) {
        window.refresh();
    }
}

/// Attach System theme tracking to a concrete GPUI window.
///
/// `init_liora_with_mode(cx, ThemeMode::System)` runs before a window exists and
/// can only use the app-level appearance snapshot. Following Zed's main-window
/// pattern, create the window with `WindowOptions { show: false, .. }`, call this
/// at the start of the `open_window` callback before constructing the root view,
/// then activate the returned window handle after `open_window` completes.
pub fn attach_system_theme_observer(window: &mut Window, cx: &mut App) {
    sync_system_theme(window, cx);
    window
        .observe_window_appearance(|window, cx| sync_system_theme(window, cx))
        .detach();
}

pub fn render_active_popover_in_window(_window: &mut gpui::Window, cx: &mut App) {
    for entry in cx.global::<crate::popper::ActivePopover>().0.clone() {
        push_portal(
            move |_window, _cx| entry.view.clone().into_any_element(),
            cx,
        );
    }
}

pub fn render_active_modal_in_window(_window: &mut gpui::Window, cx: &mut App) {
    for entry in cx.global::<crate::popper::ActiveModal>().0.clone() {
        push_portal(
            move |_window, _cx| entry.view.clone().into_any_element(),
            cx,
        );
    }
}

pub fn render_active_drawer_in_window(_window: &mut gpui::Window, cx: &mut App) {
    for entry in cx.global::<crate::popper::ActiveDrawer>().0.clone() {
        push_portal(
            move |_window, _cx| entry.view.clone().into_any_element(),
            cx,
        );
    }
}

pub fn render_active_tooltip_in_window(window: &mut gpui::Window, cx: &mut App) {
    let mouse_pos = window.mouse_position();
    cx.global_mut::<crate::popper::ActiveTooltip>()
        .0
        .retain(|data| data.anchor_bounds.contains(&mouse_pos));

    let active = cx.global::<crate::popper::ActiveTooltip>().0.clone();
    for (tooltip_index, data) in active.into_iter().enumerate() {
        let theme = cx.global::<Config>().theme.clone();

        // Measure text accurately
        let font_size = px(theme.font_size.sm);
        let text_style = window.text_style();
        let run = TextRun {
            len: data.content.len(),
            font: text_style.font(),
            color: theme.neutral.card,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let shaped_line =
            window
                .text_system()
                .shape_line(data.content.clone(), font_size, &[run], None);

        let padding_h = px(12.0);
        let padding_v = px(4.0);
        let line_height = window.line_height();
        let content_size = gpui::Size {
            width: shaped_line.width + padding_h * 2.0,
            height: line_height + padding_v * 2.0,
        };

        push_passive_portal(
            move |window, _cx| {
                let viewport = Bounds {
                    origin: gpui::Point::default(),
                    size: window.viewport_size(),
                };

                let popper = Popper {
                    anchor_bounds: data.anchor_bounds,
                    placement: data.placement,
                    offset: data.offset,
                };

                let (pos, _final_placement) =
                    popper.calculate_position_with_flip(content_size, viewport);

                gpui::div()
                    .absolute()
                    .cursor_default()
                    .top(pos.y)
                    .left(pos.x)
                    .w(content_size.width)
                    .h(content_size.height)
                    .bg(theme.neutral.text_1)
                    .text_color(theme.neutral.card)
                    .px(padding_h)
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(theme.radius.sm))
                    .shadow_lg()
                    .text_size(font_size)
                    .child(data.content.clone())
                    .with_animation(
                        ("liora-tooltip-motion", tooltip_index),
                        Animation::new(Duration::from_millis(220))
                            .with_easing(gpui::ease_out_quint()),
                        |tooltip, delta| tooltip.opacity(delta),
                    )
                    .into_any_element()
            },
            cx,
        );
    }
}

#[cfg(test)]
mod theme_mode_tests {
    use super::*;

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[test]
    fn linux_startup_appearance_parses_synchronous_dark_preferences() {
        assert_eq!(
            appearance_from_color_scheme("'prefer-dark'"),
            Some(WindowAppearance::Dark)
        );
        assert_eq!(
            appearance_from_color_scheme("prefer-light"),
            Some(WindowAppearance::Light)
        );
        assert_eq!(
            appearance_from_gtk_settings(
                "[Settings]\ngtk-application-prefer-dark-theme=true\ngtk-theme-name=Breeze\n"
            ),
            Some(WindowAppearance::Dark)
        );
        assert_eq!(
            appearance_from_theme_name("Adwaita-dark"),
            Some(WindowAppearance::Dark)
        );
    }

    #[test]
    fn theme_mode_values_and_labels_are_stable() {
        assert_eq!(ThemeMode::System.value(), "system");
        assert_eq!(ThemeMode::Light.label(), "Light");
        assert_eq!(ThemeMode::from_value("dark"), Some(ThemeMode::Dark));
        assert_eq!(ThemeMode::from_theme(&Theme::dark()), ThemeMode::Dark);
        assert_eq!(ThemeMode::from_theme(&Theme::light()), ThemeMode::Light);
        assert_eq!(ThemeMode::from_value("unknown"), None);
    }

    #[test]
    fn system_theme_resolves_from_window_appearance() {
        assert_eq!(
            ThemeMode::System.resolve(WindowAppearance::Light).name,
            Theme::light().name
        );
        assert_eq!(
            ThemeMode::System
                .resolve(WindowAppearance::VibrantDark)
                .name,
            Theme::dark().name
        );
    }

    #[test]
    fn config_syncs_only_in_system_mode() {
        let mut config = Config {
            theme: Theme::light(),
            theme_mode: ThemeMode::Light,
            z_index_base: 1000,
        };
        assert!(!config.sync_system_theme(WindowAppearance::Dark));
        assert_eq!(config.theme.name, "light");

        config.set_theme_mode(ThemeMode::System, WindowAppearance::Dark);
        assert_eq!(config.theme.name, "dark");
        assert!(!config.sync_system_theme(WindowAppearance::VibrantDark));
        assert!(config.sync_system_theme(WindowAppearance::Light));
        assert_eq!(config.theme.name, "light");
    }

    #[test]
    fn system_theme_observer_syncs_immediately_and_stays_attached() {
        let source = include_str!("lib.rs");
        let start = source
            .find("pub fn attach_system_theme_observer")
            .expect("system theme observer helper should exist");
        let body = &source[start
            ..source[start..]
                .find("pub fn render_active_popover_in_window")
                .expect("next function should follow observer helper")
                + start];

        let sync_call = format!("{}(window, cx);", "sync_system_theme");
        let observe_call = format!("{}", "observe_window_appearance");
        let sync_index = body
            .find(&sync_call)
            .expect("observer helper should sync the current window appearance immediately");
        let observe_index = body
            .find(&observe_call)
            .expect("observer helper should observe later appearance changes");
        assert!(sync_index < observe_index);
        assert!(body.contains(".detach();"));
    }
}

#[cfg(test)]
mod motion_tests {
    #[test]
    fn tooltip_rendering_uses_gpui_motion() {
        let source = include_str!("lib.rs").split("#[cfg(test)]").next().unwrap();

        assert!(source.contains("tooltip-motion"));
        assert!(source.contains("with_animation("));
    }
}

pub fn liora_theme<'a, V>(cx: &'a Context<'a, V>) -> &'a Theme {
    &cx.global::<Config>().theme
}

pub trait ContextExt {
    fn liora(&self) -> &Theme;
}

impl<'a, V> ContextExt for Context<'a, V> {
    fn liora(&self) -> &Theme {
        liora_theme(self)
    }
}

pub trait ElementExt {
    fn liora(self, cx: &mut App) -> Self;
}

impl ElementExt for gpui::Div {
    fn liora(self, _cx: &mut App) -> Self {
        self
    }
}

pub fn z_index_popup<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 100
}

pub fn z_index_modal<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 200
}

pub fn z_index_notification<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 300
}

pub fn z_index_tooltip<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 400
}

pub fn hex_color(hex: u32) -> Hsla {
    gpui::rgb(hex).into()
}

#[cfg(test)]
mod unique_id_tests {
    use super::*;

    #[test]
    fn generated_ids_are_prefixed_and_unique() {
        let first = unique_id("component");
        let second = unique_id("component");

        assert!(first.as_ref().starts_with("component-"));
        assert!(second.as_ref().starts_with("component-"));
        assert_ne!(first, second);
    }
}
