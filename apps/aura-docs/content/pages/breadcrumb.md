# Breadcrumb

`Breadcrumb` 用于展示当前位置的层级路径，帮助用户快速理解页面归属并返回上级页面。

## 基础用法

默认使用 `/` 分隔路径层级。

### 效果

::AuraDemo{component="BreadcrumbBasic"}::

### 代码

```rust src="breadcrumb/basic.rs"
```

## 图标类型

通过 `BreadcrumbItem::icon` 给路径项追加图标，强化入口语义。

### 效果

::AuraDemo{component="BreadcrumbIcon"}::

### 代码

```rust src="breadcrumb/icon.rs"
```

## 文本分隔符

使用 `separator` 可以自定义文本分隔符。

### 效果

::AuraDemo{component="BreadcrumbSeparator"}::

### 代码

```rust src="breadcrumb/separator.rs"
```

## 图标分隔符

使用 `separator_icon` 可以把分隔符替换成图标。

### 效果

::AuraDemo{component="BreadcrumbSeparatorIcon"}::

### 代码

```rust src="breadcrumb/separator_icon.rs"
```

## 点击事件

前置路径项可以注册 `on_click` 回调，执行返回或跳转逻辑。

### 效果

::AuraDemo{component="BreadcrumbClickable"}::

### 代码

```rust src="breadcrumb/clickable.rs"
```
