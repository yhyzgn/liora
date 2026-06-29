use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Packaging data model for app id.
pub struct AppId(String);

impl AppId {
    /// Creates `AppId` initialized from the supplied id.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the stable string representation for this value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Packaging data model for one host application.
pub struct AppMetadata {
    /// Stable key used for generated config names and CLI routing.
    pub key: String,
    /// Stable identifier used for desktop metadata and automation.
    pub id: AppId,
    /// Display name shown to users for this item.
    pub name: String,
    /// Executable binary name used by packaging backends.
    pub binary: String,
    /// Package identifier used by installer metadata.
    pub package: String,
    /// Repository-relative application package directory.
    pub app_dir: PathBuf,
    /// Desktop category used by Linux metadata and storefronts.
    pub category: String,
    /// Short package description shown by installer metadata.
    pub short_description: String,
    /// Base file name for the application icon resource.
    pub icon_stem: String,
    /// Optional package license identifier written to backend metadata.
    pub license: Option<String>,
    /// Optional project homepage written to backend metadata.
    pub homepage: Option<String>,
    /// Optional package authors written to backend metadata.
    pub authors: Vec<String>,
    /// Optional package publisher/vendor written to backend metadata.
    pub publisher: Option<String>,
    /// Optional copyright notice written to backend metadata.
    pub copyright: Option<String>,
}

impl AppMetadata {
    /// Creates package metadata for a host application.
    pub fn new(
        key: impl Into<String>,
        id: impl Into<String>,
        name: impl Into<String>,
        binary: impl Into<String>,
        package: impl Into<String>,
        category: impl Into<String>,
        short_description: impl Into<String>,
        icon_stem: impl Into<String>,
    ) -> Self {
        let binary = binary.into();
        Self {
            key: key.into(),
            id: AppId::new(id),
            name: name.into(),
            package: package.into(),
            app_dir: PathBuf::from("apps").join(&binary),
            binary,
            category: category.into(),
            short_description: short_description.into(),
            icon_stem: icon_stem.into(),
            license: None,
            homepage: None,
            authors: Vec::new(),
            publisher: None,
            copyright: None,
        }
    }

    /// Overrides the repository-relative application package directory.
    pub fn with_app_dir(mut self, app_dir: impl Into<PathBuf>) -> Self {
        self.app_dir = app_dir.into();
        self
    }

    /// Sets the package license identifier for generated backend metadata.
    pub fn with_license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Sets the package homepage URL for generated backend metadata.
    pub fn with_homepage(mut self, homepage: impl Into<String>) -> Self {
        self.homepage = Some(homepage.into());
        self
    }

    /// Sets the package author list for generated backend metadata.
    pub fn with_authors(mut self, authors: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.authors = authors.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the package publisher/vendor for generated backend metadata.
    pub fn with_publisher(mut self, publisher: impl Into<String>) -> Self {
        self.publisher = Some(publisher.into());
        self
    }

    /// Sets the package copyright notice for generated backend metadata.
    pub fn with_copyright(mut self, copyright: impl Into<String>) -> Self {
        self.copyright = Some(copyright.into());
        self
    }

    /// Returns the stable app key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the filesystem path for the application package directory.
    pub fn app_dir_path(&self, root: &Path) -> PathBuf {
        root.join(&self.app_dir)
    }

    /// Returns the filesystem path for the packager config resource.
    pub fn packager_config_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join(format!("Packager.{}.toml", self.key))
    }

    /// Returns the filesystem path for the linux desktop resource.
    pub fn linux_desktop_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("linux")
            .join(format!("{}.desktop", self.binary))
    }

    /// Returns the filesystem path for the linux metainfo resource.
    pub fn linux_metainfo_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("linux")
            .join(format!("{}.metainfo.xml", self.binary))
    }

    /// Returns the filesystem path for one hicolor PNG icon size.
    pub fn hicolor_png_path(&self, root: &Path, size: u16) -> PathBuf {
        root.join("packaging")
            .join("icons")
            .join("hicolor")
            .join(format!("{size}x{size}"))
            .join("apps")
            .join(format!("{}.png", self.icon_stem))
    }

    /// Returns every Linux hicolor PNG size shipped with installers.
    pub fn hicolor_png_paths(&self, root: &Path) -> Vec<PathBuf> {
        [16, 24, 32, 48, 64, 128, 256, 512]
            .into_iter()
            .map(|size| self.hicolor_png_path(root, size))
            .collect()
    }

    /// Returns the filesystem path for the app-owned font assets directory.
    pub fn app_assets_fonts_path(&self, root: &Path) -> PathBuf {
        self.app_dir_path(root).join("assets").join("fonts")
    }

    /// Returns the filesystem path for the Windows resource build script.
    pub fn windows_resource_build_script_path(&self, root: &Path) -> PathBuf {
        self.app_dir_path(root).join("build.rs")
    }

    /// Returns the filesystem path for the shared Windows application manifest.
    pub fn windows_common_controls_manifest_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("windows")
            .join("common-controls-v6.manifest")
    }

    /// Returns the repository-relative manifest path used by app build scripts.
    pub fn windows_common_controls_manifest_include_path(&self) -> &'static str {
        "../../packaging/windows/common-controls-v6.manifest"
    }

    /// Returns the filesystem path for the icon png resource.
    pub fn icon_png_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("icons")
            .join(format!("{}.png", self.icon_stem))
    }

    /// Returns the filesystem path for the icon icns resource.
    pub fn icon_icns_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("icons")
            .join(format!("{}.icns", self.icon_stem))
    }

    /// Returns the filesystem path for the icon ico resource.
    pub fn icon_ico_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("icons")
            .join(format!("{}.ico", self.icon_stem))
    }

    /// Returns the filesystem path for the icon svg resource.
    pub fn icon_svg_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("icons")
            .join(format!("{}.svg", self.icon_stem))
    }
}
