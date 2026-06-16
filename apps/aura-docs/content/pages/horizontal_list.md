# HorizontalList

`HorizontalList` 是横向滚动列表控件，适合流程节点、快捷入口、横向卡片、步骤流、看板列等场景。它支持完全自定义 item、divider，并提供内置拖动排序状态与 `on_reorder` 回调。开启拖动后，每个 item 前端会显示明确的 Grip 拖拽把手，拖动事件只从把手开始；拖动过程中当前项会沿拖动轴跟随鼠标位移，并在经过目标项时即时重排。

## 基础横向列表

### 效果

::AuraDemo{component="HorizontalListBasic"}::

### 代码

```rust src="horizontal_list/basic.rs"
```

## 自定义 divider

### 效果

::AuraDemo{component="HorizontalListDivider"}::

### 代码

```rust src="horizontal_list/divider.rs"
```

## 拖动排序

### 效果

::AuraDemo{component="HorizontalListDraggable"}::

### 代码

```rust src="horizontal_list/draggable.rs"
```
