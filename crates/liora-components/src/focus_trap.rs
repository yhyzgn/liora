//! Focus Trap module.
//!
//! This public module implements Liora focus-trap policy data for overlay components. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

/// Policy object that overlay components can share when trapping focus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusTrap {
    /// Whether focus trapping is active for the overlay.
    pub enabled: bool,
    /// Whether focus should return to the previously focused control when the overlay closes.
    pub restore_focus: bool,
    /// Whether pressing Escape should request closing the overlay.
    pub close_on_escape: bool,
}
impl Default for FocusTrap {
    fn default() -> Self {
        Self {
            enabled: true,
            restore_focus: true,
            close_on_escape: true,
        }
    }
}
impl FocusTrap {
    /// Creates the default focus-trap policy for modal overlays.
    pub fn new() -> Self {
        Self::default()
    }
    /// Disables focus trapping while preserving the other policy flags.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    /// Sets whether focus should be restored after the overlay closes.
    pub fn restore_focus(mut self, restore: bool) -> Self {
        self.restore_focus = restore;
        self
    }
    /// Sets whether Escape should request closing the overlay.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn focus_trap_defaults_are_overlay_friendly() {
        let trap = FocusTrap::new();
        assert!(trap.enabled);
        assert!(trap.restore_focus);
        assert!(trap.close_on_escape);
        assert!(!trap.disabled().enabled);
    }
}
