# Carbon Icons

Carbon 图标库通过 `liora::icons_carbon::IconName` 暴露强类型 `IconName`。本页只展示 Carbon 的完整图标清单；点击任意正方形图标 item 即可复制可直接粘贴到代码中的完整 Rust 路径。

## 用法

```rust
use liora_icons::Icon;
use liora::icons_carbon::IconName;

let icon = Icon::new(IconName::CheckmarkFilled).size_lg();
```

## 完整 IconName 清单

::LioraDemo{component="IconCatalogCarbon"}::
