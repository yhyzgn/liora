//! One-stop public facade for the Liora native GPUI SDK.
//!
//! Add this crate when an application wants the maintained Liora surface without
//! listing every lower-level crate separately:
//!
//! ```toml
//! liora = "0.1.8"
//! ```
//!
//! The facade keeps each domain available under a stable module name while also
//! re-exporting the most common application setup entry points.

pub use liora_components::{init_liora, init_liora_with_mode};
pub use liora_core::{ThemeMode, startup_maximized_window_bounds};

pub use liora_components as components;
pub use liora_core as core;
pub use liora_icons as icons;
pub use liora_icons_lucide as icons_lucide;
pub use liora_theme as theme;
pub use liora_tray as tray;

#[cfg(feature = "packager")]
pub use liora_packager as packager;

#[cfg(feature = "updater")]
pub use liora_updater as updater;

/// Prelude for applications that prefer a compact import surface.
pub mod prelude {
    pub use liora_components::{init_liora, init_liora_with_mode};
    pub use liora_core::{ThemeMode, startup_maximized_window_bounds};

    pub use crate::{components, core, icons, icons_lucide, theme, tray};

    #[cfg(feature = "packager")]
    pub use crate::packager;

    #[cfg(feature = "updater")]
    pub use crate::updater;
}
