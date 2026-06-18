# Tabs

`Tabs` 用于在同一区域切换多个面板，适合配置页、详情页和复杂表单的分组展示。

## 基础用法

默认标签页位于顶部，通过点击标签切换对应内容。

### 效果

::LioraDemo{component="TabsBasic"}::

### 代码

```rust src="tabs/basic.rs"
```

## 自动均分

`stretch(true)` 会让顶部标签自动均分并占满宽度。

### 效果

::LioraDemo{component="TabsStretch"}::

### 代码

```rust src="tabs/stretch.rs"
```

## 卡片样式

使用 `TabType::Card` 可以把标签渲染成卡片式页签。

### 效果

::LioraDemo{component="TabsCard"}::

### 代码

```rust src="tabs/card.rs"
```

## 带边框卡片

`TabType::BorderCard` 会为标签页整体添加边框容器。

### 效果

::LioraDemo{component="TabsBorderCard"}::

### 代码

```rust src="tabs/border_card.rs"
```

## 不同位置

通过 `TabPosition::Left` 和 `TabPosition::Right` 可以把标签放在内容左右两侧。

### 效果

::LioraDemo{component="TabsPosition"}::

### 代码

```rust src="tabs/position.rs"
```

## 可编辑标签

开启 `editable(true)` 后，标签栏会显示新增和关闭入口，并可监听增删事件。

### 效果

::LioraDemo{component="TabsEditable"}::

### 代码

```rust src="tabs/editable.rs"
```
