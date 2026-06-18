# Component Authoring

新增组件时，应先把可复用能力放进 `crates/liora-components`，再在 Gallery 和 Docs 中使用。

## 推荐流程

1. 在 `crates/liora-components/src/<name>.rs` 实现组件。
2. 在 `crates/liora-components/src/lib.rs` 中 `pub mod` 和 `pub use`。
3. 在 `apps/liora-gallery/src/demos/` 添加交互 demo。
4. 如该组件服务文档系统，在 `apps/liora-docs` 中复用它。
5. 添加最小回归测试。

## 示例：CodeBlock

`CodeBlock` 先进入组件库，然后 Liora Docs 的 fenced code block 渲染改为复用该组件。

```rust src="authoring/code_block.rs"
```

## 约束

- Demo 不能绕过组件库重新实现同类 UI。
- Docs 不能维护一套 app-local 组件替代库。
- 公共组件命名遵循当前 ADR：不加 `Liora` 前缀。
