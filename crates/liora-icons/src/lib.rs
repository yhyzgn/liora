//! Native GPUI icon rendering helpers for Liora.
//!
//! This crate exposes an `Icon` component plus the `IntoIconPath` trait used by
//! Liora applications. Icons are rendered as GPUI SVG/image assets and do not
//! require a web icon runtime.

use gpui::{
    App, AssetSource, DefiniteLength, Hsla, IntoElement, Radians, RenderOnce, SharedString,
    Transformation, ViewElement as Component, Window, prelude::*, px,
};
use liora_core::Config;
use std::{
    borrow::Cow,
    env, fs,
    path::{Path, PathBuf},
};

/// Virtual asset prefix used for SVGs embedded directly in the binary.
///
/// Kept for caller-provided inline SVG snippets and backwards compatibility.
pub const INLINE_SVG_ASSET_PREFIX: &str = "liora-icon-inline:";

/// Virtual asset prefix used by optimized bundled icon sets.
pub const ICON_SVG_ASSET_PREFIX: &str = "liora-icon://";

const FALLBACK_ICON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" data-liora-icon-fallback="true" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="4"/><path d="M9.5 9a2.5 2.5 0 0 1 4.25-1.77 2.5 2.5 0 0 1-.35 3.84c-.86.58-1.4 1.02-1.4 2.18"/><path d="M12 17h.01"/></svg>"##;

/// Builds a virtual SVG asset path from embedded SVG source text.
pub fn inline_svg_asset_path(svg: &'static str) -> Cow<'static, str> {
    Cow::Owned(format!("{INLINE_SVG_ASSET_PREFIX}{svg}"))
}

/// Builds a virtual SVG asset path for an icon-library SVG file.
///
/// Optimized packages should place resources under
/// `assets/liora-icons/<set>/<file>`. The optional development fallback keeps
/// local `cargo run` working before installer resources are staged.
pub fn icon_svg_asset_path(
    set: &'static str,
    file: &'static str,
    dev_path: Option<String>,
) -> Cow<'static, str> {
    match dev_path {
        Some(dev_path) => Cow::Owned(format!(
            "{ICON_SVG_ASSET_PREFIX}{set}/{file}?dev={dev_path}"
        )),
        None => Cow::Owned(format!("{ICON_SVG_ASSET_PREFIX}{set}/{file}")),
    }
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
/// `IconAssetSource` can load inline SVG payloads, optimized icon bundle
/// resources, and explicit filesystem SVG paths.
#[derive(Debug, Default, Clone, Copy)]
pub struct IconAssetSource;

impl AssetSource for IconAssetSource {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some(svg) = path.strip_prefix(INLINE_SVG_ASSET_PREFIX) {
            return Ok(Some(Cow::Owned(svg.as_bytes().to_vec())));
        }

        if let Some(request) = IconAssetRequest::parse(path) {
            return load_virtual_icon_asset(path, &request);
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

#[derive(Debug, Clone)]
struct IconAssetRequest {
    set: String,
    file: String,
    dev_path: Option<PathBuf>,
}

impl IconAssetRequest {
    fn parse(path: &str) -> Option<Self> {
        let rest = path.strip_prefix(ICON_SVG_ASSET_PREFIX)?;
        let (resource, query) = rest.split_once('?').unwrap_or((rest, ""));
        let (set, file) = resource.split_once('/')?;
        if set.is_empty() || file.is_empty() || file.contains("..") || set.contains("..") {
            return None;
        }
        let dev_path = query
            .split('&')
            .find_map(|part| part.strip_prefix("dev="))
            .filter(|value| !value.is_empty())
            .map(|value| PathBuf::from(value.strip_prefix("file://").unwrap_or(value)));
        Some(Self {
            set: set.to_string(),
            file: file.to_string(),
            dev_path,
        })
    }

    fn candidate_paths(&self) -> Vec<PathBuf> {
        let mut candidates = Vec::new();
        if let Ok(dir) = env::var("LIORA_ICON_ASSETS_DIR") {
            candidates.push(PathBuf::from(dir).join(&self.set).join(&self.file));
        }
        if let Ok(exe) = env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                candidates.push(self.from_resource_root(exe_dir.join("assets/liora-icons")));
                candidates.push(self.from_resource_root(exe_dir.join("../assets/liora-icons")));
                candidates
                    .push(self.from_resource_root(exe_dir.join("../Resources/assets/liora-icons")));
            }
            if let Some(binary) = exe.file_name().and_then(|name| name.to_str()) {
                candidates.push(
                    self.from_resource_root(
                        PathBuf::from("/usr/lib")
                            .join(binary)
                            .join("assets/liora-icons"),
                    ),
                );
            }
        }
        if let Ok(current_dir) = env::current_dir() {
            candidates.push(self.from_resource_root(current_dir.join("assets/liora-icons")));
            candidates.extend(target_icon_bundle_candidates(
                &current_dir,
                &self.set,
                &self.file,
            ));
        }
        if let Some(dev_path) = &self.dev_path {
            candidates.push(dev_path.clone());
        }
        candidates
    }

    fn from_resource_root(&self, root: PathBuf) -> PathBuf {
        root.join(&self.set).join(&self.file)
    }
}

fn load_virtual_icon_asset(
    original_path: &str,
    request: &IconAssetRequest,
) -> gpui::Result<Option<Cow<'static, [u8]>>> {
    let debug = icon_debug_enabled();
    let candidates = request.candidate_paths();
    if debug {
        eprintln!(
            "liora-icons: resolving {original_path} with {} candidate path(s)",
            candidates.len()
        );
    }

    for candidate in candidates {
        match fs::read(&candidate) {
            Ok(bytes) => {
                if debug {
                    eprintln!("liora-icons: hit {}", candidate.display());
                }
                return Ok(Some(Cow::Owned(bytes)));
            }
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory
                ) =>
            {
                if debug {
                    eprintln!(
                        "liora-icons: miss {} ({})",
                        candidate.display(),
                        error.kind()
                    );
                }
            }
            Err(error) => {
                if debug {
                    eprintln!("liora-icons: error {}: {error}", candidate.display());
                }
                return Err(error.into());
            }
        }
    }

    if debug {
        eprintln!("liora-icons: using fallback placeholder for {original_path}");
    }
    Ok(Some(Cow::Borrowed(FALLBACK_ICON_SVG.as_bytes())))
}

fn icon_debug_enabled() -> bool {
    env::var("LIORA_ICON_DEBUG")
        .map(|value| {
            let value = value.trim();
            !(value.is_empty()
                || value == "0"
                || value.eq_ignore_ascii_case("false")
                || value.eq_ignore_ascii_case("off"))
        })
        .unwrap_or(false)
}

fn target_icon_bundle_candidates(current_dir: &Path, set: &str, file: &str) -> Vec<PathBuf> {
    let root = current_dir.join("target/liora/icons");
    let mut candidates = vec![root.join("assets/liora-icons").join(set).join(file)];
    candidates.extend(target_icon_bundle_candidates_under(
        &root.join("apps"),
        set,
        file,
    ));
    candidates.extend(target_icon_bundle_candidates_under(&root, set, file));
    candidates
}

fn target_icon_bundle_candidates_under(root: &Path, set: &str, file: &str) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            let resource_root = path.join("assets/liora-icons");
            if resource_root.is_dir() {
                candidates.push(resource_root.join(set).join(file));
            }
        }
    }
    candidates
}

/// Native GPUI SVG icon element with size, color, hover, and rotation controls.
pub struct Icon {
    size: Option<DefiniteLength>,
    color: Option<Hsla>,
    group_hover_color: Option<(SharedString, Hsla)>,
    group_hover_primary: Option<SharedString>,
    rotation: Option<Radians>,
    render_scale: Option<f32>,
    asset_path: String,
}

impl Icon {
    /// Creates `Icon` initialized from the supplied path.
    pub fn new(path: impl IntoIconPath) -> Self {
        Self {
            size: None,
            color: None,
            group_hover_color: None,
            group_hover_primary: None,
            rotation: None,
            render_scale: None,
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

    /// Set explicit color. If not called, uses the current theme's `neutral.icon`.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Change icon color when a parent/group member is hovered.
    pub fn group_hover_color(mut self, group: impl Into<SharedString>, color: Hsla) -> Self {
        self.group_hover_color = Some((group.into(), color));
        self
    }

    /// Change icon color to the active theme primary color when a group member is hovered.
    pub fn group_hover_primary(mut self, group: impl Into<SharedString>) -> Self {
        self.group_hover_primary = Some(group.into());
        self
    }

    /// Rotate the icon around its center while preserving layout and hitbox.
    pub fn rotation(mut self, rotation: Radians) -> Self {
        self.rotation = Some(rotation);
        self
    }

    /// Scales the rendered SVG around its center without changing the icon layout box.
    ///
    /// This is useful for oversampled animated icons: callers can request a
    /// larger SVG layout and scale the rendered primitive down to reduce
    /// visible stair-stepping while keeping the outer component dimensions stable.
    pub fn render_scale(mut self, scale: f32) -> Self {
        self.render_scale = Some(scale.max(0.1));
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
        if let Some(group) = self.group_hover_primary {
            let primary = theme.primary.base;
            el = el.group_hover(group, move |style| style.text_color(primary));
        }
        if self.rotation.is_some() || self.render_scale.is_some() {
            let mut transformation = Transformation::default();
            if let Some(scale) = self.render_scale {
                transformation = transformation.with_scaling(gpui::size(scale, scale));
            }
            if let Some(rotation) = self.rotation {
                transformation = transformation.with_rotation(rotation);
            }
            el = el.with_transformation(transformation);
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
    fn icon_supports_theme_primary_group_hover() {
        let source = include_str!("lib.rs");

        assert!(source.contains("pub fn group_hover_primary"));
        assert!(source.contains("theme.primary.base"));
    }

    #[test]
    fn icon_rotation_tracks_transform_request() {
        assert_eq!(
            Icon::new("loader").rotation(gpui::radians(1.0)).rotation,
            Some(gpui::radians(1.0))
        );
    }

    #[test]
    fn icon_render_scale_tracks_transform_request() {
        let source = include_str!("lib.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("production source should precede tests");

        assert_eq!(
            Icon::new("loader").render_scale(0.5).render_scale,
            Some(0.5)
        );
        assert!(production.contains("with_scaling(gpui::size(scale, scale))"));
        assert!(production.contains("with_rotation(rotation)"));
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

    #[test]
    fn target_icon_bundle_candidates_prefer_app_bundles_and_ignore_reports() {
        let root = std::env::temp_dir().join(format!(
            "liora-icons-candidates-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let icons_root = root.join("target/liora/icons");
        let app_bundle = icons_root.join("apps/sample-app/assets/liora-icons/lucide");
        let legacy_bundle = icons_root.join("sample-app/assets/liora-icons/lucide");
        std::fs::create_dir_all(&app_bundle).unwrap();
        std::fs::create_dir_all(&legacy_bundle).unwrap();
        std::fs::create_dir_all(icons_root.join("reports")).unwrap();
        std::fs::write(icons_root.join("reports/sample-app.md"), "report").unwrap();
        std::fs::write(app_bundle.join("a-arrow-down.svg"), "<svg />").unwrap();
        std::fs::write(legacy_bundle.join("a-arrow-down.svg"), "<svg />").unwrap();

        let candidates = target_icon_bundle_candidates(&root, "lucide", "a-arrow-down.svg");
        let rendered = candidates
            .iter()
            .map(|path| path.to_string_lossy().replace('\\', "/"))
            .collect::<Vec<_>>();

        let app_index = rendered
            .iter()
            .position(|path| {
                path.ends_with(
                    "target/liora/icons/apps/sample-app/assets/liora-icons/lucide/a-arrow-down.svg",
                )
            })
            .expect("new app bundle path should be searched");
        let legacy_index = rendered
            .iter()
            .position(|path| {
                path.ends_with(
                    "target/liora/icons/sample-app/assets/liora-icons/lucide/a-arrow-down.svg",
                )
            })
            .expect("legacy app bundle path should remain compatible");
        assert!(app_index < legacy_index);
        assert!(
            !rendered
                .iter()
                .any(|path| path.contains("reports/assets/liora-icons"))
        );

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn virtual_icon_missing_resource_uses_visible_placeholder() {
        use gpui::AssetSource;

        let path = format!("{ICON_SVG_ASSET_PREFIX}missing/nope.svg");
        let bytes = IconAssetSource
            .load(&path)
            .expect("missing virtual icon should not error")
            .expect("missing virtual icon should return fallback placeholder");
        let svg = std::str::from_utf8(&bytes).unwrap();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("data-liora-icon-fallback=\"true\""));
        assert!(svg.contains("<rect"));
        assert!(svg.contains("M12 17h.01"));
    }

    #[test]
    fn icon_asset_source_loads_virtual_icon_with_dev_fallback() {
        use gpui::AssetSource;

        let dev_path = format!(
            "{}/../liora-icons-lucide/assets/svgs/loader-circle.svg",
            env!("CARGO_MANIFEST_DIR")
        );
        let path = icon_svg_asset_path("lucide", "loader-circle.svg", Some(dev_path));
        let bytes = IconAssetSource
            .load(&path)
            .expect("virtual icon asset loading should not error")
            .expect("virtual icon should load from dev fallback");
        let svg = std::str::from_utf8(&bytes).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
    }
}
