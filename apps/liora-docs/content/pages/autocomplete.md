# Autocomplete

`Autocomplete` 在输入时展示建议列表，适合搜索、路由跳转和命令面板入口。

## 基础用法

### 效果

::LioraDemo{component="AutocompleteBasic"}::

### 代码

```rust src="autocomplete/basic.rs"
```

## 自定义建议

自定义建议也可以配置关闭策略，例如禁用点击外部与 ESC 自动关闭。

### 效果

::LioraDemo{component="AutocompleteCustom"}::

### 代码

```rust src="autocomplete/custom.rs"
```

## 无右侧图标

### 效果

::LioraDemo{component="AutocompleteNoSuffix"}::

### 代码

```rust src="autocomplete/no_suffix.rs"
```

## 禁用状态

### 效果

::LioraDemo{component="AutocompleteDisabled"}::

### 代码

```rust src="autocomplete/disabled.rs"
```
