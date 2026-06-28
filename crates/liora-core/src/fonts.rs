//! Application-level font loading and typography helpers.
//!
//! GPUI resolves text in two separate steps: first an application registers any
//! private font bytes with `App::text_system().add_fonts`, then elements refer to
//! a family name such as `"PingFang SC"`, `"Inter"`, or an already installed
//! system family such as `"Segoe UI"`. This module keeps those steps explicit so
//! a packaged application can mount large font files next to the executable while
//! a bare executable can still fall back to small embedded font bytes.
//!
//! Important: GPUI's `add_fonts` API reports transport/registration errors, but
//! some platform backends silently ignore font bytes they cannot parse. Use
//! [`FontLoadOptions::require_family`] when the application must know that a
//! selected family, such as `"PingFang SC"`, is actually visible after loading.

use gpui::{App, SharedString};
use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

/// Font file extensions that Liora will try to register with GPUI.
///
/// GPUI accepts raw font bytes and delegates parsing to its platform text
/// backend: font-kit/CoreText on macOS, DirectWrite on Windows, and
/// cosmic-text/fontdb on Linux. The exact parser support can vary by backend.
/// For maximum native compatibility prefer `ttf`, `otf`, `ttc`, or `otc`; web
/// formats are accepted as inputs but must be verified with
/// [`FontLoadOptions::require_family`] because a backend can ignore bytes it
/// cannot parse without returning an error.
pub const SUPPORTED_FONT_EXTENSIONS: &[&str] = &["ttf", "otf", "ttc", "otc", "woff", "woff2"];

/// Controls which resource location is used when loading app fonts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontLoadMode {
    /// Only load fonts that were embedded in the executable with `include_bytes!`.
    Embedded,
    /// Only load fonts from external filesystem directories or GPUI assets.
    External,
    /// Prefer external mounted files/assets and fall back to embedded bytes when
    /// no external font could be registered.
    ExternalThenEmbedded,
    /// Always register both external and embedded fonts. Use this when the app
    /// intentionally ships different families in different locations.
    Mixed,
}

impl Default for FontLoadMode {
    fn default() -> Self {
        Self::ExternalThenEmbedded
    }
}

/// One embedded font included in the executable.
#[derive(Clone, Debug)]
pub struct EmbeddedFont {
    /// Human-readable source name used in reports and logs.
    pub name: SharedString,
    /// Raw font bytes, commonly produced by `include_bytes!`.
    pub bytes: Cow<'static, [u8]>,
}

impl EmbeddedFont {
    /// Creates an embedded font from static or owned bytes.
    pub fn new(name: impl Into<SharedString>, bytes: impl Into<Cow<'static, [u8]>>) -> Self {
        Self {
            name: name.into(),
            bytes: bytes.into(),
        }
    }
}

/// Options used by [`load_app_fonts`] to combine embedded and mounted fonts.
#[derive(Clone, Debug, Default)]
pub struct FontLoadOptions {
    /// Resource selection policy.
    pub mode: FontLoadMode,
    /// External directories scanned recursively for supported font files.
    pub external_dirs: Vec<PathBuf>,
    /// Font asset paths resolved through the current GPUI [`gpui::AssetSource`].
    pub asset_paths: Vec<SharedString>,
    /// Embedded fallback fonts bundled into the executable.
    pub embedded_fonts: Vec<EmbeddedFont>,
    /// Family names that must be visible after loading.
    ///
    /// This is stronger than checking `FontLoadReport.loaded`: a GPUI backend can
    /// accept bytes and still fail to expose a family if the file format is not
    /// supported on that platform.
    pub required_families: Vec<SharedString>,
}

impl FontLoadOptions {
    /// Creates an option set for the supplied resource mode.
    pub fn new(mode: FontLoadMode) -> Self {
        Self {
            mode,
            external_dirs: Vec::new(),
            asset_paths: Vec::new(),
            embedded_fonts: Vec::new(),
            required_families: Vec::new(),
        }
    }

    /// Adds a recursively scanned external font directory.
    pub fn external_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.external_dirs.push(dir.into());
        self
    }

    /// Adds a GPUI asset path such as `"fonts/PingFangSC-Regular.ttf"`.
    pub fn asset_path(mut self, path: impl Into<SharedString>) -> Self {
        self.asset_paths.push(path.into());
        self
    }

    /// Adds one embedded font file to the fallback set.
    pub fn embedded(
        mut self,
        name: impl Into<SharedString>,
        bytes: impl Into<Cow<'static, [u8]>>,
    ) -> Self {
        self.embedded_fonts.push(EmbeddedFont::new(name, bytes));
        self
    }

    /// Requires a family name to be visible after loading completes.
    ///
    /// `ExternalThenEmbedded` uses this list to decide whether external files
    /// really satisfied the selected typography. If an external source returns
    /// `Ok(())` but the family is still absent, embedded fallback fonts are tried
    /// before the final report is produced.
    pub fn require_family(mut self, family: impl Into<SharedString>) -> Self {
        self.required_families.push(family.into());
        self
    }
}

/// Files discovered under an external font directory before GPUI registration.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FontDiscoveryReport {
    /// Supported font files found recursively, sorted for deterministic loading.
    pub font_files: Vec<PathBuf>,
    /// Regular files skipped because their extension is not in
    /// [`SUPPORTED_FONT_EXTENSIONS`].
    pub skipped_unsupported: usize,
}

/// One font source that could not be read or registered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontLoadFailure {
    /// File path, asset path, or embedded source label.
    pub source: String,
    /// Error message returned by the filesystem, asset source, or GPUI backend.
    pub error: String,
}

/// Summary returned after attempting to load app fonts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FontLoadReport {
    /// Number of font faces or font files passed to GPUI successfully.
    pub loaded: usize,
    /// External files skipped because their extension is not supported.
    pub skipped_unsupported: usize,
    /// External directories that do not exist. Missing directories are not fatal
    /// because packaged and source-tree layouts often differ.
    pub missing_external_dirs: Vec<PathBuf>,
    /// Sources that existed but failed to read, resolve, or register.
    pub failures: Vec<FontLoadFailure>,
    /// Required families that are still not visible to GPUI after all selected
    /// sources and fallbacks have been attempted.
    pub missing_required_families: Vec<SharedString>,
}

impl FontLoadReport {
    /// Returns `true` when at least one font source was passed to GPUI without
    /// a transport-level error.
    ///
    /// This does not prove a specific family became available. Use
    /// [`FontLoadOptions::require_family`] and inspect
    /// [`FontLoadReport::missing_required_families`] for that stronger check.
    pub fn loaded_any(&self) -> bool {
        self.loaded > 0
    }

    /// Returns `true` when all families listed with
    /// [`FontLoadOptions::require_family`] were visible after loading.
    pub fn required_families_available(&self) -> bool {
        self.missing_required_families.is_empty()
    }

    fn extend(&mut self, other: Self) {
        self.loaded += other.loaded;
        self.skipped_unsupported += other.skipped_unsupported;
        self.missing_external_dirs
            .extend(other.missing_external_dirs);
        self.failures.extend(other.failures);
        self.missing_required_families
            .extend(other.missing_required_families);
    }

    fn failure(&mut self, source: impl Into<String>, error: impl ToString) {
        self.failures.push(FontLoadFailure {
            source: source.into(),
            error: error.to_string(),
        });
    }
}

/// Returns whether the path has a font extension Liora should try to register.
pub fn is_supported_font_path(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            SUPPORTED_FONT_EXTENSIONS
                .iter()
                .any(|supported| extension.eq_ignore_ascii_case(supported))
        })
        .unwrap_or(false)
}

/// Recursively discovers supported font files under `dir`.
pub fn discover_font_files(dir: impl AsRef<Path>) -> std::io::Result<FontDiscoveryReport> {
    let mut report = FontDiscoveryReport::default();
    discover_font_files_inner(dir.as_ref(), &mut report)?;
    report.font_files.sort();
    Ok(report)
}

fn discover_font_files_inner(dir: &Path, report: &mut FontDiscoveryReport) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            discover_font_files_inner(&path, report)?;
        } else if file_type.is_file() {
            if is_supported_font_path(&path) {
                report.font_files.push(path);
            } else {
                report.skipped_unsupported += 1;
            }
        }
    }
    Ok(())
}

/// Registers embedded font bytes with GPUI.
pub fn load_embedded_fonts(
    cx: &mut App,
    fonts: impl IntoIterator<Item = EmbeddedFont>,
) -> FontLoadReport {
    let mut report = FontLoadReport::default();
    for font in fonts {
        match cx.text_system().add_fonts(vec![font.bytes]) {
            Ok(()) => report.loaded += 1,
            Err(error) => report.failure(font.name.to_string(), error),
        }
    }
    report
}

/// Reads and registers explicit font files.
pub fn load_font_files(cx: &mut App, paths: impl IntoIterator<Item = PathBuf>) -> FontLoadReport {
    let mut report = FontLoadReport::default();
    for path in paths {
        if !is_supported_font_path(&path) {
            report.skipped_unsupported += 1;
            continue;
        }
        match fs::read(&path) {
            Ok(bytes) => match cx.text_system().add_fonts(vec![Cow::Owned(bytes)]) {
                Ok(()) => report.loaded += 1,
                Err(error) => report.failure(path.display().to_string(), error),
            },
            Err(error) => report.failure(path.display().to_string(), error),
        }
    }
    report
}

/// Recursively reads and registers fonts from an external directory.
pub fn load_fonts_from_dir(cx: &mut App, dir: impl AsRef<Path>) -> FontLoadReport {
    let dir = dir.as_ref();
    if !dir.exists() {
        return FontLoadReport {
            missing_external_dirs: vec![dir.to_path_buf()],
            ..Default::default()
        };
    }
    match discover_font_files(dir) {
        Ok(discovery) => {
            let mut report = load_font_files(cx, discovery.font_files);
            report.skipped_unsupported += discovery.skipped_unsupported;
            report
        }
        Err(error) => {
            let mut report = FontLoadReport::default();
            report.failure(dir.display().to_string(), error);
            report
        }
    }
}

/// Resolves and registers font bytes from GPUI's configured asset source.
pub fn load_font_assets(
    cx: &mut App,
    paths: impl IntoIterator<Item = SharedString>,
) -> FontLoadReport {
    let mut report = FontLoadReport::default();
    let asset_source = cx.asset_source().clone();
    for path in paths {
        if !is_supported_font_path(path.as_ref()) {
            report.skipped_unsupported += 1;
            continue;
        }
        match asset_source.load(path.as_ref()) {
            Ok(Some(bytes)) => match cx.text_system().add_fonts(vec![bytes]) {
                Ok(()) => report.loaded += 1,
                Err(error) => report.failure(path.to_string(), error),
            },
            Ok(None) => report.failure(path.to_string(), "asset not found"),
            Err(error) => report.failure(path.to_string(), error),
        }
    }
    report
}

/// Loads application fonts according to `options`.
pub fn load_app_fonts(cx: &mut App, options: FontLoadOptions) -> FontLoadReport {
    let required_families = options.required_families.clone();
    let mut report = match options.mode {
        FontLoadMode::Embedded => load_embedded_fonts(cx, options.embedded_fonts),
        FontLoadMode::External => {
            load_external_fonts(cx, options.external_dirs, options.asset_paths)
        }
        FontLoadMode::Mixed => {
            let mut report = load_external_fonts(cx, options.external_dirs, options.asset_paths);
            report.extend(load_embedded_fonts(cx, options.embedded_fonts));
            report
        }
        FontLoadMode::ExternalThenEmbedded => {
            let mut report = load_external_fonts(cx, options.external_dirs, options.asset_paths);
            let missing_after_external = missing_required_families(cx, &required_families);
            let should_try_embedded = if required_families.is_empty() {
                !report.loaded_any()
            } else {
                !missing_after_external.is_empty()
            };

            if should_try_embedded {
                report.extend(load_embedded_fonts(cx, options.embedded_fonts));
            }
            report
        }
    };

    report.missing_required_families = missing_required_families(cx, &required_families);
    report
}

fn load_external_fonts(
    cx: &mut App,
    external_dirs: Vec<PathBuf>,
    asset_paths: Vec<SharedString>,
) -> FontLoadReport {
    let mut report = FontLoadReport::default();
    for dir in external_dirs {
        report.extend(load_fonts_from_dir(cx, dir));
    }
    report.extend(load_font_assets(cx, asset_paths));
    report
}

/// Returns whether the named family is currently visible to GPUI.
///
/// This works for system-installed families and for memory fonts after they are
/// registered. It is a diagnostic helper; applications may still set a family
/// optimistically and rely on GPUI's fallback stack if a platform reports names
/// differently.
pub fn is_font_family_available(cx: &App, family: &str) -> bool {
    cx.text_system()
        .all_font_names()
        .iter()
        .any(|name| name == family)
}

fn missing_required_families(cx: &App, families: &[SharedString]) -> Vec<SharedString> {
    families
        .iter()
        .filter(|family| !is_font_family_available(cx, family.as_ref()))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn supported_font_extensions_cover_native_and_web_font_files() {
        for path in [
            "Inter.ttf",
            "Inter.otf",
            "PingFang.ttc",
            "PingFang.otc",
            "Brand.woff",
            "Brand.woff2",
            "UpperCase.TTF",
        ] {
            assert!(is_supported_font_path(path), "{path} should be accepted");
        }

        for path in ["README.md", "font.txt", "no-extension"] {
            assert!(!is_supported_font_path(path), "{path} should be rejected");
        }
    }

    #[test]
    fn discover_font_files_recurses_and_skips_unsupported_files() {
        let root = temp_dir("liora-font-discovery");
        let nested = root.join("nested");
        fs::create_dir_all(&nested).unwrap();
        fs::write(root.join("PingFangSC-Regular.ttf"), b"fake").unwrap();
        fs::write(nested.join("PingFangSC-Regular.woff2"), b"fake").unwrap();
        fs::write(nested.join("README.md"), b"ignore").unwrap();

        let report = discover_font_files(&root).unwrap();
        let names = report
            .font_files
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec!["PingFangSC-Regular.ttf", "PingFangSC-Regular.woff2"]
        );
        assert_eq!(report.skipped_unsupported, 1);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn load_options_default_to_external_then_embedded_for_package_friendly_apps() {
        let options = FontLoadOptions::new(FontLoadMode::ExternalThenEmbedded)
            .external_dir("assets/fonts")
            .embedded("Inter-Regular.ttf", b"font-bytes" as &'static [u8])
            .require_family("Inter");

        assert_eq!(options.mode, FontLoadMode::ExternalThenEmbedded);
        assert_eq!(options.external_dirs.len(), 1);
        assert_eq!(options.embedded_fonts.len(), 1);
        assert_eq!(
            options.required_families.as_slice(),
            [SharedString::from("Inter")]
        );
    }

    #[test]
    fn report_tracks_missing_required_families() {
        let report = FontLoadReport {
            missing_required_families: vec![SharedString::from("PingFang SC")],
            ..Default::default()
        };

        assert!(!report.required_families_available());
    }

    fn temp_dir(label: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{label}-{unique}"))
    }
}
