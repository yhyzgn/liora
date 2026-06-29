# Tag

`Tag` 用于标记、分类和轻量选择。Docs 按单个效果拆分展示，代码紧贴对应效果。

## 基础类型

### 效果

::Demo{component="TagTypes"}::

### 代码

```rust src="tag/types.rs"
```

## 可移除标签

### 效果

::Demo{component="TagClosable"}::

### 代码

```rust src="tag/closable.rs"
```

## 不同主题

### 效果

::Demo{component="TagThemes"}::

### 代码

```rust src="tag/themes.rs"
```

## 不同尺寸

### 效果

::Demo{component="TagSizes"}::

### 代码

```rust src="tag/sizes.rs"
```

## 圆角标签

### 效果

::Demo{component="TagRound"}::

### 代码

```rust src="tag/round.rs"
```

## 流式布局与折叠

### 效果

::Demo{component="TagFlow"}::

### 代码

`TagFlow` 仍复用原 `Tag`，只负责 wrap、gap、align 与轻量折叠策略。`max_rows` 会根据 `estimated_items_per_row` 计算展示数量，并自动追加 overflow indicator。

```rust src="tag/flow.rs"
```
