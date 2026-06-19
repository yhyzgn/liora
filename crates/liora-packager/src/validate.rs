use std::{fmt, fs, path::PathBuf};

use crate::known_apps;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Validation failures that can be reported by packaging layout checks.
pub enum ValidationError {
    /// Reports a required packaging path that does not exist.
    MissingPath {
        /// Human-readable label for the required packaging resource.
        label: String,
        /// Expected filesystem path that was missing.
        path: PathBuf,
    },
    /// Reports a packaging asset that exists but fails validation.
    InvalidAsset {
        /// Human-readable label for the invalid packaging resource.
        label: String,
        /// Filesystem path of the invalid asset.
        path: PathBuf,
        /// Explanation of why the asset failed validation.
        reason: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPath { label, path } => {
                write!(f, "missing {label}: {}", path.display())
            }
            Self::InvalidAsset {
                label,
                path,
                reason,
            } => {
                write!(f, "invalid {label}: {} ({reason})", path.display())
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Result of validating the package layout and app assets.
pub struct ValidationReport {
    /// Validation errors collected while checking the operation.
    pub errors: Vec<ValidationError>,
}

impl ValidationReport {
    /// Returns whether validation completed without errors.
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    fn require_path(&mut self, label: impl Into<String>, path: PathBuf) {
        if !path.exists() {
            self.errors.push(ValidationError::MissingPath {
                label: label.into(),
                path,
            });
        }
    }

    fn require_magic(&mut self, label: impl Into<String>, path: PathBuf, magic: &[u8]) {
        let label = label.into();
        match fs::read(&path) {
            Ok(bytes) if bytes.starts_with(magic) => {}
            Ok(bytes) => self.errors.push(ValidationError::InvalidAsset {
                label,
                path,
                reason: format!("unexpected file header; {} bytes", bytes.len()),
            }),
            Err(_) => self.require_path(label, path),
        }
    }

    fn require_svg(&mut self, label: impl Into<String>, path: PathBuf) {
        let label = label.into();
        match fs::read_to_string(&path) {
            Ok(text) if text.contains("<svg") && text.contains("</svg>") => {}
            Ok(text) => self.errors.push(ValidationError::InvalidAsset {
                label,
                path,
                reason: format!("not an SVG document; {} bytes", text.len()),
            }),
            Err(_) => self.require_path(label, path),
        }
    }
}

/// Validates that all required packaging assets, metadata files, and icon resources exist.
pub fn validate_packaging_layout(root: impl Into<PathBuf>) -> ValidationReport {
    let root = root.into();
    let mut report = ValidationReport::default();

    report.require_path("packaging directory", root.join("packaging"));
    report.require_path("packaging icons directory", root.join("packaging/icons"));
    report.require_path("packaging linux directory", root.join("packaging/linux"));
    report.require_path("packaging macos directory", root.join("packaging/macos"));
    report.require_path(
        "packaging windows directory",
        root.join("packaging/windows"),
    );

    require_icon_set(&mut report, &root, "liora", "Liora brand");

    for app in known_apps() {
        let metadata = app.metadata();
        report.require_path(
            format!("{} packager config", metadata.binary),
            metadata.packager_config_path(&root),
        );
        report.require_path(
            format!("{} linux desktop entry", metadata.binary),
            metadata.linux_desktop_path(&root),
        );
        report.require_path(
            format!("{} linux metainfo", metadata.binary),
            metadata.linux_metainfo_path(&root),
        );
        require_icon_set(&mut report, &root, &metadata.icon_stem, &metadata.name);
    }

    report
}

fn require_icon_set(
    report: &mut ValidationReport,
    root: &std::path::Path,
    stem: &str,
    label: &str,
) {
    let dir = root.join("packaging/icons");
    report.require_svg(format!("{label} svg icon"), dir.join(format!("{stem}.svg")));
    report.require_magic(
        format!("{label} png icon"),
        dir.join(format!("{stem}.png")),
        b"\x89PNG\r\n\x1a\n",
    );
    report.require_magic(
        format!("{label} ico icon"),
        dir.join(format!("{stem}.ico")),
        b"\x00\x00\x01\x00",
    );
    report.require_magic(
        format!("{label} icns icon"),
        dir.join(format!("{stem}.icns")),
        b"icns",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_layout_reports_missing_paths() {
        let report = validate_packaging_layout("target/definitely-missing-liora-packaging-layout");
        assert!(!report.is_ok());
        assert!(
            report
                .errors
                .iter()
                .any(|error| error.to_string().contains("packaging directory"))
        );
    }
}
