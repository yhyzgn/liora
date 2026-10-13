# Icon Optimizer

`liora-icons-optimizer` 是 Liora 的图标资源自动瘦身工具。它解决的问题是：Liora 内置图标库数量很大，但普通应用通常只使用几十个图标。应用代码仍然写强类型 `IconName::Search` / `IconName::Settings`，构建脚本自动扫描实际使用点，只把需要的 SVG staging 到打包资源目录。

这个流程是 **自动的**：接入一次 `build.rs` 后，`cargo check`、`cargo run`、`cargo build`、CI 和 xtask 打包都会触发扫描。开发者不需要手动复制 SVG，不需要手动运行 optimizer，也不需要在业务代码里区分开发/发布路径。

## 何时需要接入

接入 optimizer 的条件很简单：应用使用了 Liora 内置强类型图标库，例如 `liora-icons-lucide`、`liora-icons-antd`、`liora-icons-ionic`、`liora-icons-tabler`、`liora-icons-carbon` 或 `liora-icons-material`。

如果应用只使用自己的 `assets/icons/*.svg`、`file:///...` 图标或 `inline_svg_asset_path(...)`，optimizer 不会处理这些资源；它们仍然应该由应用自己的 assets/packager 流程负责。

## 1. 在 Cargo.toml 添加 build dependency

外部应用使用 crates.io 版本：

```toml
[build-dependencies]
liora-icons-optimizer = "0.4"
```

Liora monorepo 内部应用使用 workspace path：

```toml
[build-dependencies]
liora-icons-optimizer = { path = "../../crates/liora-icons-optimizer" }
```

## 2. 在唯一的 build.rs 中启用自动 bundle

```rust
fn main() {
    liora_icons_optimizer::Optimizer::new()
        .bundle_auto()
        .run();

    // Keep existing build.rs logic here, such as Windows resources or metadata.
}
```

`run()` 会把错误转换成 Cargo 友好的构建失败。如果你需要在测试或自定义 xtask 中处理错误，可以使用 `try_run()` 返回 `Result<OptimizationReport, OptimizerError>`。

## 3. 运行时安装 IconAssetSource

如果应用使用 `gpui_platform::application()` 并渲染内置 SVG payload，启动时安装 Liora icon asset source：

```rust
fn main() {
    gpui_platform::application()
        .with_assets(liora_icons::IconAssetSource)
        .run(|cx| {
            liora::init_liora(cx);
            // open windows...
        });
}
```

`IconAssetSource` 会自动查找安装包资源、portable 资源、开发期生成 bundle，以及强类型图标 crate 自带的 `dev=` fallback 路径。如果虚拟图标最终仍然找不到，Liora 会显示一个可见占位图标，避免 UI 静默空白。

## 裸可执行程序的嵌入兜底

安装包和 portable archive 会携带 `assets/liora-icons` 外部资源目录；但是 GitHub Release 里单独上传的裸 `exe` / binary 不能假设旁边一定有 assets 目录。因此应用如果要发布单文件可执行程序，应在 `build.rs` 里使用 `try_run()` 拿到 `OptimizationReport`，再生成一个 `OUT_DIR` Rust 文件，把已优化过的 SVG 作为兜底嵌入：

```rust
fn main() {
    let report = liora_icons_optimizer::Optimizer::new()
        .scan_dir("src")
        .bundle_auto()
        .try_run()
        .expect("icon optimization should complete");

    write_embedded_icon_bundle(&report);
}
```

运行时的 asset source 先查嵌入 bundle，再委托给 `liora_icons::IconAssetSource` 查安装包 / portable / 开发期路径：

```rust
mod embedded_icon_bundle {
    include!(concat!(env!("OUT_DIR"), "/embedded_icon_bundle.rs"));
}

struct AppAssetSource;

impl gpui::AssetSource for AppAssetSource {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        if let Some(request) = path.strip_prefix(liora_icons::ICON_SVG_ASSET_PREFIX) {
            let (resource, _) = request.split_once('?').unwrap_or((request, ""));
            if let Some(bytes) = embedded_icon_bundle::load(resource) {
                return Ok(Some(std::borrow::Cow::Borrowed(bytes)));
            }
        }

        liora_icons::IconAssetSource.load(path)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        liora_icons::IconAssetSource.list(path)
    }
}
```

Gallery 和 Docs 已采用这套策略，所以 raw executable、installer、portable archive 都能显示同一套优化后的图标。普通业务应用不需要手动运行任何命令；只要 `build.rs` 写好，Cargo 构建会自动完成扫描、复制和嵌入兜底生成。

## 输出目录和报告

默认输出位置由当前 package 名和 workspace root 决定：

```text
target/liora/icons/apps/<app>/assets/liora-icons
target/liora/icons/reports/<app>.md
```

报告会列出扫描根目录、复制的 SVG、运行时搜索路径和最终资源数量。报告用于诊断，不是业务 API，也不应该作为应用运行时依赖。

## 自定义扫描和输出

大多数应用只需要 `.bundle_auto()`。如果图标使用点存在于额外目录，例如 Markdown 内容或代码生成输入，可以显式增加扫描目录：

```rust
fn main() {
    liora_icons_optimizer::Optimizer::new()
        .scan_dir("src")
        .scan_dir("content/pages")
        .asset_out_dir("target/custom-icons")
        .report_file("target/icon-report.md")
        .bundle_auto()
        .run();
}
```

相对路径会从当前 package root 解析。自定义输出目录通常只用于特殊打包流水线；普通应用应使用默认路径，让 Liora packager/xtask 自动收集资源。

## 自定义业务 SVG 的边界

optimizer 只扫描 Liora 内置强类型图标库的 `IconName::...` 使用点。以下业务图标不会被复制、改写、删除或重命名：

```rust
use liora::icons::{Icon, inline_svg_asset_path};

let brand = Icon::new("assets/icons/brand-mark.svg").size_lg();
let external = Icon::new("file:///opt/acme/icons/status.svg");
let inline = Icon::new(inline_svg_asset_path(
    r#"<svg viewBox="0 0 24 24"><path d="M4 12h16"/></svg>"#,
));
```

如果这些 SVG 需要随安装包外部挂载，把它们放在应用自己的 `assets/` 目录，并让打包器复制该目录。不要把业务图标混进 `assets/liora-icons`，这个目录保留给 optimizer 生成的内置图标 bundle。

## 自定义强类型图标库

如果你有一套复用的产品图标，希望也获得 `IconName` 枚举和 optimizer 支持，推荐新增一个独立 crate，例如 `liora-icons-yourpack`：

1. 固定 SVG 目录结构。
2. 暴露 `IconName` enum。
3. 实现 `liora_icons::IntoIconPath`。
4. 提供 `IconName::all()`、`IconName::file()`、`IconName::svg_path()`。
5. 让 optimizer 明确识别该图标包。

少量业务 SVG 不需要这样做；普通 assets 更简单也更清晰。

## 避免误打包全量图标

`IconName::all()` 会告诉 optimizer 打包整个图标库。它适合 Liora Docs 这种图标浏览器页面；普通生产应用应该引用具体枚举变体，让 bundle 保持小体积。

## Debug 开关

排查路径问题时可以临时设置：

```bash
LIORA_ICON_DEBUG=1 cargo run -p your-app
```

它会打印候选路径链路、命中路径和 fallback 决策。正常开发和 CI 不需要设置这个变量。
