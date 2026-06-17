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
  - package artifact 上传前运行 `cargo run -p xtask -- package smoke ...`，对 portable `.tar.gz` 做结构校验，并对其他格式做 runner-safe 头部/非空检查。
  - CI 真实反馈修正：`cargo-generate-rpm --metadata-overwrite` 必须使用 `GenerateRpm.<app>.toml#package.metadata.generate-rpm` 分支加载，且生成 TOML 必须把 metadata 放在 `[package.metadata.generate-rpm]` 下、依赖放在 `[package.metadata.generate-rpm.requires]` 下。
  - CI 真实反馈修正：artifact collection/smoke 必须忽略 `.cargo-packager` 等隐藏后端工作目录，避免把 deb 内部 `control.tar.gz` 误判为 Aura portable tar.gz。
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
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
```

### GitHub preview runner 验证

- `27613242837` / commit `5a3615d`：`Package native Aura apps` workflow 成功。
- Linux/macOS/Windows matrix 均完成：release binary build、package generation、artifact smoke、raw binary upload、package artifact upload。
- Linux 真实生成路径已覆盖 AppImage、deb、rpm、portable tar.gz；macOS 覆盖 app/dmg；Windows preview 覆盖 NSIS。
- 修复过的 CI 反馈：RPM metadata-overwrite 分支、RPM TOML 嵌套结构、artifact smoke 忽略 `.cargo-packager` 内部工作目录。

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

### 2. Linux runtime dependency metadata（已补）

已在生成配置中补齐 Linux 运行依赖：

- `.deb`：`[deb].depends` 包含 GTK3、Ayatana AppIndicator、X11、Wayland、xkbcommon、fontconfig/freetype、Vulkan、ALSA、xdg-utils。
- `.rpm`：`GenerateRpm.<app>.toml` 的 `[requires]` 包含对应 RPM 依赖，并继续保留 `auto-req = "builtin"` 和 `require-sh = false`。

后续只需在真实 Linux 发行版安装 smoke 后微调包名兼容性。

### 3. 真正的 portable `.tar.gz` backend（已补）

`tar.gz` 不再映射为 cargo-packager `pacman`，而是 Aura supplemental backend：

- 收集 `target/release/<binary>`；
- 收集 PNG/SVG app icons；
- Linux 下收集 `.desktop` 与 metainfo；
- 生成顶层启动脚本 `./<binary>`；
- 生成 portable `README.md`；
- 使用系统 `tar -czf` 输出 `<package>-<version>-<platform>-<target-triple>.tar.gz`；
- 非 dry-run 会被 manifest/checksum/release-notes 扫描记录。

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

### 7. Artifact naming and metadata normalization（部分已补）

已完成：

- portable `.tar.gz` 命名为 `<package>-<version>-<platform>-<target-triple>.tar.gz`；
- `package-manifest.json` 增加 `targetTriple` 与 `gitSha`；
- `release-notes.md` 展示 version、target triple、git sha；
- checksum 继续覆盖全部已发现产物。

待真实 cargo-packager 后端 smoke 后再对 `.deb` / `.rpm` / `.dmg` / `.exe` / `.msi` 做最终重命名清洗。

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


## Install/uninstall smoke plan — 2026-06-17

已新增并修正 `cargo run -p xtask -- package install-smoke ...`：

- 默认 plan-only，复用已存在 artifact discovery + `package smoke`，输出每个产物的 install / launch-smoke / uninstall 命令。
- `--dry-run` 是真正的 runner-safe 计划模式：根据 app / platform / format 生成预期产物路径和 install/uninstall plan，不依赖真实后端产物存在，也不会误扫陈旧 `target/packages` artifact。
- 写入 `target/packages/install-smoke-plan.md`，便于 CI artifact 审计。
- `--execute-install` 仅允许 portable `.tar.gz` 做安全解压/验证/删除；系统级 deb/rpm/AppImage/macOS/Windows 安装仍保持计划输出，等待真实 runner policy、签名和人工 QA 后再放开。
- GitHub Actions `package.yml` 已在 artifact smoke 后加入 plan-only install/uninstall smoke gate。
