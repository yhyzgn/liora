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
    FontLoadReport, FontWeight, LocaleId, Locales, LocalesConfig, LocalesLoadError, LocalesMap,
    Options, TextDirection, ThemeMode, Translator, apply_locale, clear_translator, current_locale,
    discover_font_files, fallback_locale, is_font_family_available, is_supported_font_path,
    load_app_fonts, load_custom_fonts, load_embedded_fonts, load_font_assets, load_font_files,
    load_fonts_from_dir, load_locale_file, load_locales_dir, locales, locales_version,
    set_font_config, set_locale, set_locales_config, set_shared_translator, set_translator,
    startup_maximized_window_bounds, switch_locale_from_dir, tr,
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
        FontConfig, FontWeight, LocaleId, Locales, LocalesConfig, LocalesMap, Options,
        TextDirection, ThemeMode, Translator, apply_locale, current_locale, load_custom_fonts,
        load_locales_dir, locales, set_font_config, set_locale, startup_maximized_window_bounds,
        tr,
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

#[cfg(test)]
mod tests {
    crate::locales! {
        mod facade_test_keys {
            docs { subtitle }
        }
    }

    #[test]
    fn facade_reexports_typed_locales_and_macro() {
        assert_eq!(
            crate::locales::empty::description.as_str(),
            "empty.description"
        );
        assert_eq!(facade_test_keys::docs::subtitle.as_str(), "docs.subtitle");
    }

    #[test]
    fn facade_reexports_font_weight_for_font_config_api() {
        let weight = crate::FontWeight::MEDIUM;
        let fonts = crate::FontConfig::system().with_ui_weight(weight);
        assert_eq!(fonts.ui_weight(), Some(weight));
    }
}
