# Link

`Link` 是文本型跳转入口，支持语义颜色、禁用状态、下划线控制和图标。

外部跳转交由 GPUI 的 `App::open_url` 和系统默认处理器执行，控件不自行拼接或运行 shell 命令；禁用或未设置 `href` 时不会打开 URL。目标及允许的协议由宿主决定，页内锚点滚动请使用 `Anchor`，不要把单独的 `#fragment` 当作外部 URL。

构造原生元素时只读取当前主题的颜色和字号，不复制完整主题；前图标、文字、后图标直接按顺序组合，不创建临时闭包列表。这是构造分配优化，不代表整帧耗时或 FPS 验收。

## 类型

### 效果

::Demo{component="LinkVariants"}::

### 代码

```rust src="link/variants.rs"
```

## 下划线

### 效果

::Demo{component="LinkUnderline"}::

### 代码

```rust src="link/underline.rs"
```

## 禁用状态

### 效果

::Demo{component="LinkStates"}::

### 代码

```rust src="link/states.rs"
```

## 图标

### 效果

::Demo{component="LinkIcons"}::

### 代码

```rust src="link/icons.rs"
```
