//! Dynamic tray icon switching for running states with app-owned assets.

use liora_tray::{LioraTray, Result, TrayCommand, TrayIconImage, icon_from_png_bytes};

fn tray_icon_bytes(name: &str) -> &'static [u8] {
    match name {
        "syncing" => include_bytes!("../../../../liora-gallery/assets/tray-icons/syncing.png"),
        "error" => include_bytes!("../../../../liora-gallery/assets/tray-icons/error.png"),
        _ => include_bytes!("../../../../liora-gallery/assets/tray-icons/default.png"),
    }
}

pub fn icon_for_command(command: &TrayCommand) -> Result<Option<TrayIconImage>> {
    match command {
        TrayCommand::SetIcon(name) => Ok(Some(icon_from_png_bytes(tray_icon_bytes(name))?)),
        _ => Ok(None),
    }
}

pub fn apply_icon_command(tray: &LioraTray, command: &TrayCommand) -> Result<bool> {
    if let Some(icon) = icon_for_command(command)? {
        tray.set_icon(icon)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
