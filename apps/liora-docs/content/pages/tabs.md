# Tabs

`Tabs` 用于在同一区域切换多个面板，适合配置页、详情页和复杂表单的分组展示。

## 基础用法

默认标签页位于顶部，通过点击标签切换对应内容。

`pane` 的 `name` 是业务键：重复时仅保留首次声明，不同键可以使用相同的显示标签。初始键不存在时，首次渲染选中首项，不调用 `on_tab_click`；空列表没有选中项或活动面板。

Tab / Shift+Tab 进入或离开标签组。水平栏用 ← / →，垂直栏用 ↑ / ↓ 循环移动标签焦点，Home / End 移到首项 / 尾项。移动焦点不会切换面板；Enter / Space 才激活当前焦点标签。离开标签后再次进入时，从选中标签进入。

原生语义包含标签列表的方向、标签名称和选中状态，以及带名称的活动面板。当前固定的 GPUI 公共 API 不提供真实节点间的关联入口，未实现标签到面板的 `controls` / AT-SPI `ControllerFor`；名称相同不代表已建立关联。

### 效果

::Demo{component="TabsBasic"}::

### 代码

```rust src="tabs/basic.rs"
```

## 自动均分

`stretch(true)` 会让顶部标签自动均分并占满宽度。

### 效果

::Demo{component="TabsStretch"}::

### 代码

```rust src="tabs/stretch.rs"
```

## 卡片样式

使用 `TabType::Card` 可以把标签渲染成卡片式页签。

### 效果

::Demo{component="TabsCard"}::

### 代码

```rust src="tabs/card.rs"
```

## 带边框卡片

`TabType::BorderCard` 会为标签页整体添加边框容器。

### 效果

::Demo{component="TabsBorderCard"}::

### 代码

```rust src="tabs/border_card.rs"
```

## 不同位置

通过 `TabPosition::Left` 和 `TabPosition::Right` 可以把标签放在内容左右两侧。

### 效果

::Demo{component="TabsPosition"}::

### 代码

```rust src="tabs/position.rs"
```

## 可编辑标签

开启 `editable(true)` 后，标签栏会显示新增和关闭入口，并可监听增删事件。
关闭与新增是独立的键盘按钮，用 Enter / Space 操作；也可在可关闭的标签上按 Delete。关闭不会触发标签激活回调；删除活动项时选中下一项（若无下一项则选中上一项），删除持有焦点的标签或其关闭按钮时也按此顺序移动焦点，若已无标签则聚焦新增按钮。新增后聚焦并选中新标签。

### 效果

::Demo{component="TabsEditable"}::

### 代码

```rust src="tabs/editable.rs"
```
