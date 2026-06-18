# Cascader

`Cascader` 用于从层级数据中逐级选择，适合地区、组织、产品目录等强父子关系的数据。

## 基础用法

点击含子级的选项展开下一列，点击叶子节点完成选择。

### 效果

::LioraDemo{component="CascaderBasic"}::

### 代码

```rust src="cascader/basic.rs"
```

## 默认选中

使用 `selected_path` 预置已选择的路径。

### 效果

::LioraDemo{component="CascaderSelected"}::

### 代码

```rust src="cascader/selected.rs"
```

## 禁用状态

通过 `disabled(true)` 禁用整个级联选择器。

### 效果

::LioraDemo{component="CascaderDisabled"}::

### 代码

```rust src="cascader/disabled.rs"
```

## 可搜索

开启 `filterable(true)` 后可按路径叶子节点搜索；示例预置 `search_query("hang")` 展示匹配状态。

### 效果

::LioraDemo{component="CascaderFilterable"}::

### 代码

```rust src="cascader/filterable.rs"
```

## 懒加载

开启 `lazy(true)` 后，点击空子级分支时通过 `on_lazy_load` 写回远程子节点。

### 效果

::LioraDemo{component="CascaderLazy"}::

### 代码

```rust src="cascader/lazy.rs"
```
