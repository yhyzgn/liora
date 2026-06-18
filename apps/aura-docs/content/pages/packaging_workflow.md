# Packaging Workflow

Aura keeps its native application packaging in the repository instead of relying on Tauri. The workflow builds pure Rust + GPUI desktop applications, stages the raw release executables, then creates platform installer/package formats through `aura-packager` and backend tools.

This page documents the packaging pipeline used by this repository and the pieces that downstream projects usually need to copy.

## Scope

Aura currently packages two applications:

- `aura-gallery` — the component gallery and visual demo application.
- `aura-docs` — the standalone documentation application.

The reusable packaging logic lives in:

- `crates/aura-packager` — package metadata, format decisions, generated backend config, manifest/checksum helpers.
- `xtask` — command-line entry point used locally and by CI.
- `.github/workflows/ci.yml` — ordinary quality gate for every pull request and `main` push.
- `.github/workflows/package.yml` — GitHub Actions preview/release pipeline.
- `packaging/` — icons, desktop metadata, macOS/Windows/Linux package resources.

## Local Commands

Validate packaging resources before building installers:

```bash
cargo run -p xtask -- package validate
```

Build release binaries only:

```bash
cargo run -p xtask -- package build --all-apps
```

Generate backend configs without building packages:

```bash
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
```

Build platform default packages for both applications:

```bash
cargo run -p xtask -- package ci --all-apps --format platform-defaults
```

Build one application and one format:

```bash
cargo run -p xtask -- package ci --app docs --format deb
```

## Workflow Responsibilities

Aura intentionally separates ordinary quality gates from packaging/release generation:

| Workflow | Trigger | Responsibility | Should publish release assets? |
| --- | --- | --- | --- |
| `.github/workflows/ci.yml` | pull requests, `main` pushes, manual dispatch | Fast correctness gate split into two jobs: `rust-quality` runs formatting, workspace check/test, and docs snippets with native Linux build dependencies; `packaging-dry-run` runs package metadata validation, packaging dry-run, and install-smoke dry-run with only lightweight packaging prerequisites. | No. It must not upload installers or mutate GitHub Releases. |
| `.github/workflows/package.yml` | `main` pushes, `v*` tags, manual dispatch | Native app packaging matrix for Linux/macOS/Windows: build raw release binaries, generate installer/package artifacts, smoke package outputs, and plan install/uninstall checks. | Only `v*` tag runs publish GitHub Release assets. `main` runs produce preview Actions artifacts for QA. |

Keep `ci.yml` small and dependency-light enough to run on every code change. The workflow intentionally separates `rust-quality` from `packaging-dry-run` so package metadata changes can fail quickly without waiting for the full native workspace test job, and so packaging dry-run does not inherit GTK/Wayland/X11 dependencies it does not use. Keep `package.yml` responsible for expensive platform-specific packaging, raw binary staging, artifact upload, grouped changelog generation, and release publishing. If a new package validation can run without real backend artifacts, add it to both workflows: `ci.yml` as a dry-run gate and `package.yml` before packaging. If a step builds installers, uploads artifacts, or calls `gh release`, it belongs only in `package.yml`.

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

Windows installs `cargo-packager`. Preview Windows builds intentionally use NSIS only because MSI requires a numeric-only Windows Installer version and does not accept Aura preview metadata like `0.1.0-preview.123.abcdef0`.

### 4. Validate and test packaging logic

CI runs:

```bash
cargo run -p xtask -- package validate
cargo test -p aura-packager
```

Validation catches missing icons, desktop metadata, package resources, or generated config prerequisites before the expensive package step.

### 5. Build raw runnable binaries

CI builds release-mode application executables before packaging:

```bash
cargo run -p xtask -- package build --all-apps
```

The built programs are staged under `target/aura-raw-binaries/` with a `checksums.txt` file and uploaded as a separate Actions artifact named:

```text
aura-<preview|release>-binaries-<platform>
```

> Project-specific note: uploading the raw pre-installer executables is an Aura repository release policy. It is useful for smoke tests, direct debugging, and verifying the exact executable that was fed into packaging. Projects that only consume Aura as a component library do not need to upload raw binaries unless their own product release process requires it.

### 6. Build installer/package artifacts

After raw binaries are built, package generation reuses them via `--skip-build`:

```bash
cargo run -p xtask -- package ci --all-apps --format platform-defaults --skip-build
```

Generated package outputs are uploaded as:

```text
aura-<preview|release>-packages-<platform>
```

The package artifact bundle includes generated backend TOML files and, when outputs are discovered, package manifest/checksum/release-note files under `target/packages/`. The manifest records `version`, `platform`, `targetTriple`, optional `gitSha`, format, path, checksum, and signing state.

Linux `.deb` and `.rpm` configs include explicit runtime dependency metadata for GTK3, Ayatana/AppIndicator, X11/Wayland, xkbcommon, fontconfig/freetype, Vulkan, ALSA, and xdg desktop integration.

Portable `.tar.gz` is generated by Aura's supplemental backend, not by mapping to Arch pacman packages. It stages the release binary, PNG/SVG icons, Linux desktop/metainfo files, a top-level launcher script, and a README, then writes:

```text
<package>-<version>-<platform>-<target-triple>.tar.gz
```

### 7. Smoke package artifacts

When package files exist, CI runs a runner-safe smoke gate before upload:

```bash
cargo run -p xtask -- package smoke --all-apps --format platform-defaults
```

The smoke command validates package headers for distro/installer formats where possible and fully inspects portable `.tar.gz` archives for the expected binary, launcher, icons, README, and Linux desktop metadata.

CI then generates a runner-safe install/uninstall smoke plan:

```bash
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults
```

`install-smoke` defaults to plan-only mode: it validates the discovered artifacts, writes `target/packages/install-smoke-plan.md`, and prints the exact install, launch-smoke, and uninstall commands that should be used for each format. It does not mutate the runner or install system packages unless a developer explicitly passes `--execute-install`. The only executable path currently allowed by `--execute-install` is portable `.tar.gz`, which extracts to `target/install-smoke/`, verifies the launcher and binary layout, then deletes the directory.

### 8. Preview artifacts

Every push to `main` produces preview artifacts for Linux, macOS, and Windows. These artifacts are retained for a shorter period and are intended for quick QA rather than public distribution. The preview matrix was runner-verified on GitHub Actions run `27613242837` for Linux, macOS, and Windows package generation, artifact smoke, raw binary upload, and package artifact upload.

### 9. GitHub Release assets

When the workflow runs from a `v*` tag, the release job downloads both groups:

- `aura-release-packages-*`
- `aura-release-binaries-*`

It then flattens them into `release-assets/`, generates grouped release notes, and uploads everything to the GitHub Release.

The release notes include three sections:

1. grouped changelog by commit type (`feat`, `fix`, `docs`, `ci`, `build`, `refactor`, `perf`, `test`, `style`, `chore`, `revert`, and `Other`),
2. installer/package artifacts,
3. raw runnable binaries with a clear note that they are Aura-project convenience outputs.

## Downstream Project Guidance

If another GPUI project wants to reuse this packaging approach, copy the structure and keep only the pieces that match its product policy:

1. Keep `aura-packager`-style metadata for app id, binary name, icons, category, desktop metadata, and target formats.
2. Keep an `xtask` package command so local and CI packaging use the same path.
3. Keep `package validate` and packager unit tests in CI.
4. Use preview builds on branch pushes and release builds on version tags.
5. Upload installer/package artifacts for QA and releases.
6. Keep a neutral portable archive backend if users need a direct unpack-and-run fallback independent of distro package managers.
7. Upload raw runnable binaries only if the project explicitly wants direct executable downloads or release debugging artifacts.

