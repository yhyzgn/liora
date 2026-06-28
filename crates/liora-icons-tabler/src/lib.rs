//! Bundled Tabler icon names for Liora's native GPUI icon component.
//!
//! The generated `IconName` enum maps each synchronized Tabler SVG asset to a
//! strongly typed Rust variant. Each variant can be passed directly to
//! `liora_icons::Icon::new(...)` or converted into a GPUI element.

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use std::borrow::Cow;

impl IconName {
    /// Returns the virtual path for the embedded SVG asset for this icon.
    ///
    /// The returned value is intentionally not a filesystem path: raw release
    /// executables must be able to render icons without access to the source
    /// tree that existed on the build runner.
    pub fn svg_path(&self) -> String {
        liora_icons::inline_svg_asset_path(self.svg_source()).into_owned()
    }
}

impl liora_icons::IntoIconPath for IconName {
    fn icon_path(&self) -> Cow<'static, str> {
        liora_icons::inline_svg_asset_path(self.svg_source())
    }
}

impl gpui::IntoElement for IconName {
    type Element = gpui::Component<liora_icons::Icon>;
    fn into_element(self) -> Self::Element {
        liora_icons::Icon::new(self).into_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::AssetSource;
    use liora_icons::IntoIconPath;

    #[test]
    fn icon_paths_are_embedded_for_raw_release_binaries() {
        let icon = IconName::all()
            .first()
            .copied()
            .expect("icon set should not be empty");
        let path = icon.icon_path();
        assert!(path.starts_with(liora_icons::INLINE_SVG_ASSET_PREFIX));
        assert!(
            !path.contains(env!("CARGO_MANIFEST_DIR")),
            "release icon paths must not point at the build machine source tree"
        );

        let bytes = liora_icons::IconAssetSource
            .load(&path)
            .expect("embedded icon loading should not error")
            .expect("embedded icon should resolve through IconAssetSource");
        let svg = std::str::from_utf8(&bytes).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox") || svg.contains("viewbox"));
    }
}
