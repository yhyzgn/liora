//! Page-level residency configuration for tray-enabled GPUI apps.
//!
//! Keep this state in your app/page model. When users disable residency, hide
//! the tray icon and return to `LastWindowClosed` so no invisible process is
//! left behind.

use gpui::{App, QuitMode};
use liora_tray::{LioraTray, Result, TrayCommand, TrayControlCenter};

pub struct TrayResidencyConfig {
    pub resident_enabled: bool,
    pub tray_visible: bool,
}

impl TrayResidencyConfig {
    pub fn apply(&self, tray: &LioraTray, cx: &mut App) -> Result<()> {
        tray.set_visible(self.resident_enabled && self.tray_visible)?;
        cx.set_quit_mode(if self.resident_enabled {
            QuitMode::Explicit
        } else {
            QuitMode::LastWindowClosed
        });
        Ok(())
    }
}

pub fn toggle_residency_from_page(cx: &mut App) {
    if cx.has_global::<TrayControlCenter>() {
        cx.global::<TrayControlCenter>()
            .dispatch(TrayCommand::Custom("resident-enabled".into()));
    }
}
