//! Native system tray facade for Aura applications.
//!
//! `aura-tray` intentionally keeps `tray-icon` and `muda` behind a small Aura
//! command API so GPUI apps can stay focused on window lifecycle commands while
//! still supporting dynamic icon updates, check items, and deep nested menus.

use std::{collections::HashMap, path::Path};

pub use tray_icon::menu::{CheckMenuItem, MenuEvent, MenuId};
use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
pub use tray_icon::{Icon as TrayIconImage, MouseButton, MouseButtonState, TrayIconEvent};
use tray_icon::{TrayIcon, TrayIconBuilder};

#[derive(Debug, thiserror::Error)]
pub enum AuraTrayError {
    #[error("tray icon error: {0}")]
    Tray(#[from] tray_icon::Error),
    #[error("bad tray icon: {0}")]
    BadIcon(#[from] tray_icon::BadIcon),
    #[error("menu error: {0}")]
    Menu(#[from] tray_icon::menu::Error),
    #[error("image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("failed to initialize tray platform runtime: {0}")]
    PlatformInit(String),
    #[error("invalid rgba icon buffer {width}x{height}: expected {expected} bytes, got {actual}")]
    InvalidRgba {
        width: u32,
        height: u32,
        expected: usize,
        actual: usize,
    },
}

pub type Result<T> = std::result::Result<T, AuraTrayError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrayCommand {
    Show,
    Hide,
    Toggle,
    Quit,
    SetIcon(String),
    Custom(String),
}

impl TrayCommand {
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
pub enum TrayMenuItemSpec {
    Action {
        label: String,
        command: TrayCommand,
        enabled: bool,
    },
    Check {
        label: String,
        command: TrayCommand,
        checked: bool,
        enabled: bool,
    },
    Submenu {
        label: String,
        enabled: bool,
        children: Vec<TrayMenuItemSpec>,
    },
    Separator,
}

impl TrayMenuItemSpec {
    pub fn action(label: impl Into<String>, command: TrayCommand) -> Self {
        Self::Action {
            label: label.into(),
            command,
            enabled: true,
        }
    }

    pub fn check(label: impl Into<String>, command: TrayCommand, checked: bool) -> Self {
        Self::Check {
            label: label.into(),
            command,
            checked,
            enabled: true,
        }
    }

    pub fn submenu(label: impl Into<String>, children: Vec<TrayMenuItemSpec>) -> Self {
        Self::Submenu {
            label: label.into(),
            enabled: true,
            children,
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }
}

#[derive(Clone)]
pub struct TrayConfig {
    pub id: String,
    pub tooltip: Option<String>,
    pub icon: Option<TrayIconImage>,
    pub icon_is_template: bool,
    pub menu_on_left_click: bool,
    pub menu_on_right_click: bool,
    pub menu: Vec<TrayMenuItemSpec>,
}

impl TrayConfig {
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

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn icon(mut self, icon: TrayIconImage) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_is_template(mut self, is_template: bool) -> Self {
        self.icon_is_template = is_template;
        self
    }

    pub fn menu(mut self, menu: Vec<TrayMenuItemSpec>) -> Self {
        self.menu = menu;
        self
    }
}

pub struct AuraTray {
    tray: TrayIcon,
    command_by_id: HashMap<String, TrayCommand>,
    check_by_id: HashMap<String, CheckMenuItem>,
}

impl AuraTray {
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

    pub fn command_for_menu_id(&self, id: &MenuId) -> Option<&TrayCommand> {
        self.command_by_id.get(&id.0)
    }

    pub fn command_for_event(&self, event: &MenuEvent) -> Option<&TrayCommand> {
        self.command_for_menu_id(event.id())
    }

    pub fn set_icon(&self, icon: TrayIconImage) -> Result<()> {
        self.tray.set_icon(Some(icon))?;
        Ok(())
    }

    pub fn clear_icon(&self) -> Result<()> {
        self.tray.set_icon(None)?;
        Ok(())
    }

    pub fn set_icon_from_rgba(&self, rgba: Vec<u8>, width: u32, height: u32) -> Result<()> {
        self.set_icon(icon_from_rgba(rgba, width, height)?)
    }

    pub fn set_icon_from_path(&self, path: impl AsRef<Path>) -> Result<()> {
        self.set_icon(icon_from_path(path)?)
    }

    pub fn set_tooltip(&self, tooltip: Option<&str>) -> Result<()> {
        self.tray.set_tooltip(tooltip)?;
        Ok(())
    }

    pub fn set_visible(&self, visible: bool) -> Result<()> {
        self.tray.set_visible(visible)?;
        Ok(())
    }

    pub fn set_check_state(&self, command: &TrayCommand, checked: bool) -> bool {
        let id = command.id();
        if let Some(item) = self.check_by_id.get(&id) {
            item.set_checked(checked);
            true
        } else {
            false
        }
    }

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
fn init_platform_tray_runtime() -> Result<()> {
    if gtk::is_initialized() {
        return Ok(());
    }

    gtk::init().map_err(|error| AuraTrayError::PlatformInit(error.to_string()))
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn init_platform_tray_runtime() -> Result<()> {
    Ok(())
}

pub fn icon_from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<TrayIconImage> {
    let expected = width as usize * height as usize * 4;
    let actual = rgba.len();
    if actual != expected {
        return Err(AuraTrayError::InvalidRgba {
            width,
            height,
            expected,
            actual,
        });
    }
    Ok(TrayIconImage::from_rgba(rgba, width, height)?)
}

pub fn icon_from_path(path: impl AsRef<Path>) -> Result<TrayIconImage> {
    let image = image::open(path)?.into_rgba8();
    let (width, height) = image.dimensions();
    icon_from_rgba(image.into_raw(), width, height)
}

pub fn solid_icon(color: [u8; 4], size: u32) -> Result<TrayIconImage> {
    let mut rgba = Vec::with_capacity(size as usize * size as usize * 4);
    for _ in 0..size * size {
        rgba.extend_from_slice(&color);
    }
    icon_from_rgba(rgba, size, size)
}

pub fn default_aura_tray_menu() -> Vec<TrayMenuItemSpec> {
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
        let menu = default_aura_tray_menu();
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
}
