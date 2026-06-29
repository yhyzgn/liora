# Timeline

`Timeline` 用于按时间顺序展示一组事件，适合审计记录、流程日志和发布动态。

## 基础用法

使用 `timestamp` 和 `content` 描述事件发生的时间与内容。

### 效果

::Demo{component="TimelineBasic"}::

### 代码

```rust src="timeline/basic.rs"
```

## 自定义节点样式

节点可以使用语义色、空心样式或图标来表达事件状态。

### 效果

::Demo{component="TimelineCustom"}::

### 代码

```rust src="timeline/custom.rs"
```

## 时间戳位置

通过 `TimelinePlacement` 控制时间戳显示在内容上方或下方。

### 效果

::Demo{component="TimelinePlacement"}::

### 代码

```rust src="timeline/placement.rs"
```

## 倒序排列

`reverse` 可以把事件倒序展示，适合最新事件优先的场景。

### 效果

::Demo{component="TimelineReverse"}::

### 代码

```rust src="timeline/reverse.rs"
```
