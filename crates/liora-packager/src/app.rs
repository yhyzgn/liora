use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnownApp {
    Gallery,
    Docs,
}

impl KnownApp {
    pub fn key(self) -> &'static str {
        match self {
            Self::Gallery => "gallery",
            Self::Docs => "docs",
        }
    }

    pub fn package(self) -> &'static str {
        match self {
            Self::Gallery => "liora-gallery",
            Self::Docs => "liora-docs",
        }
    }

    pub fn binary(self) -> &'static str {
        self.package()
    }

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
pub struct AppId(String);

impl AppId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppMetadata {
    pub app: KnownApp,
    pub id: AppId,
    pub name: String,
    pub binary: String,
    pub package: String,
    pub category: String,
    pub short_description: String,
    pub icon_stem: String,
}

impl AppMetadata {
    pub fn packager_config_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join(format!("Packager.{}.toml", self.app.key()))
    }

    pub fn linux_desktop_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("linux")
            .join(format!("{}.desktop", self.binary))
    }

    pub fn linux_metainfo_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("linux")
            .join(format!("{}.metainfo.xml", self.binary))
    }

    pub fn icon_png_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("icons")
            .join(format!("{}.png", self.icon_stem))
    }

    pub fn icon_icns_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("icons")
            .join(format!("{}.icns", self.icon_stem))
    }

    pub fn icon_ico_path(&self, root: &Path) -> PathBuf {
        root.join("packaging")
            .join("icons")
            .join(format!("{}.ico", self.icon_stem))
    }
}

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
