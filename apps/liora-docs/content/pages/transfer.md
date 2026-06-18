# Transfer

在两个列表之间移动条目，适合权限分配、人员选择、城市/角色迁移等场景。

## 基础用法

通过 `target_keys` 指定右侧已选项，通过 `checked_source_keys` 预先勾选源列表项。

### 效果

::LioraDemo{component="TransferBasic"}::

### 代码

```rust src="transfer/basic.rs"
```

## 可过滤列表

开启 `filterable(true)` 后可显示过滤输入。示例预置过滤文本，业务中可以绑定自己的搜索状态。

### 效果

::LioraDemo{component="TransferFilterable"}::

### 代码

```rust src="transfer/filterable.rs"
```

## 禁用项

禁用条目不可勾选或移动，已在目标列表中的禁用项仍会保留。

### 效果

::LioraDemo{component="TransferDisabled"}::

### 代码

```rust src="transfer/disabled.rs"
```
