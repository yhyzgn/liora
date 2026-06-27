# Sidebar

`Sidebar` 是 Liora 的自管理侧栏组件。它拥有自己的宽度、边框、固定 header/footer 和可滚动内容区，适合承载 `Menu`、过滤器、工作区信息或账号入口。

## 独立侧栏

### 效果

:::LioraDemo{component="SidebarBasic"}::

### 代码

```rust src="sidebar/basic.rs"
```

## 使用建议

- `Sidebar` 自己管理宽度和边框；放入 `Container::aside(...)` 时请配合 `.aside_passthrough()`，避免双重侧栏宽度导致布局错乱。
- `Menu` 等有状态子组件应保存在父 View 的 `Entity<T>` 字段中。
- 侧栏只负责布局与滚动，路由、过滤、选中状态应留在应用层。
