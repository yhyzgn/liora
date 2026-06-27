# Shell

`TitleBar` 和 `Sidebar` 用于构建应用级原生窗口骨架。`TitleBar` 负责自定义标题栏、拖拽区域、窗口控制按钮和右侧操作；`Sidebar` 负责侧栏宽度、折叠模式、头部、底部和滚动内容区域。实际导航仍推荐交给 `Menu`，页面主体继续由 `Container` 或业务 View 组织。

## 组合应用框架

### 效果

:::LioraDemo{component="ShellBasic"}::

### 代码

```rust src="shell/basic.rs"
```

## 使用建议

- `Menu`、`Input` 等有状态组件应保存在父 View 的 `Entity<T>` 字段中，不要在每次 `render` 临时创建。
- `TitleBar` 的拖动区域不要包住按钮，避免按钮点击被窗口拖动拦截。
- `Sidebar` 只负责外层布局和滚动容器，active route、搜索过滤和业务状态应留在应用层。
- 使用自定义标题栏时，窗口选项仍需要通过 `apply_window_frame_mode(..., WindowFrameMode::Custom)` 同步 GPUI 的 decoration 行为。
