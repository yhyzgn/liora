//! Basic liora-tray installation shape with bundled demo icons.
//!
//! In a real GPUI app, keep the returned `LioraTray` in your app state for as
//! long as the tray icon should remain visible.

use liora_tray::{
    BundledTrayIconSet, BundledTrayIconState, LioraTray, Result, TrayConfig, bundled_tray_icon,
    default_liora_tray_menu,
};

pub fn install_basic_tray() -> Result<LioraTray> {
    let icon = bundled_tray_icon(BundledTrayIconSet::Gallery, BundledTrayIconState::Default)?;

    LioraTray::install(
        TrayConfig::new("liora-gallery")
            .tooltip("Liora Gallery")
            .icon(icon)
            .menu(default_liora_tray_menu()),
    )
}
