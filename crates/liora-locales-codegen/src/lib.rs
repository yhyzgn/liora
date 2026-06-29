//! Build-time locale key generation for Liora applications.
//!
//! Use this crate from an application `build.rs` to generate typed
//! `locales::section::key` constants from external TOML resources. By default
//! it scans `./assets/locales` relative to the current package. Additional
//! directories can be configured in `Cargo.toml`:
//!
//! ```toml
//! [package.metadata.liora.locales]
//! paths = ["assets/locales", "../shared/locales"]
//! ```

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

type LocaleKeys = BTreeMap<String, BTreeMap<String, String>>;

/// Options for generating typed locale constants from TOML resources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalesCodegenOptions {
    /// Package directory. Defaults to `CARGO_MANIFEST_DIR`.
    pub manifest_dir: PathBuf,
    /// Cargo output directory. Defaults to `OUT_DIR`.
    pub out_dir: PathBuf,
    /// Output filename under `out_dir`.
    pub output_file: String,
    /// Rust path of the generated constant type.
    pub key_type_path: String,
    /// Default directory scanned when metadata paths are absent.
    pub default_dir: PathBuf,
}

impl LocalesCodegenOptions {
    /// Creates options from Cargo build-script environment variables.
    pub fn from_env(key_type_path: impl Into<String>) -> Self {
        let manifest_dir =
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
        let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
        Self {
            default_dir: manifest_dir.join("assets/locales"),
            manifest_dir,
            out_dir,
            output_file: "locales_keys.rs".into(),
            key_type_path: key_type_path.into(),
        }
    }

    /// Returns the generated Rust file path.
    pub fn output_path(&self) -> PathBuf {
        self.out_dir.join(&self.output_file)
    }
}

/// Generates `locales_keys.rs` for the current package.
///
/// The generator scans `./assets/locales` by default. If the current package's
/// `Cargo.toml` contains `[package.metadata.liora.locales] paths = [...]`, those
/// paths are scanned instead. Relative metadata paths are resolved against the
/// package directory.
pub fn generate_locales_from_package(key_type_path: impl Into<String>) {
    let options = LocalesCodegenOptions::from_env(key_type_path);
    generate_locales_with_options(&options);
}

/// Generates typed locale constants using explicit options.
pub fn generate_locales_with_options(options: &LocalesCodegenOptions) {
    let dirs = configured_locale_dirs(options);
    generate_locales_module(&dirs, &options.output_path(), &options.key_type_path);
}

/// Returns locale directories configured for `options`.
pub fn configured_locale_dirs(options: &LocalesCodegenOptions) -> Vec<PathBuf> {
    let manifest = options.manifest_dir.join("Cargo.toml");
    println!("cargo:rerun-if-changed={}", manifest.display());

    let mut dirs = package_metadata_locale_paths(&manifest, &options.manifest_dir);
    if dirs.is_empty() {
        dirs.push(options.default_dir.clone());
    }
    dirs
}

/// Generates a Rust module file from locale directories.
pub fn generate_locales_module(locale_dirs: &[PathBuf], output: &Path, key_type_path: &str) {
    let mut keys = LocaleKeys::new();
    for dir in locale_dirs {
        println!("cargo:rerun-if-changed={}", dir.display());
        collect_locale_dir(dir, &mut keys);
    }

    let generated = render_locales_module(&keys, key_type_path);
    fs::write(output, generated).expect("write generated locale keys");
}

fn package_metadata_locale_paths(manifest: &Path, manifest_dir: &Path) -> Vec<PathBuf> {
    let Ok(content) = fs::read_to_string(manifest) else {
        return Vec::new();
    };
    let value: toml::Value = toml::from_str(&content).unwrap_or_else(|error| {
        panic!(
            "failed to parse Cargo manifest {}: {error}",
            manifest.display()
        )
    });
    let Some(paths) = value
        .get("package")
        .and_then(|value| value.get("metadata"))
        .and_then(|value| value.get("liora"))
        .and_then(|value| value.get("locales"))
        .and_then(|value| value.get("paths"))
        .and_then(toml::Value::as_array)
    else {
        return Vec::new();
    };

    paths
        .iter()
        .map(|value| {
            let Some(path) = value.as_str() else {
                panic!("[package.metadata.liora.locales].paths entries must be strings");
            };
            let path = PathBuf::from(path);
            if path.is_absolute() {
                path
            } else {
                manifest_dir.join(path)
            }
        })
        .collect()
}

fn collect_locale_dir(dir: &Path, keys: &mut LocaleKeys) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut files = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
            files.push(path);
        }
    }
    files.sort();

    for path in files {
        println!("cargo:rerun-if-changed={}", path.display());
        let content = fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!("failed to read locale file {}: {error}", path.display())
        });
        let value: toml::Value = toml::from_str(&content).unwrap_or_else(|error| {
            panic!("failed to parse locale file {}: {error}", path.display())
        });
        collect_toml_keys(None, &value, keys);
    }
}

fn collect_toml_keys(prefix: Option<String>, value: &toml::Value, keys: &mut LocaleKeys) {
    match value {
        toml::Value::String(_) => {
            let Some(path) = prefix else {
                return;
            };
            let Some((group, key)) = path.split_once('.') else {
                panic!(
                    "locale key `{path}` must use at least one table group, e.g. [window] title = ..."
                );
            };
            keys.entry(sanitize_ident(group, &path))
                .or_default()
                .insert(sanitize_ident(key, &path), path);
        }
        toml::Value::Table(table) => {
            for (key, value) in table {
                let next = match &prefix {
                    Some(prefix) => format!("{prefix}.{key}"),
                    None => key.clone(),
                };
                collect_toml_keys(Some(next), value, keys);
            }
        }
        _ => {}
    }
}

fn sanitize_ident(raw: &str, full_path: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    if out.is_empty() || out.starts_with(|ch: char| ch.is_ascii_digit()) {
        out.insert(0, '_');
    }
    if !out
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        panic!("locale key `{full_path}` cannot be converted into a Rust identifier");
    }
    out
}

fn render_locales_module(keys: &LocaleKeys, key_type_path: &str) -> String {
    let mut out = String::from(
        "// @generated by liora-locales-codegen. Do not edit manually.\n\
         // Edit assets/locales/*.toml or [package.metadata.liora.locales].paths and rerun cargo.\n",
    );
    for (group, entries) in keys {
        out.push_str(&format!("pub mod {group} {{\n"));
        for (key, path) in entries {
            out.push_str("    #[allow(dead_code, non_upper_case_globals)]\n");
            out.push_str(&format!(
                "    pub const {key}: {key_type_path} = {key_type_path}::new({path:?});\n"
            ));
        }
        out.push_str("}\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn metadata_paths_are_resolved_against_manifest_dir() {
        let dir = temp_dir("liora-locales-codegen");
        fs::create_dir_all(dir.join("custom/locales")).unwrap();
        fs::write(
            dir.join("Cargo.toml"),
            r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"

[package.metadata.liora.locales]
paths = ["custom/locales"]
"#,
        )
        .unwrap();
        let options = LocalesCodegenOptions {
            manifest_dir: dir.clone(),
            out_dir: dir.join("out"),
            output_file: "locales_keys.rs".into(),
            key_type_path: "liora_core::Locales".into(),
            default_dir: dir.join("assets/locales"),
        };
        assert_eq!(
            configured_locale_dirs(&options),
            vec![dir.join("custom/locales")]
        );
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn default_scans_assets_locales() {
        let dir = temp_dir("liora-locales-codegen-default");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("Cargo.toml"),
            r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
"#,
        )
        .unwrap();
        let options = LocalesCodegenOptions {
            manifest_dir: dir.clone(),
            out_dir: dir.join("out"),
            output_file: "locales_keys.rs".into(),
            key_type_path: "liora_core::Locales".into(),
            default_dir: dir.join("assets/locales"),
        };
        assert_eq!(
            configured_locale_dirs(&options),
            vec![dir.join("assets/locales")]
        );
        fs::remove_dir_all(dir).unwrap();
    }

    fn temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{label}-{unique}"))
    }
}
