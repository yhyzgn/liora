# Collapse

`Collapse` 用于将多段内容收纳在折叠面板中，降低长页面的信息密度。

## 基础用法

多个面板可以独立展开与收起。

### 效果

::Demo{component="CollapseBasic"}::

### 代码

```rust src="collapse/basic.rs"
```

## 手风琴模式

调用 `accordion()` 后，同一时间只保留一个面板展开。

### 效果

::Demo{component="CollapseAccordion"}::

### 代码

```rust src="collapse/accordion.rs"
```
