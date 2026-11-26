# Theme System

Liora 的主题系统提供 `System`、`Light`、`Dark` 三种模式。默认推荐使用 `System`，它会通过 GPUI 的 `WindowAppearance` 跟随操作系统外观；需要固定外观时再显式切换到 `Light` 或 `Dark`。

## 三种模式

- `System`：GPUI 已初始化外观时使用其当前快照；尚未初始化时使用同步可用的桌面偏好，最后回退到 GPUI 默认值。通知发生本身不等于系统初值已返回。
- `Light`：固定浅色主题，不再跟随后续系统外观变化。
- `Dark`：固定深色主题，不再跟随后续系统外观变化。

## 原生实时预览

下方复用 Gallery 的 Theme 示例。使用应用顶部的 System / Light / Dark 控件切换模式，观察 Theme mode、主题名称、语义色和控件表面同步更新。System 表示跟随当前宿主外观；仅切换到 System 不代表已验证操作系统外观变化事件。

::Demo{component="Theme"}::

```rust src="theme/system_mode.rs"
```

## Token 使用原则

组件内部不要写死白底、黑遮罩、浅灰边框或固定 hover 色。优先从 `Config` 读取当前主题，并使用语义 token：

- 表面：`theme.neutral.body`、`theme.neutral.card`、`theme.neutral.popover`、`theme.neutral.modal`
- 文本：`theme.neutral.text_1`、`theme.neutral.text_2`、`theme.neutral.text_3`、`theme.neutral.text_disabled`
- 交互：`theme.neutral.hover`、`theme.neutral.pressed`、`theme.primary.light_9`
- 遮罩：`theme.neutral.overlay`、`theme.neutral.mask`
- 彩色面上的文本：`theme.neutral.inverted`

### 默认中性文本对比度

默认 Light / Dark 的 `text_1`、`text_2`、`text_3` 和 `placeholder`，在 body / card / modal / popover 表面及其 neutral hover / pressed 状态上，按透明度合成后的普通文本对比度至少为 4.5:1。占位符仍是需要可读的文本，不沿用豁免于该阈值的 disabled 颜色。自定义主题、额外叠层、彩色表面与完整无障碍符合性需要单独验证。

上方 Theme live 示例展示三级文本；色块标签放在中性表面而不是颜色内部。实际输入占位符可在 Input 的原生示例中查看。


## 暗色模式下的 subtle token

`primary.light_9` / `light_8` / `light_7` 在浅色模式下是偏白的色彩 tint；在暗色模式下则是低透明度的语义色 overlay。这样 Table hover、Picker selected chip、Upload drag hover 等区域不会在深色背景上变成刺眼的浅色块。

## 应用层接入

Gallery 和 Docs 都使用同一套接入方式：

1. `liora::init_liora(cx)` 初始化默认跟随系统，并统一注册组件全局服务和 key bindings。
2. 如果产品需要固定启动主题，调用 `liora::init_liora_with_mode(cx, ThemeMode::Light | ThemeMode::Dark | ThemeMode::System)`。
3. 最大化启动窗口使用 `startup_maximized_window_bounds(cx, fallback)`：它保留 GPUI 的 `WindowBounds::Maximized` 语义，并把当前显示器可用区域作为 restore/fallback bounds。
4. 当前 GPUI 使用本仓库维护的 `third_party/gpui` 源码修订，基于官方 `zed-industries/zed` 的 `f0d896f2c18a543335eb155f729d36b624695591`；这不是新的官方 revision。`gpui` 与 `gpui_platform` 必须使用根 `Cargo.toml` 记录的同一 Liora Git revision，九个相互依赖的平台 crate 保持同一 GPUI 类型身份。
5. SDK 保留 `gpui 0.2.2` registry fallback；crates.io 消费者在自己的应用根为 `gpui` 配置匹配的 `[patch.crates-io]`。不能依赖库的 root patch 传播，也不能把开发者本地 path 写入发布依赖；平台 feature 仍只由最终 app 启用。
6. `third_party/zed` 仅保留旧实验；其 manifest 使用 `.reference` 后缀防止 Cargo Git 包名冲突，不是当前依赖。AccessKit Busy/Disabled 回移仍是独立且不传播的应用根例外；其他 source override 不因此开放。来源与移除边界见 `third_party/gpui/README.md` 和 `docs/accesskit-atspi-backport.md`。
7. 窗口选项保留 `show: false` 并在 `open_window` 返回 handle 后调用 `window.activate_window()`，与 Zed 主窗口显示时机保持一致；但 Linux 尺寸首帧正确性最终取决于应用选择的 GPUI backend 是否在平台窗口创建阶段处理初始 `Maximized` / `Fullscreen` 状态。
8. 在 `open_window` 回调一开始、创建 root view 之前调用 `attach_system_theme_observer(window, cx)`；它会先同步一次，再保活 `observe_window_appearance` 以跟随后续系统变化。
9. Linux / FreeBSD 的 GPUI 外观尚未初始化时，`System` 读取同步桌面偏好（`GTK_THEME`、GTK settings、`gsettings org.gnome.desktop.interface color-scheme`），避免等待 portal 异步初值期间误用默认 Light。
10. 用户切换分段控件时调用 `apply_theme_mode(window, cx, mode)`，通过 GPUI 原生 `App::refresh_windows()` 为所有已打开窗口安排重绘。
11. 通过 `App::is_window_appearance_initialized()` 与 `Window::is_appearance_initialized()` 区分 pending Light 与已确认 Light；窗口颜色和 ready 状态来自同一份缓存快照。初值早于窗口/观察器、无窗口期间变化、固定模式切回 System 都不依赖通知历史。真实同色通知和窗口装饰通知保留，但 pending 状态不能覆盖同步启动提示。仅主题实际改变时刷新所有窗口，固定 Light/Dark 不受系统通知影响。portal 在初次读取前订阅同一个颜色 stream，避免读完初值后、尚未订阅时丢失变化；不新增平行监听、定时猜测或产品轮询。

GTK settings 的 `gtk-theme-name` 与 `gtk-application-prefer-dark-theme` 不依赖配置行顺序：`true` 优先请求深色变体；`false` 表示不额外请求深色，而不是强制浅色，因此仍保留主题名指定的深色外观。未取得可识别主题名时，显式 `false` 保留浅色启动回退。此规则只处理 GPUI 尚未初始化时的同步提示；参见 [GTK 属性定义](https://docs.gtk.org/gtk3/property.Settings.gtk-application-prefer-dark-theme.html)。

这套能力仍然是纯 Rust + GPUI 原生应用能力，不依赖 Tauri、WebView、HTML、CSS 或 DOM runtime。
