# Icon Libraries

Liora ships typed SVG icon packs for Lucide, Ant Design, Ionicons, Tabler, Carbon, and Material Design. Every icon pack follows the same API shape as `liora-icons-lucide`: each crate exposes an `IconName` enum, `IconName::all()`, `IconName::file()`, `IconName::svg_source()`, and implements `liora_icons::IntoIconPath` plus `gpui::IntoElement`.

## 快速使用

使用顶层 `liora` facade 时可以直接从对应模块导入图标名；使用拆分 crate 时也可以从 `liora-icons-*` 包导入。

```rust src="icon/libraries.rs"
```

## 命名规则

| 图标库 | crate | facade 模块 | IconName 规则 | 示例 |
| --- | --- | --- | --- | --- |
| Lucide | `liora-icons-lucide` | `liora::icons_lucide` | upstream kebab-case 转 PascalCase | `IconName::Settings` |
| Ant Design | `liora-icons-antd` | `liora::icons_antd` | 图标名 + `Filled` / `Outlined` / `Twotone` | `IconName::SaveOutlined` |
| Ionicons | `liora-icons-ionic` | `liora::icons_ionic` | 基础名，必要时追加 `Outline` / `Sharp` | `IconName::AddCircleOutline` |
| Tabler | `liora-icons-tabler` | `liora::icons_tabler` | outline 使用基础名，filled 追加 `Filled` | `IconName::HomeFilled` |
| Carbon | `liora-icons-carbon` | `liora::icons_carbon` | Carbon 名称扁平化为 PascalCase，每个图标保留一个优先尺寸 | `IconName::CheckmarkFilled` |
| Material | `liora-icons-material` | `liora::icons_material` | 默认名，或追加 `Outlined` / `Round` / `Sharp` / `Twotone` | `IconName::SearchOutlined` |

## 完整 IconName 清单

下面的清单由当前编译进应用的 `IconName::all()` 数据生成，覆盖每个内置图标库中的每个图标。点击任意 `IconName` 即可复制可直接粘贴到代码中的完整路径。

::LioraDemo{component="IconLibraryCatalog"}::
