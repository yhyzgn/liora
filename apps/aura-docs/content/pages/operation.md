# Operation

`Operation` 是左侧 Label/说明 + 右侧操作区域的两端对齐布局，适合设置项、控制面板、工具栏行和列表内操作。右侧 action 完全由调用方提供，可以是 `Button`、`Switch`、`Checkbox` 或任意 GPUI 元素。

## 基础操作行

### 效果

::AuraDemo{component="OperationBasic"}::

### 代码

```rust src="operation/basic.rs"
```
