# Image

`Image` 用于渲染远程或本地图片，支持填充模式、圆角/圆形、阴影、占位、失败态以及预览能力。

## 基础用法

远程图片用于验证网络加载效果，本地图片适合稳定的文档和业务资源。

### 效果

::Demo{component="ImageBasic"}::

### 代码

```rust src="image/basic.rs"
```

## 填充方式

使用 `ImageFit` 控制图片在固定容器中的裁剪和缩放策略。

### 效果

::Demo{component="ImageFit"}::

### 代码

```rust src="image/fit.rs"
```

## 形状与状态

图片可以设置圆形裁剪、圆角边界、ring sleeve、阴影、空状态和失败态。

### 效果

::Demo{component="ImageStates"}::

### 代码

```rust src="image/states.rs"
```

## 点击预览

启用 `preview(true)` 后，点击图片会打开原生预览层。

### 效果

::Demo{component="ImagePreview"}::

### 代码

```rust src="image/preview.rs"
```
