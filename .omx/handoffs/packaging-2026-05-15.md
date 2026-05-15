# Aura packaging handoff — 2026-05-15

## Current state

Implemented and pushed:

- `crates/aura-packager`: packaging domain logic for app metadata, package formats, checksum, manifest, cargo-packager config generation, RPM metadata generation.
- `xtask`: unified entrypoint via `cargo xtask package ...` and `cargo xtask package ci ...`.
- `packaging/`: app icons, Linux desktop/metainfo files, macOS entitlements placeholder, Windows folders.
- `.github/workflows/package.yml`: Linux/macOS/Windows packaging matrix, dry-run workflow dispatch, `v*` tag packaging path.
- Generated release metadata after real packaging runs:
  - `target/packages/package-manifest.json`
  - `target/packages/checksums.txt`
  - `target/packages/release-notes.md`
- Main Aura logo selected: candidate #3 ribbon, promoted to `packaging/icons/aura.*`.

## Known validation already run

```bash
cargo check -p xtask -p aura-packager
cargo test -p aura-packager
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package validate
```

Dry-run currently generates:

- `target/aura-packager/Packager.gallery.toml`
- `target/aura-packager/Packager.docs.toml`
- `target/aura-packager/GenerateRpm.gallery.toml`
- `target/aura-packager/GenerateRpm.docs.toml`

## Remaining work, recommended order

### 1. Real backend smoke validation

Install local/CI backends and remove `--dry-run`:

```bash
cargo install cargo-packager --locked
cargo install cargo-generate-rpm --locked
cargo xtask package ci --all-apps --format platform-defaults
```

Validate real artifacts for:

- Linux: AppImage, deb, rpm, pacman/tar-like package.
- macOS: `.app`, `.dmg`.
- Windows: NSIS `.exe`, WiX MSI `.msi`.

### 2. Linux runtime dependency metadata

Complete `.deb` / `.rpm` runtime dependencies for GPUI + tray:

- Vulkan / GPU driver expectations.
- GTK3.
- AppIndicator / Ayatana.
- X11 / Wayland.
- fontconfig / freetype.
- xdg desktop integration as needed.

### 3. True portable `.tar.gz` backend

Current `tar.gz` maps to cargo-packager `pacman`. If a neutral portable archive is required, add a dedicated backend that collects:

- release binary;
- icons;
- desktop/metainfo files;
- README / launch script;
- checksum and manifest entries.

### 4. Signing and notarization

Wire but keep secrets external:

- macOS: `codesign`, `notarytool`, `stapler`.
- Windows: `signtool`, timestamp server.
- CI secrets and unsigned fallback policy.

### 5. GitHub Release automation

Workflow currently uploads artifacts only. Add release creation on `v*` tags:

- create GitHub Release;
- upload installers;
- upload `checksums.txt` and `package-manifest.json`;
- use generated `release-notes.md` as release body.

### 6. Install/uninstall smoke scripts

Add platform smoke scripts:

- deb: `dpkg -i`, launch smoke, uninstall.
- rpm: `rpm -i`, launch smoke, uninstall.
- AppImage: executable launch smoke.
- macOS/Windows: limited runner-safe install/open checks.

### 7. Artifact naming/version metadata

Normalize final output names and manifest fields:

- include version, platform, target triple, git sha.
- examples:
  - `aura-gallery-<version>-linux-x86_64.deb`
  - `aura-docs-<version>-windows-x86_64-setup.exe`

### 8. License and metadata cleanup

Repo currently lacks an explicit `LICENSE` file. RPM config uses `LicenseRef-Aura` until a license decision is made.

Next developer should either:

- add a formal OSS license file and update package metadata; or
- keep private/proprietary metadata explicit.

### 9. CI real-run iteration

The workflow is structurally checked locally but not proven by a full GitHub runner packaging run. Expect platform-specific fixes, especially:

- Linux AppImage dependencies/tools;
- Windows WiX/NSIS availability;
- macOS dmg/codesign behavior.

## Guardrails

- Do **not** convert Aura apps to Tauri.
- Keep apps pure Rust + GPUI native.
- Packaging tools may be used, but no WebView/HTML/CSS/browser runtime should become part of the app architecture.
- Keep `xtask` as the public packaging entrypoint.
