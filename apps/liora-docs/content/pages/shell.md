# Shell

`Shell` 是 Liora 的高层应用框架控件，统一封装应用窗口常见区域：可选自定义 `TitleBar`、header、left sidebar、right sidebar / inspector、可滚动 main、footer 和 overlay。它适合大多数产品型桌面应用；只有在需要更底层控制时，才直接组合 `AppWindowFrame`、`Container`、`TitleBar` 和 `Sidebar`。

本页同步 Gallery 当前的 Shell 用例：每一个效果后面都紧跟对应代码，避免先堆效果再集中展示代码。

## 完整产品框架

### 效果

::LioraDemo{component="ShellFullProduct"}::

### 代码

```rust src="shell/full_product.rs"
```

## System frame 内容优先工作区

### 效果

::LioraDemo{component="ShellContentFirst"}::

### 代码

```rust src="shell/content_first.rs"
```

## 嵌入式 Shell

### 效果

::LioraDemo{component="ShellMinimal"}::

### 代码

```rust src="shell/minimal.rs"
```

## 使用建议

- `Menu`、`Input` 等有状态组件应保存在父 View 的 `Entity<T>` 字段中，不要在每次 `render` 临时创建。
- `Shell` 负责区域结构；业务路由、搜索过滤、菜单 active 状态仍放在应用层。
- `titlebar(...)` 只在 custom frame 场景渲染；应用首次打开窗口时仍应通过 `apply_window_frame_mode(..., WindowFrameMode::Custom)` 同步 GPUI decoration 行为。
- Gallery / Docs 示例不要直接使用 GPUI 原生元素布局来绕过 SDK；如果缺少布局能力，应补充 Liora 组件封装。
