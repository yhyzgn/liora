use std::{
    fs,
    io::ErrorKind,
    path::{Component, Path, PathBuf},
};

use liora_packager::{Checksum, PackageArtifact, PackageManifest, Platform, sha256_file};
use serde_json::Value;

// Preserve provenance only while the recorded bytes still match the artifact.
pub(super) fn merge_existing(
    root: &Path,
    manifest: &mut PackageManifest,
    replaced_dirs: &[PathBuf],
) -> Result<(), String> {
    let package_dir = root.join("target/packages");
    let path = package_dir.join("package-manifest.json");
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(format!("failed to read {}: {error}", path.display())),
    };
    let mut previous: Value = serde_json::from_slice(&bytes)
        .map_err(|error| format!("invalid {}: {error}", path.display()))?;
    let artifacts = previous
        .get_mut("artifacts")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| format!("invalid artifacts in {}", path.display()))?;
    let canonical_package_dir = fs::canonicalize(&package_dir)
        .map_err(|error| format!("failed to resolve {}: {error}", package_dir.display()))?;
    for value in artifacts.drain(..) {
        let artifact = parse_artifact(value)
            .map_err(|error| format!("invalid {}: {error}", path.display()))?;
        if !artifact.path.starts_with(&package_dir)
            || artifact
                .path
                .components()
                .any(|part| part == Component::ParentDir)
        {
            return Err(format!(
                "package artifact outside {}: {}",
                package_dir.display(),
                artifact.path.display()
            ));
        }
        if replaced_dirs
            .iter()
            .any(|dir| artifact.path.starts_with(dir))
        {
            continue;
        }
        let metadata = match fs::symlink_metadata(&artifact.path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(format!(
                    "failed to inspect {}: {error}",
                    artifact.path.display()
                ));
            }
        };
        if !metadata.file_type().is_file() {
            return Err(format!(
                "package artifact is not a regular file: {}",
                artifact.path.display()
            ));
        }
        let canonical_artifact = fs::canonicalize(&artifact.path)
            .map_err(|error| format!("failed to resolve {}: {error}", artifact.path.display()))?;
        if !canonical_artifact.starts_with(&canonical_package_dir) {
            return Err(format!(
                "package artifact resolves outside {}: {}",
                package_dir.display(),
                artifact.path.display()
            ));
        }
        let checksum = sha256_file(&canonical_artifact)
            .map_err(|error| format!("failed to hash {}: {error}", artifact.path.display()))?;
        if checksum != artifact.checksum {
            return Err(format!(
                "package artifact changed since packaging: {}",
                artifact.path.display()
            ));
        }
        manifest.push(artifact);
    }
    manifest
        .artifacts
        .sort_unstable_by(|left, right| left.path.cmp(&right.path));
    Ok(())
}

fn take_string(value: &mut Value, key: &str) -> Result<String, String> {
    match value.get_mut(key).map(Value::take) {
        Some(Value::String(text)) => Ok(text),
        _ => Err(format!("expected string field {key}")),
    }
}

fn parse_artifact(mut value: Value) -> Result<PackageArtifact, String> {
    let platform = match take_string(&mut value, "platform")?.as_str() {
        "linux" => Platform::Linux,
        "macos" => Platform::Macos,
        "windows" => Platform::Windows,
        other => return Err(format!("unknown platform {other}")),
    };
    let git_sha = match value
        .get_mut("gitSha")
        .map(Value::take)
        .unwrap_or(Value::Null)
    {
        Value::Null => None,
        Value::String(sha) => Some(sha),
        _ => return Err("expected string or null gitSha".into()),
    };
    let checksum = value.get_mut("checksum").ok_or("missing checksum")?;
    if take_string(checksum, "algorithm")? != "sha256" {
        return Err("unsupported checksum algorithm".into());
    }
    let hex = take_string(checksum, "hex")?;
    if hex.len() != 64 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("invalid SHA-256 digest".into());
    }
    Ok(PackageArtifact {
        app: take_string(&mut value, "app")?,
        version: take_string(&mut value, "version")?,
        platform,
        target_triple: take_string(&mut value, "targetTriple")?,
        git_sha,
        format: take_string(&mut value, "format")?.parse()?,
        path: take_string(&mut value, "path")?.into(),
        checksum: Checksum {
            algorithm: "sha256",
            hex,
        },
        signed: value["signed"].as_bool().ok_or("expected boolean signed")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_records_preserve_platform_and_nullable_provenance() {
        for platform in ["linux", "macos", "windows"] {
            for git_sha in [Value::Null, Value::String("original-source".into())] {
                let record = serde_json::json!({
                    "app": "liora-gallery", "version": "1.2.3", "platform": platform,
                    "targetTriple": "original-target", "gitSha": git_sha,
                    "format": "rpm", "path": "package path/字体.rpm",
                    "checksum": {"algorithm": "sha256", "hex": "a".repeat(64)},
                    "signed": true
                });
                let artifact = parse_artifact(record).unwrap();
                assert_eq!(artifact.platform.as_str(), platform);
                assert_eq!(artifact.git_sha.as_deref(), git_sha.as_str());
                assert_eq!(artifact.path, Path::new("package path/字体.rpm"));
                assert!(artifact.signed);
            }
        }
    }

    #[test]
    fn malformed_stored_records_return_errors_instead_of_panicking() {
        for record in [
            Value::Null,
            Value::Bool(true),
            serde_json::json!([]),
            serde_json::json!({"platform": 3}),
            serde_json::json!({"platform": "linux", "checksum": false}),
            serde_json::json!({"platform": "linux", "checksum": {"algorithm": "md5"}}),
            serde_json::json!({"platform": "linux", "checksum": {"algorithm": "sha256", "hex": "invalid"}}),
        ] {
            assert!(parse_artifact(record).is_err());
        }
    }

    #[cfg(unix)]
    #[test]
    fn retained_artifact_cannot_escape_through_symlinked_parent() {
        use std::os::unix::fs::symlink;

        let root =
            std::env::temp_dir().join(format!("liora-manifest-symlink-{}", std::process::id()));
        let package_dir = root.join("target/packages");
        let outside_dir = root.join("outside");
        fs::create_dir_all(&package_dir).unwrap();
        fs::create_dir_all(&outside_dir).unwrap();
        let outside_file = outside_dir.join("artifact.rpm");
        fs::write(&outside_file, "outside package").unwrap();
        symlink(&outside_dir, package_dir.join("linked")).unwrap();
        let artifact = PackageArtifact {
            app: "liora-gallery".into(),
            version: "1.2.3".into(),
            platform: Platform::Linux,
            target_triple: "x86_64-unknown-linux-gnu".into(),
            git_sha: None,
            format: liora_packager::PackageFormat::Rpm,
            path: package_dir.join("linked/artifact.rpm"),
            checksum: sha256_file(&outside_file).unwrap(),
            signed: false,
        };
        fs::write(
            package_dir.join("package-manifest.json"),
            PackageManifest {
                artifacts: vec![artifact],
            }
            .to_json_pretty(),
        )
        .unwrap();

        let error = merge_existing(&root, &mut PackageManifest::default(), &[]).unwrap_err();
        assert!(error.contains("resolves outside"), "{error}");
        fs::remove_dir_all(root).unwrap();
    }
}
