//! Native GPUI icon rendering helpers for Liora.
//!
//! This crate exposes an `Icon` component plus the `IntoIconPath` trait used by
//! Liora demos and docs. Icons are rendered as GPUI SVG/image assets and do not
//! require a web icon runtime.

use gpui::{
    App, AssetSource, Component, DefiniteLength, Hsla, IntoElement, Radians, RenderOnce,
    SharedString, Transformation, Window, prelude::*, px,
};
use liora_core::Config;
use std::{borrow::Cow, fs};

/// Virtual asset prefix used for SVGs embedded directly in the binary.
///
/// `liora-icons-lucide` uses this prefix so raw release executables can render
/// bundled icons without shipping a separate `assets/svgs` directory.
pub const INLINE_SVG_ASSET_PREFIX: &str = "liora-icon-inline:";

/// Builds a virtual SVG asset path from embedded SVG source text.
pub fn inline_svg_asset_path(svg: &'static str) -> Cow<'static, str> {
    Cow::Owned(format!("{INLINE_SVG_ASSET_PREFIX}{svg}"))
}

/// Converts icon identifiers into SVG asset paths that `Icon` can render.
pub trait IntoIconPath {
    /// Returns the SVG asset path used by the icon renderer.
    fn icon_path(&self) -> Cow<'static, str>;
}

impl IntoIconPath for &str {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}
impl IntoIconPath for String {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.clone())
    }
}

/// Asset source for Liora SVG icons.
///
/// GPUI's `svg().path(...)` resolves through the application asset source.
/// `IconAssetSource` can load both embedded Lucide SVG payloads and explicit
/// filesystem SVG paths, so release raw executables do not need a source-tree
/// `assets/svgs` directory next to the binary.
#[derive(Debug, Default, Clone, Copy)]
pub struct IconAssetSource;

impl AssetSource for IconAssetSource {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some(svg) = path.strip_prefix(INLINE_SVG_ASSET_PREFIX) {
            return Ok(Some(Cow::Owned(svg.as_bytes().to_vec())));
        }

        let path = path.strip_prefix("file://").unwrap_or(path);
        if path.is_empty() {
            return Ok(None);
        }
        match fs::read(path) {
            Ok(bytes) => Ok(Some(Cow::Owned(bytes))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}

/// Native GPUI SVG icon element with size, color, hover, and rotation controls.
pub struct Icon {
    size: Option<DefiniteLength>,
    color: Option<Hsla>,
    group_hover_color: Option<(SharedString, Hsla)>,
    rotation: Option<Radians>,
    asset_path: String,
}

impl Icon {
    /// Creates `Icon` initialized from the supplied path.
    pub fn new(path: impl IntoIconPath) -> Self {
        Self {
            size: None,
            color: None,
            group_hover_color: None,
            rotation: None,
            asset_path: path.icon_path().into_owned(),
        }
    }

    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, sz: impl Into<DefiniteLength>) -> Self {
        self.size = Some(sz.into());
        self
    }

    /// Sets an explicit icon size from application-facing design units.
    pub fn size_units(self, size: f32) -> Self {
        self.size(px(size))
    }

    /// Applies the predefined size xs sizing preset.
    pub fn size_xs(self) -> Self {
        self.size(px(12.0))
    }

    /// Applies the predefined size md sizing preset.
    pub fn size_md(self) -> Self {
        self.size(px(18.0))
    }

    /// Applies the predefined size lg sizing preset.
    pub fn size_lg(self) -> Self {
        self.size(px(24.0))
    }

    /// Applies the predefined size xl sizing preset.
    pub fn size_xl(self) -> Self {
        self.size(px(32.0))
    }

    /// Set explicit color. If not called, inherits parent's text_color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Change icon color when a parent/group member is hovered.
    pub fn group_hover_color(mut self, group: impl Into<SharedString>, color: Hsla) -> Self {
        self.group_hover_color = Some((group.into(), color));
        self
    }

    /// Rotate the icon around its center while preserving layout and hitbox.
    pub fn rotation(mut self, rotation: Radians) -> Self {
        self.rotation = Some(rotation);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        let sz = self.size.unwrap_or_else(|| px(18.0).into());
        let mut el = gpui::svg().path(self.asset_path).size(sz);
        if let Some(color) = self.color {
            el = el.text_color(color);
        } else {
            el = el.text_color(theme.neutral.icon);
        }
        if let Some((group, color)) = self.group_hover_color {
            el = el.group_hover(group, move |style| style.text_color(color));
        }
        if let Some(rotation) = self.rotation {
            el = el.with_transformation(Transformation::rotate(rotation));
        }
        el
    }
}

impl IntoElement for Icon {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_size_helpers_set_common_demo_sizes() {
        assert_eq!(
            Icon::new("home").size_units(16.0).size,
            Some(px(16.0).into())
        );
        assert_eq!(Icon::new("home").size_xs().size, Some(px(12.0).into()));
        assert_eq!(Icon::new("home").size_md().size, Some(px(18.0).into()));
        assert_eq!(Icon::new("home").size_lg().size, Some(px(24.0).into()));
        assert_eq!(Icon::new("home").size_xl().size, Some(px(32.0).into()));
    }

    #[test]
    fn icon_rotation_tracks_transform_request() {
        assert_eq!(
            Icon::new("loader").rotation(gpui::radians(1.0)).rotation,
            Some(gpui::radians(1.0))
        );
    }

    #[test]
    fn icon_asset_source_loads_absolute_svg_files() {
        use gpui::AssetSource;

        let path = format!(
            "{}/../liora-icons-lucide/assets/svgs/loader-circle.svg",
            env!("CARGO_MANIFEST_DIR")
        );
        let bytes = IconAssetSource
            .load(&path)
            .expect("icon asset loading should not error")
            .expect("loader-circle svg should exist");
        assert!(std::str::from_utf8(&bytes).unwrap().contains("<svg"));
        assert!(
            IconAssetSource
                .load("/definitely/missing/liora.svg")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn icon_asset_source_loads_embedded_svg_payloads() {
        use gpui::AssetSource;

        let path = format!(
            "{}{}",
            INLINE_SVG_ASSET_PREFIX, r#"<svg viewBox="0 0 24 24"><path d="M1 1h22v22H1z"/></svg>"#
        );
        let bytes = IconAssetSource
            .load(&path)
            .expect("embedded icon asset loading should not error")
            .expect("embedded SVG payload should load");
        let svg = std::str::from_utf8(&bytes).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
    }
}
