//! Safe updater planning for Liora desktop applications.
//!
//! This crate talks to the public GitHub Releases API for `yhyzgn/liora`,
//! selects release assets for the maintained Liora apps, downloads an asset into
//! caller-controlled cache/temp storage, verifies it against `SHA256SUMS.txt`,
//! and builds an explicit install plan. It never performs privileged or
//! system-wide installation as part of update checks or downloads.

use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fs::{self, File},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    time::Duration,
};
use thiserror::Error;

pub const DEFAULT_OWNER: &str = "yhyzgn";
pub const DEFAULT_REPO: &str = "liora";
pub const DEFAULT_API_BASE: &str = "https://api.github.com";
pub const CHECKSUMS_ASSET: &str = "SHA256SUMS.txt";
const DEFAULT_USER_AGENT: &str = concat!("liora-updater/", env!("CARGO_PKG_VERSION"));

/// Maintained release applications in the Liora repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LioraApp {
    Docs,
    Gallery,
}

impl LioraApp {
    pub fn release_name(self) -> &'static str {
        match self {
            Self::Docs => "liora-docs",
            Self::Gallery => "liora-gallery",
        }
    }
}

/// Release platform encoded in public GitHub release asset names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    LinuxX64,
    MacosArm64,
    WindowsX64,
}

impl Platform {
    pub fn current() -> Option<Self> {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("linux", "x86_64") => Some(Self::LinuxX64),
            ("macos", "aarch64") => Some(Self::MacosArm64),
            ("windows", "x86_64") => Some(Self::WindowsX64),
            _ => None,
        }
    }

    pub fn asset_fragment(self) -> &'static str {
        match self {
            Self::LinuxX64 => "linux-x64",
            Self::MacosArm64 => "macos-arm64",
            Self::WindowsX64 => "windows-x64",
        }
    }

    pub fn os_name(self) -> &'static str {
        match self {
            Self::LinuxX64 => "linux",
            Self::MacosArm64 => "macos",
            Self::WindowsX64 => "windows",
        }
    }
}

/// Preferred asset style for a selected app/platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetKind {
    /// Raw executable release asset.
    RawExecutable,
    /// Platform installer/package asset where available.
    Installer,
    /// Portable Linux archive.
    PortableArchive,
}

/// A GitHub release asset relevant to updates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseAsset {
    pub name: String,
    pub download_url: String,
    pub size: u64,
}

/// A GitHub release with parsed Liora assets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub tag: String,
    pub name: Option<String>,
    pub prerelease: bool,
    pub draft: bool,
    pub assets: Vec<ReleaseAsset>,
}

impl Release {
    pub fn version(&self) -> Option<Version> {
        Version::parse_tag(&self.tag)
    }

    pub fn checksum_asset(&self) -> Option<&ReleaseAsset> {
        self.assets
            .iter()
            .find(|asset| asset.name == CHECKSUMS_ASSET)
    }
}

/// Minimal semver-ish `vX.Y.Z` version used by Liora release tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    pub fn parse_tag(tag: &str) -> Option<Self> {
        let version = tag.strip_prefix('v').unwrap_or(tag);
        let core = version
            .split_once(['-', '+'])
            .map_or(version, |(core, _)| core);
        let mut parts = core.split('.');
        let major = parts.next()?.parse().ok()?;
        let minor = parts.next()?.parse().ok()?;
        let patch = parts.next()?.parse().ok()?;
        if parts.next().is_some() {
            return None;
        }
        Some(Self {
            major,
            minor,
            patch,
        })
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Compare two Liora release tags. Invalid tags sort before valid tags.
pub fn compare_release_tags(left: &str, right: &str) -> Ordering {
    match (Version::parse_tag(left), Version::parse_tag(right)) {
        (Some(left), Some(right)) => left.cmp(&right),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => left.cmp(right),
    }
}

/// Parsed SHA256SUMS.txt entries keyed by release asset filename.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChecksumManifest {
    entries: BTreeMap<String, String>,
}

impl ChecksumManifest {
    pub fn parse(input: &str) -> Self {
        let mut entries = BTreeMap::new();
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut fields = line.split_whitespace();
            let Some(hex) = fields.next() else { continue };
            if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                continue;
            }
            let Some(name) = fields.next() else { continue };
            entries.insert(
                name.trim_start_matches('*').to_string(),
                hex.to_ascii_lowercase(),
            );
        }
        Self { entries }
    }

    pub fn expected_sha256(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(String::as_str)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// An explicitly caller-owned installation plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    pub app: LioraApp,
    pub platform: Platform,
    pub asset_kind: AssetKind,
    pub asset_name: String,
    pub asset_path: PathBuf,
    pub action: InstallAction,
    pub notes: Vec<String>,
}

impl InstallPlan {
    /// Execute installer-style plans with `std::process::Command`.
    ///
    /// This method is intentionally explicit. It only runs commands for assets
    /// that are already executable installers (AppImage, DMG open, Windows EXE
    /// or MSI). Package-manager formats and raw executables are returned as
    /// manual actions so the caller can request privileges or user consent in
    /// its own UI.
    pub fn install(&self) -> Result<InstallOutcome, UpdaterError> {
        let mut command = self.action.command()?;
        let status = command.status()?;
        Ok(InstallOutcome { status })
    }
}

#[derive(Debug)]
pub struct InstallOutcome {
    pub status: ExitStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallAction {
    RunExecutable { program: PathBuf, args: Vec<String> },
    OpenWithSystem { program: String, args: Vec<String> },
    Manual { description: String },
}

impl InstallAction {
    fn command(&self) -> Result<Command, UpdaterError> {
        match self {
            Self::RunExecutable { program, args } => {
                let mut command = Command::new(program);
                command.args(args);
                Ok(command)
            }
            Self::OpenWithSystem { program, args } => {
                let mut command = Command::new(program);
                command.args(args);
                Ok(command)
            }
            Self::Manual { description } => Err(UpdaterError::ManualInstall(description.clone())),
        }
    }
}

/// GitHub-backed updater client.
#[derive(Debug, Clone)]
pub struct Updater {
    owner: String,
    repo: String,
    api_base: String,
    user_agent: String,
    timeout: Duration,
}

impl Default for Updater {
    fn default() -> Self {
        Self::new(DEFAULT_OWNER, DEFAULT_REPO)
    }
}

impl Updater {
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
            api_base: DEFAULT_API_BASE.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_api_base(mut self, api_base: impl Into<String>) -> Self {
        self.api_base = api_base.into();
        self
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn releases(&self) -> Result<Vec<Release>, UpdaterError> {
        let url = format!(
            "{}/repos/{}/{}/releases",
            self.api_base.trim_end_matches('/'),
            self.owner,
            self.repo
        );
        let response = self.get(&url)?.into_string()?;
        let releases: Vec<GithubRelease> = serde_json::from_str(&response)?;
        Ok(releases.into_iter().map(Release::from).collect())
    }

    pub fn latest_release(
        &self,
        include_prerelease: bool,
    ) -> Result<Option<Release>, UpdaterError> {
        let mut releases = self.releases()?;
        releases.retain(|release| !release.draft && (include_prerelease || !release.prerelease));
        releases.sort_by(|left, right| compare_release_tags(&right.tag, &left.tag));
        Ok(releases.into_iter().next())
    }

    pub fn update_available(
        &self,
        current_tag: &str,
        include_prerelease: bool,
    ) -> Result<Option<Release>, UpdaterError> {
        let Some(latest) = self.latest_release(include_prerelease)? else {
            return Ok(None);
        };
        if compare_release_tags(&latest.tag, current_tag).is_gt() {
            Ok(Some(latest))
        } else {
            Ok(None)
        }
    }

    pub fn download_verified_asset(
        &self,
        release: &Release,
        asset: &ReleaseAsset,
        destination_dir: &Path,
    ) -> Result<PathBuf, UpdaterError> {
        fs::create_dir_all(destination_dir)?;
        let checksums = release
            .checksum_asset()
            .ok_or(UpdaterError::MissingChecksumAsset)?;
        let checksum_text = self.get(&checksums.download_url)?.into_string()?;
        let manifest = ChecksumManifest::parse(&checksum_text);
        let expected = manifest
            .expected_sha256(&asset.name)
            .ok_or_else(|| UpdaterError::MissingChecksumEntry(asset.name.clone()))?
            .to_string();

        let destination = destination_dir.join(safe_asset_filename(&asset.name));
        let mut response = self.get(&asset.download_url)?.into_reader();
        let mut file = File::create(&destination)?;
        io::copy(&mut response, &mut file)?;
        file.flush()?;

        let actual = sha256_file(&destination)?;
        if actual != expected {
            let _ = fs::remove_file(&destination);
            return Err(UpdaterError::ChecksumMismatch {
                asset: asset.name.clone(),
                expected,
                actual,
            });
        }
        Ok(destination)
    }

    fn get(&self, url: &str) -> Result<ureq::Response, UpdaterError> {
        Ok(ureq::AgentBuilder::new()
            .timeout(self.timeout)
            .build()
            .get(url)
            .set("Accept", "application/vnd.github+json")
            .set("User-Agent", &self.user_agent)
            .call()?)
    }
}

/// Select the best release asset for an app/platform preference.
pub fn select_asset(
    release: &Release,
    app: LioraApp,
    platform: Platform,
    preferred: AssetKind,
) -> Option<ReleaseAsset> {
    let priorities = asset_priorities(app, platform, preferred);
    priorities.into_iter().find_map(|kind| {
        release
            .assets
            .iter()
            .filter(|asset| asset.name.starts_with(app.release_name()))
            .filter(|asset| asset.name.contains(platform.asset_fragment()))
            .find(|asset| asset_matches_kind(&asset.name, kind))
            .cloned()
    })
}

/// Build an install plan for a downloaded and verified release asset.
pub fn build_install_plan(
    app: LioraApp,
    platform: Platform,
    asset: &ReleaseAsset,
    asset_path: impl Into<PathBuf>,
) -> InstallPlan {
    let asset_path = asset_path.into();
    let asset_kind = classify_asset_kind(&asset.name, app);
    let (action, notes) = install_action(platform, asset_kind, &asset_path, &asset.name);
    InstallPlan {
        app,
        platform,
        asset_kind,
        asset_name: asset.name.clone(),
        asset_path,
        action,
        notes,
    }
}

fn asset_priorities(app: LioraApp, platform: Platform, preferred: AssetKind) -> Vec<AssetKind> {
    let mut kinds = match (app, platform) {
        (LioraApp::Docs, _) => vec![AssetKind::RawExecutable],
        (LioraApp::Gallery, Platform::LinuxX64) => vec![
            AssetKind::Installer,
            AssetKind::PortableArchive,
            AssetKind::RawExecutable,
        ],
        (LioraApp::Gallery, Platform::MacosArm64 | Platform::WindowsX64) => {
            vec![AssetKind::Installer, AssetKind::RawExecutable]
        }
    };
    if let Some(index) = kinds.iter().position(|kind| *kind == preferred) {
        let selected = kinds.remove(index);
        kinds.insert(0, selected);
    }
    kinds
}

fn asset_matches_kind(name: &str, kind: AssetKind) -> bool {
    match kind {
        AssetKind::RawExecutable => {
            !(name.ends_with(".AppImage")
                || name.ends_with(".deb")
                || name.ends_with(".rpm")
                || name.ends_with(".tar.gz")
                || name.ends_with(".dmg")
                || name.ends_with(".msi")
                || name.ends_with("-setup.exe"))
        }
        AssetKind::Installer => {
            name.ends_with(".AppImage")
                || name.ends_with(".deb")
                || name.ends_with(".rpm")
                || name.ends_with(".dmg")
                || name.ends_with(".msi")
                || name.ends_with("-setup.exe")
        }
        AssetKind::PortableArchive => name.ends_with(".tar.gz"),
    }
}

fn classify_asset_kind(name: &str, app: LioraApp) -> AssetKind {
    if app == LioraApp::Docs {
        AssetKind::RawExecutable
    } else if name.ends_with(".tar.gz") {
        AssetKind::PortableArchive
    } else if asset_matches_kind(name, AssetKind::Installer) {
        AssetKind::Installer
    } else {
        AssetKind::RawExecutable
    }
}

fn install_action(
    platform: Platform,
    kind: AssetKind,
    path: &Path,
    name: &str,
) -> (InstallAction, Vec<String>) {
    match (platform, kind) {
        (_, AssetKind::RawExecutable) => (
            InstallAction::Manual {
                description: format!(
                    "{} is a raw executable. The caller should place it in an application-specific location and mark it executable if needed.",
                    name
                ),
            },
            vec!["No privileged installation is performed automatically.".to_string()],
        ),
        (Platform::LinuxX64, AssetKind::PortableArchive) => (
            InstallAction::Manual {
                description: format!(
                    "{} is a portable archive. Extract it to a caller-selected application directory.",
                    name
                ),
            },
            vec!["Archive extraction target should be chosen by the application/user.".to_string()],
        ),
        (Platform::LinuxX64, AssetKind::Installer) if name.ends_with(".AppImage") => (
            InstallAction::RunExecutable {
                program: path.to_path_buf(),
                args: Vec::new(),
            },
            vec!["Runs the AppImage directly; package installation is not attempted.".to_string()],
        ),
        (Platform::LinuxX64, AssetKind::Installer) => (
            InstallAction::Manual {
                description: format!(
                    "{} requires a system package manager such as apt, dnf, rpm, or a graphical installer.",
                    name
                ),
            },
            vec!["Package-manager installs may require privileges and must be initiated by the caller.".to_string()],
        ),
        (Platform::MacosArm64, AssetKind::Installer) => (
            InstallAction::OpenWithSystem {
                program: "open".to_string(),
                args: vec![path.display().to_string()],
            },
            vec!["Opens the disk image; the user/caller completes app placement.".to_string()],
        ),
        (Platform::WindowsX64, AssetKind::Installer) if name.ends_with(".msi") => (
            InstallAction::OpenWithSystem {
                program: "msiexec".to_string(),
                args: vec!["/i".to_string(), path.display().to_string()],
            },
            vec!["Starts Windows Installer; elevation prompts remain under OS/user control.".to_string()],
        ),
        (Platform::WindowsX64, AssetKind::Installer) => (
            InstallAction::RunExecutable {
                program: path.to_path_buf(),
                args: Vec::new(),
            },
            vec!["Starts the installer executable; elevation prompts remain under OS/user control.".to_string()],
        ),
        (_, AssetKind::PortableArchive) => (
            InstallAction::Manual {
                description: format!("{} should be unpacked by the caller.", name),
            },
            vec!["Portable archive handling is caller-owned.".to_string()],
        ),
    }
}

fn safe_asset_filename(name: &str) -> &Path {
    Path::new(name)
        .file_name()
        .map(Path::new)
        .unwrap_or_else(|| Path::new("liora-release-asset"))
}

fn sha256_file(path: &Path) -> Result<String, UpdaterError> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 16 * 1024];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[derive(Debug, Error)]
pub enum UpdaterError {
    #[error("GitHub request failed: {0}")]
    Http(#[from] Box<ureq::Error>),
    #[error("I/O failed: {0}")]
    Io(#[from] io::Error),
    #[error("GitHub JSON response was invalid: {0}")]
    Json(#[from] serde_json::Error),
    #[error("release is missing SHA256SUMS.txt")]
    MissingChecksumAsset,
    #[error("SHA256SUMS.txt has no entry for {0}")]
    MissingChecksumEntry(String),
    #[error("checksum mismatch for {asset}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        asset: String,
        expected: String,
        actual: String,
    },
    #[error("install action is manual: {0}")]
    ManualInstall(String),
}

impl From<ureq::Error> for UpdaterError {
    fn from(error: ureq::Error) -> Self {
        Self::Http(Box::new(error))
    }
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    name: Option<String>,
    prerelease: bool,
    draft: bool,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

impl From<GithubRelease> for Release {
    fn from(value: GithubRelease) -> Self {
        Self {
            tag: value.tag_name,
            name: value.name,
            prerelease: value.prerelease,
            draft: value.draft,
            assets: value.assets.into_iter().map(ReleaseAsset::from).collect(),
        }
    }
}

impl From<GithubAsset> for ReleaseAsset {
    fn from(value: GithubAsset) -> Self {
        Self {
            name: value.name,
            download_url: value.browser_download_url,
            size: value.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asset(name: &str) -> ReleaseAsset {
        ReleaseAsset {
            name: name.to_string(),
            download_url: format!("https://example.invalid/{name}"),
            size: 42,
        }
    }

    fn release_with_assets(names: &[&str]) -> Release {
        Release {
            tag: "v0.2.0".to_string(),
            name: None,
            prerelease: false,
            draft: false,
            assets: names.iter().map(|name| asset(name)).collect(),
        }
    }

    #[test]
    fn version_tags_compare_semverish_values() {
        assert_eq!(compare_release_tags("v0.1.3", "v0.1.2"), Ordering::Greater);
        assert_eq!(compare_release_tags("0.2.0", "v0.10.0"), Ordering::Less);
        assert_eq!(compare_release_tags("v1.0.0", "v1.0.0"), Ordering::Equal);
        assert_eq!(compare_release_tags("nightly", "v0.1.0"), Ordering::Less);
        assert_eq!(Version::parse_tag("v2.3.4-beta.1").unwrap().patch, 4);
    }

    #[test]
    fn asset_selection_prefers_gallery_installers_by_platform() {
        let release = release_with_assets(&[
            "liora-gallery-v0.2.0-linux-x64",
            "liora-gallery-v0.2.0-linux-x64.tar.gz",
            "liora-gallery-v0.2.0-linux-x64.AppImage",
            "liora-docs-v0.2.0-linux-x64",
            "SHA256SUMS.txt",
        ]);

        let selected = select_asset(
            &release,
            LioraApp::Gallery,
            Platform::LinuxX64,
            AssetKind::Installer,
        )
        .unwrap();
        assert_eq!(selected.name, "liora-gallery-v0.2.0-linux-x64.AppImage");

        let selected = select_asset(
            &release,
            LioraApp::Gallery,
            Platform::LinuxX64,
            AssetKind::PortableArchive,
        )
        .unwrap();
        assert_eq!(selected.name, "liora-gallery-v0.2.0-linux-x64.tar.gz");
    }

    #[test]
    fn asset_selection_keeps_docs_to_raw_executables() {
        let release = release_with_assets(&[
            "liora-docs-v0.2.0-windows-x64.exe",
            "liora-gallery-v0.2.0-windows-x64-setup.exe",
        ]);

        let selected = select_asset(
            &release,
            LioraApp::Docs,
            Platform::WindowsX64,
            AssetKind::Installer,
        )
        .unwrap();
        assert_eq!(selected.name, "liora-docs-v0.2.0-windows-x64.exe");
    }

    #[test]
    fn checksum_manifest_parses_sha256sum_formats() {
        let manifest = ChecksumManifest::parse(
            "# comment\n\
             e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  liora-docs-v0.1.3-linux-x64\n\
             ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad *liora-gallery-v0.1.3-linux-x64.AppImage\n\
             not-a-checksum  ignored\n",
        );
        assert_eq!(
            manifest.expected_sha256("liora-docs-v0.1.3-linux-x64"),
            Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
        assert_eq!(
            manifest.expected_sha256("liora-gallery-v0.1.3-linux-x64.AppImage"),
            Some("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        );
        assert!(manifest.expected_sha256("ignored").is_none());
    }

    #[test]
    fn install_plan_does_not_auto_privilege_package_manager_assets() {
        let asset = asset("liora-gallery-v0.2.0-linux-x64.deb");
        let plan = build_install_plan(
            LioraApp::Gallery,
            Platform::LinuxX64,
            &asset,
            PathBuf::from("/tmp/liora.deb"),
        );
        assert!(matches!(plan.action, InstallAction::Manual { .. }));
    }
}
