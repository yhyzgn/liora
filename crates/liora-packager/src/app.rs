use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Options that control known app behavior.
pub enum KnownApp {
    /// Identifies the Gallery application in packaging manifests.
    Gallery,
    /// Identifies the Docs application in packaging manifests.
    Docs,
}

impl KnownApp {
    /// Returns the stable key used to identify this value in callbacks or manifests.
    pub fn key(self) -> &'static str {
        match self {
            Self::Gallery => "gallery",
            Self::Docs => "docs",
        }
    }

    /// Returns the Cargo package name for this known application.
    pub fn package(self) -> &'static str {
        match self {
            Self::Gallery => "liora-gallery",
            Self::Docs => "liora-docs",
        }
    }

    /// Returns the release binary name for this known application.
    pub fn binary(self) -> &'static str {
        self.package()
    }

    /// Builds the full packaging metadata for this known application.
    pub fn metadata(self) -> AppMetadata {
        match self {
            Self::Gallery => AppMetadata {
                app: self,
                id: AppId::new("dev.liora.Gallery"),
                name: "Liora Gallery".into(),
                binary: self.binary().into(),
                package: self.package().into(),
                category: "DeveloperTool".into(),
                short_description: "Native GPUI component gallery for Liora.".into(),
                icon_stem: "liora-gallery".into(),
            },
            Self::Docs => AppMetadata {
                app: self,
                id: AppId::new("dev.liora.Docs"),
                name: "Liora Docs".into(),
                binary: self.binary().into(),
                package: self.package().into(),
                category: "DeveloperTool".into(),
                short_description: "Native GPUI documentation app for Liora.".into(),
                icon_stem: "liora-docs".into(),
            },
        }
    }
}

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
/// Packaging data model for app metadata.
pub struct AppMetadata {
    /// Application metadata associated with this package artifact.
    pub app: KnownApp,
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: AppId,
    /// Display name shown to users for this item.
    pub name: String,
    /// Executable binary name used by packaging backends.
    pub binary: String,
    /// Package identifier used by installer metadata.
    pub package: String,
    /// Desktop category used by Linux metadata and storefronts.
    pub category: String,
    /// Short package description shown by installer metadata.
    pub short_description: String,
    /// Base file name for the application icon resource.
    pub icon_stem: String,
}

impl AppMetadata {
    /// Returns the filesystem path for the packager config resource.
    pub fn packager_config_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join(format!("Packager.{}.toml", self.app.key()))
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
}

/// Returns the built-in Liora applications supported by the package pipeline.
pub fn known_apps() -> [KnownApp; 2] {
    [KnownApp::Gallery, KnownApp::Docs]
}

impl std::str::FromStr for KnownApp {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "gallery" | "liora-gallery" => Ok(Self::Gallery),
            "docs" | "liora-docs" => Ok(Self::Docs),
            other => Err(format!("unknown app '{other}'")),
        }
    }
}
