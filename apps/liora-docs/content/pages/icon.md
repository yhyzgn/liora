# Icon

`Icon` 基于 `liora-icons` 的 `IntoIconPath` 渲染原生矢量图标。Lucide、Ant Design、Ionicons、Tabler、Carbon 和 Material 图标库都提供同构的 `IconName` 枚举，常用于按钮、表单前后缀、空状态和导航项。

## 常用图标

从 `IconName` 选择图标，并用 `Icon::new` 创建原生图标元素。

### 效果

::LioraDemo{component="IconLucide"}::

### 代码

```rust src="icon/lucide.rs"
```


## 多图标库

所有内置图库都实现同一套 `Icon` API：`Icon::new(SomeLibraryIconName::SomeIcon)`。为了让名称短且可读，各库只把有意义的风格后缀放进 `IconName`：AntD 使用 `Filled` / `Outlined` / `Twotone`，Ionic 使用 `Outline` / `Sharp`，Tabler 使用 `Filled`，Material 使用 `Outlined` / `Round` / `Sharp` / `Twotone`，Carbon 则按图标名扁平化。

### 效果

::LioraDemo{component="IconLibraries"}::

### 代码

```rust src="icon/libraries.rs"
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
