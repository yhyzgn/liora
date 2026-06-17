# Tour

`Tour` 用步骤卡片引导用户理解界面关键区域。它是受控组件，`active_index` 由调用方保存，`on_change` / `on_finish` / `on_close` 负责状态推进。

## 基础引导

### 效果

::AuraDemo{component="Tour"}::

### 代码

```rust src="tour/basic.rs"
```

## 中间步骤

### 效果

::AuraDemo{component="Tour"}::

### 代码

```rust src="tour/middle.rs"
```

## 无遮罩模式

### 效果

::AuraDemo{component="Tour"}::

### 代码

```rust src="tour/no_mask.rs"
```
