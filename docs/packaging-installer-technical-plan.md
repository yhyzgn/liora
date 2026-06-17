# Aura 程序安装器构建打包技术方案

日期：2026-05-15  
状态：实施中 / preview CI 全平台打包已通过；install-smoke plan 已补；签名、公证、真实系统安装/卸载执行待外部策略
适用范围：`aura-gallery`、`aura-docs` 以及后续所有纯 GPUI Aura 主程序

## 1. 目标

为 Aura 的 GPUI 原生桌面程序建立一套可持续维护的跨平台安装器与发布产物构建体系，覆盖 Linux、macOS、Windows 的主流安装格式，并能在 CI 中自动构建、校验、归档、上传发布产物。

本阶段的核心目标不是改变应用运行时，而是补齐 **构建、打包、安装、卸载、桌面集成、签名预留、发布归档** 能力。

## 2. 绝对约束

- Aura 仍然是 **纯 Rust + GPUI 原生应用**。
- 严禁把 `aura-gallery`、`aura-docs` 或后续主程序改造成 Tauri 应用。
- 严禁引入 WebView、HTML/CSS/DOM、浏览器运行时、前端构建链作为应用主体。
- 可以参考 Tauri 的打包格式与 bundler 思路，但只能使用与运行时解耦的打包工具，不引入 Tauri Runtime。
- 打包配置必须面向多个 Aura app 复用，不能只为单个二进制硬编码。

## 3. 调研结论

### 3.1 GPUI / Zed

GPUI 本身不提供完整安装器体系。Zed 作为 GPUI 官方级大型应用，采用自建脚本体系：

- Linux：构建 release 二进制，收集动态库、图标、`.desktop` 文件，组装成 app 目录并打 `.tar.gz`，另有 Flatpak 脚本。
- macOS：使用 `cargo bundle` 生成 `.app`，再用 `hdiutil` 生成 `.dmg`，支持 codesign / notarization。
- Windows：PowerShell 脚本构建 release 二进制，通过 Inno Setup 生成安装器，并处理签名、资源与附加组件。

Zed 的优点是控制力强，缺点是脚本复杂度高，不适合 Aura 初期直接全量复制。

参考：

- Zed 安装文档：<https://zed.dev/docs/installation>
- Zed 打包脚本目录：<https://github.com/zed-industries/zed/tree/main/script>
- Zed Bundle Metadata：<https://github.com/zed-industries/zed/blob/main/crates/zed/Cargo.toml>

### 3.2 Zedis

Zedis 是更接近 Aura 当前体量的 GPUI 应用。它主要使用：

- `cargo build --release`
- `cargo bundle --release`
- `Cargo.toml` 中的 `[package.metadata.bundle]`
- `winres` / app icon / AppImage metadata

Zedis 的方式简单，适合作为初期参考，但格式覆盖不够完整，不能单独满足 Aura 的全平台安装器目标。

参考：

- Zedis 仓库：<https://github.com/vicanso/zedis>

### 3.3 Tauri / Tauri Bundler 思路

Tauri 的 bundler 覆盖面完整，包括：

- macOS：`.app`、`.dmg`
- Linux：`.deb`、`.rpm`、`.AppImage`
- Windows：NSIS `.exe`、MSI `.msi`

但完整 Tauri 是 WebView 应用框架，不适合 Aura。本项目只参考其打包产物与配置思想，不采用 Tauri Runtime。

### 3.4 Cargo Packager

`cargo-packager` 是当前最符合 Aura 需求的主工具：它是纯打包工具和库，用于给已有可执行文件生成安装器或 app bundle，不要求应用使用 Tauri Runtime。

官方支持格式：

- macOS：`.app`、`.dmg`
- Linux：`.deb`、`.AppImage`、Pacman `.tar.gz` / `PKGBUILD`
- Windows：NSIS `.exe`、WiX MSI `.msi`

参考：

- `cargo-packager` docs.rs：<https://docs.rs/cargo-packager/latest/cargo_packager/>
- Cargo Packager 文档：<https://docs.crabnebula.dev/packager/>

## 4. 最终技术选型

采用 **`aura-packager` 领域逻辑库 + `xtask` 编排 + `cargo-packager` 主打包 + 平台补充工具** 的混合方案。

### 4.0 命名决策

内部模块命名确定为 **`aura-packager`**，不使用 `aura-installer`。

原因：

- `installer` 容易被理解为安装时运行的 bootstrapper 或 GUI installer runtime。
- 本模块职责是打包领域逻辑、配置校验、manifest/checksum 生成，不是最终安装器进程。
- `packager` 与 `cargo-packager`、Zed bundle scripts、发布产物构建语义一致。

职责边界：

| 模块 | 职责 |
|---|---|
| `crates/aura-packager` | 可测试的打包领域逻辑库：app metadata、format enum、manifest、checksum、资源校验 |
| `xtask` | 命令入口和流程编排：调用 cargo build、cargo-packager、RPM 工具、CI 输出 |
| `packaging/` | 静态资源和平台配置：icons、desktop、metainfo、entitlements、Windows 配置 |

### 4.1 主路径

| 层级 | 工具 | 责任 |
|---|---|---|
| 打包领域逻辑 | `crates/aura-packager` | app metadata、format enum、manifest、checksum、配置校验、平台资源校验 |
| 构建编排 | `xtask` | 统一命令入口、app 选择、版本、目标格式、CI 适配 |
| Release 构建 | `cargo build --release -p <app>` | 生成 GPUI 原生二进制 |
| 主安装器生成 | `cargo-packager` | `.app`、`.dmg`、`.deb`、`.AppImage`、Pacman、NSIS、MSI |
| RPM 补充 | `cargo-generate-rpm` 或 `nfpm` | 生成 `.rpm` |
| Linux 桌面集成 | `.desktop`、metainfo、icons | 应用菜单、图标、MIME/URL scheme 预留 |
| macOS 签名预留 | `codesign` / `notarytool` | 后续正式发布签名和 notarization |
| Windows 签名预留 | `signtool` | 后续正式发布签名 |

### 4.2 为什么不用完整 Tauri

- Aura 已经是 GPUI 原生渲染，不需要 WebView。
- Tauri Runtime 会引入不必要的窗口模型、事件模型、配置语义与安全边界。
- 后续 Tray、窗口驻留、原生控件、GPUI 渲染路径都应由 Aura 自己控制。
- 打包工具可以借鉴 Tauri 生态，但应用架构不能迁移到 Tauri。

### 4.3 为什么不用纯 Zed 脚本方案

- Zed 脚本为大型编辑器服务，包含 remote server、CLI、Sentry、签名、Auto Update 等大量专用逻辑。
- Aura 当前需要可维护、可渐进演进的打包体系。
- `cargo-packager` 可减少平台脚本数量，同时保留必要的 escape hatch。

## 5. 产物覆盖矩阵

### 5.1 必须支持

| 平台 | 格式 | 用途 | 工具 |
|---|---|---|---|
| Linux | `.AppImage` | 通用便携运行 | `cargo-packager` |
| Linux | `.deb` | Debian / Ubuntu 安装 | `cargo-packager` |
| Linux | `.rpm` | Fedora / RHEL / openSUSE 安装 | `cargo-generate-rpm` 或 `nfpm` |
| macOS | `.app` | 原生 app bundle | `cargo-packager` 或 `cargo-bundle` |
| macOS | `.dmg` | 用户分发安装镜像 | `cargo-packager` |
| Windows | NSIS `.exe` | 推荐用户安装器 | `cargo-packager` |
| Windows | MSI `.msi` | 企业/系统管理分发 | `cargo-packager` + WiX |

### 5.2 应支持 / 后续增强

| 平台 | 格式 | 说明 |
|---|---|---|
| Linux | Pacman `.tar.gz` / `PKGBUILD` | Arch 系用户，后续可选 |
| Linux | portable `.tar.gz` | 中立解压运行包，Aura supplemental backend 已接入 |
| Linux | Flatpak | 后续如需 sandbox / Flathub 分发再补 |
| Linux | Snap | 非首选，按用户需求再补 |
| All | `.tar.gz` / `.zip` portable archive | CI smoke、开发者下载、回滚备用 |
| All | SHA256 checksums | 发布完整性校验 |
| All | minisign / cosign signature | 后续正式发布安全增强 |

## 6. 目录结构设计

新增：

```text
crates/
  aura-packager/
    Cargo.toml
    src/
      lib.rs
      app.rs
      checksum.rs
      format.rs
      manifest.rs
      validate.rs

xtask/
  Cargo.toml
  src/
    main.rs
    packaging/
      mod.rs
      app.rs
      build.rs
      linux.rs
      macos.rs
      windows.rs
      validate.rs

packaging/
  Packager.gallery.toml
  Packager.docs.toml
  icons/
    aura-gallery.png
    aura-gallery.icns
    aura-gallery.ico
    aura-docs.png
    aura-docs.icns
    aura-docs.ico
  linux/
    aura-gallery.desktop
    aura-docs.desktop
    aura-gallery.metainfo.xml
    aura-docs.metainfo.xml
    rpm/
      aura-gallery.toml
      aura-docs.toml
  macos/
    Entitlements.plist
  windows/
    wix/
    nsis/
  README.md
```

说明：

- `aura-packager` 是可测试、可复用的打包领域逻辑库，不直接作为最终用户命令入口。
- `xtask` 是唯一对外构建入口。
- `packaging/Packager.*.toml` 按 app 拆分，避免单文件配置复杂化。
- app icon 与 tray icon 分离：tray icon 服务运行时状态栏，app icon 服务安装器和系统桌面。
- Linux desktop/metainfo 需要显式维护，不能从 tray icon 推导。

## 7. 命令设计

### 7.1 本地命令

```bash
# 构建 release 二进制
cargo xtask package build --app gallery
cargo xtask package build --app docs

# 打单个格式
cargo xtask package --app gallery --format appimage
cargo xtask package --app gallery --format deb
cargo xtask package --app docs --format dmg
cargo xtask package --app docs --format nsis

# 当前平台所有推荐格式
cargo xtask package --app gallery --format platform-defaults

# 所有 app 当前平台默认格式
cargo xtask package --all-apps --format platform-defaults

# 校验打包资源和元数据
cargo xtask package validate
```

### 7.2 CI 命令

```bash
cargo xtask package ci --app gallery --format appimage
cargo xtask package ci --app docs --format deb
cargo xtask package ci --all-apps --format platform-defaults

# workflow_dispatch 默认为 dry-run，只生成并校验后端配置
cargo xtask package ci --all-apps --format platform-defaults --dry-run --skip-build
```

CI 模式要求：

- 输出路径稳定：`target/packages/<app>/<version>/<platform>/`
- 生成 `checksums.txt`
- 输出 machine-readable manifest：`package-manifest.json`
- 不依赖交互输入
- 签名缺失时允许构建 unsigned 包，但必须在 manifest 中标记

## 8. 应用元数据

每个主程序需要维护：

```text
id: dev.aura.Gallery / dev.aura.Docs
name: Aura Gallery / Aura Docs
binary: aura-gallery / aura-docs
version: workspace/app Cargo.toml version
vendor: Aura
category: DeveloperTool / Documentation
short_description: Native GPUI component gallery / Native Aura documentation app
long_description: ...
icons: png/icns/ico
license: project license
homepage: repository URL
repository: repository URL
```

建议 app id：

| App | App ID | Binary |
|---|---|---|
| Aura Gallery | `dev.aura.Gallery` | `aura-gallery` |
| Aura Docs | `dev.aura.Docs` | `aura-docs` |

如果未来有公司/组织域名，应统一替换为正式反向域名，例如 `cn.recycloud.aura.Gallery`。

## 9. 平台细节

### 9.1 Linux

必须安装运行依赖需要覆盖 GPUI 和 tray：

- Vulkan / GPU driver
- X11 / Wayland runtime
- fontconfig / freetype
- GTK / AppIndicator / ayatana-appindicator 相关库
- xdg-desktop-portal 相关能力

打包要求：

- `.desktop` 必须包含 `Name`、`Exec`、`Icon`、`Categories`、`StartupNotify`。
- AppImage 必须包含图标和 desktop entry。
- `.deb` / `.rpm` 必须声明系统依赖，不能只复制二进制。
- tray 驻留应用必须保证关闭最后窗口时进程不退出；这属于运行时逻辑，打包只负责 desktop integration。

推荐首批：

- `.AppImage`
- `.deb`
- `.rpm`
- portable `.tar.gz`

Flatpak 暂缓，原因：

- GPUI + tray + portal 权限需要单独验证。
- Flatpak sandbox 会改变文件系统、tray、portal 行为。
- 待常规包稳定后再作为独立子阶段。

### 9.2 macOS

必须支持：

- `.app`
- `.dmg`
- app icon `.icns`
- bundle identifier
- minimum system version
- entitlements 预留

正式发布前需要：

- Developer ID Application certificate
- `codesign --options runtime`
- `notarytool submit`
- `stapler staple`

本地开发可先生成 unsigned `.app` / `.dmg`。

### 9.3 Windows

必须支持：

- NSIS `.exe` 用户安装器
- MSI `.msi` 企业分发
- app icon `.ico`
- Start Menu shortcut
- uninstall entry

正式发布前需要：

- Authenticode 代码签名证书
- `signtool` 时间戳签名
- Windows Defender / SmartScreen 发行信誉积累

优先 NSIS，MSI 作为同阶段或下一阶段补齐。

## 10. CI / Release 设计

GitHub Actions matrix：

```text
linux-x86_64:
  - cargo test
  - cargo xtask package --all-apps --format appimage
  - cargo xtask package --all-apps --format deb
  - cargo xtask package --all-apps --format rpm

macos-aarch64 / macos-x86_64:
  - cargo test
  - cargo xtask package --all-apps --format dmg

windows-x86_64:
  - cargo test
  - cargo xtask package --all-apps --format nsis
  - cargo xtask package --all-apps --format msi
```

发布产物命名：

```text
aura-gallery-<version>-linux-x86_64.AppImage
aura-gallery-<version>-linux-x86_64.deb
aura-gallery-<version>-linux-x86_64.rpm
aura-gallery-<version>-macos-aarch64.dmg
aura-gallery-<version>-windows-x86_64-setup.exe
aura-gallery-<version>-windows-x86_64.msi

aura-docs-<version>-...
```

同时生成：

```text
checksums.txt
package-manifest.json
release-notes.md
```

## 11. 实施阶段

### Phase 1：资源和元数据准备

- [x] 生成/提交 app icon：PNG、ICNS、ICO。
- [x] 新增 `packaging/` 目录。
- [x] 为 Gallery / Docs 编写 Packager 配置。
- [x] 为 Linux 编写 `.desktop` 和 metainfo。
- [x] 明确 app id、名称、描述、license、homepage。

验收：

- `cargo xtask package validate` 能发现缺失资源。
- 所有配置可被解析。

### Phase 2：xtask 基础编排

- [x] 新增 `crates/aura-packager` crate。
- [x] 新增 `xtask` crate。
- [x] 实现 app registry：`gallery`、`docs`。
- [x] 实现 release build。
- [x] 实现格式路由与输出目录规范。
- [x] 生成 cargo-packager 兼容配置并通过 `cargo xtask package --dry-run` 输出真实后端命令。
- [x] 自动生成 checksums 和 manifest（真实后端产物生成后自动扫描 `target/packages` 并写入 `package-manifest.json` / `checksums.txt`）。

验收：

- `cargo xtask package build --app gallery` 成功。
- `cargo xtask package validate` 成功。

### Phase 3：Linux 包

- [ ] AppImage（cargo-packager 后端命令已接入，待安装本机后端工具后产物 smoke）。
- [ ] deb（cargo-packager 后端命令已接入，待安装本机后端工具后产物 smoke）。
- [x] rpm（`cargo-generate-rpm` supplemental backend 配置生成和命令路由已接入，待安装本机后端工具后产物 smoke）。
- [x] portable tar.gz（Aura supplemental backend 已接入，不再映射为 cargo-packager pacman）。
- [ ] Pacman / PKGBUILD（如需 Arch 原生包再补）。
- [x] artifact smoke：`xtask package smoke` 验证 portable tar.gz 结构，并对其他格式做 runner-safe 头部/非空检查。
- [x] 安装/卸载 smoke plan：`xtask package install-smoke` 默认生成 runner-safe plan-only 安装/启动/卸载命令，并可对 portable `.tar.gz` 显式 `--execute-install` 做安全解压/删除验证。

验收：

- `cargo xtask package --app gallery --format appimage`
- `cargo xtask package --app docs --format deb`
- 本地 Linux 至少完成 AppImage 启动 smoke。

### Phase 4：macOS 包

- [ ] `.app`。
- [ ] `.dmg`。
- [ ] unsigned 本地构建。
- [ ] signing/notarization 配置预留。

验收：

- macOS runner 能产出 `.dmg`。
- unsigned 包可本地打开。

### Phase 5：Windows 包

- [ ] NSIS `.exe`。
- [ ] MSI `.msi`。
- [ ] Start Menu shortcut。
- [x] uninstall smoke plan：`xtask package install-smoke` 生成 NSIS/MSI 静默安装/卸载计划；真实执行仍需 Windows runner policy / signing 后再打开。
- [ ] signing 配置预留。

验收：

- Windows runner 能产出 `.exe` 和 `.msi`。
- 安装后能从 Start Menu 启动。

### Phase 6：CI 发布流水线

- [x] 添加 GitHub Actions packaging workflow（`.github/workflows/package.yml`）。
- [x] tag push 触发 release build（`v*` tags）。
- [x] 上传 artifacts（`target/packages/**` 与生成的后端配置）。
- [x] 生成 checksums。
- [x] 生成 release manifest。
- [x] 生成 release notes markdown。

验收：

- tag 构建能产生 Gallery / Docs 全部目标平台安装包。

## 12. 风险与对策

| 风险 | 影响 | 对策 |
|---|---|---|
| Linux 运行依赖遗漏 | 安装后无法启动 | deb/rpm 显式依赖 + AppImage smoke |
| tray 在 Flatpak/sandbox 下行为不同 | 驻留能力异常 | Flatpak 延后单独验证 |
| macOS 未签名包安全提示 | 用户体验差 | 初期允许 unsigned，正式发布补签名/notarization |
| Windows SmartScreen | 用户安装阻碍 | 后续代码签名与发行信誉积累 |
| 多 app 配置重复 | 维护成本升高 | xtask app registry + 模板化 metadata |
| cargo-packager 不覆盖 RPM | Linux 覆盖不完整 | RPM 使用 `cargo-generate-rpm` 或 `nfpm` 补充 |
| GPUI 动态库/系统库处理不完整 | 包体运行失败 | 借鉴 Zed Linux `ldd` 检查和 smoke test |

## 13. 验收标准

本阶段最终完成时必须满足：

- 不引入 Tauri Runtime。
- `aura-gallery`、`aura-docs` 都能生成平台安装包。
- Linux 至少生成 `.AppImage`、`.deb`、`.rpm`。
- macOS 至少生成 `.app`、`.dmg`。
- Windows 至少生成 NSIS `.exe`、MSI `.msi`。
- 每个包都有正确 app icon、名称、版本、描述。
- 安装后可以从系统应用入口启动。
- Tray 驻留能力在安装包启动路径下仍可用。
- CI 可归档安装包、checksum、manifest。
- 所有打包逻辑通过 `cargo xtask package ...` 统一入口执行。

## 14. 当前实现状态

截至 2026-05-15：

- `cargo xtask package --app <gallery|docs> --format <fmt> --dry-run --skip-build` 会生成 `target/aura-packager/Packager.<app>.toml`，并打印实际 `cargo packager ...` 调用。
- `cargo xtask package ci ...` 已作为 CI 入口别名接入，`.github/workflows/package.yml` 在 Linux/macOS/Windows 矩阵中调用该入口。
- 非 dry-run 打包完成后会扫描 `target/packages/<app>/<platform>/` 下的安装包文件，生成 `target/packages/package-manifest.json`、`target/packages/checksums.txt` 和 `target/packages/release-notes.md`。
- `appimage`、`deb`、`app`、`dmg`、`nsis`、`msi` 走 cargo-packager 主后端；其中 `msi` 映射为 cargo-packager 的 `wix` 格式。
- `tar.gz` 走 Aura supplemental backend：收集 release binary、icons、Linux desktop/metainfo、README 和启动脚本，输出 `<package>-<version>-<platform>-<target-triple>.tar.gz`。
- `rpm` 仍归类为 supplemental backend，但已优先接入 `cargo-generate-rpm` 的 metadata overwrite 配置生成和 `cargo generate-rpm` 命令路由；`nfpm` 仅作为后备方案。
- `.deb` / `.rpm` 生成配置已补 Linux runtime dependency metadata。
- manifest 已扩充 `targetTriple` 与 `gitSha`，release notes 同步展示 version、target triple、git sha。
- `xtask package smoke` 已接入 CI artifact 上传前检查；`xtask package install-smoke` 已新增 plan-only 安装/启动/卸载命令审计，并允许 portable `.tar.gz` 显式 `--execute-install` 安全解压/删除验证。真正系统级 deb/rpm/AppImage/macOS/Windows install/uninstall 执行 gate 仍待签名、runner policy 和专用环境后再放开。Artifact 扫描会忽略 `.cargo-packager` 等隐藏后端工作目录，避免把 deb/rpm 内部归档误判为发布产物。
- `cargo-generate-rpm --metadata-overwrite` 使用 `GenerateRpm.<app>.toml#package.metadata.generate-rpm` 分支加载生成配置；生成 TOML 将 metadata 放在 `[package.metadata.generate-rpm]` 下、依赖放在 `[package.metadata.generate-rpm.requires]` 下，避免真实 runner 从错误表读取而丢失 `assets`。
- GitHub preview runner `27613242837` / commit `5a3615d` 已通过 Linux/macOS/Windows packaging matrix：真实 package generation、artifact smoke、raw binary upload、package artifact upload 均成功。
- 后续仍需在 `v*` tag 上验证 release job、GitHub Release asset 上传、Windows MSI，以及签名/公证和真正安装/卸载 smoke。

## 15. 决策记录

- 采用 `aura-packager` 作为 Aura 内部打包领域逻辑库。
- 采用 `cargo-packager` 作为主打包工具。
- 采用 `xtask` 作为统一编排入口，命令仍为 `cargo xtask package ...`，不直接要求用户运行 `cargo run -p aura-packager`。
- RPM 使用补充工具实现，不阻塞主路径。
- Flatpak/Snap 不作为第一批必须目标。
- 不采用完整 Tauri，不改变 GPUI 原生运行时。
- 不全量复制 Zed 打包脚本，只借鉴其平台经验和 smoke 思路。


### 2026-06-17 update: install/uninstall smoke plan

- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults` 已新增。默认是 plan-only，不会安装系统包；它会复用 artifact discovery 与 `xtask package smoke`，校验产物后写出 `target/packages/install-smoke-plan.md`。
- `--execute-install` 目前只允许 portable `.tar.gz` 走安全执行路径：解压到 `target/install-smoke/<package>`，验证 launcher 与 `bin/<binary>`，再删除目录。deb/rpm/AppImage/macOS/Windows 安装路径仍保持计划输出，避免 CI runner 或开发机被误安装/污染。
- GitHub Actions package workflow 在 artifact smoke 后新增 plan-only install/uninstall smoke gate，保证每次 preview/release 包都有明确可审计的安装、启动 smoke、卸载命令。
