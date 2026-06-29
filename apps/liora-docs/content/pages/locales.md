# Locales 国际化开发指南

Liora 的 Locales 系统用于应用壳、组件默认文案和业务 UI 的运行时国际化。它的设计目标是低耦合：语言资源放在外部 TOML 文件里，Rust 调用处使用类型化 key，翻译后端可以由应用完全替换。

## 适用场景

使用 Locales 能解决这些问题：

- 语言包独立于 Rust 源码，放在 `assets/locales/<locale>.toml`。
- 调用处写 `locales::section::key`，不写 `"section.key"` 这种硬编码字符串。
- 应用运行时可以通过 `apply_locale(window, cx, locale)` 自由切换语言。
- 缺失 key 会 fallback 到配置的 fallback locale、Liora 内置核心资源，最后返回 key 本身。
- 如果应用已有翻译系统，可以实现 `Translator` 完全替换 Liora 默认查询逻辑。

## 目录结构

推荐每个应用 crate 自己维护语言资源：

```text
my-app/
├── Cargo.toml
├── build.rs
├── assets/
│   └── locales/
│       ├── en-US.toml
│       └── zh-CN.toml
└── src/
    └── main.rs
```

语言文件示例：

```toml
# assets/locales/zh-CN.toml
[common]
ok = "确定"
cancel = "取消"
loading = "加载中"

[docs]
title = "Liora Docs"
subtitle = "原生 GPUI 文档"

[empty]
description = "暂无数据"
```

```toml
# assets/locales/en-US.toml
[common]
ok = "OK"
cancel = "Cancel"
loading = "Loading"

[docs]
title = "Liora Docs"
subtitle = "Native GPUI documentation"

[empty]
description = "No data"
```

TOML 表会在运行时展平成点分隔路径，例如 `[docs] subtitle = "..."` 对应内部 key `docs.subtitle`。Rust 调用处不需要写这个字符串，而是使用生成的 `locales::docs::subtitle`。

## 生成类型化 key

在应用 crate 添加 build dependency：

```toml
[build-dependencies]
liora-locales-codegen = "0.1"
```

添加 `build.rs`：

```rust
fn main() {
    liora_locales_codegen::generate_locales_from_package("liora_core::Locales");
}
```

在应用代码里 include 生成模块：

```rust
pub mod locales {
    include!(concat!(env!("OUT_DIR"), "/locales_keys.rs"));
}
```

默认情况下，生成器扫描当前 package 的 `./assets/locales`。当你新增或删除 TOML key 后，重新运行 `cargo check` / `cargo run`，Cargo build script 会重新生成 `locales::section::key` 常量。

## 自定义扫描目录

如果语言资源不在默认目录，可以在当前 package 的 `Cargo.toml` 配置：

```toml
[package.metadata.liora.locales]
paths = ["assets/locales", "../shared/locales"]
```

路径可以是绝对路径，也可以是相对当前 package root 的路径。不要把 Gallery 或 Docs 的路径硬编码到 `liora-core`；应用应该由自己的 package 配置决定扫描范围。

## 初始化时加载语言资源

```rust
use liora::{Options, init_liora_with_options};

fn setup(cx: &mut gpui::App) {
    let options = Options::system()
        .with_locale("zh-CN")
        .with_fallback_locale("en-US")
        .try_with_locales_dir("assets/locales")
        .unwrap_or_else(|_| Options::system().with_locale("zh-CN"));

    init_liora_with_options(cx, options);
}
```

实际应用打包后，当前工作目录可能不是 crate 根目录。Gallery 和 Docs 使用绝对路径 helper：

```rust
fn app_locales_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/locales")
}
```

然后初始化时调用：

```rust
let options = Options::system()
    .with_locale("zh-CN")
    .with_fallback_locale("en-US")
    .try_with_locales_dir(app_locales_dir())
    .unwrap_or_else(|_| Options::system().with_locale("zh-CN"));
```

## 在组件里直接使用 key

很多 Liora 文本类组件支持 `LocalizedText`，可以直接传类型化 key。组件会在 render 阶段解析当前语言，因此运行时切换语言并刷新窗口后会自动显示新语言。

```rust
use liora::components::{Button, Empty, Paragraph, SegmentedOption, Text, Title};

fn content() -> impl gpui::IntoElement {
    gpui::div()
        .child(Title::new(locales::docs::title).h2())
        .child(Text::new(locales::docs::subtitle))
        .child(Paragraph::with_text(locales::empty::description))
        .child(Button::new(locales::common::ok).primary())
        .child(Empty::new().description(locales::empty::description))
}

fn language_options() -> Vec<SegmentedOption> {
    vec![
        SegmentedOption::new(locales::language::zh_cn, "zh-CN"),
        SegmentedOption::new(locales::language::en_us, "en-US"),
    ]
}
```

适合直接传 key 的常见位置包括：`Button::new(...)`、`Text::new(...)`、`Title::new(...)`、`Paragraph::with_text(...)`、`Tag::new(...)`、`Label::new(...)`、`Menu::new(...)`、`MenuItem::new(...)`、`SegmentedOption::new(...)`、`Checkbox::label(...)`、`Input::placeholder(...)`、`Empty::description(...)`、`MessageBox::new(...)`。

## 什么时候使用 `tr(cx, key)`

有些 API 需要立即拿到 `SharedString` 或 `String`，这时使用 `tr(cx, key)`：

```rust
use liora::{locales, tr};

fn window_title(cx: &gpui::App) -> gpui::SharedString {
    tr(cx, locales::docs::title)
}

fn template(cx: &gpui::App, version: &str) -> String {
    tr(cx, locales::update_status::available)
        .to_string()
        .replace("{version}", version)
}
```

经验规则：组件构造器能接收 key 时直接写 `locales::...`；需要立即字符串、拼接模板、toast、窗口标题或第三方 API 文本时用 `tr(cx, locales::...)`。

## 运行时切换语言

使用 `apply_locale` 切换当前语言并刷新当前窗口：

```rust
use liora::apply_locale;

fn switch_to_english(window: &mut gpui::Window, cx: &mut gpui::App) {
    if let Err(error) = apply_locale(window, cx, "en-US") {
        eprintln!("failed to switch locale: {error}");
    }
}
```

如果目标语言文件尚未加载，可以使用 `switch_locale_from_dir(window, cx, locale, dir)`。它会先从目录加载 `<locale>.toml`，再切换并刷新窗口。

切换语言时需要注意避免 GPUI entity 重入更新：不要在某个控件自己的 update 闭包里同步 update 同一个控件。Gallery/Docs 的做法是先更新全局 locale，再刷新需要重建 label/options 的实体，必要时通过 `window.defer(...)` 延后。

## Fallback 行为

查询顺序如下：

1. 应用自定义 `Translator`。
2. 当前 locale 的 TOML 资源。
3. fallback locale 的 TOML 资源。
4. Liora 内置核心 fallback TOML。
5. key 路径本身。

这意味着缺失 key 不会 panic，但会显示类似 `docs.subtitle` 的路径，方便你发现漏翻译。

## 自定义 Translator

应用可以完全替换翻译系统：

```rust
use gpui::SharedString;
use liora::{LocaleId, Options, Translator, init_liora_with_options};

struct AppTranslator;

impl Translator for AppTranslator {
    fn translate(&self, locale: &LocaleId, key: &str) -> Option<SharedString> {
        Some(format!("{}:{}", locale.as_str(), key).into())
    }

    fn has_locale(&self, locale: &LocaleId) -> bool {
        matches!(locale.as_str(), "zh-CN" | "en-US")
    }
}

fn setup(cx: &mut gpui::App) {
    init_liora_with_options(
        cx,
        Options::system()
            .with_locale("zh-CN")
            .with_translator(AppTranslator),
    );
}
```

运行时也可以使用 `set_translator(cx, translator)`、`set_shared_translator(cx, translator)` 或 `clear_translator(cx)` 替换/清除 translator。

## 宏定义轻量 key 模块

如果你不想使用外部 TOML 生成器，也可以用宏定义少量 key：

```rust
liora::locales! {
    pub mod app_locales {
        docs { title, subtitle }
        dashboard { refresh, empty_state }
    }
}

fn subtitle(cx: &gpui::App) -> gpui::SharedString {
    liora::tr(cx, app_locales::docs::subtitle)
}
```

宏适合小范围手写 key。正式应用建议优先使用外部 TOML + `liora-locales-codegen`，因为它能从真实语言资源生成 key，并随 TOML 变更自动更新。

## 开发检查清单

- 新增 UI 文案时，先写到对应 `assets/locales/*.toml`。
- 运行 `cargo check` 让 build script 生成新的 `locales::section::key`。
- 调用处优先传 `locales::section::key`，不要写 `"section.key"`。
- 需要立即字符串时才使用 `tr(cx, key)`。
- Gallery/Docs 应用壳文案要双语；Docs markdown 正文不需要拆成双语文件，除非产品明确要求。
- 如果看到界面显示 `section.key`，说明当前 locale 和 fallback locale 都缺少该翻译。
