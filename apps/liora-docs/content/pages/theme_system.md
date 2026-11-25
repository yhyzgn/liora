# Theme System

Liora 的主题系统提供 `System`、`Light`、`Dark` 三种模式。默认推荐使用 `System`，它会通过 GPUI 的 `WindowAppearance` 跟随操作系统外观；需要固定外观时再显式切换到 `Light` 或 `Dark`。

## 三种模式

- `System`：启动时优先采用可同步获取的系统外观偏好，未取得时回退到 GPUI 外观；后续通过 `observe_window_appearance` 采用 GPUI 更新后的窗口外观。
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
4. Liora SDK 不耦合、不嵌入本仓库 `third_party/zed` 下的 GPUI patch；当前依赖面使用 Zed 官方 `zed-industries/zed` git revision `f0d896f2c18a543335eb155f729d36b624695591`，并让 `gpui` 与 `gpui_platform` 保持同一 revision。
5. Gallery / Docs 也按同一官方 git revision 构建，并只在最终 app crate 中启用平台 feature。不要把本地 path GPUI、`[patch]` override 或非官方 fork 写入可发布 SDK manifest。
6. 注意：`third_party/zed` 是未发布的上游源码对照材料，不是 SDK 依赖；如需临时验证本地 patch，只能放在 app-only 临时分支。
   工作区旧有的全面 patch 禁令现仅对 `third_party/accesskit_atspi_common` 中官方 `0.19.1` 的 Busy/Disabled 上游回移作出显式应用根例外；不是 GPUI patch，不进入可发布 SDK manifest，也不会被 SDK 宿主继承。其他 crate/source override 仍禁止；具体来源、固定源码获取及宿主 root override 见仓库 `docs/accesskit-atspi-backport.md`。
7. 窗口选项保留 `show: false` 并在 `open_window` 返回 handle 后调用 `window.activate_window()`，与 Zed 主窗口显示时机保持一致；但 Linux 尺寸首帧正确性最终取决于应用选择的 GPUI backend 是否在平台窗口创建阶段处理初始 `Maximized` / `Fullscreen` 状态。
8. 在 `open_window` 回调一开始、创建 root view 之前调用 `attach_system_theme_observer(window, cx)`；它会先同步一次，再保活 `observe_window_appearance` 以跟随后续系统变化。
9. Linux / FreeBSD 启动时，`System` 会优先读取同步可用的桌面偏好（`GTK_THEME`、GTK settings、`gsettings org.gnome.desktop.interface color-scheme`），避免 GPUI Linux 后端默认 `Light` 等待 xdg-desktop-portal 异步回传时造成首帧浅色。
10. 用户切换分段控件时调用 `apply_theme_mode(window, cx, mode)`，通过 GPUI 原生 `App::refresh_windows()` 为所有已打开窗口安排重绘。
11. 收到首次 `observe_window_appearance` 通知前，初次挂载与手动切回 `System` 仍使用上述同步启动偏好。收到通知后（包括固定模式或主题颜色不变时），`System` 重选、显式同步和新窗口挂载均采用该窗口当前的 `window.appearance()`，不再让旧 GTK / gsettings 值覆盖实时外观；仅主题实际改变时刷新所有已打开窗口，固定 `Light` / `Dark` 的主题不受通知影响。新平台窗口从 GPUI 的应用级外观初始化，因此无窗口期间的系统变化也不会回退为旧窗口颜色。冷启动中 portal 初值早于观察器挂载且此后没有通知的 ready 竞争仍未解决。

这套能力仍然是纯 Rust + GPUI 原生应用能力，不依赖 Tauri、WebView、HTML、CSS 或 DOM runtime。
