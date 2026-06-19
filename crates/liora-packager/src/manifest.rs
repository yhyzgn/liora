use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{Checksum, PackageFormat, Platform, sha256_file};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Manifest entry describing one produced native package artifact.
pub struct PackageArtifact {
    /// Application metadata associated with this package artifact.
    pub app: String,
    /// Version string associated with this package, release, or update.
    pub version: String,
    /// Target platform for the artifact or update operation.
    pub platform: Platform,
    /// Rust target triple used to build the artifact.
    pub target_triple: String,
    /// Optional source commit SHA associated with the artifact.
    pub git_sha: Option<String>,
    /// Package format represented by this artifact.
    pub format: PackageFormat,
    /// Filesystem path associated with this item.
    pub path: PathBuf,
    /// Checksum data used to verify downloaded or packaged bytes.
    pub checksum: Checksum,
    /// Whether the artifact is expected to be signed by the release process.
    pub signed: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Release manifest that groups generated package artifacts and checksums.
pub struct PackageManifest {
    /// Package artifacts collected for the release manifest.
    pub artifacts: Vec<PackageArtifact>,
}

impl PackageManifest {
    /// Appends one entry to the collection while preserving existing entries.
    pub fn push(&mut self, artifact: PackageArtifact) {
        self.artifacts.push(artifact);
    }

    /// Appends multiple entries to the collection in iteration order.
    pub fn extend(&mut self, artifacts: impl IntoIterator<Item = PackageArtifact>) {
        self.artifacts.extend(artifacts);
    }

    /// Returns whether this collection or manifest contains no entries.
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }

    /// Renders a SHA256SUMS-compatible checksum listing for release assets.
    pub fn checksums_txt(&self) -> String {
        let mut out = String::new();
        for artifact in &self.artifacts {
            out.push_str(&format!(
                "{}  {}\n",
                artifact.checksum.hex,
                artifact.path.display()
            ));
        }
        out
    }

    /// Renders a Markdown table summarizing release artifacts and checksums.
    pub fn release_notes_markdown(&self) -> String {
        let mut out = String::from("# Liora native package release\n\n");
        if self.artifacts.is_empty() {
            out.push_str("No package artifacts were discovered. This file was generated before backend package outputs existed.\n");
            return out;
        }

        let mut current_platform = None;
        for artifact in &self.artifacts {
            if current_platform != Some(artifact.platform) {
                current_platform = Some(artifact.platform);
                out.push_str(&format!("\n## {}\n\n", artifact.platform.as_str()));
            }
            out.push_str(&format!(
                "- `{}` `{}` `{}` `{}`  \n  SHA256: `{}`{}\n",
                artifact.app,
                artifact.version,
                artifact.target_triple,
                artifact.format.as_str(),
                artifact.checksum.hex,
                artifact
                    .git_sha
                    .as_ref()
                    .map(|sha| format!("  \n  Git: `{sha}`"))
                    .unwrap_or_default()
            ));
        }
        out
    }

    /// Converts this value into json pretty.
    pub fn to_json_pretty(&self) -> String {
        let mut out = String::from("{\n  \"artifacts\": [");
        for (idx, artifact) in self.artifacts.iter().enumerate() {
            if idx > 0 {
                out.push(',');
            }
            out.push_str(&format!(
                "\n    {{\n      \"app\": \"{}\",\n      \"version\": \"{}\",\n      \"platform\": \"{}\",\n      \"targetTriple\": \"{}\",\n      \"gitSha\": {},\n      \"format\": \"{}\",\n      \"path\": \"{}\",\n      \"checksum\": {{ \"algorithm\": \"{}\", \"hex\": \"{}\" }},\n      \"signed\": {}\n    }}",
                escape(&artifact.app),
                escape(&artifact.version),
                artifact.platform.as_str(),
                escape(&artifact.target_triple),
                json_option_string(artifact.git_sha.as_deref()),
                artifact.format.as_str(),
                escape(&artifact.path.display().to_string()),
                artifact.checksum.algorithm,
                artifact.checksum.hex,
                artifact.signed
            ));
        }
        out.push_str("\n  ]\n}\n");
        out
    }
}

/// Discovers generated package artifacts and attaches checksum metadata for release manifests.
pub fn collect_package_artifacts(
    app: &str,
    version: &str,
    platform: Platform,
    target_triple: &str,
    git_sha: Option<&str>,
    out_dir: &Path,
    formats: &[PackageFormat],
) -> io::Result<Vec<PackageArtifact>> {
    let mut artifacts = Vec::new();
    if !out_dir.exists() {
        return Ok(artifacts);
    }
    collect_package_artifacts_in_dir(
        app,
        version,
        platform,
        target_triple,
        git_sha,
        out_dir,
        formats,
        &mut artifacts,
    )?;
    artifacts.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(artifacts)
}

fn collect_package_artifacts_in_dir(
    app: &str,
    version: &str,
    platform: Platform,
    target_triple: &str,
    git_sha: Option<&str>,
    dir: &Path,
    formats: &[PackageFormat],
    artifacts: &mut Vec<PackageArtifact>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }
            collect_package_artifacts_in_dir(
                app,
                version,
                platform,
                target_triple,
                git_sha,
                &path,
                formats,
                artifacts,
            )?;
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        let Some(format) = PackageFormat::from_artifact_path(&path) else {
            continue;
        };
        if !formats.contains(&format) {
            continue;
        }
        artifacts.push(PackageArtifact {
            app: app.to_string(),
            version: version.to_string(),
            platform,
            target_triple: target_triple.to_string(),
            git_sha: git_sha.map(ToOwned::to_owned),
            format,
            checksum: sha256_file(&path)?,
            path,
            signed: false,
        });
    }
    Ok(())
}

fn json_option_string(value: Option<&str>) -> String {
    value
        .map(|value| format!("\"{}\"", escape(value)))
        .unwrap_or_else(|| "null".to_string())
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_serializes_artifact_fields() {
        let mut manifest = PackageManifest::default();
        manifest.push(PackageArtifact {
            app: "liora-gallery".into(),
            version: "0.1.0".into(),
            platform: Platform::Linux,
            target_triple: "x86_64-unknown-linux-gnu".into(),
            git_sha: Some("abc1234".into()),
            format: PackageFormat::AppImage,
            path: "target/packages/liora-gallery.AppImage".into(),
            checksum: Checksum {
                algorithm: "sha256",
                hex: "abc".into(),
            },
            signed: false,
        });
        let json = manifest.to_json_pretty();
        assert!(json.contains("\"app\": \"liora-gallery\""));
        assert!(json.contains("\"targetTriple\": \"x86_64-unknown-linux-gnu\""));
        assert!(json.contains("\"gitSha\": \"abc1234\""));
        assert!(json.contains("\"format\": \"appimage\""));
        assert!(json.contains("\"signed\": false"));
        assert!(
            manifest
                .checksums_txt()
                .contains("abc  target/packages/liora-gallery.AppImage")
        );
        let notes = manifest.release_notes_markdown();
        assert!(notes.contains("# Liora native package release"));
        assert!(notes.contains("SHA256: `abc`"));
        assert!(notes.contains("Git: `abc1234`"));
        assert!(!notes.contains("\\n  Git"));
    }

    #[test]
    fn collects_matching_package_artifacts() {
        let root = std::env::temp_dir().join(format!("liora-packager-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let artifact = root.join("liora-gallery_0.1.0_amd64.deb");
        fs::write(&artifact, b"deb").unwrap();
        fs::write(root.join("ignore.txt"), b"ignore").unwrap();
        fs::create_dir_all(root.join(".cargo-packager/deb/internal")).unwrap();
        fs::write(
            root.join(".cargo-packager/deb/internal/control.tar.gz"),
            b"internal",
        )
        .unwrap();

        let artifacts = collect_package_artifacts(
            "liora-gallery",
            "0.1.0",
            Platform::Linux,
            "x86_64-unknown-linux-gnu",
            None,
            &root,
            &[PackageFormat::Deb],
        )
        .unwrap();
        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].path, artifact);
        assert_eq!(artifacts[0].format, PackageFormat::Deb);
        assert_eq!(artifacts[0].target_triple, "x86_64-unknown-linux-gnu");
        assert_eq!(artifacts[0].git_sha, None);
        assert_eq!(artifacts[0].checksum.algorithm, "sha256");

        let _ = fs::remove_dir_all(&root);
    }
}
