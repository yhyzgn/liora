# Liora Updater

Safe GitHub Release updater planning library.

## What it provides

The updater crate helps applications check GitHub releases, select platform-appropriate assets, verify checksums, and produce install plans. On Unix, a verified Linux AppImage download is marked executable for its owner before its install plan is returned. It does not silently escalate privileges or force app-specific hardcoded behavior.

Update checks compare release tags by SemVer precedence, with or without a leading `v`. A stable release is newer than a prerelease with the same major, minor, and patch, so `v0.6.2` is offered to an app running `v0.6.2-rc.1`. Build metadata does not make a release newer.

Generated install commands qualify relative artifact paths beginning with a filename or directory (including names beginning with `-`) with `./`, so they resolve against the current working directory at install time. Absolute paths and paths beginning with `.`, `..`, or a platform prefix are preserved. Commands supplied directly through a caller-owned `InstallAction` are passed through unchanged.

Downloads use an exclusively created staging file in the destination directory, verify its checksum and apply executable permissions before replacing a same-name cached asset with `std::fs::rename`. Failed download, verification, permission, or replacement operations remove the staging file (cleanup errors are returned); the updater never deletes the previous destination to make replacement succeed. Existing destination links are replaced as directory entries rather than opened for writing. Repeated downloads to the same cache remain supported.

The caller must own the cache directory and its parents and prevent untrusted concurrent modification; this API does not secure a shared writable directory against path replacement races. Same-directory rename is atomic on Unix. Windows replacement follows Rust's OS/filesystem-dependent rename behavior and can fail because of file sharing or permissions; no Windows atomicity or crash-durability guarantee is made. A replacement failure is reported without a delete-and-retry fallback.

## Quick example

```rust
use liora_updater::{AssetSelector, Platform, UpdateRequest};

let platform = Platform::current().unwrap_or(Platform::LinuxX64);
let _request = UpdateRequest::new(
    "my-native-app",
    "v0.2.3",
    platform,
    std::env::temp_dir().join("liora-update-cache"),
);
let _selector = AssetSelector::for_platform(platform).matching_prefix("my-native-app");
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-updater` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## RPM installation and updates

RPM plans remain manual: SHA-256 verification does not establish publisher trust. Before installation, independently verify the publisher's full OpenPGP fingerprint, explicitly import its public key with `sudo rpm --import <key>`, and require `rpmkeys --define '_pkgverify_level all' --checksig ./package.rpm` to succeed. Fedora/RHEL users can then run `sudo dnf --setopt=localpkg_gpgcheck=1 install ./package.rpm`; openSUSE/SLES users can run `sudo zypper install ./package.rpm`. Reuse the trusted key for later updates. The updater never imports keys, elevates privileges, re-signs downloads, or disables system signature policy. Fedora's `rpm-setup-autosign` is for local `rpmbuild` testing, not for authenticating downloaded publisher packages.

## GPUI dependency note

Liora is developed against the official `zed-industries/zed` GPUI source. When building an application, follow the repository README for the exact GPUI git revision and patching guidance. Do not use unofficial GPUI forks unless the application owner explicitly accepts that risk.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

