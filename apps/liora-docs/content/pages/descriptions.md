# Descriptions

`Descriptions` 用于展示一组字段信息，适合详情页、用户资料和订单摘要。

## 基础用法

无边框模式适合轻量信息展示，字段可以通过 `span` 跨列。

### 效果

::Demo{component="DescriptionsBasic"}::

### 代码

```rust src="descriptions/basic.rs"
```

## 带边框样式

开启 `border(true)` 后，字段区域会有明确的表格式边界，并可通过 `extra` 添加操作区。

### 效果

::Demo{component="DescriptionsBorder"}::

### 代码

```rust src="descriptions/border.rs"
```

## 垂直带边框

使用 `DescriptionsDirection::Vertical` 可让标签与内容上下排列。

### 效果

::Demo{component="DescriptionsVertical"}::

### 代码

```rust src="descriptions/vertical.rs"
```
