# Settings UI

`SettingsPage`、`SettingsGroup`、`SettingsItem` 用于搭建桌面应用设置页。它们不是新的表单校验系统，而是把设置页的信息架构、分组、说明文案、控件区域和敏感状态统一成可复用布局模式。

## 完整设置页

### 效果

::Demo{component="SettingsPageBasic"}::

### 代码

```rust src="settings/page.rs"
```

## 敏感与禁用项

### 效果

::Demo{component="SettingsSensitive"}::

### 代码

```rust src="settings/sensitive.rs"
```
