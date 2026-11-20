# Button

`Button` 用于触发操作。Docs 中每个配置都按“效果 + 代码”展示；Gallery 只负责交互预览。

## 类型

标准不透明语义填色按钮在默认、悬停和按下时分别选用黑色或白色标签文字，不改变填色或边框。不涵盖文字按钮、次要按钮、渐变、自定义色、半透明宿主主题或显式着色的图标及子元素。

### 效果

::Demo{component="ButtonTypes"}::

### 代码

```rust src="button/types.rs"
```

## 次要按钮

### 效果

::Demo{component="ButtonSecondary"}::

### 代码

```rust src="button/secondary.rs"
```

## 文字按钮

`.primary().text()` 与 `.text().primary()` 等价：默认状态下背景透明、无边框，文字保留主色。`info`、`success`、`warning` 和 `danger` 同样不受调用顺序影响。

五种语义文字按钮在内置浅色主题的已知 body/card 背景上，按默认、悬停和按下背景配对调整前景亮度，保留语义色系；默认背景仍透明。内置深色的已合格前景保持不变。此配对不涵盖未知或半透明祖先背景、自定义主题调色板、显式自定义色、渐变、禁用/不可用状态、Default/Tertiary 或显式着色子元素，不承诺透明按钮在任意宿主背景上均满足 AA。

### 效果

::Demo{component="ButtonText"}::

### 代码

```rust src="button/text.rs"
```

## 尺寸

### 效果

::Demo{component="ButtonSizes"}::

### 代码

```rust src="button/sizes.rs"
```

## 状态

可用按钮支持 Tab / Shift+Tab 焦点导航和 Enter / Space 激活，并显示键盘焦点边框。禁用与加载按钮跳过焦点且不触发回调。此处与 Gallery 使用同一个有状态示例；点击或按键更新 Activations 和 last，Disabled / Loading 不改变计数。

仅图标按钮在可用、禁用和加载时保持相同的正方形尺寸。禁用时图标大小不变，加载时用旋转指示器替换原图标。

`Button::new(label)` 的构造标签默认同时提供原生按钮名称，包括 icon-only、禁用和加载状态。`.aria_label(locales::common::add)` 仅覆盖原生名称，可让可见的 `+` 保持原样；翻译随当前语言在渲染时解析。禁用设置 disabled，实际显示加载指示器时另外设置 busy；两者同时配置时禁用优先。平台 AT 状态需单独检查，不能用组件 Node flags 或截图代替；当前固定示例不提供动态状态切换。

### 效果

::Demo{component="ButtonStates"}::

### 代码

```rust src="button/states.rs"
```

## 按钮组

`ButtonGroup` 将相关操作相邻排列。下面的前后翻页和编辑操作与 Gallery 的按钮组使用同一渲染器；点击可用按钮更新 Last action，禁用的 Delete 保持上次操作不变。当前组内按钮保留各自的圆角和边框。

### 效果

::Demo{component="ButtonGroup"}::

### 代码

```rust src="button/group.rs"
```

## 圆角

### 效果

::Demo{component="ButtonRounded"}::

### 代码

```rust src="button/rounded.rs"
```


## 自定义颜色

### 效果

::Demo{component="ButtonCustomColors"}::

### 代码

```rust src="button/custom_colors.rs"
```

## 渐变按钮

### 效果

::Demo{component="ButtonGradient"}::

### 代码

```rust src="button/gradient.rs"
```
