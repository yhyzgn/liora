# Segmented

`Segmented` 是分段控制器，用于在一组选项中选择单个值，适合视图模式、时间范围或筛选维度切换。

## 基础用法

通过 `SegmentedOption` 定义选项，并用 `on_change` 监听选中值变化。

### 效果

::Demo{component="SegmentedBasic"}::

### 代码

```rust src="segmented/basic.rs"
```

## 禁用选项

单个选项可以设置为 disabled，用户无法选择该项。

### 效果

::Demo{component="SegmentedDisabled"}::

### 代码

```rust src="segmented/disabled.rs"
```

## Block 模式

开启 `block(true)` 后，分段控制器会撑满父容器宽度，各项均分空间。

### 效果

::Demo{component="SegmentedBlock"}::

### 代码

```rust src="segmented/block.rs"
```

## 键盘与可访问性

`Tab` 进入当前选中的可用选项；如果选中项不可用，则进入第一个可用选项。再次按 `Tab` 离开整组，`Shift+Tab` 反向移动。方向键循环切换并跳过禁用项，`Home` / `End` 定位第一个 / 最后一个可用选项；`Space` 或 `Enter` 选择当前聚焦项。空组或全部禁用的组不会占用 Tab 停靠点。

组件导出原生单选组、选项名称、选中及禁用状态；选项名称与显示文字均在渲染时解析 `LocalizedText`。禁用项可以保留程序设定的选中状态，但不能通过用户操作聚焦或选取。请保留组件的 `Entity`，以维持选择与焦点。

`on_change` 仅在序列化值变化时触发；不同选项即使共享同一个值，也只会有一个选中项，彼此切换不会重复通知同值。`set_options` 按值及同值选项的出现次序保留仍存在的选项身份、选择与焦点，避免重排时将焦点转给原位置上的另一个选项。
