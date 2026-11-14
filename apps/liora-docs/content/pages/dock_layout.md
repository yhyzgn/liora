# DockLayout

`DockLayout` 是原生工作区布局，包含边缘面板、可交互中心 tab group，以及真正渲染内容的 Center panel。点击 tab 选择，点击 × 关闭；通过 GPUI 原生拖拽重排、跨组移动与分屏。

## Workbench 工作台

### 效果

::Demo{component="DockLayoutWorkbench"}::

### 代码

```rust src="dock_layout/workbench.rs"
```

## 左右检查器 + 底部日志

### 效果

::Demo{component="DockLayoutInspector"}::

### 代码

```rust src="dock_layout/inspector.rs"
```

## 原生交互

拖动 tab 到另一个 tab 前方插入，到 group 的内容区域末尾追加，或到 `Split ← / → / ↑ / ↓` 区域创建新分屏。关闭最后一个 tab 或把它移出 group 后会折叠空 group。`closable(false)` 隐藏关闭操作。原生拖拽只接受同一布局状态发起的 tab；即使 key 相同，另一个工作台的拖入也不会移动或复制内容。

`id` 绑定 GPUI 持久布局状态。重绘不会重新打开已关闭的 tab；宿主先移除后重新加入同一个 key 时视为新 tab。默认 id 来自调用位置；重复调用位置必须指定不同 id。

## 保存与恢复

`on_layout_change` 在选择、关闭、重排、移动或分屏后接收 `DockState`。使用 `to_json()` 持久化，使用 `DockState::from_json()` 解析并验证：重复 tab/group、无效 active key、无效分屏比例、非法 panel 尺寸均返回错误，而不是静默恢复伪布局。

新实例通过 `.layout_state(restored)` 初始化。此选项只用于首次创建 keyed state；要重置已存在工作台，请换一个 `id`。Gallery 的 “Restore last changed layout” 演示将修改后的 JSON 恢复到新实例。

描述只序列化稳定 key、group、active tab、方向、比例与 panel 位置/尺寸，不包含标题或 `AnyElement`。恢复时宿主仍须提供 `.tab(DockTab::new(...))` 和 `.panel(DockPanel::new(...))`；不存在的内容 key 会从布局清理，新内容 key 会加入首组。

业务内容由宿主所有。`Entity<T>` 可直接传入 tab/panel，并应由宿主持有以保留编辑状态；builder 每帧接收原生内容，布局 keyed state 不存储或消耗业务实体。不要存储一次性的 `AnyElement` 作为业务状态。

`DockState` 也提供 `select_tab`、`close_tab`、`move_tab`、`split_tab`。`move_tab` 的 index 是移动前目标组的插入间隙索引，`usize::MAX` 表示追加。分屏 group identity 耗尽时 `split_tab` 返回 `false`，保留原有 tab、group 和计数器，不产生部分修改。原生事件及新实例恢复后的内容、group/panel 几何与继续交互回归见 `layout_smoke`；模型及非法恢复边界见 `dock_layout` 单元测试。
