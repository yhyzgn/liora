# Tray

`liora-tray` 是 Liora 的系统托盘封装层。它把 `tray-icon` 与 `muda` 收敛为 Liora 自己的 `TrayConfig`、`TrayMenuItemSpec` 和 `TrayCommand`，让 GPUI 主程序只处理窗口生命周期与业务命令。

> 托盘能力属于应用壳层，不是普通视图组件。官方 Zed GPUI 当前不暴露公开退出模式切换 API；应用需要自己在窗口关闭钩子中决定退出进程、隐藏窗口或保持托盘入口。

## 主窗口控制用例

`cargo run -p liora-gallery` 与 `cargo run -p liora-docs` 会在应用启动时分别创建独立的系统托盘图标。Gallery 和 Docs 的 PNG/SVG 托盘状态图标分别位于 `apps/liora-gallery/assets/tray-icons/` 与 `apps/liora-docs/assets/tray-icons/`，由应用层通过 `icon_from_png_bytes(...)` 加载；`liora-tray` 只提供通用托盘 API，不内嵌任何应用专用资源。

本页和 Gallery 内部的 Tray 页面额外在主窗口中提供可视化控制台：状态来自全局 `TrayControlCenter`，按钮会分发真实 `TrayCommand`，用于切换动态图标、显示/隐藏主窗口、开启/关闭状态栏驻留、显示/隐藏托盘图标、切换自动显示。这样 Docs 看到的并不是普通按钮列表，而是与 Gallery 同步的真实 tray 状态预览。

### 效果

::LioraDemo{component="Tray"}::

### 代码

```rust src="tray/basic.rs"
```

## 状态栏驻留开关

应用可以在页面/设置中暴露「是否开启状态栏驻留」，不要只藏在托盘菜单里。开启时保留 `LioraTray` 并在关闭窗口时隐藏到托盘；关闭时隐藏托盘并在用户选择退出时调用 `cx.quit()`，避免留下不可见进程。

### 代码

```rust src="tray/residency.rs"
```

## 基础安装

最小配置包含托盘 id、tooltip、图标和菜单。`LioraTray` 需要被保存到应用状态中，确保生命周期覆盖整个进程。

### 代码

```rust src="tray/basic.rs"
```

## 动态修改图标

托盘图标可在运行时切换，适合同步中、异常、离线、未读消息等状态。菜单项可以映射为 `TrayCommand::SetIcon(name)`，主程序收到命令后调用 `set_icon` 或 `set_icon_from_path`。

### 代码

```rust src="tray/dynamic_icon.rs"
```

## CheckBox 菜单

`CheckMenuItem` 用于表达开关型设置，例如「开机启动」「启动时自动显示」「静音通知」。配置中保留稳定 command id 后，可用 `set_check_state` 与 `is_checked` 同步状态。

### 代码

```rust src="tray/checkbox.rs"
```

## 关闭窗口确认

启用状态栏驻留后，用户点击窗口关闭按钮时不应直接退出。推荐弹出确认：

- **关闭进程**：真正退出应用。
- **隐藏到托盘**：关闭主窗口，进程继续驻留在 tray；之后可从托盘菜单或主窗口控制入口重新创建并显示窗口。
- **记住本次选择**：保存为 `TrayCloseAction`，下次关闭窗口时直接执行该策略；也可以在主窗口页面恢复为「每次询问」。

### 代码

```rust src="tray/close_confirm.rs"
```

## 二级 / 三级 / N 级菜单

`TrayMenuItemSpec::submenu` 是递归结构，可以描述二级、三级甚至更深的原生菜单。建议业务命令仍保持扁平化，使用稳定 command id 做事件路由。

### 代码

```rust src="tray/nested_menu.rs"
```

## 平台注意事项

- Linux 需要 GTK/AppIndicator 相关系统库，并且托盘与事件循环在同一线程创建。
- macOS 要在主线程创建托盘；模板图标可用 `icon_is_template(true)` 适配深浅色菜单栏。
- Windows/Linux/macOS 的托盘对象都必须常驻保存，不要在局部函数结束时丢弃。
