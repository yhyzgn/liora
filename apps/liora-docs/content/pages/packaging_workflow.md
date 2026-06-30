# Packaging Workflow

Liora keeps its native application packaging in the repository instead of relying on Tauri. The workflow builds pure Rust + GPUI desktop applications, stages the raw release executables, then creates platform installer/package formats through `liora-packager` and backend tools.

This page documents the packaging pipeline used by this repository and the pieces that downstream projects usually need to copy.

## Scope

Liora publishes three kinds of release outputs:

- crates.io GPUI SDK — `liora` is the one-stop facade published to crates.io; `liora-theme`, `liora-locales-codegen`, `liora-core`, `liora-icons`, `liora-icons-lucide`, `liora-icons-antd`, `liora-icons-ionic`, `liora-icons-tabler`, `liora-icons-carbon`, `liora-icons-material`, `liora-components`, `liora-tray`, `liora-packager`, and `liora-updater` are published with Cargo's `gpui = 0.2.2` registry fallback and are verified through a downstream `[patch.crates-io]` override to the official Zed GPUI git revision.
- `liora-docs` raw executables — cross-platform runnable documentation binaries for Linux, macOS, and Windows.
- `liora-gallery` raw executables plus installers — the component gallery is both directly downloadable as a binary and packaged into the planned native installer formats.

The reusable packaging logic lives in:

- `crates/liora-packager` — generic package metadata, format decisions, generated backend config, manifest/checksum helpers. App identities, publisher/license metadata, icons, and application directories are supplied by the caller or repository-local `xtask`, not hardcoded in the SDK.
- `crates/liora-updater` — generic GitHub Release checks, caller-defined platform asset selection, cached downloads, checksum verification, and install plans. Gallery/Docs wire their own app-local selectors in their app crates; the SDK does not contain official app presets.
- `xtask` — command-line entry point used locally and by CI.
- `.github/workflows/ci.yml` — ordinary quality gate for every pull request and `main` push.
- `.github/workflows/package.yml` — GitHub Actions preview/release pipeline for native app binaries and Gallery installers.
- `.github/workflows/release-sdk.yml` — GitHub Actions workflow that audits SDK crate metadata, packages the publishable crates, verifies a patched downstream consumer, and publishes all Liora SDK crates to crates.io in dependency order.
- `.github/workflows/runtime-verify.yml` — independent runtime verification workflow that starts after the native package workflow, downloads the produced binaries/installers, runs platform smoke checks on Linux/macOS/Windows runners, uploads machine-readable and Markdown reports, and appends the summary to the GitHub Release body for tagged releases.
- `scripts/runtime_verify.py` — shared runner-safe verifier used by the workflow for raw executable launch checks and package install/run/uninstall smoke checks.
- `packaging/` — icons, desktop metadata, macOS/Windows/Linux package resources.

## Local Commands

Validate packaging resources before building installers:

```bash
cargo run -p xtask -- package validate
```

Check release readiness policy gates before publishing:

```bash
cargo run -p xtask -- package release-readiness
```

The readiness command writes `target/packages/release-readiness.md` and checks packaging layout, explicit license policy, release tag/version policy, signing/notarization input policy, and the GitHub Release workflow wiring.

Build release binaries only:

```bash
cargo run -p xtask -- package build --all-apps --font-variant without-fonts
cargo run -p xtask -- package build --all-apps --font-variant with-fonts
```

Generate backend configs without building packages:

```bash
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
```

Build the planned Gallery installer/package formats:

```bash
cargo run -p xtask -- package ci --app gallery --format platform-defaults
```

Build one Gallery format:

```bash
cargo run -p xtask -- package ci --app gallery --format deb
```

Docs is intentionally distributed from this repository as raw executables only. Do not add Docs installer generation to `package.yml` unless the release policy changes.

## Workflow Responsibilities

Liora intentionally separates ordinary quality gates from packaging/release generation:

| Workflow | Trigger | Responsibility | Should publish release assets? |
| --- | --- | --- | --- |
| `.github/workflows/ci.yml` | pull requests, `main` pushes, manual dispatch | Fast correctness gate split into two jobs: `rust-quality` runs formatting, workspace check/test, and docs snippets with native Linux build dependencies; `packaging-dry-run` runs package metadata validation, packaging dry-run, and install-smoke dry-run with only lightweight packaging prerequisites. | No. It must not upload installers or mutate GitHub Releases. |
| `.github/workflows/package.yml` | `main` pushes, `v*` tags, manual dispatch | Native app release matrix for Linux/macOS/Windows: build raw release binaries for Docs/Gallery, generate installer/package artifacts for Gallery only, smoke Gallery package outputs, and plan Gallery install/uninstall checks. | Only `v*` tag runs publish GitHub Release assets. `main` runs produce preview Actions artifacts for QA. |
| `.github/workflows/release-sdk.yml` | manual dispatch, `v*` tags | SDK crate pipeline: static manifest audit verifies official `zed-industries/zed` pins, crates.io publish metadata, Cargo multiple-location `gpui` fallback, package archives, and a patched downstream consumer smoke check. | Only explicit `publish=true` manual runs or `v*` tags publish SDK crates to crates.io. Package verification alone never publishes. |
| `.github/workflows/runtime-verify.yml` | successful completion of `package.yml`, manual dispatch | Runtime gate for built release assets: launches raw Docs/Gallery executables, performs runner-safe Gallery package install/run/uninstall checks on the matching OS, and writes JSON/Markdown reports. | It does not create release assets. For `v*` releases it updates the existing GitHub Release body with the runtime verification report. |

Because Cargo does not allow crates.io packages to depend on git-only dependencies, Liora publishes SDK crates with Cargo's multiple-location `gpui` dependency: local development uses the official Zed git rev, while crates.io receives the registry fallback. Downstream applications should depend on `liora` from crates.io and add `[patch.crates-io] gpui = { git = "https://github.com/zed-industries/zed", rev = "..." }` so every transitive GPUI dependency resolves to the matching official Zed commit.

Keep `ci.yml` small and dependency-light enough to run on every code change. The workflow intentionally separates `rust-quality` from `packaging-dry-run` so package metadata changes can fail quickly without waiting for the full native workspace test job, and so packaging dry-run does not inherit GTK/Wayland/X11 dependencies it does not use. Keep `package.yml` responsible for expensive platform-specific app builds, raw binary staging, Gallery installer artifacts, grouped changelog generation, and GitHub Release publishing. Keep `release-sdk.yml` responsible for SDK metadata audits, patched-consumer verification, and crates.io package publication; it is the only workflow that may read `CRATES_IO_TOKEN` or call `cargo publish`. If a new package validation can run without real backend artifacts, add it to both workflows: `ci.yml` as a dry-run gate and `package.yml` before packaging. If a step builds installers, uploads artifacts, or calls `gh release`, it belongs only in `package.yml`. If a step publishes crates.io SDK crates, it belongs only in `release-sdk.yml`.

## CI Pipeline Steps

The packaging workflow runs on `push` to `main`, on `v*` tags, and by manual dispatch. The ordinary quality workflow also runs on pull requests and `main` pushes, but it stops at validation/dry-run gates.

### 1. Checkout and Rust toolchain

The workflow checks out the repository, installs the stable Rust toolchain, restores the cargo cache, and forces the toolchain cargo shim onto `PATH`. The cargo shim step avoids hosted-runner cache issues where an older cargo launcher may accidentally invoke `rustup-init`.

### 2. Configure package channel

The workflow derives the package channel and version from the Git ref:

- `main` push: `preview`
- `v*` tag: `release`

Preview versions use this shape:

```text
<base-version>-preview.<github-run-number>.<short-sha>
```

Release versions come from the tag name without the leading `v`.

### 3. Install platform packaging prerequisites

Linux installs GTK/Wayland/X11/audio/font/icon/rpm prerequisites plus `cargo-packager` and `cargo-generate-rpm`.

macOS installs `cargo-packager`.

Windows installs `cargo-packager`. Preview Windows builds intentionally use NSIS only because MSI requires a numeric-only Windows Installer version and does not accept Liora preview metadata like `<base-version>-preview.123.abcdef0`.

### 4. Validate and test packaging logic

CI runs:

```bash
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo test -p liora-packager
```

Validation catches missing icons, desktop metadata, package resources, generated config prerequisites, release policy gaps, and missing strict signing inputs before the expensive package step.

### 5. Build raw runnable binaries

CI builds release-mode application executables before packaging. `all` builds both runnable apps; `docs` builds only Docs; `gallery` builds only Gallery:

```bash
cargo run -p xtask -- package build --all-apps --font-variant without-fonts
cargo run -p xtask -- package build --all-apps --font-variant with-fonts
```

The built programs are staged under `target/liora-raw-binaries/<without-fonts|with-fonts>/...` with a `checksums.txt` file and uploaded as a separate Actions artifact named:

```text
liora-<preview|release>-binaries-<platform>
```

> Project-specific note: raw executables are first-class release outputs. Docs is released as cross-platform raw executables only. Gallery is released as raw executables and as installer/package artifacts.

### 6. Build installer/package artifacts

After raw binaries are built, Gallery package generation reuses the Gallery release executable via `--skip-build`:

```bash
cargo run -p xtask -- package ci --app gallery --format platform-defaults --skip-build --font-variant without-fonts
cargo run -p xtask -- package ci --app gallery --format platform-defaults --skip-build --font-variant with-fonts
```

Generated Gallery package outputs are uploaded as:

```text
liora-<preview|release>-gallery-packages-<platform>
```

The Actions package artifact bundle includes generated backend TOML files and package manifest/checksum/release-note files under `target/packages/`; the public GitHub Release uploads only distributable binaries/installers/archives plus one `SHA256SUMS.txt`. Public asset names include `without-fonts` or `with-fonts`: default `without-fonts` assets are smaller and do not bundle app font files, while `with-fonts` assets include MiSans resources or embedded fallback where applicable. The manifest records `version`, `platform`, `targetTriple`, optional `gitSha`, format, path, checksum, and signing state.

Linux `.deb` and `.rpm` configs include explicit runtime dependency metadata for GTK3, Ayatana/AppIndicator, X11/Wayland, xkbcommon, fontconfig/freetype, Vulkan, ALSA, and xdg desktop integration.

Portable `.tar.gz` is generated by Liora's supplemental backend, not by mapping to Arch pacman packages. It stages the release binary, PNG/SVG icons, Linux desktop/metainfo files, a top-level launcher script, and a README, then writes:

```text
<package>-<version>-<platform>-<target-triple>-<without-fonts|with-fonts>.tar.gz
```

### 7. Smoke package artifacts

When package files exist, CI runs a runner-safe smoke gate before upload:

```bash
cargo run -p xtask -- package smoke --app gallery --format platform-defaults
```

The smoke command validates package headers for distro/installer formats where possible and fully inspects portable `.tar.gz` archives for the expected binary, launcher, icons, README, and Linux desktop metadata.

CI then generates a runner-safe install/uninstall smoke plan:

```bash
cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults
```

`install-smoke` defaults to plan-only mode: it validates the discovered artifacts, writes `target/packages/install-smoke-plan.md`, and prints the exact install, launch-smoke, and uninstall commands that should be used for each format. It does not mutate the runner or install system packages unless a developer explicitly passes `--execute-install`. The only executable path currently allowed by `--execute-install` is portable `.tar.gz`, which extracts to `target/install-smoke/`, verifies the launcher and binary layout, then deletes the directory.

### 8. Preview artifacts

Every push to `main` produces preview artifacts for Linux, macOS, and Windows. These artifacts are retained for a shorter period and are intended for quick QA rather than public distribution. The preview matrix was runner-verified on GitHub Actions run `27613242837` for Linux, macOS, and Windows package generation, artifact smoke, raw binary upload, and package artifact upload.

### 9. Runtime verification report

After `package.yml` finishes successfully, `runtime-verify.yml` downloads the package run artifacts and verifies them on matching GitHub-hosted operating systems:

- Linux: raw executables are launched under `xvfb-run` when available; `.tar.gz` portable archives are extracted and launched; AppImage is run directly or through AppImage extraction fallback; `.deb` and `.rpm` packages are installed, launched, and removed in a runner-safe way.
- macOS: raw executables are launched long enough to prove startup; `.dmg` artifacts are mounted, the `.app` bundle is copied to a temporary directory, opened, and the image is detached.
- Windows: raw `.exe` files are launched; NSIS and MSI installers are installed silently into temporary locations where possible, the installed executable is launched, and uninstall cleanup is attempted.

The verifier treats a GUI process that remains alive until the smoke timeout as a successful startup. Each platform uploads JSON and Markdown reports, then the summary job publishes a combined runtime report. For tagged releases, the report is appended to the GitHub Release body between stable markers so repeated verification runs replace the previous report instead of duplicating it.

### 9. GitHub Release assets

When the workflow runs from a `v*` tag, the release job downloads both groups:

- `liora-release-gallery-packages-*`
- `liora-release-binaries-*`

It then selects only distributable files into `release-assets/`, gives them compact release names, generates grouped release notes with the asset table, writes one `SHA256SUMS.txt`, and uploads those files to the GitHub Release.

The release notes include three sections:

1. grouped changelog by commit type (`feat`, `fix`, `docs`, `ci`, `build`, `refactor`, `perf`, `test`, `style`, `chore`, `revert`, and `Other`),
2. installer/package artifacts,
3. raw runnable binaries with a clear note that Docs is raw-only and Gallery also has installer artifacts. Generated `.md` notes, TOML configs, package manifests, and portable staging internals are kept out of public release assets.


### 10. Release readiness, signing, and notarization gates

P12 no longer treats signing, notarization, license policy, and real release validation as loose TODO notes. They are represented by explicit repository gates:

- `LICENSE.md` documents the current `LicenseRef-Liora` policy until the owner chooses a formal OSS or commercial license.
- `packaging/signing-policy.md` documents macOS `codesign`/`notarytool`/`stapler` and Windows `signtool` inputs.
- `cargo run -p xtask -- package release-readiness` checks layout, license policy, release tag/version matching, signing inputs, and package workflow release wiring.
- `.github/workflows/ci.yml` runs the readiness gate in non-strict mode for ordinary validation.
- `.github/workflows/package.yml` runs the readiness gate before package generation; tagged `v*` release runs sign only when the release environment sets `LIORA_REQUIRE_SIGNING=true`; otherwise macOS/Windows artifacts are allowed as unsigned first-release builds and the release notes call that out.

For a real signed public release, create a protected `vX.Y.Z` tag that matches `crates/liora-packager/Cargo.toml`, configure the documented signing secrets, set `LIORA_REQUIRE_SIGNING=true`, and let `package.yml` publish the GitHub Release. If signing is not required, the workflow publishes unsigned artifacts and records that status in the release notes.


## Release Candidate Checklist

The repository-owned release-candidate checklist lives at `docs/release-candidate-checklist.md`. It covers the Liora 0.1.x release gate:

- local validation commands for formatting, workspace checks/tests, snippet checks, Rustdoc, packaging validation, release-readiness, dry-run packaging, install-smoke dry-run, and Gallery/Docs GUI smoke;
- package metadata expectations: SDK crates (`liora`, `liora-theme`, `liora-locales-codegen`, `liora-core`, `liora-icons`, `liora-icons-lucide`, `liora-icons-antd`, `liora-icons-ionic`, `liora-icons-tabler`, `liora-icons-carbon`, `liora-icons-material`, `liora-components`, `liora-tray`, `liora-packager`, `liora-updater`) use the repository license file and are crates.io-publishable; apps and `xtask` remain `publish = false`;
- the canonical app boundary: Gallery and Docs only, with no standalone `minimal-app` or `dashboard-app`;
- protected release-only work such as real `vX.Y.Z` tag publication, macOS notarization, Windows signing, and destructive system installer smoke tests.

## Downstream Project Guidance

If another GPUI project wants to reuse this packaging approach, copy the structure and keep only the pieces that match its product policy:

1. Keep `liora-packager`-style metadata for app id, binary name, icons, category, desktop metadata, and target formats.
2. Keep an `xtask` package command so local and CI packaging use the same path.
3. Keep `package validate` and packager unit tests in CI.
4. Use preview builds on branch pushes and release builds on version tags.
5. Upload installer/package artifacts for QA and releases.
6. Keep a neutral portable archive backend if users need a direct unpack-and-run fallback independent of distro package managers.
7. Split crates.io SDK publishing from app artifact publishing, and keep the crates.io token out of native app packaging jobs.
