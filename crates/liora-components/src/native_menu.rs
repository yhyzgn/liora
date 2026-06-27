//! Native Menu module.
//!
//! This public module implements a platform-neutral menu description for application/native menu integration. It keeps the reusable
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

use gpui::SharedString;

/// Platform-neutral native menu item description.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeMenuItem {
    pub id: SharedString,
    pub label: SharedString,
    pub shortcut: Option<SharedString>,
    pub disabled: bool,
}
impl NativeMenuItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            disabled: false,
        }
    }
    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
/// Platform-neutral native menu description.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeMenu {
    pub title: SharedString,
    pub items: Vec<NativeMenuItem>,
}
impl NativeMenu {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
        }
    }
    pub fn item(mut self, item: NativeMenuItem) -> Self {
        self.items.push(item);
        self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_menu_tracks_items() {
        let menu =
            NativeMenu::new("File").item(NativeMenuItem::new("open", "Open").shortcut("Ctrl+O"));
        assert_eq!(menu.items.len(), 1);
        assert_eq!(
            menu.items[0].shortcut.as_ref().map(|s| s.as_ref()),
            Some("Ctrl+O")
        );
    }
}
