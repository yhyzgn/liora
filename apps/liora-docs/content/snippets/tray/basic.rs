//! Basic liora-tray installation shape with app-owned icon and menu assets.
//!
//! In a real GPUI app, keep the returned `Tray` in your app state for as
//! long as the tray icon should remain visible. Store product icons and menu
//! policy in the app crate, not in `liora-tray`.

use liora_tray::{Result, Tray, TrayCommand, TrayConfig, TrayMenuItemSpec, icon_from_png_bytes};

const GALLERY_TRAY_ICON: &[u8] =
    include_bytes!("../../../../liora-gallery/assets/tray-icons/default.png");

fn app_tray_menu() -> Vec<TrayMenuItemSpec> {
    vec![
        TrayMenuItemSpec::action("Show Window", TrayCommand::Show),
        TrayMenuItemSpec::action("Hide Window", TrayCommand::Hide),
        TrayMenuItemSpec::separator(),
        TrayMenuItemSpec::action("Quit", TrayCommand::Quit),
    ]
}

pub fn install_basic_tray() -> Result<Tray> {
    let icon = icon_from_png_bytes(GALLERY_TRAY_ICON)?;

    Tray::install(
        TrayConfig::new("liora-gallery")
            .tooltip("Liora Gallery")
            .icon(icon)
            .menu(app_tray_menu()),
    )
}
