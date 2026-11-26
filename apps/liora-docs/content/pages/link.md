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

## 键盘与原生语义

Tab / Shift+Tab 在可用链接间移动，跳过禁用和未设置 `href` 的项。聚焦后按下并释放无修饰键的 Enter 打开目标；Space 不打开链接。焦点框只在键盘输入时显示，不改变文字布局。鼠标从链接上按下并在其上释放才激活，单独释放或移出后释放不会跳转。

Enter 按下后失焦会取消键盘激活；禁用或移除目标会取消待激活的输入，重新启用不会补发旧点击。已处理的 Enter 不会继续触发父级键盘处理器。

设置 `href` 后提供原生 Link 角色、可访问名称和目标 URL；禁用项保留角色及禁用状态，但不接受键盘或指针激活。无目标项只是静态展示。宿主仍负责提供可区分的标签并决定允许的协议；这些元数据不代表所有平台和屏幕阅读器均已验收。

### 效果

::Demo{component="LinkKeyboard"}::

### 代码

```rust src="link/keyboard.rs"
```
