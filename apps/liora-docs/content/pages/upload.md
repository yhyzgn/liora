# Upload

上传入口与文件列表组件，负责展示选择文件、进度、状态、数量限制和禁用态。真实上传流程由业务层接入，组件保持 GPUI 原生交互。

## 基础文件列表

展示普通上传按钮、文件状态和进度。可使用 `accept`、`max_size` 与 `tip` 告知用户约束。

### 效果

::LioraDemo{component="UploadBasic"}::

### 代码

```rust src="upload/basic.rs"
```

## 拖拽样式

开启 `drag(true)` 后使用更大面积的拖拽入口样式。文件真实拖放和上传任务可由宿主通过回调扩展。

### 效果

::LioraDemo{component="UploadDrag"}::

### 代码

```rust src="upload/drag.rs"
```

## 图片卡片

`picture_card` 适合图片、封面、素材等视觉文件列表，能同时呈现成功和上传中状态。

### 效果

::LioraDemo{component="UploadPictureCard"}::

### 代码

```rust src="upload/picture_card.rs"
```

## 数量限制与禁用

`limit` 达到上限后入口自动禁用；`disabled(true)` 可禁用整个上传控件。

### 效果

::LioraDemo{component="UploadLimits"}::

### 代码

```rust src="upload/limits.rs"
```
