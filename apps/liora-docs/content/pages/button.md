# Button

`Button` 用于触发操作。Docs 中每个配置都按“效果 + 代码”展示；Gallery 只负责交互预览。

## 类型

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
