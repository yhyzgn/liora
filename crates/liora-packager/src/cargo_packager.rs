use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{AppMetadata, PackageFormat, Platform};

pub const LINUX_DEB_RUNTIME_DEPENDENCIES: &[&str] = &[
    "libgtk-3-0",
    "libayatana-appindicator3-1",
    "libx11-6",
    "libwayland-client0",
    "libxkbcommon0",
    "libfontconfig1",
    "libfreetype6",
    "libvulkan1",
    "libasound2",
    "xdg-utils",
];

/// Controls whether generated packages include application font resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontVariant {
    /// Build/package without bundling application font files.
    WithoutFonts,
    /// Build/package with application font files included as external resources.
    WithFonts,
}

impl FontVariant {
    /// Returns the stable release-asset suffix for this font variant.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WithoutFonts => "without-fonts",
            Self::WithFonts => "with-fonts",
        }
    }

    /// Returns whether external font resources should be bundled.
    pub fn includes_fonts(self) -> bool {
        matches!(self, Self::WithFonts)
    }
}

impl Default for FontVariant {
    fn default() -> Self {
        Self::WithoutFonts
    }
}

impl std::str::FromStr for FontVariant {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "without-fonts" | "no-fonts" | "default" => Ok(Self::WithoutFonts),
            "with-fonts" | "fonts" => Ok(Self::WithFonts),
            other => Err(format!(
                "unknown font variant '{other}', expected 'without-fonts' or 'with-fonts'"
            )),
        }
    }
}

pub const LINUX_RPM_RUNTIME_DEPENDENCIES: &[(&str, &str)] = &[
    ("gtk3", "*"),
    ("libayatana-appindicator-gtk3", "*"),
    ("libX11", "*"),
    ("wayland", "*"),
    ("libxkbcommon", "*"),
    ("fontconfig", "*"),
    ("freetype", "*"),
    ("vulkan-loader", "*"),
    ("alsa-lib", "*"),
    ("xdg-utils", "*"),
];

/// A generated cargo-packager invocation plan for one host application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoPackagerPlan {
    /// Application metadata associated with this package artifact.
    pub app: AppMetadata,
    /// Platform targeted by this package, artifact, or update plan.
    pub platform: Platform,
    /// Package formats that should be generated for this plan.
    pub formats: Vec<PackageFormat>,
    /// Path where the generated cargo-packager configuration is written.
    pub config_path: PathBuf,
    /// Directory where package artifacts are expected to be produced.
    pub out_dir: PathBuf,
    /// Directory containing release-mode binaries to package.
    pub binaries_dir: PathBuf,
}

impl CargoPackagerPlan {
    /// Builds the `cargo packager` command-line arguments for this package plan.
    pub fn command_args(&self) -> Vec<String> {
        let mut args = vec![
            "packager".to_string(),
            "--config".to_string(),
            self.config_path.display().to_string(),
            "--out-dir".to_string(),
            self.out_dir.display().to_string(),
            "--binaries-dir".to_string(),
            self.binaries_dir.display().to_string(),
        ];

        let cargo_formats = self
            .formats
            .iter()
            .filter_map(|format| format.cargo_packager_format())
            .collect::<Vec<_>>()
            .join(",");
        if !cargo_formats.is_empty() {
            args.push("--formats".to_string());
            args.push(cargo_formats);
        }
        args
    }
}

/// Returns the subset of formats handled by cargo-packager directly.
pub fn cargo_packager_formats(formats: &[PackageFormat]) -> Vec<PackageFormat> {
    formats
        .iter()
        .copied()
        .filter(|format| format.cargo_packager_format().is_some())
        .collect()
}

/// Returns the subset of formats that require supplemental backends.
pub fn supplemental_formats(formats: &[PackageFormat]) -> Vec<PackageFormat> {
    formats
        .iter()
        .copied()
        .filter(|format| format.cargo_packager_format().is_none())
        .collect()
}

/// Returns the path for the generated cargo-packager configuration file.
pub fn generated_config_path(root: &Path, app: &AppMetadata) -> PathBuf {
    root.join("target")
        .join("liora-packager")
        .join(format!("Packager.{}.toml", app.key()))
}

/// Returns the path for the generated cargo-packager configuration file for a font variant.
pub fn generated_config_path_for(
    root: &Path,
    app: &AppMetadata,
    font_variant: FontVariant,
) -> PathBuf {
    root.join("target").join("liora-packager").join(format!(
        "Packager.{}.{}.toml",
        app.key(),
        font_variant.as_str()
    ))
}

/// Returns the directory where package artifacts for an app and platform are written.
pub fn package_out_dir(root: &Path, app: &AppMetadata, platform: Platform) -> PathBuf {
    root.join("target")
        .join("packages")
        .join(&app.package)
        .join(platform.as_str())
}

/// Returns the package output directory for an app, platform, and font variant.
pub fn package_out_dir_for(
    root: &Path,
    app: &AppMetadata,
    platform: Platform,
    font_variant: FontVariant,
) -> PathBuf {
    root.join("target")
        .join("packages")
        .join(&app.package)
        .join(platform.as_str())
        .join(font_variant.as_str())
}

/// Returns the staging directory used for release-mode raw binaries.
pub fn release_binaries_dir(root: &Path) -> PathBuf {
    root.join("target").join("release")
}

/// Returns the generated RPM metadata path for the selected app.
pub fn generated_rpm_config_path(root: &Path, app: &AppMetadata) -> PathBuf {
    root.join("target")
        .join("liora-packager")
        .join(format!("GenerateRpm.{}.toml", app.key()))
}

/// Returns the generated RPM metadata path for the selected app and font variant.
pub fn generated_rpm_config_path_for(
    root: &Path,
    app: &AppMetadata,
    font_variant: FontVariant,
) -> PathBuf {
    root.join("target").join("liora-packager").join(format!(
        "GenerateRpm.{}.{}.toml",
        app.key(),
        font_variant.as_str()
    ))
}

fn hicolor_icon_sizes() -> [u16; 8] {
    [16, 24, 32, 48, 64, 128, 256, 512]
}

/// Renders the render generate rpm config layer into native GPUI elements.
pub fn render_generate_rpm_config(root: &Path, app: &AppMetadata) -> String {
    render_generate_rpm_config_for(root, app, FontVariant::WithoutFonts)
}

/// Renders generate-rpm config for the selected font variant.
pub fn render_generate_rpm_config_for(
    root: &Path,
    app: &AppMetadata,
    font_variant: FontVariant,
) -> String {
    let mut out = String::new();
    line(
        &mut out,
        "# Generated by cargo xtask package. Do not edit by hand.",
    );
    line(&mut out, "[package.metadata.generate-rpm]");
    kv(&mut out, "name", &app.package);
    kv(&mut out, "version", &rpm_package_version());
    if let Some(license) = &app.license {
        kv(&mut out, "license", license);
    }
    kv(&mut out, "summary", &app.short_description);
    if let Some(homepage) = &app.homepage {
        kv(&mut out, "url", homepage);
    }
    if let Some(publisher) = &app.publisher {
        kv(&mut out, "vendor", publisher);
    }
    kv(&mut out, "release", "1");
    kv(&mut out, "auto-req", "builtin");
    line(&mut out, "require-sh = false");

    line(&mut out, "assets = [");
    rpm_asset(
        &mut out,
        &root.join("target/release").join(&app.binary),
        &format!("/usr/bin/{}", app.binary),
        "755",
    );
    rpm_asset(
        &mut out,
        &app.linux_desktop_path(root),
        &format!("/usr/share/applications/{}.desktop", app.binary),
        "644",
    );
    rpm_asset(
        &mut out,
        &app.linux_metainfo_path(root),
        &format!("/usr/share/metainfo/{}.metainfo.xml", app.binary),
        "644",
    );
    for size in hicolor_icon_sizes() {
        rpm_asset(
            &mut out,
            &app.hicolor_png_path(root, size),
            &format!(
                "/usr/share/icons/hicolor/{size}x{size}/apps/{}.png",
                app.binary
            ),
            "644",
        );
    }
    rpm_asset(
        &mut out,
        &root
            .join("packaging/icons")
            .join(format!("{}.svg", app.icon_stem)),
        &format!("/usr/share/icons/hicolor/scalable/apps/{}.svg", app.binary),
        "644",
    );
    if font_variant.includes_fonts() {
        append_rpm_font_assets(&mut out, root, app);
    }
    line(&mut out, "]");

    line(&mut out, "");
    line(&mut out, "[package.metadata.generate-rpm.requires]");
    for (package, version) in LINUX_RPM_RUNTIME_DEPENDENCIES {
        kv(&mut out, package, version);
    }
    out
}

fn rpm_asset(out: &mut String, source: &Path, dest: &str, mode: &str) {
    out.push_str("  { source = \"");
    out.push_str(&escape(&source.display().to_string()));
    out.push_str("\", dest = \"");
    out.push_str(&escape(dest));
    out.push_str("\", mode = \"");
    out.push_str(mode);
    out.push_str("\" },\n");
}

/// Renders the render cargo packager config layer into native GPUI elements.
pub fn render_cargo_packager_config(
    root: &Path,
    app: &AppMetadata,
    formats: &[PackageFormat],
    out_dir: &Path,
    binaries_dir: &Path,
) -> String {
    render_cargo_packager_config_for(
        root,
        app,
        formats,
        out_dir,
        binaries_dir,
        FontVariant::WithoutFonts,
    )
}

/// Renders cargo-packager config for the selected font variant.
pub fn render_cargo_packager_config_for(
    root: &Path,
    app: &AppMetadata,
    formats: &[PackageFormat],
    out_dir: &Path,
    binaries_dir: &Path,
    font_variant: FontVariant,
) -> String {
    let cargo_formats = cargo_packager_formats(formats)
        .into_iter()
        .filter_map(PackageFormat::cargo_packager_format)
        .collect::<Vec<_>>();

    let mut out = String::new();
    line(
        &mut out,
        "# Generated by cargo xtask package. Do not edit by hand.",
    );
    kv(&mut out, "name", &app.package);
    kv(&mut out, "productName", &app.name);
    kv(&mut out, "version", &package_version());
    kv(&mut out, "identifier", app.id.as_str());
    kv(&mut out, "description", &app.short_description);
    if let Some(homepage) = &app.homepage {
        kv(&mut out, "homepage", homepage);
    }
    if !app.authors.is_empty() {
        let authors = app.authors.iter().map(String::as_str).collect::<Vec<_>>();
        arr(&mut out, "authors", &authors);
    }
    if let Some(publisher) = &app.publisher {
        kv(&mut out, "publisher", publisher);
    }
    if let Some(copyright) = &app.copyright {
        kv(&mut out, "copyright", copyright);
    }
    kv(&mut out, "category", &app.category);
    arr(&mut out, "formats", &cargo_formats);
    path_kv(&mut out, "outDir", out_dir);
    path_kv(&mut out, "binariesDir", binaries_dir);
    let mut icon_paths = app
        .hicolor_png_paths(root)
        .into_iter()
        .map(abs)
        .collect::<Vec<_>>();
    icon_paths.push(abs(app.icon_png_path(root)));
    icon_paths.push(abs(app.icon_icns_path(root)));
    icon_paths.push(abs(app.icon_ico_path(root)));
    let icon_refs = icon_paths.iter().map(String::as_str).collect::<Vec<_>>();
    arr(&mut out, "icons", &icon_refs);
    let font_assets_dir = app.app_assets_fonts_path(root);
    if font_variant.includes_fonts() && font_assets_dir.is_dir() {
        resources(&mut out, &[(&font_assets_dir, Path::new("assets/fonts"))]);
    }

    line(&mut out, "");
    line(&mut out, "[[binaries]]");
    kv(&mut out, "path", &app.binary);
    line(&mut out, "main = true");

    line(&mut out, "");
    line(&mut out, "[linux]");
    line(&mut out, "generateDesktopEntry = true");

    line(&mut out, "");
    line(&mut out, "[deb]");
    path_kv(&mut out, "desktopTemplate", &app.linux_desktop_path(root));
    kv(&mut out, "section", "devel");
    kv(&mut out, "priority", "optional");
    kv(&mut out, "packageName", &app.package);
    arr(&mut out, "depends", LINUX_DEB_RUNTIME_DEPENDENCIES);

    line(&mut out, "");
    line(&mut out, "[macos]");
    path_kv(
        &mut out,
        "entitlements",
        &root.join("packaging/macos/Entitlements.plist"),
    );

    line(&mut out, "");
    line(&mut out, "[nsis]");
    path_kv(
        &mut out,
        "installerIcon",
        &root
            .join("packaging/icons")
            .join(format!("{}.ico", app.icon_stem)),
    );
    line(&mut out, "installMode = \"currentUser\"");

    out
}

fn append_rpm_font_assets(out: &mut String, root: &Path, app: &AppMetadata) {
    let fonts_dir = app.app_assets_fonts_path(root);
    if !fonts_dir.is_dir() {
        return;
    }

    let Ok(mut files) = collect_regular_files(&fonts_dir) else {
        return;
    };
    files.sort();

    for file in files {
        if let Ok(relative) = file.strip_prefix(&fonts_dir) {
            rpm_asset(
                out,
                &file,
                &format!(
                    "/usr/lib/{}/assets/fonts/{}",
                    app.binary,
                    relative.display()
                ),
                "644",
            );
        }
    }
}

fn collect_regular_files(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            files.extend(collect_regular_files(&path)?);
        } else if file_type.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}

fn package_version() -> String {
    env::var("LIORA_PACKAGE_VERSION").unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string())
}

fn rpm_package_version() -> String {
    // RPM version strings cannot contain hyphens because '-' separates version and release.
    package_version().replace('-', "_")
}

fn abs(path: PathBuf) -> String {
    path.display().to_string()
}

fn line(out: &mut String, value: &str) {
    out.push_str(value);
    out.push('\n');
}

fn kv(out: &mut String, key: &str, value: &str) {
    out.push_str(key);
    out.push_str(" = \"");
    out.push_str(&escape(value));
    out.push_str("\"\n");
}

fn path_kv(out: &mut String, key: &str, value: &Path) {
    kv(out, key, &value.display().to_string());
}

fn arr(out: &mut String, key: &str, values: &[&str]) {
    out.push_str(key);
    out.push_str(" = [");
    for (idx, value) in values.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(&escape(value));
        out.push('"');
    }
    out.push_str("]\n");
}

fn resources(out: &mut String, values: &[(&Path, &Path)]) {
    out.push_str("resources = [");
    for (idx, (src, target)) in values.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push_str("{ src = \"");
        out.push_str(&escape(&src.display().to_string()));
        out.push_str("\", target = \"");
        out.push_str(&escape(&target.display().to_string()));
        out.push_str("\" }");
    }
    out.push_str("]\n");
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_app() -> AppMetadata {
        AppMetadata::new(
            "sample",
            "dev.example.Sample",
            "Sample App",
            "sample-app",
            "sample-app",
            "Utility",
            "Sample native app.",
            "sample-app",
        )
        .with_license("MIT")
        .with_homepage("https://example.dev/sample")
        .with_authors(["Example Contributors"])
        .with_publisher("Example")
        .with_copyright("Copyright © Example Contributors")
    }

    fn test_root(name: &str) -> PathBuf {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("target")
            .join("liora-packager-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("apps/sample-app/assets/fonts"))
            .expect("create sample font fixture");
        std::fs::write(
            root.join("apps/sample-app/assets/fonts/SampleSans-Regular.ttf"),
            b"sample font fixture",
        )
        .expect("write sample font fixture");
        root
    }

    #[test]
    fn renders_cargo_packager_config_with_binary_and_icons() {
        let root = Path::new("/repo/liora");
        let app = sample_app();
        let text = render_cargo_packager_config(
            root,
            &app,
            &[
                PackageFormat::Deb,
                PackageFormat::Rpm,
                PackageFormat::AppImage,
            ],
            Path::new("/repo/project/target/packages/sample-app/linux"),
            Path::new("/repo/liora/target/release"),
        );
        assert!(text.contains("productName = \"Sample App\""));
        assert!(text.contains("homepage = \"https://example.dev/sample\""));
        assert!(text.contains("authors = [\"Example Contributors\"]"));
        assert!(text.contains("publisher = \"Example\""));
        assert!(text.contains("copyright = \"Copyright © Example Contributors\""));
        assert!(text.contains("formats = [\"deb\", \"appimage\"]"));
        assert!(text.contains("depends = [\"libgtk-3-0\""));
        assert!(text.contains("\"libayatana-appindicator3-1\""));
        assert!(text.contains("\"libvulkan1\""));
        assert!(text.contains("[[binaries]]"));
        assert!(text.contains("path = \"sample-app\""));
        assert!(text.contains("installerIcon"));
        assert!(text.contains("[nsis]"));
        assert!(text.contains("installMode = \"currentUser\""));
        assert!(!text.contains("fragmentPaths"));
        let normalized_text = normalized_test_paths(&text);
        assert!(normalized_text.contains("hicolor/16x16/apps/sample-app.png"));
        assert!(normalized_text.contains("hicolor/512x512/apps/sample-app.png"));
        assert!(!text.contains("assets/fonts"));
    }

    fn normalized_test_paths(text: &str) -> String {
        let mut normalized = text.replace('\\', "/");
        while normalized.contains("//") {
            normalized = normalized.replace("//", "/");
        }
        normalized
    }

    #[test]
    fn renders_cargo_packager_config_with_font_resources_when_requested() {
        let root = test_root("cargo-packager-fonts");
        let app = sample_app();
        let without_fonts = render_cargo_packager_config_for(
            &root,
            &app,
            &[PackageFormat::Deb],
            Path::new("target/packages/test"),
            Path::new("target/release"),
            FontVariant::WithoutFonts,
        );
        assert!(!without_fonts.contains("assets/fonts"));

        let text = render_cargo_packager_config_for(
            &root,
            &app,
            &[PackageFormat::Deb],
            Path::new("target/packages/test"),
            Path::new("target/release"),
            FontVariant::WithFonts,
        );

        assert!(text.contains("resources = [{ src ="));
        assert!(text.contains("assets/fonts"));
        assert!(!text.contains(r#"resources = ["{"#));
    }

    #[test]
    fn renders_generate_rpm_config_with_desktop_and_icons() {
        let root = Path::new("/repo/liora");
        let app = sample_app();
        let text = render_generate_rpm_config(root, &app);
        assert!(text.contains("[package.metadata.generate-rpm]"));
        assert!(text.contains("name = \"sample-app\""));
        assert!(text.contains("license = \"MIT\""));
        assert!(text.contains("url = \"https://example.dev/sample\""));
        assert!(text.contains("vendor = \"Example\""));
        assert!(text.contains("/usr/bin/sample-app"));
        assert!(text.contains("[package.metadata.generate-rpm.requires]"));
        assert!(text.contains("gtk3 = \"*\""));
        assert!(text.contains("vulkan-loader = \"*\""));
        assert!(text.contains("/usr/share/applications/sample-app.desktop"));
        assert!(text.contains("/usr/share/icons/hicolor/16x16/apps/sample-app.png"));
        assert!(text.contains("/usr/share/icons/hicolor/512x512/apps/sample-app.png"));
        assert!(text.contains("/usr/share/icons/hicolor/scalable/apps/sample-app.svg"));
        assert!(!text.contains("/usr/lib/sample-app/assets/fonts"));
    }

    #[test]
    fn renders_rpm_font_assets_only_when_requested() {
        let root = test_root("rpm-fonts");
        let app = sample_app();
        let without_fonts = render_generate_rpm_config_for(&root, &app, FontVariant::WithoutFonts);
        let with_fonts = render_generate_rpm_config_for(&root, &app, FontVariant::WithFonts);

        assert!(!without_fonts.contains("/usr/lib/sample-app/assets/fonts"));
        assert!(with_fonts.contains("/usr/lib/sample-app/assets/fonts/SampleSans-Regular.ttf"));
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn tar_gz_is_liora_supplemental_not_cargo_packager_pacman() {
        assert_eq!(PackageFormat::TarGz.cargo_packager_format(), None);
        assert_eq!(cargo_packager_formats(&[PackageFormat::TarGz]), Vec::new());
        assert_eq!(
            supplemental_formats(&[PackageFormat::TarGz]),
            vec![PackageFormat::TarGz]
        );
    }
}
