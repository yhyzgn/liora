//! Reusable GitHub Release updater primitives for native Rust applications.
//!
//! `liora-updater` is intentionally split into two layers:
//!
//! - generic update primitives (`Updater`, `AssetSelector`, `UpdateRequest`,
//!   `PreparedUpdate`) that any application can configure for its own GitHub
//!   repository and asset naming convention;
//! - small Liora presets (`UpdateApp`, `liora_asset_selector`, `select_asset`) used
//!   by the official Gallery and Docs apps.
//!
//! The crate checks GitHub Releases, selects a platform asset, downloads it into
//! caller-controlled cache/temp storage, verifies it against `SHA256SUMS.txt`,
//! and returns an explicit install plan. It never performs privileged or
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

/// Default GitHub owner queried by official Liora update checks.
pub const DEFAULT_OWNER: &str = "yhyzgn";
/// Default GitHub repository queried by official Liora update checks.
pub const DEFAULT_REPO: &str = "liora";
/// Default GitHub REST API base URL used when no custom endpoint is configured.
pub const DEFAULT_API_BASE: &str = "https://api.github.com";
/// Stable checksums asset constant used by the liora updater API.
pub const CHECKSUMS_ASSET: &str = "SHA256SUMS.txt";
const DEFAULT_USER_AGENT: &str = concat!("liora-updater/", env!("CARGO_PKG_VERSION"));

/// Official applications published from the Liora repository.
///
/// Other applications should use [`AssetSelector`] and [`UpdateRequest`]
/// directly instead of this preset enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdateApp {
    /// Targets the official Liora Docs application release assets.
    Docs,
    /// Targets the official Liora Gallery application release assets.
    Gallery,
}

impl UpdateApp {
    /// Returns the release asset prefix used by official Liora applications.
    pub fn release_name(self) -> &'static str {
        match self {
            Self::Docs => "liora-docs",
            Self::Gallery => "liora-gallery",
        }
    }
}

impl From<UpdateApp> for String {
    fn from(value: UpdateApp) -> Self {
        value.release_name().to_string()
    }
}

/// Common desktop platforms encoded in release asset names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    /// Targets Linux x86_64 release assets.
    LinuxX64,
    /// Targets macOS Apple Silicon release assets.
    MacosArm64,
    /// Targets Windows x86_64 release assets.
    WindowsX64,
}

impl Platform {
    /// Returns the platform or environment value for the current target.
    pub fn current() -> Option<Self> {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("linux", "x86_64") => Some(Self::LinuxX64),
            ("macos", "aarch64") => Some(Self::MacosArm64),
            ("windows", "x86_64") => Some(Self::WindowsX64),
            _ => None,
        }
    }

    /// Asset-name fragment used by Liora release packages.
    pub fn asset_fragment(self) -> &'static str {
        match self {
            Self::LinuxX64 => "linux-x64",
            Self::MacosArm64 => "macos-arm64",
            Self::WindowsX64 => "windows-x64",
        }
    }

    /// Returns the operating-system fragment used in release asset names.
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
    /// Portable archive such as `.tar.gz`.
    PortableArchive,
}

/// A GitHub release asset relevant to updates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseAsset {
    /// Display name shown to users for this item.
    pub name: String,
    /// Direct GitHub asset download URL.
    pub download_url: String,
    /// File size in bytes reported by GitHub release metadata.
    pub size: u64,
}

/// A GitHub release with parsed assets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// Git tag that identifies the release.
    pub tag: String,
    /// Display name shown to users for this item.
    pub name: Option<String>,
    /// Whether the GitHub release is marked as a prerelease.
    pub prerelease: bool,
    /// Whether the GitHub release is still a draft.
    pub draft: bool,
    /// Assets attached to the GitHub release.
    pub assets: Vec<ReleaseAsset>,
}

impl Release {
    /// Returns the release tag stripped of the leading `v` prefix when present.
    pub fn version(&self) -> Option<Version> {
        Version::parse_tag(&self.tag)
    }

    /// Finds the default checksum manifest asset attached to a release.
    pub fn checksum_asset(&self) -> Option<&ReleaseAsset> {
        self.checksum_asset_named(CHECKSUMS_ASSET)
    }

    /// Finds a checksum manifest asset by exact file name.
    pub fn checksum_asset_named(&self, name: &str) -> Option<&ReleaseAsset> {
        self.assets.iter().find(|asset| asset.name == name)
    }
}

/// Minimal semver-ish `vX.Y.Z` version used by release tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    /// Major semantic-version component.
    pub major: u64,
    /// Minor semantic-version component.
    pub minor: u64,
    /// Patch semantic-version component.
    pub patch: u64,
}

impl Version {
    /// Parses a `vMAJOR.MINOR.PATCH` release tag into comparable version parts.
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

/// Compare two release tags. Invalid tags sort before valid tags.
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
    /// Parses a checksum manifest into asset-name to digest mappings.
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

    /// Returns the expected SHA-256 digest for a named asset.
    pub fn expected_sha256(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(String::as_str)
    }

    /// Returns whether this collection or manifest contains no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Generic asset selector for an application's release naming convention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetSelector {
    name_prefix: Option<String>,
    platform_fragment: Option<String>,
    kind_priority: Vec<AssetKind>,
}

impl AssetSelector {
    /// Create a selector with the common preference order:
    /// installer/package, portable archive, raw executable.
    pub fn new() -> Self {
        Self {
            name_prefix: None,
            platform_fragment: None,
            kind_priority: vec![
                AssetKind::Installer,
                AssetKind::PortableArchive,
                AssetKind::RawExecutable,
            ],
        }
    }

    /// Create a selector matching a conventional `app-platform` asset scheme.
    pub fn for_platform(platform: Platform) -> Self {
        Self::new().matching_platform(platform)
    }

    /// Returns true when an asset name matches the configured app prefix.
    pub fn matching_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.name_prefix = Some(prefix.into());
        self
    }

    /// Returns true when an asset name targets the configured platform.
    pub fn matching_platform(mut self, platform: Platform) -> Self {
        self.platform_fragment = Some(platform.asset_fragment().to_string());
        self
    }

    /// Use a custom platform/name fragment when your release assets do not use
    /// Liora's `linux-x64`, `macos-arm64`, `windows-x64` convention.
    pub fn matching_platform_fragment(mut self, fragment: impl Into<String>) -> Self {
        self.platform_fragment = Some(fragment.into());
        self
    }

    /// Replace the asset-kind priority order. Empty input means "match any kind".
    pub fn kind_priority<I>(mut self, kinds: I) -> Self
    where
        I: IntoIterator<Item = AssetKind>,
    {
        self.kind_priority = dedupe_kinds(kinds);
        self
    }

    /// Returns the app-specific file-name prefix preferred by this selector.
    pub fn name_prefix(&self) -> Option<&str> {
        self.name_prefix.as_deref()
    }

    /// Returns the platform-specific name fragment preferred by this selector.
    pub fn platform_fragment(&self) -> Option<&str> {
        self.platform_fragment.as_deref()
    }

    /// Returns asset kinds in preferred selection order.
    pub fn kind_priority_order(&self) -> &[AssetKind] {
        &self.kind_priority
    }

    fn matches_name(&self, asset: &ReleaseAsset) -> bool {
        self.name_prefix
            .as_ref()
            .is_none_or(|prefix| asset.name.starts_with(prefix))
            && self
                .platform_fragment
                .as_ref()
                .is_none_or(|fragment| asset.name.contains(fragment))
    }
}

impl Default for AssetSelector {
    fn default() -> Self {
        Self::new()
    }
}

/// A full update request for one application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateRequest {
    /// Application name prefix used when selecting release assets.
    pub app_name: String,
    /// Currently installed release tag used for update comparison.
    pub current_tag: String,
    /// Whether prerelease GitHub releases are eligible update candidates.
    pub include_prerelease: bool,
    /// Platform targeted by this package, artifact, or update plan.
    pub platform: Platform,
    /// Asset-selection strategy used to choose the downloadable artifact.
    pub selector: AssetSelector,
    /// Directory used to store downloaded release assets.
    pub cache_dir: PathBuf,
}

impl UpdateRequest {
    /// Creates `UpdateRequest` with default theme-driven styling and no optional callbacks attached.
    pub fn new(
        app_name: impl Into<String>,
        current_tag: impl Into<String>,
        platform: Platform,
        cache_dir: impl Into<PathBuf>,
    ) -> Self {
        let app_name = app_name.into();
        Self {
            selector: AssetSelector::for_platform(platform).matching_prefix(app_name.clone()),
            app_name,
            current_tag: current_tag.into(),
            include_prerelease: false,
            platform,
            cache_dir: cache_dir.into(),
        }
    }

    /// Configures whether prerelease GitHub releases may be selected.
    pub fn include_prerelease(mut self, include_prerelease: bool) -> Self {
        self.include_prerelease = include_prerelease;
        self
    }

    /// Replaces the asset-selection policy for this update request.
    pub fn selector(mut self, selector: AssetSelector) -> Self {
        self.selector = selector;
        self
    }
}

/// A downloaded, verified update ready for a visible install action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedUpdate {
    /// GitHub release metadata returned by the updater.
    pub release: Release,
    /// Selected release asset metadata.
    pub asset: ReleaseAsset,
    /// Local path of the downloaded release asset.
    pub asset_path: PathBuf,
    /// Installation plan derived from the selected release asset.
    pub install_plan: InstallPlan,
}

impl PreparedUpdate {
    /// Returns the selected release tag for this update result.
    pub fn release_tag(&self) -> &str {
        &self.release.tag
    }
}

/// An explicitly caller-owned installation plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    /// Application name prefix used when selecting release assets.
    pub app_name: String,
    /// Platform targeted by this package, artifact, or update plan.
    pub platform: Platform,
    /// Class of artifact selected for installation.
    pub asset_kind: AssetKind,
    /// File name of the selected release asset.
    pub asset_name: String,
    /// Local path of the downloaded release asset.
    pub asset_path: PathBuf,
    /// Installation action chosen for the artifact.
    pub action: InstallAction,
    /// Human-readable notes explaining installation behavior or limitations.
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
/// Exit status returned after executing an installer-style update action.
pub struct InstallOutcome {
    /// Current lifecycle status shown by this item.
    pub status: ExitStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Executable, system-open, or manual action required to install an update asset.
pub enum InstallAction {
    /// Runs a downloaded executable installer directly.
    RunExecutable {
        /// Executable path to launch for installer-style assets.
        program: PathBuf,
        /// Command-line arguments passed to the executable.
        args: Vec<String>,
    },
    /// Opens an installer asset with the platform system opener.
    OpenWithSystem {
        /// System opener command used for platform-native installer assets.
        program: String,
        /// Command-line arguments passed to the opener command.
        args: Vec<String>,
    },
    /// Reports that the caller must guide the user through a manual install step.
    Manual {
        /// Human-readable explanation of the manual installation step.
        description: String,
    },
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
    checksum_asset_name: String,
}

impl Default for Updater {
    fn default() -> Self {
        Self::new(DEFAULT_OWNER, DEFAULT_REPO)
    }
}

impl Updater {
    /// Creates `Updater` initialized from the supplied owner, and repo.
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
            api_base: DEFAULT_API_BASE.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            timeout: Duration::from_secs(30),
            checksum_asset_name: CHECKSUMS_ASSET.to_string(),
        }
    }

    /// Sets the api base value used by the component.
    pub fn with_api_base(mut self, api_base: impl Into<String>) -> Self {
        self.api_base = api_base.into();
        self
    }

    /// Sets the user agent value used by the component.
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Sets the timeout value used by the component.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Override the release asset that stores SHA-256 checksums.
    ///
    /// The default is `SHA256SUMS.txt`, but applications can use names such as
    /// `checksums.txt` or `sha256.txt` if their release pipeline emits those.
    pub fn with_checksum_asset_name(mut self, checksum_asset_name: impl Into<String>) -> Self {
        self.checksum_asset_name = checksum_asset_name.into();
        self
    }

    /// Fetches GitHub releases for the configured owner and repository.
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

    /// Returns the newest eligible release according to the update request policy.
    pub fn latest_release(
        &self,
        include_prerelease: bool,
    ) -> Result<Option<Release>, UpdaterError> {
        let mut releases = self.releases()?;
        releases.retain(|release| !release.draft && (include_prerelease || !release.prerelease));
        releases.sort_by(|left, right| compare_release_tags(&right.tag, &left.tag));
        Ok(releases.into_iter().next())
    }

    /// Checks whether a newer eligible release and matching asset are available.
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

    /// Check, select, download, verify, and plan an update for one app.
    pub fn prepare_update(
        &self,
        request: &UpdateRequest,
    ) -> Result<Option<PreparedUpdate>, UpdaterError> {
        let Some(release) =
            self.update_available(&request.current_tag, request.include_prerelease)?
        else {
            return Ok(None);
        };
        let asset = select_asset_with(&release, &request.selector).ok_or_else(|| {
            UpdaterError::NoMatchingAsset {
                release: release.tag.clone(),
                selector: format!("{:?}", request.selector),
            }
        })?;
        let asset_path = self.download_verified_asset(&release, &asset, &request.cache_dir)?;
        let install_plan = build_install_plan(
            request.app_name.clone(),
            request.platform,
            &asset,
            asset_path.clone(),
        );
        Ok(Some(PreparedUpdate {
            release,
            asset,
            asset_path,
            install_plan,
        }))
    }

    /// Downloads the selected asset and verifies it against the release checksum manifest.
    pub fn download_verified_asset(
        &self,
        release: &Release,
        asset: &ReleaseAsset,
        destination_dir: &Path,
    ) -> Result<PathBuf, UpdaterError> {
        fs::create_dir_all(destination_dir)?;
        let checksums = release
            .checksum_asset_named(&self.checksum_asset_name)
            .ok_or_else(|| UpdaterError::MissingChecksumAsset(self.checksum_asset_name.clone()))?;
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

/// Select the best release asset using a generic selector.
pub fn select_asset_with(release: &Release, selector: &AssetSelector) -> Option<ReleaseAsset> {
    if selector.kind_priority.is_empty() {
        return release
            .assets
            .iter()
            .find(|asset| selector.matches_name(asset))
            .cloned();
    }

    selector.kind_priority.iter().find_map(|kind| {
        release
            .assets
            .iter()
            .filter(|asset| selector.matches_name(asset))
            .find(|asset| asset_matches_kind(&asset.name, *kind))
            .cloned()
    })
}

/// Select the best release asset for an official Liora app/platform preference.
pub fn select_asset(
    release: &Release,
    app: UpdateApp,
    platform: Platform,
    preferred: AssetKind,
) -> Option<ReleaseAsset> {
    select_asset_with(release, &liora_asset_selector(app, platform, preferred))
}

/// Build the official Liora asset selector while keeping the core selector API generic.
pub fn liora_asset_selector(
    app: UpdateApp,
    platform: Platform,
    preferred: AssetKind,
) -> AssetSelector {
    AssetSelector::for_platform(platform)
        .matching_prefix(app.release_name())
        .kind_priority(liora_asset_priorities(app, platform, preferred))
}

/// Build an install plan for a downloaded and verified release asset.
pub fn build_install_plan(
    app_name: impl Into<String>,
    platform: Platform,
    asset: &ReleaseAsset,
    asset_path: impl Into<PathBuf>,
) -> InstallPlan {
    let asset_path = asset_path.into();
    let asset_kind = classify_asset_kind(&asset.name);
    let (action, notes) = install_action(platform, asset_kind, &asset_path, &asset.name);
    InstallPlan {
        app_name: app_name.into(),
        platform,
        asset_kind,
        asset_name: asset.name.clone(),
        asset_path,
        action,
        notes,
    }
}

fn liora_asset_priorities(
    app: UpdateApp,
    platform: Platform,
    preferred: AssetKind,
) -> Vec<AssetKind> {
    let mut kinds = match (app, platform) {
        (UpdateApp::Docs, _) => vec![AssetKind::RawExecutable],
        (UpdateApp::Gallery, Platform::LinuxX64) => vec![
            AssetKind::Installer,
            AssetKind::PortableArchive,
            AssetKind::RawExecutable,
        ],
        (UpdateApp::Gallery, Platform::MacosArm64 | Platform::WindowsX64) => {
            vec![AssetKind::Installer, AssetKind::RawExecutable]
        }
    };
    if let Some(index) = kinds.iter().position(|kind| *kind == preferred) {
        let selected = kinds.remove(index);
        kinds.insert(0, selected);
    }
    kinds
}

fn dedupe_kinds<I>(kinds: I) -> Vec<AssetKind>
where
    I: IntoIterator<Item = AssetKind>,
{
    let mut deduped = Vec::new();
    for kind in kinds {
        if !deduped.contains(&kind) {
            deduped.push(kind);
        }
    }
    deduped
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

fn classify_asset_kind(name: &str) -> AssetKind {
    if name.ends_with(".tar.gz") {
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
        .unwrap_or_else(|| Path::new("release-asset"))
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
/// Errors that can occur while checking, downloading, verifying, or installing updates.
pub enum UpdaterError {
    #[error("GitHub request failed: {0}")]
    /// Reports a http failure.
    Http(#[from] Box<ureq::Error>),
    #[error("I/O failed: {0}")]
    /// Reports a io failure.
    Io(#[from] io::Error),
    #[error("GitHub JSON response was invalid: {0}")]
    /// Reports a json failure.
    Json(#[from] serde_json::Error),
    #[error("release is missing checksum asset {0}")]
    /// Reports a missing checksum asset failure.
    MissingChecksumAsset(String),
    #[error("SHA256SUMS.txt has no entry for {0}")]
    /// Reports a missing checksum entry failure.
    MissingChecksumEntry(String),
    #[error("checksum mismatch for {asset}: expected {expected}, got {actual}")]
    /// Reports a checksum mismatch failure.
    ChecksumMismatch {
        /// Asset name whose checksum was verified.
        asset: String,
        /// Expected SHA-256 digest read from the checksum manifest.
        expected: String,
        /// Actual SHA-256 digest computed from the downloaded asset.
        actual: String,
    },
    #[error("release {release} has no asset matching selector {selector}")]
    /// Indicates that a release did not contain an asset accepted by the selector.
    NoMatchingAsset {
        /// Release tag or version whose assets were inspected.
        release: String,
        /// Selector description that failed to match any release asset.
        selector: String,
    },
    #[error("install action is manual: {0}")]
    /// Reports a manual install failure.
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
    fn generic_asset_selector_supports_custom_apps() {
        let release = release_with_assets(&[
            "acme-notes_0.4.0_x86_64.AppImage",
            "acme-notes_0.4.0_x86_64.tar.gz",
            "liora-gallery-v0.2.0-linux-x64.AppImage",
        ]);
        let selector = AssetSelector::new()
            .matching_prefix("acme-notes")
            .matching_platform_fragment("x86_64")
            .kind_priority([AssetKind::PortableArchive, AssetKind::Installer]);

        let selected = select_asset_with(&release, &selector).unwrap();
        assert_eq!(selected.name, "acme-notes_0.4.0_x86_64.tar.gz");
    }

    #[test]
    fn update_request_defaults_to_app_prefix_and_platform() {
        let request = UpdateRequest::new(
            "acme-notes",
            "v0.3.0",
            Platform::LinuxX64,
            "/tmp/acme-updates",
        );
        assert_eq!(request.selector.name_prefix(), Some("acme-notes"));
        assert_eq!(request.selector.platform_fragment(), Some("linux-x64"));
        assert!(!request.include_prerelease);
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
            UpdateApp::Gallery,
            Platform::LinuxX64,
            AssetKind::Installer,
        )
        .unwrap();
        assert_eq!(selected.name, "liora-gallery-v0.2.0-linux-x64.AppImage");

        let selected = select_asset(
            &release,
            UpdateApp::Gallery,
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
            UpdateApp::Docs,
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
            UpdateApp::Gallery,
            Platform::LinuxX64,
            &asset,
            PathBuf::from("/tmp/liora.deb"),
        );
        assert!(matches!(plan.action, InstallAction::Manual { .. }));
    }
}
