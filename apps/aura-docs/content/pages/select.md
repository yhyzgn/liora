# Select

`Select` 用于从有限选项中选择一个值，弹层和选择状态都保持在原生 GPUI 视图中。

## 基础选择

默认点击外部或 ESC 会关闭下拉层；需要固定决策上下文时可配置 `close_on_click_outside(false)` / `close_on_escape(false)`。

### 效果

::AuraDemo{component="SelectBasic"}::

### 代码

```rust src="select/basic.rs"
```
