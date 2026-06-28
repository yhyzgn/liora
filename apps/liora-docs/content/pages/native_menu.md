# NativeMenu

`NativeMenu` / `NativeMenuItem` 是平台中立的应用菜单描述和原生 GPUI 预览组件。它适合三类场景：未来接入 macOS/Windows/Linux 平台菜单、自定义 `TitleBar` 菜单、以及把应用命令结构桥接到 Command Palette。

## 能力

- 顶层菜单标题和命令项。
- 快捷键展示。
- 分隔线。
- 禁用项。
- 嵌套 submenu 描述。
- 普通 item 自动使用 pointer cursor，并可通过 `on_select` 分发 action。
- 内置常用 action：NewWindow、Open、OpenFile、OpenFiles、OpenFolder、OpenFolders、Save、SaveAs、Close、Quit、CommandPalette、ToggleSidebar、ToggleStatusBar、ZoomIn、ZoomOut、ZoomReset、OpenUrl、CopyText、Custom。
- 可直接渲染为 Liora 原生预览，便于 Gallery/Docs/设置页展示同一份 descriptor。

## Descriptor + Preview

### 效果

::LioraDemo{component="NativeMenuDescriptor"}::

### 代码

```rust src="native_menu/descriptor.rs"
```


## Horizontal Menu Bar

横向多菜单组适合自定义 `TitleBar`、顶部应用菜单栏或设置页中的菜单预览。每个菜单组仍然是独立的 `NativeMenu` descriptor，可以单独复用到平台菜单、命令面板或快捷键说明中。

### 效果

::LioraDemo{component="NativeMenuBar"}::

### 代码

```rust src="native_menu/bar.rs"
```

## Action Catalog

`NativeMenuAction::catalog()` 可用于在帮助页、设置页或文档中展示全部内置 action。

### 效果

::LioraDemo{component="NativeMenuActions"}::

### 代码

```rust src="native_menu/actions.rs"
```

## 集成建议

`NativeMenu` 是描述层，不直接绑定具体平台菜单生命周期。应用可以把同一份 descriptor：

1. 交给平台适配层注册系统菜单；
2. 渲染到自定义 `TitleBar` 的菜单入口；
3. 转换为命令面板数据源；
4. 在设置页或关于页中展示当前快捷键。

其中 `Open` / `OpenFile` / `OpenFiles` / `OpenFolder` / `OpenFolders` 会通过 GPUI 官方 `prompt_for_paths` 打开系统文件/目录选择窗口，`Save` / `SaveAs` 会通过 `prompt_for_new_path` 打开系统保存路径窗口，并用 `on_paths_selected` 回传 `Option<Vec<PathBuf>>`。`Close`、`Quit`、`OpenUrl`、`CopyText`、`ZoomIn`、`ZoomOut`、`ZoomReset` 也是 Liora 可以直接执行的通用平台效果；NewWindow、CommandPalette、ToggleSidebar、ToggleStatusBar、Custom 只表达标准命令语义，应用应在 `on_select` 中根据自己的窗口、文件、布局、缩放或命令面板状态完成处理。Gallery/Docs 预览面板通常使用 `.perform_builtin_actions(false)`，避免点击示例时真正退出程序、打开浏览器或写入剪贴板；真实应用菜单可保持默认开启。
