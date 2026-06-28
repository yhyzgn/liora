//! One-stop public facade for the Liora native GPUI SDK.
//!
//! Add this crate when an application wants the maintained Liora surface without
//! listing every lower-level crate separately:
//!
//! ```toml
//! [dependencies]
//! liora = "0.1"
//! gpui = { version = "0.2.2", default-features = false }
//!
//! [patch.crates-io]
//! gpui = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718" }
//! ```
//!
//! The published crates use Cargo's registry fallback for `gpui = 0.2.2`, then
//! applications override that fallback with a matching official Zed GPUI git
//! revision. This keeps Liora available through crates.io without coupling the
//! SDK to a community fork or forcing downstream projects onto the stale
//! registry implementation.
//!
//! The facade keeps each domain available under a stable module name while also
//! re-exporting the most common application setup entry points.

pub use liora_components::{init_liora, init_liora_with_mode, init_liora_with_options};
pub use liora_core::{
    EmbeddedFont, FontConfig, FontDiscoveryReport, FontLoadFailure, FontLoadMode, FontLoadOptions,
    FontLoadReport, LioraOptions, ThemeMode, discover_font_files, is_font_family_available,
    is_supported_font_path, load_app_fonts, load_custom_fonts, load_embedded_fonts,
    load_font_assets, load_font_files, load_fonts_from_dir, set_font_config,
    startup_maximized_window_bounds,
};

pub use liora_components as components;
pub use liora_core as core;
pub use liora_icons as icons;
pub use liora_icons_antd as icons_antd;
pub use liora_icons_carbon as icons_carbon;
pub use liora_icons_ionic as icons_ionic;
pub use liora_icons_lucide as icons_lucide;
pub use liora_icons_material as icons_material;
pub use liora_icons_tabler as icons_tabler;
pub use liora_theme as theme;
pub use liora_tray as tray;

#[cfg(feature = "packager")]
pub use liora_packager as packager;

#[cfg(feature = "updater")]
pub use liora_updater as updater;

/// Prelude for applications that prefer a compact import surface.
pub mod prelude {
    pub use liora_components::{init_liora, init_liora_with_mode, init_liora_with_options};
    pub use liora_core::{
        FontConfig, LioraOptions, ThemeMode, load_custom_fonts, set_font_config,
        startup_maximized_window_bounds,
    };

    pub use crate::{
        components, core, icons, icons_antd, icons_carbon, icons_ionic, icons_lucide,
        icons_material, icons_tabler, theme, tray,
    };

    #[cfg(feature = "packager")]
    pub use crate::packager;

    #[cfg(feature = "updater")]
    pub use crate::updater;
}
