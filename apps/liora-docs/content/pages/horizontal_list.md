# HorizontalList

`HorizontalList` 是横向滚动列表控件，适合流程节点、快捷入口、横向卡片、步骤流、看板列等场景。它支持完全自定义 item、divider，并提供内置拖动排序状态与 `on_reorder` 回调。开启拖动后，每个 item 前端会显示明确的 Grip 拖拽把手，拖动事件只从把手开始；拖动过程中当前项会沿拖动轴跟随鼠标位移，并在经过目标项时即时重排。

## 基础横向列表

### 效果

::Demo{component="HorizontalListBasic"}::

### 代码

```rust src="horizontal_list/basic.rs"
```

## 自定义 divider

### 效果

::Demo{component="HorizontalListDivider"}::

### 代码

```rust src="horizontal_list/divider.rs"
```

## 拖动排序

### 效果

::Demo{component="HorizontalListDraggable"}::

### 代码

```rust src="horizontal_list/draggable.rs"
```

## 直接组合公共拖拽 helpers

按住 Grip 并移动超过 GPUI 原生拖拽阈值后开始，`offset(DragAxis::Vertical)` 从激活时记录位移；左键在目标行松开时通过 `finish()` 和 `reorder_indices` 提交重排，右键或列表外松开由原生预览释放清理并取消。这个示例直接组合公共拖拽 helpers，不使用 `HorizontalList`。

### 效果

::Demo{component="HorizontalListDragHelpers"}::

### 代码

```rust src="horizontal_list/drag_helpers.rs"
```
