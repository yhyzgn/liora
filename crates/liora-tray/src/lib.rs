//! Native system tray facade for Liora applications.
//!
//! `liora-tray` intentionally keeps `tray-icon` and `muda` behind a small Liora
//! command API so GPUI apps can stay focused on window lifecycle commands while
//! still supporting dynamic icon updates, check items, and deep nested menus.

use std::{collections::HashMap, path::Path, sync::mpsc};

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use std::sync::Once;

use gpui::Global;
pub use tray_icon::menu::{CheckMenuItem, MenuEvent, MenuId};
use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
pub use tray_icon::{Icon as TrayIconImage, MouseButton, MouseButtonState, TrayIconEvent};
use tray_icon::{TrayIcon, TrayIconBuilder};

#[derive(Debug, thiserror::Error)]
/// Errors raised while creating or updating platform tray integrations.
pub enum LioraTrayError {
    #[error("tray icon error: {0}")]
    /// Reports a tray failure.
    Tray(#[from] tray_icon::Error),
    #[error("bad tray icon: {0}")]
    /// Reports a bad icon failure.
    BadIcon(#[from] tray_icon::BadIcon),
    #[error("menu error: {0}")]
    /// Reports a menu failure.
    Menu(#[from] tray_icon::menu::Error),
    #[error("image error: {0}")]
    /// Reports a image failure.
    Image(#[from] image::ImageError),
    #[error("failed to initialize tray platform runtime: {0}")]
    /// Reports a platform init failure.
    PlatformInit(String),
    #[error("invalid rgba icon buffer {width}x{height}: expected {expected} bytes, got {actual}")]
    /// Reports a invalid rgba failure.
    InvalidRgba {
        /// Icon width in pixels.
        width: u32,
        /// Icon height in pixels.
        height: u32,
        /// Required number of RGBA bytes for the provided dimensions.
        expected: usize,
        /// Actual number of bytes provided by the caller.
        actual: usize,
    },
}

/// Type alias for result values used by the liora tray API.
pub type Result<T> = std::result::Result<T, LioraTrayError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Close-window policy used by resident tray applications.
pub enum TrayCloseAction {
    /// Handles the ask tray case.
    Ask,
    /// Handles the exit process tray case.
    ExitProcess,
    /// Handles the hide to tray tray case.
    HideToTray,
}

/// Creates a tray icon from encoded PNG bytes.
pub fn icon_from_png_bytes(bytes: &[u8]) -> Result<TrayIconImage> {
    let image = image::load_from_memory(bytes)?.into_rgba8();
    let (width, height) = image.dimensions();
    icon_from_rgba(image.into_raw(), width, height)
}

#[derive(Debug, Clone)]
/// Mutable state model for resident tray behavior.
pub struct TrayControlState {
    /// Icon that should currently be shown in the system tray.
    pub active_icon: String,
    /// Whether the application should continue running after closing its window.
    pub resident_enabled: bool,
    /// Whether a tray icon is currently installed and visible.
    pub tray_visible: bool,
    /// Whether the application should automatically show the window for tray events.
    pub auto_show: bool,
    /// Last close behavior chosen by the user or host application.
    pub remembered_close_action: TrayCloseAction,
}

impl Default for TrayControlState {
    fn default() -> Self {
        Self {
            active_icon: "default".into(),
            resident_enabled: true,
            tray_visible: true,
            auto_show: true,
            remembered_close_action: TrayCloseAction::Ask,
        }
    }
}

/// Command dispatcher for tray lifecycle and close behavior decisions.
pub struct TrayControlCenter {
    sender: mpsc::Sender<TrayCommand>,
    /// Mutable tray control state shared by the host application.
    pub state: TrayControlState,
}

impl Global for TrayControlCenter {}

impl TrayControlCenter {
    /// Creates a new value with the required baseline configuration.
    pub fn new(sender: mpsc::Sender<TrayCommand>) -> Self {
        Self {
            sender,
            state: TrayControlState::default(),
        }
    }

    /// Applies a tray command to the mutable control state and returns the requested close behavior.
    pub fn dispatch(&self, command: TrayCommand) {
        let _ = self.sender.send(command);
    }

    /// Updates the stored active icon value and keeps the existing component identity.
    pub fn set_active_icon(&mut self, name: impl Into<String>) {
        self.state.active_icon = name.into();
    }

    /// Updates the stored resident enabled value and keeps the existing component identity.
    pub fn set_resident_enabled(&mut self, enabled: bool) {
        self.state.resident_enabled = enabled;
        if !enabled {
            self.state.tray_visible = false;
        }
    }

    /// Updates the stored tray visible value and keeps the existing component identity.
    pub fn set_tray_visible(&mut self, visible: bool) {
        self.state.tray_visible = visible;
        if visible {
            self.state.resident_enabled = true;
        }
    }

    /// Updates the stored auto show value and keeps the existing component identity.
    pub fn set_auto_show(&mut self, enabled: bool) {
        self.state.auto_show = enabled;
    }

    /// Updates the stored remembered close action value and keeps the existing component identity.
    pub fn set_remembered_close_action(&mut self, action: TrayCloseAction) {
        self.state.remembered_close_action = action;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// High-level command emitted by tray menu items and events.
pub enum TrayCommand {
    /// Emits the show tray command.
    Show,
    /// Emits the hide tray command.
    Hide,
    /// Emits the toggle tray command.
    Toggle,
    /// Emits the quit tray command.
    Quit,
    /// Emits the set icon tray command.
    SetIcon(String),
    /// Emits the custom tray command.
    Custom(String),
}

impl TrayCommand {
    /// Returns the stable tray command identifier used for menu event routing.
    pub fn id(&self) -> String {
        match self {
            Self::Show => "show".into(),
            Self::Hide => "hide".into(),
            Self::Toggle => "toggle".into(),
            Self::Quit => "quit".into(),
            Self::SetIcon(name) => format!("set-icon:{name}"),
            Self::Custom(name) => format!("custom:{name}"),
        }
    }

    /// Creates this value from id.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "show" => Some(Self::Show),
            "hide" => Some(Self::Hide),
            "toggle" => Some(Self::Toggle),
            "quit" => Some(Self::Quit),
            _ => id
                .strip_prefix("set-icon:")
                .map(|name| Self::SetIcon(name.to_string()))
                .or_else(|| {
                    id.strip_prefix("custom:")
                        .map(|name| Self::Custom(name.to_string()))
                }),
        }
    }
}

#[derive(Clone)]
/// Declarative tray menu item tree consumed by platform tray backends.
pub enum TrayMenuItemSpec {
    /// Handles the action tray case.
    Action {
        /// Label displayed for the tray menu item.
        label: String,
        /// Command emitted when the tray item is activated.
        command: TrayCommand,
        /// Whether the tray menu item is enabled.
        enabled: bool,
    },
    /// Handles the check tray case.
    Check {
        /// Label displayed for the tray menu item.
        label: String,
        /// Command emitted when the tray item is activated.
        command: TrayCommand,
        /// Whether the checkable tray item is currently checked.
        checked: bool,
        /// Whether the tray menu item is enabled.
        enabled: bool,
    },
    /// Handles the submenu tray case.
    Submenu {
        /// Label displayed for the tray menu item.
        label: String,
        /// Whether the tray menu item is enabled.
        enabled: bool,
        /// Nested menu items displayed under this submenu.
        children: Vec<TrayMenuItemSpec>,
    },
    /// Handles the separator tray case.
    Separator,
}

impl TrayMenuItemSpec {
    /// Creates a tray menu action item specification.
    pub fn action(label: impl Into<String>, command: TrayCommand) -> Self {
        Self::Action {
            label: label.into(),
            command,
            enabled: true,
        }
    }

    /// Creates a tray menu check item specification.
    pub fn check(label: impl Into<String>, command: TrayCommand, checked: bool) -> Self {
        Self::Check {
            label: label.into(),
            command,
            checked,
            enabled: true,
        }
    }

    /// Creates a tray menu submenu item specification.
    pub fn submenu(label: impl Into<String>, children: Vec<TrayMenuItemSpec>) -> Self {
        Self::Submenu {
            label: label.into(),
            enabled: true,
            children,
        }
    }

    /// Creates a tray menu separator item specification.
    pub fn separator() -> Self {
        Self::Separator
    }
}

#[derive(Clone)]
/// Platform tray configuration supplied by the host application.
pub struct TrayConfig {
    /// Stable identifier used to connect rendered UI, callbacks, and external state.
    pub id: String,
    /// Tooltip text shown by the operating-system tray area.
    pub tooltip: Option<String>,
    /// Optional icon rendered with the item.
    pub icon: Option<TrayIconImage>,
    /// Whether the tray icon should be treated as a platform template image.
    pub icon_is_template: bool,
    /// Whether a left-click should open the tray menu.
    pub menu_on_left_click: bool,
    /// Whether a right-click should open the tray menu.
    pub menu_on_right_click: bool,
    /// Menu tree rendered by the tray backend.
    pub menu: Vec<TrayMenuItemSpec>,
}

impl TrayConfig {
    /// Creates a new value with the required baseline configuration.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tooltip: None,
            icon: None,
            icon_is_template: false,
            menu_on_left_click: true,
            menu_on_right_click: true,
            menu: Vec::new(),
        }
    }

    /// Sets the tray tooltip configuration value.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Sets the tray icon configuration value.
    pub fn icon(mut self, icon: TrayIconImage) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets whether the tray icon should be treated as a platform template image.
    pub fn icon_is_template(mut self, is_template: bool) -> Self {
        self.icon_is_template = is_template;
        self
    }

    /// Sets the tray menu configuration value.
    pub fn menu(mut self, menu: Vec<TrayMenuItemSpec>) -> Self {
        self.menu = menu;
        self
    }
}

/// Installed tray integration handle owned by the host application.
pub struct LioraTray {
    tray: TrayIcon,
    command_by_id: HashMap<String, TrayCommand>,
    check_by_id: HashMap<String, CheckMenuItem>,
}

impl LioraTray {
    /// Executes installer-style update actions and returns the process exit status.
    pub fn install(config: TrayConfig) -> Result<Self> {
        init_platform_tray_runtime()?;
        let (menu, command_by_id, check_by_id) = build_menu(&config.menu)?;
        let mut builder = TrayIconBuilder::new()
            .with_id(config.id)
            .with_menu(Box::new(menu))
            .with_menu_on_left_click(config.menu_on_left_click)
            .with_menu_on_right_click(config.menu_on_right_click)
            .with_icon_as_template(config.icon_is_template);

        if let Some(icon) = config.icon {
            builder = builder.with_icon(icon);
        }
        if let Some(tooltip) = config.tooltip {
            builder = builder.with_tooltip(tooltip);
        }

        Ok(Self {
            tray: builder.build()?,
            command_by_id,
            check_by_id,
        })
    }

    /// Finds the tray command associated with a menu item identifier.
    pub fn command_for_menu_id(&self, id: &MenuId) -> Option<&TrayCommand> {
        self.command_by_id.get(&id.0)
    }

    /// Maps a raw tray event into a high-level tray command when possible.
    pub fn command_for_event(&self, event: &MenuEvent) -> Option<&TrayCommand> {
        self.command_for_menu_id(event.id())
    }

    /// Updates the stored icon value and keeps the existing component identity.
    pub fn set_icon(&self, icon: TrayIconImage) -> Result<()> {
        self.tray.set_icon(Some(icon))?;
        Ok(())
    }

    /// Clears the active platform tray icon from the installed tray handle.
    pub fn clear_icon(&self) -> Result<()> {
        self.tray.set_icon(None)?;
        Ok(())
    }

    /// Updates the stored icon from rgba value and keeps the existing component identity.
    pub fn set_icon_from_rgba(&self, rgba: Vec<u8>, width: u32, height: u32) -> Result<()> {
        self.set_icon(icon_from_rgba(rgba, width, height)?)
    }

    /// Updates the stored icon from path value and keeps the existing component identity.
    pub fn set_icon_from_path(&self, path: impl AsRef<Path>) -> Result<()> {
        self.set_icon(icon_from_path(path)?)
    }

    /// Updates the stored tooltip value and keeps the existing component identity.
    pub fn set_tooltip(&self, tooltip: Option<&str>) -> Result<()> {
        self.tray.set_tooltip(tooltip)?;
        Ok(())
    }

    /// Updates the stored visible value and keeps the existing component identity.
    pub fn set_visible(&self, visible: bool) -> Result<()> {
        self.tray.set_visible(visible)?;
        Ok(())
    }

    /// Updates the stored check state value and keeps the existing component identity.
    pub fn set_check_state(&self, command: &TrayCommand, checked: bool) -> bool {
        let id = command.id();
        if let Some(item) = self.check_by_id.get(&id) {
            item.set_checked(checked);
            true
        } else {
            false
        }
    }

    /// Returns whether checked is currently true for this value.
    pub fn is_checked(&self, command: &TrayCommand) -> Option<bool> {
        self.check_by_id
            .get(&command.id())
            .map(CheckMenuItem::is_checked)
    }
}

/// Pumps platform tray events when the backend requires an external event loop.
///
/// On Linux/FreeBSD `tray-icon` uses GTK/AppIndicator. GPUI's own event loop
/// does not drive GTK, so tray-enabled GPUI apps should call this periodically
/// on the same thread that created the tray icon.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub fn pump_platform_events() {
    while gtk::events_pending() {
        gtk::main_iteration_do(false);
    }
}

/// No-op on platforms where `tray-icon` is integrated with the native app loop.
#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
pub fn pump_platform_events() {}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
static AYATANA_LOG_HANDLER: Once = Once::new();

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn install_ayatana_deprecation_filter() {
    AYATANA_LOG_HANDLER.call_once(|| {
        gtk::glib::log_set_handler(
            Some("libayatana-appindicator"),
            gtk::glib::LogLevels::LEVEL_WARNING,
            false,
            false,
            |domain, _level, message| {
                if message.contains("libayatana-appindicator is deprecated")
                    && message.contains("libayatana-appindicator-glib")
                {
                    return;
                }

                eprintln!(
                    "{}-WARNING **: {}",
                    domain.unwrap_or("libayatana-appindicator"),
                    message
                );
            },
        );
    });
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn init_platform_tray_runtime() -> Result<()> {
    install_ayatana_deprecation_filter();

    if gtk::is_initialized() {
        return Ok(());
    }

    gtk::init().map_err(|error| LioraTrayError::PlatformInit(error.to_string()))
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn init_platform_tray_runtime() -> Result<()> {
    Ok(())
}

/// Creates a tray icon from raw RGBA pixel data.
pub fn icon_from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<TrayIconImage> {
    let expected = width as usize * height as usize * 4;
    let actual = rgba.len();
    if actual != expected {
        return Err(LioraTrayError::InvalidRgba {
            width,
            height,
            expected,
            actual,
        });
    }
    Ok(TrayIconImage::from_rgba(rgba, width, height)?)
}

/// Loads a tray icon from an image path on disk.
pub fn icon_from_path(path: impl AsRef<Path>) -> Result<TrayIconImage> {
    let image = image::open(path)?.into_rgba8();
    let (width, height) = image.dimensions();
    icon_from_rgba(image.into_raw(), width, height)
}

/// Creates a solid-color RGBA tray icon for tests and simple placeholders.
pub fn solid_icon(color: [u8; 4], size: u32) -> Result<TrayIconImage> {
    let mut rgba = Vec::with_capacity(size as usize * size as usize * 4);
    for _ in 0..size * size {
        rgba.extend_from_slice(&color);
    }
    icon_from_rgba(rgba, size, size)
}

/// Configures the default liora tray menu used before user interaction changes state.
pub fn default_liora_tray_menu() -> Vec<TrayMenuItemSpec> {
    vec![
        TrayMenuItemSpec::action("显示窗口", TrayCommand::Show),
        TrayMenuItemSpec::action("隐藏窗口", TrayCommand::Hide),
        TrayMenuItemSpec::check(
            "状态栏驻留",
            TrayCommand::Custom("resident-enabled".into()),
            true,
        ),
        TrayMenuItemSpec::check(
            "启动时自动显示",
            TrayCommand::Custom("auto-show".into()),
            true,
        ),
        TrayMenuItemSpec::separator(),
        TrayMenuItemSpec::submenu(
            "切换图标",
            vec![
                TrayMenuItemSpec::action("默认图标", TrayCommand::SetIcon("default".into())),
                TrayMenuItemSpec::action("同步中", TrayCommand::SetIcon("syncing".into())),
                TrayMenuItemSpec::action("错误状态", TrayCommand::SetIcon("error".into())),
            ],
        ),
        TrayMenuItemSpec::submenu(
            "多级菜单",
            vec![TrayMenuItemSpec::submenu(
                "二级菜单",
                vec![TrayMenuItemSpec::submenu(
                    "三级菜单",
                    vec![TrayMenuItemSpec::action(
                        "三级动作",
                        TrayCommand::Custom("deep-action".into()),
                    )],
                )],
            )],
        ),
        TrayMenuItemSpec::separator(),
        TrayMenuItemSpec::action("退出", TrayCommand::Quit),
    ]
}

fn build_menu(
    specs: &[TrayMenuItemSpec],
) -> Result<(
    Menu,
    HashMap<String, TrayCommand>,
    HashMap<String, CheckMenuItem>,
)> {
    let menu = Menu::new();
    let mut commands = HashMap::new();
    let mut checks = HashMap::new();
    append_specs_to_menu(&menu, specs, &mut commands, &mut checks)?;
    Ok((menu, commands, checks))
}

fn append_specs_to_menu(
    menu: &Menu,
    specs: &[TrayMenuItemSpec],
    commands: &mut HashMap<String, TrayCommand>,
    checks: &mut HashMap<String, CheckMenuItem>,
) -> Result<()> {
    for spec in specs {
        match spec {
            TrayMenuItemSpec::Action {
                label,
                command,
                enabled,
            } => {
                let id = command.id();
                let item = MenuItem::with_id(id.clone(), label, *enabled, None);
                commands.insert(id, command.clone());
                menu.append(&item)?;
            }
            TrayMenuItemSpec::Check {
                label,
                command,
                checked,
                enabled,
            } => {
                let id = command.id();
                let item = CheckMenuItem::with_id(id.clone(), label, *enabled, *checked, None);
                commands.insert(id.clone(), command.clone());
                checks.insert(id, item.clone());
                menu.append(&item)?;
            }
            TrayMenuItemSpec::Submenu {
                label,
                enabled,
                children,
            } => {
                let submenu = build_submenu(label, *enabled, children, commands, checks)?;
                menu.append(&submenu)?;
            }
            TrayMenuItemSpec::Separator => {
                menu.append(&PredefinedMenuItem::separator())?;
            }
        }
    }
    Ok(())
}

fn build_submenu(
    label: &str,
    enabled: bool,
    specs: &[TrayMenuItemSpec],
    commands: &mut HashMap<String, TrayCommand>,
    checks: &mut HashMap<String, CheckMenuItem>,
) -> Result<Submenu> {
    let submenu = Submenu::new(label, enabled);
    append_specs_to_submenu(&submenu, specs, commands, checks)?;
    Ok(submenu)
}

fn append_specs_to_submenu(
    submenu: &Submenu,
    specs: &[TrayMenuItemSpec],
    commands: &mut HashMap<String, TrayCommand>,
    checks: &mut HashMap<String, CheckMenuItem>,
) -> Result<()> {
    for spec in specs {
        match spec {
            TrayMenuItemSpec::Action {
                label,
                command,
                enabled,
            } => {
                let id = command.id();
                let item = MenuItem::with_id(id.clone(), label, *enabled, None);
                commands.insert(id, command.clone());
                submenu.append(&item)?;
            }
            TrayMenuItemSpec::Check {
                label,
                command,
                checked,
                enabled,
            } => {
                let id = command.id();
                let item = CheckMenuItem::with_id(id.clone(), label, *enabled, *checked, None);
                commands.insert(id.clone(), command.clone());
                checks.insert(id, item.clone());
                submenu.append(&item)?;
            }
            TrayMenuItemSpec::Submenu {
                label,
                enabled,
                children,
            } => {
                let nested = build_submenu(label, *enabled, children, commands, checks)?;
                submenu.append(&nested)?;
            }
            TrayMenuItemSpec::Separator => {
                submenu.append(&PredefinedMenuItem::separator())?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_ids_are_stable_for_event_mapping() {
        assert_eq!(TrayCommand::Show.id(), "show");
        assert_eq!(
            TrayCommand::SetIcon("syncing".into()).id(),
            "set-icon:syncing"
        );
        assert_eq!(
            TrayCommand::Custom("deep-action".into()).id(),
            "custom:deep-action"
        );
        assert_eq!(TrayCommand::from_id("show"), Some(TrayCommand::Show));
        assert_eq!(
            TrayCommand::from_id("set-icon:error"),
            Some(TrayCommand::SetIcon("error".into()))
        );
        assert_eq!(
            TrayCommand::from_id("custom:auto-show"),
            Some(TrayCommand::Custom("auto-show".into()))
        );
    }

    #[test]
    fn default_menu_covers_check_dynamic_icon_and_deep_submenus() {
        let menu = default_liora_tray_menu();
        assert!(matches!(menu[2], TrayMenuItemSpec::Check { .. }));
        assert!(matches!(menu[3], TrayMenuItemSpec::Check { .. }));
        assert!(menu.iter().any(
            |item| matches!(item, TrayMenuItemSpec::Submenu { label, .. } if label == "切换图标")
        ));
        assert!(menu.iter().any(|item| matches!(item, TrayMenuItemSpec::Submenu { label, children, .. } if label == "多级菜单" && !children.is_empty())));
    }

    #[test]
    fn solid_icon_validates_rgba_size() {
        assert!(solid_icon([32, 96, 255, 255], 16).is_ok());
        assert!(icon_from_rgba(vec![0; 3], 1, 1).is_err());
    }

    #[test]
    fn tray_crate_keeps_application_icons_caller_owned() {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(
            !manifest_dir.join("assets").exists(),
            "liora-tray must not bundle Gallery/Docs icon assets"
        );
        assert!(
            !manifest_dir.join("src/assets").exists(),
            "liora-tray source must stay free of app-specific image assets"
        );
    }
}
