# Segmented

`Segmented` 是分段控制器，用于在一组选项中选择单个值，适合视图模式、时间范围或筛选维度切换。

## 基础用法

通过 `SegmentedOption` 定义选项，并用 `on_change` 监听选中值变化。

### 效果

::LioraDemo{component="SegmentedBasic"}::

### 代码

```rust src="segmented/basic.rs"
```

## 禁用选项

单个选项可以设置为 disabled，用户无法选择该项。

### 效果

::LioraDemo{component="SegmentedDisabled"}::

### 代码

```rust src="segmented/disabled.rs"
```

## Block 模式

开启 `block(true)` 后，分段控制器会撑满父容器宽度，各项均分空间。

### 效果

::LioraDemo{component="SegmentedBlock"}::

### 代码

```rust src="segmented/block.rs"
```
