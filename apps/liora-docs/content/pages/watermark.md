# Watermark

`Watermark` 为任意 GPUI 原生内容添加文字水印，适合文档预览、设计稿、导出截图或敏感信息区域。

## 覆盖水印

### 效果

::Demo{component="WatermarkCover"}::

### 代码

```rust src="watermark/cover.rs"
```

## 页眉/页脚水印

### 效果

::Demo{component="WatermarkHeader"}::

### 代码

```rust src="watermark/header.rs"
```

## 自定义旋转、倾斜、旋转中心与密度

### 效果

::Demo{component="WatermarkCustom"}::

### 代码

```rust src="watermark/custom.rs"
```

- `.rotate(degrees)` 设置实际旋转角度，单位为度；默认 `-22°`。
- `.skew(x_degrees, y_degrees)` 设置双轴错切（倾斜）角度。
- `.rotation_center(x, y)` 设置每块水印文字的变换中心；使用 `0..=1` 归一化坐标，默认 `(0.5, 0.5)` 为中心，`(0.0, 0.0)` 为左上角。
- 变换仅影响水印，不改变内容布局；超出内容区域的水印会被裁剪。
