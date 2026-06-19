# Theme System

Liora 的主题系统提供 `System`、`Light`、`Dark` 三种模式。默认推荐使用 `System`，它会通过 GPUI 的 `WindowAppearance` 跟随操作系统外观；需要固定外观时再显式切换到 `Light` 或 `Dark`。

## 三种模式

- `System`：启动时读取当前窗口外观，并通过 `observe_window_appearance` 响应系统浅色 / 深色变化。
- `Light`：固定浅色主题，不再跟随后续系统外观变化。
- `Dark`：固定深色主题，不再跟随后续系统外观变化。

```rust
src="theme/system_mode.rs"
```

## Token 使用原则

组件内部不要写死白底、黑遮罩、浅灰边框或固定 hover 色。优先从 `Config` 读取当前主题，并使用语义 token：

- 表面：`theme.neutral.body`、`theme.neutral.card`、`theme.neutral.popover`、`theme.neutral.modal`
- 文本：`theme.neutral.text_1`、`theme.neutral.text_2`、`theme.neutral.text_3`、`theme.neutral.text_disabled`
- 交互：`theme.neutral.hover`、`theme.neutral.pressed`、`theme.primary.light_9`
- 遮罩：`theme.neutral.overlay`、`theme.neutral.mask`
- 彩色面上的文本：`theme.neutral.inverted`

## 暗色模式下的 subtle token

`primary.light_9` / `light_8` / `light_7` 在浅色模式下是偏白的色彩 tint；在暗色模式下则是低透明度的语义色 overlay。这样 Table hover、Picker selected chip、Upload drag hover 等区域不会在深色背景上变成刺眼的浅色块。

## 应用层接入

Gallery 和 Docs 都使用同一套接入方式：

1. `liora::init_liora(cx)` 初始化默认跟随系统，并统一注册组件全局服务和 key bindings。
2. 如果产品需要固定启动主题，调用 `liora::init_liora_with_mode(cx, ThemeMode::Light | ThemeMode::Dark | ThemeMode::System)`。
3. 最大化启动窗口使用 `startup_maximized_window_bounds(cx, fallback)`：它保留 GPUI 的 `WindowBounds::Maximized` 语义，并把当前显示器可用区域作为 restore/fallback bounds。
4. 发布到 crates.io 的 Liora SDK 不耦合、不嵌入本仓库 `third_party/zed` 下的 GPUI patch；SDK 依赖面保持在 crates.io `open-gpui` / `open-gpui-platform`，避免把应用根 manifest 才能决定的 backend patch 传播给库用户。
5. Gallery / Docs 默认也按工作区 `open-gpui` 依赖构建；如需本地验证 Linux 首帧最大化修复，可在临时分支修改根 `[workspace.dependencies]` alias，让所有 Liora crates 一起切到 Zed git `gpui` / `gpui_platform` 包，并在应用根使用 `[patch."https://github.com/zed-industries/zed"]` 指向 `third_party/zed/crates/gpui` 与 `third_party/zed/crates/gpui_linux`。不要把这种 override 提交到发布 SDK manifest，也不要发布到 Liora crates。
6. 注意：`third_party/zed` 中的包名是 upstream `gpui` / `gpui_linux`，不能直接 patch crates.io 包名 `open-gpui` / `open-gpui-linux`；如果应用仍依赖 `open-gpui`，需要使用重命名后的私有 fork，或在临时分支改用 Zed git 依赖再应用 git-source patch。
7. 窗口选项保留 `show: false` 并在 `open_window` 返回 handle 后调用 `window.activate_window()`，与 Zed 主窗口显示时机保持一致；但 Linux 尺寸首帧正确性最终取决于应用选择的 GPUI backend 是否在平台窗口创建阶段处理初始 `Maximized` / `Fullscreen` 状态。
8. 在 `open_window` 回调一开始、创建 root view 之前调用 `attach_system_theme_observer(window, cx)`；它会先同步一次，再保活 `observe_window_appearance` 以跟随后续系统变化。
9. Linux / FreeBSD 启动时，`System` 会优先读取同步可用的桌面偏好（`GTK_THEME`、GTK settings、`gsettings org.gnome.desktop.interface color-scheme`），避免 GPUI Linux 后端默认 `Light` 等待 xdg-desktop-portal 异步回传时造成首帧浅色。
10. 用户切换分段控件时调用 `apply_theme_mode(window, cx, mode)`。
11. 处于 `System` 模式时，系统外观变化由 `sync_system_theme(window, cx)` 自动刷新。

这套能力仍然是纯 Rust + GPUI 原生应用能力，不依赖 Tauri、WebView、HTML、CSS 或 DOM runtime。
