# QrCode

`QrCode` 提供二维码生成显示和识别 API。控件渲染仍然是纯 Rust + GPUI native；识别 API 适合在业务层读取上传图片、截图或文件后调用。

## 交互式生成

### 效果

::LioraDemo{component="QrCodeBasic"}::

### 代码

```rust src="qr_code/basic.rs"
```

## 颜色、模块风格、Logo 和前景渐变

支持普通矩阵码的模块形状、定位角、颜色、中心 Logo、角标 Logo，以及前景色渐变。渐变可传入颜色数组，并指定 8 个方向：上、右上、右、右下、下、左下、左、左上。

### 效果

::LioraDemo{component="QrCodeStyle"}::

### 代码

```rust src="qr_code/style.rs"
```

## 识别二维码

### 说明

上方交互示例使用 `Upload` 打开本地图片文件；选择后自动调用 `QrCode::decode_file`。业务层也可以直接用 `decode_bytes` / `decode_file` / `decode_image`，返回内容、纠错等级和版本。

### 代码

```rust src="qr_code/decode.rs"
```
