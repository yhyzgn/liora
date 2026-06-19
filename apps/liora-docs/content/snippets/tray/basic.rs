//! Basic liora-tray installation shape with an app-owned icon asset.
//!
//! In a real GPUI app, keep the returned `LioraTray` in your app state for as
//! long as the tray icon should remain visible. Store product icons in the app
//! crate, not in `liora-tray`.

use liora_tray::{LioraTray, Result, TrayConfig, default_liora_tray_menu, icon_from_png_bytes};

const GALLERY_TRAY_ICON: &[u8] =
    include_bytes!("../../../../liora-gallery/assets/tray-icons/default.png");

pub fn install_basic_tray() -> Result<LioraTray> {
    let icon = icon_from_png_bytes(GALLERY_TRAY_ICON)?;

    LioraTray::install(
        TrayConfig::new("liora-gallery")
            .tooltip("Liora Gallery")
            .icon(icon)
            .menu(default_liora_tray_menu()),
    )
}
