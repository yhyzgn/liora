# List

`List` 用于展示有序 / 无序内容列表，支持嵌套层级、每一级 marker 策略、单项 marker 覆盖、ordered counter 类型和 `{n}` pattern 格式。

## 无序列表

默认无序列表会按层级切换 marker：disc、circle、square、dash。

### 效果

::Demo{component="ListUnordered"}::

### 代码

```rust src="list/unordered.rs"
```

## 自定义无序 marker

通过 `unordered_markers(...)` 配置每一级 marker，也可以在 `ListItem` 上用 `.marker(...)` 覆盖单项。

### 效果

::Demo{component="ListCustomUnordered"}::

### 代码

```rust src="list/custom_unordered.rs"
```

## 有序列表

默认有序列表逐级使用 decimal、lower-alpha、lower-roman 和 leading-zero。

### 效果

::Demo{component="ListOrdered"}::

### 代码

```rust src="list/ordered.rs"
```

## 自定义有序格式

通过 `ordered_markers(...)` 为每一级指定 `OrderedCounterStyle` 和 pattern。pattern 中 `{n}` 会被替换为当前计数值。

### 效果

::Demo{component="ListCustomOrdered"}::

### 代码

```rust src="list/custom_ordered.rs"
```
