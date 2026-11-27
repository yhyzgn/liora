# Cascader

`Cascader` 用于从层级数据中逐级选择，适合地区、组织、产品目录等强父子关系的数据。

## 基础用法

点击含子级的选项展开下一列，点击叶子节点完成选择。聚焦触发器后按 Enter 或 Space 展开，按 Escape 关闭；禁用状态不响应这些操作。选项逐级选择使用鼠标。

示例通过 `on_change` 显示真实路径值，例如 `Selected zhejiang / hangzhou / xihu`；点击清空显示 `Selection cleared`。这些消息只反馈当前操作，不会持久化数据。

选项值可以包含中文、标点或连字符，回调保留原值。自动化需要定位选项时，使用 `Cascader::popup_item_id` 根据控件前缀及完整值路径生成不透明标识，不要手动拼接内部 ID。

### 效果

::Demo{component="CascaderBasic"}::

### 代码

```rust src="cascader/basic.rs"
```

## 默认选中

使用 `selected_path` 预置 `cloud / compute / ecs`；更改选择后显示实际路径消息。

### 效果

::Demo{component="CascaderSelected"}::

### 代码

```rust src="cascader/selected.rs"
```

## 禁用状态

通过 `disabled(true)` 禁用整个级联选择器，同时保留已选地区。基础示例另有禁用的 `wuzhong` 选项和加载中的节点，均不可选择。

### 效果

::Demo{component="CascaderDisabled"}::

### 代码

```rust src="cascader/disabled.rs"
```

## 可搜索

开启 `filterable(true)` 后，展开会聚焦弹层中的搜索框。示例预置 `search_query("杭州")`，可全选替换为“宁波”或清空；查询按完整标签路径匹配，空白查询恢复逐级选项，无匹配显示 `No results`。输入不会更改已选值。可点击匹配路径，或用不带修饰键的 Down / Up 高亮结果后按 Enter 选择并显示实际路径消息。首次 Down 高亮第一项，首次 Up 高亮最后一项，到达边界后不循环；导航保留搜索框焦点和光标。没有显式高亮时 Enter 不选择，也不插入换行；查询或数据更改、重新展开后清除高亮。修饰键和输入法组合仍交给输入框处理；逐级选项仍使用鼠标选择，不代表完整层级键盘导航。Escape 关闭并返回触发器，输入法组合期间保留原生取消语义；Tab / Shift+Tab 关闭并移到触发器的后一个 / 前一个控件。

点击外部关闭时保留已有的外部焦点；遮罩不提供首次点击穿透。短窗口先为固定搜索框预留高度；高度不足时可裁剪搜索框且不显示结果，不保证任意窗口高度下搜索框完整可见。

### 效果

::Demo{component="CascaderFilterable"}::

### 代码

```rust src="cascader/filterable.rs"
```

## 懒加载

开启 `lazy(true)` 后，点击空子级分支触发 `on_lazy_load`；本例立即通过 `set_children_at_path` 同步写回固定演示数据，不发起网络请求。点击最终 `leaf(true)` 节点才完成选择并显示实际路径消息。

### 效果

::Demo{component="CascaderLazy"}::

### 代码

```rust src="cascader/lazy.rs"
```
