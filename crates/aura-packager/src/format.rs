#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Linux,
    Macos,
    Windows,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(target_os = "macos") {
            Self::Macos
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else {
            Self::Linux
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Linux => "linux",
            Self::Macos => "macos",
            Self::Windows => "windows",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    AppImage,
    Deb,
    Rpm,
    TarGz,
    App,
    Dmg,
    Nsis,
    Msi,
    PlatformDefaults,
}

impl PackageFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppImage => "appimage",
            Self::Deb => "deb",
            Self::Rpm => "rpm",
            Self::TarGz => "tar.gz",
            Self::App => "app",
            Self::Dmg => "dmg",
            Self::Nsis => "nsis",
            Self::Msi => "msi",
            Self::PlatformDefaults => "platform-defaults",
        }
    }

    pub fn defaults_for(platform: Platform) -> &'static [Self] {
        match platform {
            Platform::Linux => &[Self::AppImage, Self::Deb, Self::Rpm, Self::TarGz],
            Platform::Macos => &[Self::App, Self::Dmg],
            Platform::Windows => &[Self::Nsis, Self::Msi],
        }
    }

    pub fn cargo_packager_format(self) -> Option<&'static str> {
        match self {
            Self::AppImage => Some("appimage"),
            Self::Deb => Some("deb"),
            Self::TarGz => Some("pacman"),
            Self::App => Some("app"),
            Self::Dmg => Some("dmg"),
            Self::Nsis => Some("nsis"),
            Self::Msi => Some("wix"),
            Self::Rpm | Self::PlatformDefaults => None,
        }
    }

    pub fn from_artifact_path(path: &std::path::Path) -> Option<Self> {
        let file_name = path.file_name()?.to_string_lossy().to_ascii_lowercase();
        let extension = path.extension()?.to_string_lossy().to_ascii_lowercase();
        if file_name.ends_with(".appimage") {
            Some(Self::AppImage)
        } else if file_name.ends_with(".tar.gz") || file_name.ends_with(".pkg.tar.zst") {
            Some(Self::TarGz)
        } else {
            match extension.as_str() {
                "deb" => Some(Self::Deb),
                "rpm" => Some(Self::Rpm),
                "app" => Some(Self::App),
                "dmg" => Some(Self::Dmg),
                "exe" => Some(Self::Nsis),
                "msi" => Some(Self::Msi),
                _ => None,
            }
        }
    }
}

impl std::str::FromStr for PackageFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "appimage" | "AppImage" => Ok(Self::AppImage),
            "deb" => Ok(Self::Deb),
            "rpm" => Ok(Self::Rpm),
            "tar.gz" | "tgz" | "archive" => Ok(Self::TarGz),
            "app" => Ok(Self::App),
            "dmg" => Ok(Self::Dmg),
            "nsis" | "exe" => Ok(Self::Nsis),
            "msi" => Ok(Self::Msi),
            "platform-defaults" | "defaults" => Ok(Self::PlatformDefaults),
            other => Err(format!("unknown package format '{other}'")),
        }
    }
}
