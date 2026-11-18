# InputNumber

`InputNumber` 用于输入受约束的数值，支持范围、步进、精度和控制按钮布局。

手动输入的有效数值会同步至步进状态和变更回调；编辑中的负号、小数点和末尾小数点保留到失焦或步进时再格式化。精度范围内存在可表示值时，数值向内限制于该范围；否则保留范围内的精确值并显示精确文本。

## 基础范围

### 效果

::Demo{component="InputNumberBasic"}::

### 代码

```rust src="input_number/basic.rs"
```

## 右侧控制按钮

### 效果

::Demo{component="InputNumberVertical"}::

### 代码

```rust src="input_number/vertical.rs"
```

## 精度与步进

### 效果

::Demo{component="InputNumberPrecision"}::

### 代码

```rust src="input_number/precision.rs"
```
