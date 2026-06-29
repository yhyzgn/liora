# SegmentRatioBar

`SegmentRatioBar` 使用一个水平分段条展示多个部分的占比。每段支持自定义颜色、文本 pattern 和数值 pattern。图例文本默认水平排列；每个文本块与对应分段等宽：左侧色点和 label 对齐分段左边，右侧比例/值对齐分段右边，并支持整体圆角、分段圆角和文本左右缩进。

## 下方图例

### 效果

::Demo{component="SegmentRatioBarBottom"}::

### 代码

```rust src="segment_ratio_bar/bottom.rs"
```

## 上方文本

### 效果

::Demo{component="SegmentRatioBarTop"}::

### 代码

```rust src="segment_ratio_bar/top.rs"
```

## 上下同时显示

### 效果

::Demo{component="SegmentRatioBarBoth"}::

### 代码

```rust src="segment_ratio_bar/both.rs"
```

## 隐藏文本

### 效果

::Demo{component="SegmentRatioBarHidden"}::

### 代码

```rust src="segment_ratio_bar/hidden.rs"
```

## 自定义 Pattern

### 效果

::Demo{component="SegmentRatioBarPattern"}::

### 代码

```rust src="segment_ratio_bar/pattern.rs"
```

## 细条与宽缩进

### 效果

::Demo{component="SegmentRatioBarCompact"}::

### 代码

```rust src="segment_ratio_bar/compact.rs"
```
