# Backtop

返回滚动区域顶部的快捷按钮。Backtop 与目标滚动容器共享 `ScrollHandle`，因此可以放在任意原生滚动区域上，而不是绑定全局窗口滚动。

## 基础用法

滚动超过 `visibility_height` 后显示按钮，点击后把对应 `ScrollHandle` 偏移重置到顶部。

### 效果

::LioraDemo{component="BacktopBasic"}::

### 代码

```rust src="backtop/basic.rs"
```

## 自定义内容

可以用 `content` 替换默认图标，配合 `right` / `bottom` 调整固定位置。

### 效果

::LioraDemo{component="BacktopCustom"}::

### 代码

```rust src="backtop/custom.rs"
```

## 滚动容器绑定

滚动容器必须 `track_scroll(&scroll_handle)`，Backtop 必须使用同一个 `ScrollHandle`，否则按钮无法感知滚动距离。

### 效果

::LioraDemo{component="BacktopContainer"}::

### 代码

```rust src="backtop/container.rs"
```
