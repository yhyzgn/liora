# Input

`Input` 默认是单行文本输入控件，支持 placeholder、禁用态、清除按钮、密码模式、前后缀和图标。多行场景可显式启用 `.multiline(true)`；需要按可用宽度自动换行时再启用 `.soft_wrap(true)`，光标、指针命中、上下移动和输入法候选定位使用同一份原生排版几何。默认输入路径不变。

## 基础输入

### 效果

::Demo{component="InputBasic"}::

### 代码

```rust src="input/basic.rs"
```

## 密码输入

### 效果

::Demo{component="InputPassword"}::

### 代码

```rust src="input/password.rs"
```

## 前后缀

### 效果

::Demo{component="InputAffix"}::

### 代码

```rust src="input/affix.rs"
```

## 可清除与禁用

### 效果

::Demo{component="InputStates"}::

### 代码

```rust src="input/states.rs"
```
