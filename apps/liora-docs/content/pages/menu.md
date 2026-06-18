# Menu

导航菜单用于表达页面结构与当前业务位置。Liora Menu 支持水平、垂直和折叠模式，并可配置 ESC 关闭子菜单浮层。

## 水平模式

适合窗口顶部导航。`on_select` 会返回当前激活的菜单项 id，业务可据此切换内容区域。

### 效果

::LioraDemo{component="MenuHorizontal"}::

### 代码

```rust src="menu/horizontal.rs"
```

## 垂直模式

适合左侧导航。子菜单和分组可表达更深的业务层级。

### 效果

::LioraDemo{component="MenuVertical"}::

### 代码

```rust src="menu/vertical.rs"
```

## 折叠模式

适合紧凑侧栏，只保留图标宽度，子菜单仍通过原生浮层展开。

### 效果

::LioraDemo{component="MenuCollapsed"}::

### 代码

```rust src="menu/collapsed.rs"
```
