# Menu / MenuBar

`Menu` / `MenuItem` / `MenuBar` 是基于 GPUI 官方 `Menu` / `MenuItem` / `App::set_menus` 的应用菜单描述、注册桥接和窗口内预览组件。它适合三类场景：接入 macOS/Windows/Linux 平台菜单、自定义窗口内菜单、以及把应用命令结构桥接到 Command Palette。

> 关键区别：`Menu::register(cx, menus)` 注册的是 GPUI 官方平台菜单，它不属于窗口内元素树；`MenuBar` 是 Liora 提供的窗口内可见 fallback/preview。是否需要两者同时使用，取决于平台和 frame 模式。

## 能力

- 顶层菜单标题和命令项。
- 快捷键展示。
- 分隔线。
- 禁用项。
- 嵌套 submenu 描述。
- 普通 item 自动使用 pointer cursor，并可通过 `on_select` 分发 action。
- 内置常用 action：NewWindow、Open、OpenFile、OpenFiles、OpenFolder、OpenFolders、Save、SaveAs、Close、Quit、CommandPalette、ToggleSidebar、ToggleStatusBar、ZoomIn、ZoomOut、ZoomReset、OpenUrl、CopyText、Custom。
- 可直接渲染为 Liora 原生预览，便于 Gallery/Docs/设置页展示同一份 descriptor。

## 平台菜单与窗口内 MenuBar 的区别

- **需要接入操作系统菜单语义**：使用 `Menu::register(cx, menus)`。它会直接委托 GPUI 官方 `App::set_menus`；macOS 通常显示在屏幕顶部全局菜单栏，Linux/Windows 是否显示由 GPUI 平台后端和桌面环境决定。
- **需要每个平台、每种 frame 下都在窗口内可见**：使用 `MenuBar::new(menus)`。`MenuBar` 是 Liora 的普通窗口内组件，可放在 `Container` header、`Shell` top slot 或自定义 `TitleBar`。
- **System frame 且接受平台决定菜单位置**：只调用 `Menu::register(...)`。这适合 macOS 原生体验；在某些 Linux/Wayland/KDE/GNOME/Windows 环境下窗口内可能看不到菜单。
- **System frame 但必须在窗口里看到菜单**：同时调用 `Menu::register(...)`，并在 header 中放 `MenuBar`。Gallery 采用这个模式：平台菜单保持注册，同时窗口 header 提供可见 fallback。
- **Custom frame / client-side decorations**：调用 `Menu::register(...)`，并在自定义 chrome/header 中放 `MenuBar`。自定义标题栏替换系统 chrome 后，平台不会自动把菜单插入你的 GPUI 元素树。
- **只做文档、设置页、命令结构预览**：只渲染 `MenuBar` 或单个 `Menu`，并关闭 `.perform_builtin_actions(false)`，避免示例点击触发退出、打开浏览器等真实副作用。

因此，`Menu::register(...)` 和 `MenuBar` 不是重复功能：前者交给平台，后者保证应用 UI 内稳定可见。真实应用通常维护同一份 `Menu` descriptor，然后分别用于平台注册、窗口内 fallback、命令面板和快捷键说明。

```rust
use gpui::App;
use liora_components::{Menu, MenuBar, MenuItem};

fn app_menus() -> [Menu; 2] {
    [
        Menu::new("File")
            .item(MenuItem::open_file())
            .item(MenuItem::open_folder())
            .item(MenuItem::separator())
            .item(MenuItem::quit()),
        Menu::new("Edit")
            .item(MenuItem::undo())
            .item(MenuItem::redo())
            .item(MenuItem::separator())
            .item(MenuItem::copy())
            .item(MenuItem::paste()),
    ]
}

fn register_platform_menu(cx: &mut App) {
    // GPUI official platform menu path: App::set_menus.
    Menu::register(cx, app_menus());
}

fn visible_fallback_menu_bar() -> MenuBar {
    // Window-internal fallback/preview. Put this in Container header, Shell, or TitleBar.
    MenuBar::new(app_menus()).perform_builtin_actions(false)
}
```

## GPUI 平台菜单注册

这一节是系统级 / 平台级菜单的正确集成方式。它调用 `Menu::register(cx, menus)`，实际委托给 GPUI 官方 `App::set_menus`。平台菜单不属于 Docs 内容区的 GPUI 元素树；运行 Gallery/Docs 应用时，菜单会按操作系统和窗口 frame 策略显示在平台菜单位置。

### 效果

::Demo{component="MenuPlatformRegistration"}::

### 代码

```rust src="menu/gpui_register.rs"
```

## Descriptor Preview

`Menu` descriptor 可以独立预览，用于文档、设置页、命令结构说明或调试。这不是系统级菜单栏，只是把同一份菜单数据渲染成窗口内说明面板。

### 效果

::Demo{component="MenuDescriptor"}::

### 代码

```rust src="menu/descriptor.rs"
```

## Window Fallback MenuBar

`MenuBar` 是窗口内 fallback / preview：适合自定义 `TitleBar`、`Shell` 顶部区域、Linux/Windows 下需要稳定可见的应用菜单入口，或设置页中的菜单预览。它不是 GPUI 平台菜单本体；真正的系统级平台菜单仍然应通过上面的 `Menu::register(cx, menus)` 注册。

### 效果

::Demo{component="MenuBar"}::

### 代码

```rust src="menu/bar.rs"
```

## Window Layout Usage

`MenuBar` 可以作为普通 Liora 控件放进 `TitleBar`、`Shell` 顶部栏或任何窗口内布局中。它只负责窗口内 fallback / preview；系统平台级菜单仍然通过 `Menu::register(cx, menus)` 注册。

### 效果

::Demo{component="MenuBarWindowUsage"}::

### 代码

```rust src="menu/window_usage.rs"
```

`Menu::register(cx, menus)` 只注册 GPUI 官方平台菜单；窗口内容里的 `File / Edit / View / Help` 这一行来自上面代码中的 `MenuBar::new(...)`。

## Action Catalog

`MenuAction::catalog()` 可用于在帮助页、设置页或文档中展示全部内置 action。

### 效果

::Demo{component="MenuActions"}::

### 代码

```rust src="menu/actions.rs"
```

## 集成建议

`Menu` 是 Liora 对 GPUI 官方应用菜单模型的描述层。真实平台菜单应通过 `Menu::register(cx, menus)` 或 `Menu::register_with_action_mapper(cx, menus, mapper)` 委托给官方 `App::set_menus`；窗口内可见 fallback/preview 则使用 `MenuBar`。应用可以把同一份 descriptor：

1. 通过 GPUI 官方 `App::set_menus` 注册系统菜单；
2. 渲染到自定义 `TitleBar` 的菜单入口；
3. 转换为命令面板数据源；
4. 在设置页或关于页中展示当前快捷键。

其中 `Open` / `OpenFile` / `OpenFiles` / `OpenFolder` / `OpenFolders` 会通过 GPUI 官方 `prompt_for_paths` 打开系统文件/目录选择窗口，`Save` / `SaveAs` 会通过 `prompt_for_new_path` 打开系统保存路径窗口，并用 `on_paths_selected` 回传 `Option<Vec<PathBuf>>`。`Close`、`Quit`、`OpenUrl`、`CopyText`、`ZoomIn`、`ZoomOut`、`ZoomReset` 也是 Liora 可以直接执行的通用平台效果；NewWindow、CommandPalette、ToggleSidebar、ToggleStatusBar、Custom 只表达标准命令语义，应用应在 `on_select` 中根据自己的窗口、文件、布局、缩放或命令面板状态完成处理。Gallery/Docs 预览面板通常使用 `.perform_builtin_actions(false)`，避免点击示例时真正退出程序、打开浏览器或写入剪贴板；真实应用菜单可保持默认开启。
