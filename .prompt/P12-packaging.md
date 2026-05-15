# P12 — Native Installer Packaging

## Goal

Build a cross-platform installer and package generation pipeline for Aura's pure Rust + GPUI native applications.

## Non-negotiable Constraint

Aura applications must remain pure Rust + GPUI native apps. Do not convert `aura-gallery`, `aura-docs`, or future Aura apps to Tauri. Do not introduce WebView, HTML/CSS/DOM, browser runtime, or frontend build systems as application runtime dependencies.

## Naming Decision

The internal packaging module is named `aura-packager`, not `aura-installer`.

- `aura-packager` is a Rust library for packaging domain logic.
- `xtask` is the command entrypoint: `cargo run -p xtask -- package ...`.
- `packaging/` stores static platform resources and packager configuration.

## Technical Direction

- `crates/aura-packager`: app metadata, package formats, checksums, output manifests, validation helpers.
- `xtask`: build orchestration, app/format selection, future cargo-packager/RPM backend invocation.
- `packaging/`: Packager config, icons, Linux desktop/metainfo, macOS entitlements, Windows installer resources.
- Primary backend: `cargo-packager` for app/dmg/deb/AppImage/NSIS/MSI/Pacman where practical.
- RPM backend: `cargo-generate-rpm` or `nfpm` as a supplemental path.

## Required Package Formats

- Linux: AppImage, deb, rpm, portable tar.gz.
- macOS: app, dmg.
- Windows: NSIS exe, MSI.

## Current Implementation Baseline

- `docs/packaging-installer-technical-plan.md` is the source technical plan.
- Initial `crates/aura-packager` crate exists.
- Initial `xtask package validate/build/package` command exists.
- `packaging/` contains initial app metadata and platform integration skeletons.

## Next Work

1. Replace placeholder app icons with real PNG/ICNS/ICO assets.
2. Integrate `cargo-packager` backend invocation from `xtask`.
3. Add RPM generation backend.
4. Generate checksums and `package-manifest.json` for produced artifacts.
5. Add Linux AppImage/deb/rpm smoke checks.
6. Add CI matrix for Linux/macOS/Windows packaging.

---

## Handoff Snapshot — 2026-05-15

> 接手入口：本节是打包器当前进度和剩余工作的最新交接信息。后续继续 P12 时优先阅读本节，再看 `docs/packaging-installer-technical-plan.md`。

### 已完成并推送

- `crates/aura-packager`
  - app metadata / known app registry：Gallery、Docs。
  - package format model：Linux/macOS/Windows 默认格式。
  - checksum：SHA-256。
  - package manifest：JSON manifest、`checksums.txt`、`release-notes.md`。
  - `cargo-packager` 配置生成。
  - `cargo-generate-rpm` metadata overwrite 配置生成。
- `xtask`
  - `cargo run -p xtask -- package validate`
  - `cargo run -p xtask -- package build --app <gallery|docs>`
  - `cargo run -p xtask -- package --app <gallery|docs> --format <format>`
  - `cargo run -p xtask -- package ci --all-apps --format platform-defaults`
  - `--dry-run --skip-build` 可生成后端配置并打印真实后端命令。
- `packaging/`
  - Gallery / Docs packager metadata skeleton。
  - Linux `.desktop` / metainfo。
  - macOS entitlements placeholder。
  - Windows nsis/wix resource folders（用 `.gitkeep` 跟踪空目录，确保 GitHub runner validate 通过）。
  - app icon sets：`aura-gallery.*`、`aura-docs.*`。
  - main Aura brand icon 已选第 3 套 ribbon，落到 `packaging/icons/aura.*`。
- CI
  - `.github/workflows/package.yml` 已添加 Linux/macOS/Windows packaging matrix。
  - `workflow_dispatch` 默认 dry-run。
  - `main` push 触发 preview 打包，包版本使用 `AURA_PACKAGE_VERSION=<base>-preview.<run_number>.<short_sha>`。
  - `v*` tag 触发 release 打包，包版本使用 tag 去掉 `v` 后的版本。
  - 上传 `target/packages/**` 和 `target/aura-packager/*.toml`，artifact 命名区分 `aura-preview-packages-*` / `aura-release-packages-*`。
  - release job 下载各平台 release artifacts，按 `feat` / `fix` / `docs` / `ci` / `build` / `refactor` / `perf` / `test` / `style` / `chore` / `revert` / `Other` 分组收集 git changelog，创建/更新 GitHub Release，并上传全部构建产物。

### 已验证命令

```bash
cargo check -p xtask -p aura-packager
cargo test -p aura-packager
cargo run -p xtask -- package validate
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
```

Dry-run 预期生成：

```text
target/aura-packager/Packager.gallery.toml
target/aura-packager/Packager.docs.toml
target/aura-packager/GenerateRpm.gallery.toml
target/aura-packager/GenerateRpm.docs.toml
```

## Remaining Work / Next Developer TODO

### 1. 真实后端 smoke 验证（最高优先级）

本地或 CI 安装：

```bash
cargo install cargo-packager --locked
cargo install cargo-generate-rpm --locked
cargo run -p xtask -- package ci --all-apps --format platform-defaults
```

需要验证真实产物：

- Linux：AppImage、deb、rpm、pacman/tar-like package。
- macOS：`.app`、`.dmg`。
- Windows：NSIS `.exe`、WiX MSI `.msi`。

### 2. Linux runtime dependency metadata

补齐 `.deb` / `.rpm` 运行依赖，至少覆盖：

- Vulkan / GPU driver expectations。
- GTK3。
- AppIndicator / Ayatana。
- X11 / Wayland。
- fontconfig / freetype。
- xdg desktop integration。

### 3. 真正的 portable `.tar.gz` backend

当前 `tar.gz` 暂映射为 cargo-packager 的 `pacman`。如果需要中立 portable archive，需要新增专用 backend，收集：

- release binary；
- icons；
- desktop/metainfo；
- README / launch script；
- checksum / manifest entry。

### 4. Signing / notarization

接入但不要硬编码 secrets：

- macOS：`codesign`、`notarytool`、`stapler`。
- Windows：`signtool`、timestamp server。
- CI secrets + unsigned fallback policy。

### 5. GitHub Release automation

基础能力已接入：`main` push 会生成 preview 包；`v*` tag package matrix 完成后，release job 会下载 `aura-release-packages-*` artifacts，自动收集上一个 tag 以来的 git commit changelog，并按 Conventional Commit 类型分组生成 release notes，随后创建/更新 GitHub Release 并上传全部构建产物。

后续增强：

- 增加 draft / prerelease 策略；
- 对上传产物做最终命名清洗；
- 汇总多平台 manifest/checksum 为顶层 release manifest；
- 在真实 CI 包产物验证后补 release smoke gate。

### 6. Install / uninstall smoke scripts

建议新增平台 smoke：

- deb：`dpkg -i`、启动 smoke、卸载。
- rpm：`rpm -i`、启动 smoke、卸载。
- AppImage：可执行 smoke。
- macOS / Windows：runner-safe 的有限 install/open checks。

### 7. Artifact naming and metadata normalization

最终产物命名需要统一并扩充 manifest 字段：

- version；
- platform；
- target triple；
- git sha。

示例：

```text
aura-gallery-<version>-linux-x86_64.deb
aura-docs-<version>-windows-x86_64-setup.exe
```

### 8. License / metadata cleanup

当前仓库没有明确 `LICENSE` 文件。RPM config 暂用 `LicenseRef-Aura`。

下一位开发者需要二选一：

- 添加正式 OSS license，并同步 package metadata；或
- 明确私有 / proprietary license 策略。

### 9. CI real-run iteration

`.github/workflows/package.yml` 已通过本地结构检查，但还未证明完整 GitHub runner 成功。预期会需要修：

- Linux AppImage dependencies/tools；
- Windows WiX/NSIS availability；
- macOS dmg/codesign behavior。

## Guardrails for P12 Continuation

- 绝对不要把 Aura app 改成 Tauri。
- 保持应用为纯 Rust + GPUI native。
- packaging tools 可以使用，但 WebView / HTML / CSS / browser runtime 不能进入 app architecture。
- `xtask` 继续作为唯一公开打包入口。
