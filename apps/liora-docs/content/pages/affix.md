# Affix

将内容固定在特定可视区域中，适合长表单的提交按钮、工具条或需要持续可见的操作入口。Affix 通过 GPUI 布局测量与 Liora PassivePortal 完成固钉，不依赖浏览器定位。

## 顶部固钉

当滚动容器中的元素越过顶部阈值后，Affix 会把内容固定到窗口顶部指定偏移处，并在原位置保留占位尺寸避免布局跳动。

### 效果

::Demo{component="AffixTop"}::

### 代码

```rust src="affix/top.rs"
```

## 底部固钉

通过 `position(AffixPosition::Bottom)` 可将内容固定到视口底部，适合“保存”“提交”等底部操作条。

### 效果

::Demo{component="AffixBottom"}::

### 代码

```rust src="affix/bottom.rs"
```

## 滚动容器

Affix 通常放在一个有明确高度的原生滚动容器内。容器需要 `relative` 与 `overflow_y_scroll`，长内容负责触发固钉状态变化。

### 效果

::Demo{component="AffixContainer"}::

### 代码

```rust src="affix/container.rs"
```
