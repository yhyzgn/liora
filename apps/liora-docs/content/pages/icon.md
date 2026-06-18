# Icon

`Icon` 基于 `liora-icons-lucide` 的 `IconName` 枚举渲染原生矢量图标，常用于按钮、表单前后缀、空状态和导航项。

## 常用图标

从 `IconName` 选择图标，并用 `Icon::new` 创建原生图标元素。

### 效果

::LioraDemo{component="IconLucide"}::

### 代码

```rust src="icon/lucide.rs"
```

## 语义颜色

通过 `color` 使用主题语义色，让图标与组件状态保持一致。

### 效果

::LioraDemo{component="IconColors"}::

### 代码

```rust src="icon/colors.rs"
```

## 尺寸

使用 `size_xs`、`size_md`、`size_lg`、`size_xl` 快捷方法统一图标尺寸。

### 效果

::LioraDemo{component="IconSizes"}::

### 代码

```rust src="icon/sizes.rs"
```
