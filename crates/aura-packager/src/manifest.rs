use std::{
    fmt::Write as _,
    fs, io,
    path::{Path, PathBuf},
};

use crate::{Checksum, PackageFormat, Platform, sha256_file};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageArtifact {
    pub app: String,
    pub version: String,
    pub platform: Platform,
    pub format: PackageFormat,
    pub path: PathBuf,
    pub checksum: Checksum,
    pub signed: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageManifest {
    pub artifacts: Vec<PackageArtifact>,
}

impl PackageManifest {
    pub fn push(&mut self, artifact: PackageArtifact) {
        self.artifacts.push(artifact);
    }

    pub fn extend(&mut self, artifacts: impl IntoIterator<Item = PackageArtifact>) {
        self.artifacts.extend(artifacts);
    }

    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }

    pub fn checksums_txt(&self) -> String {
        let mut out = String::new();
        for artifact in &self.artifacts {
            writeln!(
                out,
                "{}  {}",
                artifact.checksum.hex,
                artifact.path.display()
            )
            .expect("write to string");
        }
        out
    }

    pub fn to_json_pretty(&self) -> String {
        let mut out = String::from("{\n  \"artifacts\": [");
        for (idx, artifact) in self.artifacts.iter().enumerate() {
            if idx > 0 {
                out.push(',');
            }
            write!(
                out,
                "\n    {{\n      \"app\": \"{}\",\n      \"version\": \"{}\",\n      \"platform\": \"{}\",\n      \"format\": \"{}\",\n      \"path\": \"{}\",\n      \"checksum\": {{ \"algorithm\": \"{}\", \"hex\": \"{}\" }},\n      \"signed\": {}\n    }}",
                escape(&artifact.app),
                escape(&artifact.version),
                artifact.platform.as_str(),
                artifact.format.as_str(),
                escape(&artifact.path.display().to_string()),
                artifact.checksum.algorithm,
                artifact.checksum.hex,
                artifact.signed
            )
            .expect("write to string");
        }
        out.push_str("\n  ]\n}\n");
        out
    }
}

pub fn collect_package_artifacts(
    app: &str,
    version: &str,
    platform: Platform,
    out_dir: &Path,
    formats: &[PackageFormat],
) -> io::Result<Vec<PackageArtifact>> {
    let mut artifacts = Vec::new();
    if !out_dir.exists() {
        return Ok(artifacts);
    }
    collect_package_artifacts_in_dir(app, version, platform, out_dir, formats, &mut artifacts)?;
    artifacts.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(artifacts)
}

fn collect_package_artifacts_in_dir(
    app: &str,
    version: &str,
    platform: Platform,
    dir: &Path,
    formats: &[PackageFormat],
    artifacts: &mut Vec<PackageArtifact>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_package_artifacts_in_dir(app, version, platform, &path, formats, artifacts)?;
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
            format,
            checksum: sha256_file(&path)?,
            path,
            signed: false,
        });
    }
    Ok(())
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
            app: "aura-gallery".into(),
            version: "0.1.0".into(),
            platform: Platform::Linux,
            format: PackageFormat::AppImage,
            path: "target/packages/aura-gallery.AppImage".into(),
            checksum: Checksum {
                algorithm: "sha256",
                hex: "abc".into(),
            },
            signed: false,
        });
        let json = manifest.to_json_pretty();
        assert!(json.contains("\"app\": \"aura-gallery\""));
        assert!(json.contains("\"format\": \"appimage\""));
        assert!(json.contains("\"signed\": false"));
        assert!(
            manifest
                .checksums_txt()
                .contains("abc  target/packages/aura-gallery.AppImage")
        );
    }

    #[test]
    fn collects_matching_package_artifacts() {
        let root = std::env::temp_dir().join(format!("aura-packager-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let artifact = root.join("aura-gallery_0.1.0_amd64.deb");
        fs::write(&artifact, b"deb").unwrap();
        fs::write(root.join("ignore.txt"), b"ignore").unwrap();

        let artifacts = collect_package_artifacts(
            "aura-gallery",
            "0.1.0",
            Platform::Linux,
            &root,
            &[PackageFormat::Deb],
        )
        .unwrap();
        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].format, PackageFormat::Deb);
        assert_eq!(artifacts[0].checksum.algorithm, "sha256");

        let _ = fs::remove_dir_all(&root);
    }
}
