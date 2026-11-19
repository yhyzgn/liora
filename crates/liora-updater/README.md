# Liora Updater

Safe GitHub Release updater planning library.

## What it provides

The updater crate helps applications check GitHub releases, select platform-appropriate assets, verify checksums, and produce install plans. On Unix, a verified Linux AppImage download is marked executable for its owner before its install plan is returned. It does not silently escalate privileges or force app-specific hardcoded behavior.

Update checks compare release tags by SemVer precedence, with or without a leading `v`. A stable release is newer than a prerelease with the same major, minor, and patch, so `v0.6.2` is offered to an app running `v0.6.2-rc.1`. Build metadata does not make a release newer.

Generated install commands qualify relative artifact paths beginning with a filename or directory (including names beginning with `-`) with `./`, so they resolve against the current working directory at install time. Absolute paths and paths beginning with `.`, `..`, or a platform prefix are preserved. Commands supplied directly through a caller-owned `InstallAction` are passed through unchanged.

Downloads use an exclusively created staging file in the destination directory, verify its checksum and apply executable permissions before replacing a same-name cached asset with `std::fs::rename`. Failed download, verification, permission, or replacement operations remove the staging file (cleanup errors are returned); the updater never deletes the previous destination to make replacement succeed. Existing destination links are replaced as directory entries rather than opened for writing. Repeated downloads to the same cache remain supported.

The caller must own the cache directory and its parents and prevent untrusted concurrent modification; this API does not secure a shared writable directory against path replacement races. Same-directory rename is atomic on Unix. Windows replacement follows Rust's OS/filesystem-dependent rename behavior and can fail because of file sharing or permissions; no Windows atomicity or crash-durability guarantee is made. A replacement failure is reported without a delete-and-retry fallback.

`private_update_cache_dir(app, version, override_base)` creates or reuses a protected app/version cache. Without an override it uses the OS cache root (`$XDG_CACHE_HOME` or `~/.cache` on Linux, `~/Library/Caches` on macOS, LocalAppData on Windows), followed by `liora-updates/app/version`. An override is a base directory, must be absolute without `..`, and is resolved before use. Unsafe ownership, writable ancestors, app/version aliases, or unsupported filesystem protection return an error; there is no shared-temp fallback and existing permissions are not silently repaired. Gallery and Docs use this helper with their app identity, current package version and optional `LIORA_UPDATE_CACHE` base, and display failures through their existing update error state.

The helper validates resolved ancestors and restricts the app/version directories using Unix owner/permissions (including extended ACL checks on macOS) or Windows owner/DACL checks on a local ACL-enforcing volume. A trusted base may resolve through OS aliases; app/version directories may not. It protects against modification by other local users, not administrators, other processes running as the same user, or hostile remote filesystems. Always keep the returned resolved path for both download and the later install action. Raw `UpdateRequest::new` and `download_verified_asset` still accept caller-controlled directories with the ownership and concurrent-modification precondition above; they do not independently make an arbitrary shared directory race-safe.

Gallery and Docs allow only one update check or download at a time. Their update controls and action handlers reject conflicting checks, downloads and installs while that request is running. Starting either request revokes the previous install plan; a failed download or an up-to-date result therefore leaves no older plan available to install. A later request can be started after completion. This is app-owned UI coordination, not cancellation or serialization of independent SDK callers.

## Quick example

```rust
use liora_updater::{AssetSelector, Platform, UpdateRequest, private_update_cache_dir};

fn main() -> Result<(), liora_updater::UpdaterError> {
    let platform = Platform::current().unwrap_or(Platform::LinuxX64);
    let cache = private_update_cache_dir("my-native-app", "0.2.3", None)?;
    let _request = UpdateRequest::new("my-native-app", "v0.2.3", platform, cache);
    let _selector = AssetSelector::for_platform(platform).matching_prefix("my-native-app");
    Ok(())
}
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

