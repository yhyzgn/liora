# Backtop

返回滚动区域顶部的快捷按钮。Backtop 与目标滚动容器共享 `ScrollHandle`，因此可以放在任意原生滚动区域上，而不是绑定全局窗口滚动。

## 基础用法

滚动距离达到 `visibility_height` 时显示按钮（默认 200，`visibility_height_sm()` 为 100）。完整点击或无修饰键的 Enter／Space 按下并松开后，只把绑定的 `ScrollHandle` 偏移重置到顶部，并请求重绘；仅按下按键不会滚动。

可见按钮支持 Tab／Shift+Tab 遍历，键盘聚焦显示不改变布局的 2px 焦点环。隐藏按钮不进入 Tab 顺序。按钮使用原生 Button 角色和 `Back to top` 名称；这不代表完整屏幕阅读器或跨平台认证。

### 效果

::Demo{component="BacktopBasic"}::

### 代码

```rust src="backtop/basic.rs"
```

## 自定义内容

可以用 `content` 替换默认图标，配合 `right` / `bottom` 调整固定位置。Gallery 和本页预览直接执行所示的基础／自定义构造函数，自定义图标使用反色前景以适配深色主题。

### 效果

::Demo{component="BacktopCustom"}::

### 代码

```rust src="backtop/custom.rs"
```

## 滚动容器绑定

滚动容器必须 `track_scroll(&scroll_handle)`，Backtop 必须使用同一个 `ScrollHandle`，否则按钮无法感知滚动距离。将 `Entity<Backtop>` 保留在父视图中，并和该句柄一起传给下面的完整容器示例；本页三种预览都使用这个实际容器函数。

示例的 `Flex::overflow_y_scroll()` 会把滚轮事件保留在内层容器（包括到达边界时），不会同时推动外层文档。把指针移到示例外即可滚动文档页。

### 效果

::Demo{component="BacktopContainer"}::

### 代码

```rust src="backtop/container.rs"
```
