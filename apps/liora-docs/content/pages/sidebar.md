# Sidebar

`Sidebar` 是 Liora 的自管理侧栏组件。它拥有自己的宽度、边框、圆角、背景、固定 header/footer、可滚动内容区，以及常见的顶部 brand/logo 区域，适合承载 `Menu`、过滤器、工作区信息或账号入口。

本页同步 Gallery 当前的 Sidebar 用例，覆盖品牌区、长菜单滚动、右侧 inspector、折叠图标栏和完全自定义插槽。

## 品牌工作区侧栏

### 效果

::Demo{component="SidebarBrand"}::

### 代码

```rust src="sidebar/brand.rs"
```

## 长菜单滚动

### 效果

::Demo{component="SidebarScrollable"}::

### 代码

```rust src="sidebar/scrollable.rs"
```

## 右侧检查器

### 效果

::Demo{component="SidebarInspector"}::

### 代码

```rust src="sidebar/inspector.rs"
```

## 折叠图标栏

### 效果

::Demo{component="SidebarIconRail"}::

### 代码

```rust src="sidebar/icon_rail.rs"
```

## 完全自定义插槽

### 效果

::Demo{component="SidebarCustomSlots"}::

### 代码

```rust src="sidebar/custom_slots.rs"
```

## 使用建议

- 顶部品牌区可直接用 `.logo(...)`、`.brand(...)`、`.brand_subtitle(...)`、`.brand_action(...)` 快速搭建；也可以用 `.header(...)` 完全替换为自定义结构。
- 样式可以通过 `.background(...)`、`.border_color(...)`、`.border(...)`、`.rounded(...)`、`.header_padding(...)`、`.content_padding(...)`、`.footer_padding(...)`、`.gap(...)` 调整。
- `Sidebar` 自己管理宽度和边框；放入 `Container::aside(...)` 时请配合 `.aside_passthrough()`，避免双重侧栏宽度导致布局错乱。
- `Menu` 等有状态子组件应保存在父 View 的 `Entity<T>` 字段中。
- 侧栏只负责布局与滚动，路由、过滤、选中状态应留在应用层。
