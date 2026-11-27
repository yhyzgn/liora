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

开启 `filterable(true)` 后显示匹配叶子路径的结果面板；示例通过程序预置 `search_query("杭州")` 匹配路径中的选项标签。当前组件不提供可编辑搜索框，不要把该示例当成键盘输入搜索。展开后点击结果完成选择，并显示实际路径消息。

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
