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
4. Liora 的 patched GPUI 会把初始 `Maximized` / `Fullscreen` 状态传入 Linux 平台创建路径：Wayland 在首个 `surface.commit()` 前调用 `xdg_toplevel.set_maximized()`，X11 在 `MapWindow` 前写入 `_NET_WM_STATE_MAXIMIZED_VERT/HORZ`。这是避免“先默认大小、再最大化”的关键。
5. 窗口选项保留 `show: false` 并在 `open_window` 返回 handle 后调用 `window.activate_window()`，与 Zed 主窗口显示时机保持一致；但 Linux 尺寸首帧正确性依赖第 4 步的平台层初始状态，不依赖应用层延迟显示参数。
6. 在 `open_window` 回调一开始、创建 root view 之前调用 `attach_system_theme_observer(window, cx)`；它会先同步一次，再保活 `observe_window_appearance` 以跟随后续系统变化。
7. Linux / FreeBSD 启动时，`System` 会优先读取同步可用的桌面偏好（`GTK_THEME`、GTK settings、`gsettings org.gnome.desktop.interface color-scheme`），避免 GPUI Linux 后端默认 `Light` 等待 xdg-desktop-portal 异步回传时造成首帧浅色。
8. 用户切换分段控件时调用 `apply_theme_mode(window, cx, mode)`。
9. 处于 `System` 模式时，系统外观变化由 `sync_system_theme(window, cx)` 自动刷新。

这套能力仍然是纯 Rust + GPUI 原生应用能力，不依赖 Tauri、WebView、HTML、CSS 或 DOM runtime。
