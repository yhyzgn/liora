//! Page-level residency configuration for tray-enabled GPUI apps.
//!
//! Keep this state in your app/page model. When users disable residency, hide
//! the tray icon. Official Zed GPUI does not expose a public quit-mode switch;
//! dispatch an explicit quit action when your product should terminate.

use gpui::App;
use liora_tray::{Result, Tray, TrayCommand, TrayControlCenter};

pub struct TrayResidencyConfig {
    pub resident_enabled: bool,
    pub tray_visible: bool,
}

impl TrayResidencyConfig {
    pub fn apply(&self, tray: &Tray, cx: &mut App) -> Result<()> {
        tray.set_visible(self.resident_enabled && self.tray_visible)?;
        Ok(())
    }
}

pub fn toggle_residency_from_page(cx: &mut App) {
    if cx.has_global::<TrayControlCenter>() {
        cx.global::<TrayControlCenter>()
            .dispatch(TrayCommand::Custom("resident-enabled".into()));
    }
}
