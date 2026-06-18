# Theme System

Aura 的主题系统提供 `System`、`Light`、`Dark` 三种模式。默认推荐使用 `System`，它会通过 GPUI 的 `WindowAppearance` 跟随操作系统外观；需要固定外观时再显式切换到 `Light` 或 `Dark`。

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

1. `init_aura_with_mode(cx, ThemeMode::System)` 初始化默认跟随系统。
2. 窗口创建后注册 `observe_window_appearance`。
3. 用户切换分段控件时调用 `apply_theme_mode(window, cx, mode)`。
4. 处于 `System` 模式时，系统外观变化由 `sync_system_theme(window, cx)` 自动刷新。

这套能力仍然是纯 Rust + GPUI 原生应用能力，不依赖 Tauri、WebView、HTML、CSS 或 DOM runtime。
