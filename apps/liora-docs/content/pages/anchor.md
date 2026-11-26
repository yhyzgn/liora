# Anchor

为长页面提供快速跳转和滚动同步。Anchor 使用同一个 `ScrollHandle` 连接导航和内容区，`AnchorTarget` 负责把目标区域的原生布局边界回报给 Anchor。

Tab / Shift+Tab 可按树中的显示顺序在锚点间移动；无修饰键的 Enter 在释放时跳转到已绘制的目标，Space 和带修饰键的 Enter 不执行目标跳转。键盘焦点框不改变布局，链接提供原生 Link 角色和可访问名称；这不代表完整屏幕阅读器或跨平台验收。

`AnchorLink::href` 是对应 `AnchorTarget` 的 id，不会打开外部 URL。Gallery 和 Docs 的嵌套导航直接执行下方 `nested.rs`，Docs 基础导航直接执行 `basic.rs`。

## 基础锚点

创建 Anchor 时传入滚动区的 `ScrollHandle`，然后通过 `AnchorLink` 注册可跳转的目标 id。

### 效果

::Demo{component="AnchorBasic"}::

### 代码

```rust src="anchor/basic.rs"
```

## 嵌套锚点

AnchorLink 支持子链接，适合 API 文档中的二级标题，例如 Attributes、Events、Slots 等分组。

### 效果

::Demo{component="AnchorNested"}::

### 代码

```rust src="anchor/nested.rs"
```

## 目标区域

每个可跳转区域用 `AnchorTarget::new(id, anchor, child)` 包裹。它会在绘制阶段记录目标 bounds，点击锚点时滚动到对应区域。目标移除后，新一帧绘制并释放对应状态时会清除其测量值和高亮；保留的链接不会改变滚动位置，`scroll_to_target` 返回 `false`。缓存绘制中仍存在的目标保持可跳转。

### 效果

::Demo{component="AnchorTargets"}::

### 代码

```rust src="anchor/targets.rs"
```
