# Tray

`aura-tray` 是 Aura 的系统托盘封装层。它把 `tray-icon` 与 `muda` 收敛为 Aura 自己的 `TrayConfig`、`TrayMenuItemSpec` 和 `TrayCommand`，让 GPUI 主程序只处理窗口生命周期与业务命令。

> 托盘能力属于应用壳层，不是普通视图组件。启用托盘常驻时，GPUI 应用需要使用 `QuitMode::Explicit`，否则最后一个窗口关闭后进程会退出，托盘也会随之消失。

## 主窗口控制用例

`cargo run -p aura-gallery` 与 `cargo run -p aura-docs` 会在应用启动时分别创建独立的演示系统托盘图标：Gallery 使用蓝色图标，Docs 使用紫色图标；这两套 PNG 图标资源位于 `crates/aura-tray/assets/tray-icons/`，并通过 `bundled_tray_icon(...)` 引用。本页和 Gallery 内部的 Tray 页面额外在主窗口中提供按钮：切换动态图标、显示/隐藏主窗口、开启/关闭状态栏驻留、显示/隐藏托盘图标、切换自动显示。

### 效果

::AuraDemo{component="Tray"}::

### 代码

```rust src="tray/basic.rs"
```

## 状态栏驻留开关

应用可以在页面/设置中暴露「是否开启状态栏驻留」，不要只藏在托盘菜单里。开启时使用 `QuitMode::Explicit` 并保持托盘入口；关闭时隐藏托盘并恢复 `LastWindowClosed`，避免用户关闭窗口后留下不可见进程。

### 代码

```rust src="tray/residency.rs"
```

## 基础安装

最小配置包含托盘 id、tooltip、图标和菜单。`AuraTray` 需要被保存到应用状态中，确保生命周期覆盖整个进程。

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
