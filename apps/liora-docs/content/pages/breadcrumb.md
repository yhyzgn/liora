# Breadcrumb

`Breadcrumb` 用于展示当前位置的层级路径，帮助用户快速理解页面归属并返回上级页面。Gallery 与本页直接执行下面展示的同一组构造函数。

## 基础用法

默认使用 `/` 分隔路径层级。

### 效果

::Demo{component="BreadcrumbBasic"}::

### 代码

```rust src="breadcrumb/basic.rs"
```

## 图标类型

通过 `BreadcrumbItem::icon` 给路径项追加图标，强化入口语义。

### 效果

::Demo{component="BreadcrumbIcon"}::

### 代码

```rust src="breadcrumb/icon.rs"
```

## 文本分隔符

使用 `separator` 可以自定义文本分隔符。

### 效果

::Demo{component="BreadcrumbSeparator"}::

### 代码

```rust src="breadcrumb/separator.rs"
```

## 图标分隔符

使用 `separator_icon` 可以把分隔符替换成图标。

### 效果

::Demo{component="BreadcrumbSeparatorIcon"}::

### 代码

```rust src="breadcrumb/separator_icon.rs"
```

## 点击事件

只有注册了 `on_click` 回调且不是末项的路径项可以激活。用 Tab / Shift+Tab 遍历，Enter 松开后执行回调；Space 不触发导航。末项表示当前位置，保持非交互，即使注册回调也不会执行。示例点击或键盘激活后显示对应的消息提示，不在 SDK 中写入路由或启动外部页面。

同一宿主中重复显示相同路径时，分别用 `Breadcrumb::id` 指定不同的稳定命名空间，避免重复的路径项共享焦点或交互状态。默认命名空间由路径标签和分隔符生成。

### 效果

::Demo{component="BreadcrumbClickable"}::

### 代码

```rust src="breadcrumb/clickable.rs"
```
