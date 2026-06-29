# Locales 国际化

Liora 的国际化资源来自外部独立 TOML 文件。应用把语言包放在 `assets/locales/<locale>.toml`，初始化时通过 `Options::try_with_locales_dir(...)` 加载，运行时通过 `apply_locale(window, cx, locale)` 立即切换语言并刷新窗口。

```toml
# assets/locales/zh-CN.toml
[common]
ok = "确定"
cancel = "取消"

[empty]
description = "暂无数据"
```

```rust
use liora::{Options, init_liora_with_options, apply_locale, locales, tr};

fn setup(cx: &mut gpui::App) {
    let options = Options::system()
        .with_locale("zh-CN")
        .with_fallback_locale("en-US")
        .try_with_locales_dir("assets/locales")
        .unwrap_or_else(|_| Options::system().with_locale("zh-CN"));

    init_liora_with_options(cx, options);
}

fn switch_to_english(window: &mut gpui::Window, cx: &mut gpui::App) {
    let _ = apply_locale(window, cx, "en-US");
}

fn empty_label(cx: &gpui::App) -> gpui::SharedString {
    tr(cx, locales::empty::description)
}
```

组件默认文案会在 render 阶段通过 `tr(cx, locales::empty::description)` 这类型化 key 查询当前语言；用户显式传入的文案仍然优先，不会被全局语言覆盖。

## 类型化资源 Key

Liora 内置默认 key 模块，调用处应写 `locales::docs::subtitle` / `locales::empty::description`，不要写 `"docs.subtitle"` 这样的硬编码字符串。外部 TOML 文件仍保持 `[docs] subtitle = "..."` 结构。

开发端可以在 `build.rs` 中调用 `liora_locales_codegen::generate_locales_from_package("liora_core::Locales")`。它默认扫描当前 package 的 `./assets/locales`，也支持在 `Cargo.toml` 中配置额外/替代路径：

```toml
[package.metadata.liora.locales]
paths = ["assets/locales", "../shared/locales"]
```

应用自己的语言资源也可以用宏定义独立 key 模块：

```rust
liora::locales! {
    pub mod app_locales {
        docs { subtitle }
        dashboard { refresh, empty_state }
    }
}

fn subtitle(cx: &gpui::App) -> gpui::SharedString {
    tr(cx, app_locales::docs::subtitle)
}
```

## 自定义 Translator

如果应用已有自己的翻译系统，可以实现 `Translator` 并通过 `Options::with_translator(...)` 或运行时 `set_translator(...)` 替换默认查询逻辑。外部 TOML 资源仍可作为 fallback。
