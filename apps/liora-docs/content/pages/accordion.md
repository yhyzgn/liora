# Accordion

`Accordion` 是独立手风琴控件，用于 FAQ、设置分组和文档信息折叠。它不是 `Collapse` 页面的复用示例，支持单开、多开、禁用项、尺寸和边框组合。

## 基础用法

### 效果

::Demo{component="AccordionBasic"}::

### 代码

```rust src="accordion/basic.rs"
```

## 多开模式

### 效果

::Demo{component="AccordionMultiple"}::

多开模式允许多个分组同时展开，适合配置面板或需要并列查看的文档区块。

### 代码

```rust src="accordion/multiple.rs"
```

## 禁用与视觉状态

### 效果

::Demo{component="AccordionStates"}::

禁用项保留标题与说明但阻止交互，用于展示不可用能力、权限限制或分阶段开放的内容。

### 代码

```rust src="accordion/states.rs"
```
