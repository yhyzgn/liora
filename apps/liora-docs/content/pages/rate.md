# Rate

`Rate` 用于评分或表达偏好强度，支持自定义最大值。

启用的评分控件可通过 `Tab` 聚焦；方向键预览评分，`Home`/`End` 跳到首/末星，`Enter`/`Space` 提交并触发 `on_change`。键盘步进为整星；当前 API 不提供半星或清除选项。禁用控件不会进入 Tab 顺序。

## 基础评分

### 效果

::Demo{component="RateBasic"}::

### 代码

```rust src="rate/basic.rs"
```

## 自定义最大值

### 效果

::Demo{component="RateCustom"}::

### 代码

```rust src="rate/custom.rs"
```
