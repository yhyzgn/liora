# Anchor

为长页面提供快速跳转和滚动同步。Anchor 使用同一个 `ScrollHandle` 连接导航和内容区，`AnchorTarget` 负责把目标区域的原生布局边界回报给 Anchor。

## 基础锚点

创建 Anchor 时传入滚动区的 `ScrollHandle`，然后通过 `AnchorLink` 注册可跳转的目标 id。

### 效果

::LioraDemo{component="AnchorBasic"}::

### 代码

```rust src="anchor/basic.rs"
```

## 嵌套锚点

AnchorLink 支持子链接，适合 API 文档中的二级标题，例如 Attributes、Events、Slots 等分组。

### 效果

::LioraDemo{component="AnchorNested"}::

### 代码

```rust src="anchor/nested.rs"
```

## 目标区域

每个可跳转区域用 `AnchorTarget::new(id, anchor, child)` 包裹。它会在布局阶段记录目标 bounds，点击锚点时滚动到对应区域。

### 效果

::LioraDemo{component="AnchorTargets"}::

### 代码

```rust src="anchor/targets.rs"
```
