# Splitter

`Splitter` 是原生双面板布局。拖动分隔条实时调整尺寸，点击分隔条后使用方向键（10px）、Shift+方向键（50px）、Home/End。横向使用左右键，纵向使用上下键。

## 基础用法

`left` / `right` 保留现有 builder 用法；纵向时表示上/下面板。`orientation(SplitterOrientation::Vertical)` 设置纵向，`height` 设置整体高度。

### 效果

::Demo{component="SplitterBasic"}::

### 代码

```rust src="splitter/basic.rs"
```

## 尺寸与状态

- `default_size` 设置首次非受控尺寸；`min_size` / `max_size` 限制第一面板。受控 `size` 也会在绘制和交互请求时钳制到这些边界；空间足够时第二面板至少保留 80px。首次测量或容器缩小后会自动校正实际布局，无需额外输入；测量校正不会回写宿主尺寸。
- `on_change` 仅在有效交互尺寸变化后回调；宿主必须保存值并在下次渲染传回。宿主从已接受的尺寸重置后，相同的键盘请求仍会再次回调。拖拽仅作用于发起操作的实例。
- GPUI keyed state 保存非受控尺寸与焦点，不会因重绘重置。默认 identity 来自调用位置；在循环或同一调用位置创建多个实例时必须指定不同的 `id`。
- `left` / `right` 内容每次宿主渲染重新提供；有编辑状态的业务内容应使用宿主持有的 `Entity<T>`，不要把临时 element 当作业务状态存储。

Gallery 包含横向非受控及纵向受控交互；`layout_smoke` 使用真实 GPUI 鼠标、拖拽与键盘事件断言持久尺寸、多实例隔离、受控重置，以及首次测量和容器缩小后的实际面板/分隔条几何。
