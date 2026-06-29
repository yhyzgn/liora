# TitleBar

`TitleBar` 是 Liora 的原生自定义标题栏组件。它负责标题、副标题、左侧/中间/右侧插槽、拖动区域、双击标题栏行为和窗口控制按钮。真实应用通常把它交给 `AppWindowFrame::titlebar(...)` 使用；嵌入文档或组件卡片时可以关闭 `window_controls(false)`。

本页同步 Gallery 当前的 TitleBar 用例，重点展示右侧窗口控制按钮、左侧窗口控制按钮、居中命令区和无边框嵌入式工具条。

## 平台兼容边界

Liora 的自定义标题栏对齐 Zed 官方 GPUI 用法：

- Windows release `.exe` 应添加 `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`，避免 GUI 程序启动时额外出现空白控制台窗口。
- Windows/macOS 的系统标题栏隐藏由 `TitlebarOptions::appears_transparent` 在窗口创建时决定；运行中的窗口不能只靠 `request_decorations` 切掉系统 frame。
- Linux/FreeBSD 的 client/server decorations 可以通过 GPUI `WindowDecorations` 请求切换。
- 因此如果运行时切换 system/custom frame，Windows/macOS 应重开窗口并重新传入 `apply_window_frame_mode(...)` 后的 `WindowOptions`；Linux 可以 live switch。

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

let options = apply_window_frame_mode(
    gpui::WindowOptions {
        titlebar: Some(gpui::TitlebarOptions {
            title: Some("My App".into()),
            ..Default::default()
        }),
        ..Default::default()
    },
    WindowFrameMode::Custom,
);
```

## 右侧窗口控制按钮

### 效果

::Demo{component="TitleBarControlsRight"}::

### 代码

```rust src="titlebar/window_controls_right.rs"
```

## 左侧窗口控制按钮

### 效果

::Demo{component="TitleBarControlsLeft"}::

### 代码

```rust src="titlebar/window_controls_left.rs"
```

## 居中命令区

### 效果

::Demo{component="TitleBarCommandCenter"}::

### 代码

```rust src="titlebar/command_center.rs"
```

## 无边框嵌入式工具条

### 效果

::Demo{component="TitleBarBorderless"}::

### 代码

```rust src="titlebar/borderless.rs"
```

## 使用建议

- 真实窗口中搭配 `AppWindowFrame::new(...).mode(WindowFrameMode::Custom).titlebar(...)`。
- 常用样式可通过 `.height(...)`、`.padding_x(...)`、`.gap(...)`、`.actions_gap(...)`、`.background(...)`、`.border_color(...)`、`.border(...)`、`.title_color(...)`、`.subtitle_color(...)`、`.content_align(...)` 调整。
- 窗口控制位置可通过 `.window_controls_position(WindowControlsPosition::Left | Right)` 控制。
- 嵌入 demo、设置面板或普通卡片时使用 `.window_controls(false)`，避免误操作宿主窗口。
- 按钮等可点击控件应放在 `action(...)` 或 `actions(...)` 中，不要放进拖动区域。
