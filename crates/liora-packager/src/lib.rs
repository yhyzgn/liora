//! Packaging domain logic for Liora native GPUI applications.
//!
//! This crate intentionally contains no installer runtime and no Tauri runtime
//! integration. It only models package metadata, package formats, generated
//! backend config, output manifests, checksums, and validation helpers used by
//! `cargo run -p xtask -- package ...`.
//!
//! Public release policy is enforced by `xtask package release-readiness`, which
//! checks layout, `LicenseRef-Liora`, tag/version policy, signing inputs, and
//! GitHub Release workflow wiring before formal packaging.

mod app;
mod cargo_packager;
mod checksum;
mod format;
mod manifest;
mod validate;

pub use app::{AppId, AppMetadata, KnownApp, known_apps};
pub use cargo_packager::{
    CargoPackagerPlan, cargo_packager_formats, generated_config_path, generated_rpm_config_path,
    package_out_dir, release_binaries_dir, render_cargo_packager_config,
    render_generate_rpm_config, supplemental_formats,
};
pub use checksum::{Checksum, sha256_file};
pub use format::{PackageFormat, Platform};
pub use manifest::{PackageArtifact, PackageManifest, collect_package_artifacts};
pub use validate::{ValidationError, ValidationReport, validate_packaging_layout};
