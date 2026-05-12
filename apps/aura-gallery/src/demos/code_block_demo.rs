use aura_components::layout_helpers::{page, section};
use aura_components::{
    CodeBlock, CodeHighlighter, CodeLanguage, CodeTheme, Divider, MessageType, Space, show_message,
};
use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};

pub fn render(cx: &mut App) -> Entity<CodeBlockDemo> {
    cx.new(|_| CodeBlockDemo)
}

pub struct CodeBlockDemo;

impl Render for CodeBlockDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "CodeBlock 代码块",
            "原生代码高亮显示，支持语言指定、命名主题、高亮后端抽象、格式指定和复制。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "Rust 高亮 + 复制",
                    "显示语言标签并提供复制按钮，内容在容器内横向滚动。",
                    CodeBlock::new(RUST_SAMPLE)
                        .rust()
                        .selectable(true)
                        .on_copy(|_, _, cx| {
                            show_message("复制成功", MessageType::Success, cx);
                        }),
                ))
                .child(Divider::new())
                .child(section(
                    "JSON / Shell",
                    "可以通过 language(...) 或便捷方法指定语言。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(CodeBlock::new(JSON_SAMPLE).language(CodeLanguage::Json))
                        .child(CodeBlock::new(SHELL_SAMPLE).shell()),
                ))
                .child(Divider::new())
                .child(section(
                    "主题切换",
                    "默认 auto_theme() 跟随 Aura 全局主题，也可以显式指定 Aura / GitHub / One Dark / Nord / Dracula 等主题。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(CodeBlock::new(RUST_SAMPLE).rust().light_theme())
                        .child(CodeBlock::new(RUST_SAMPLE).rust().dark_theme())
                        .child(CodeBlock::new(TOML_SAMPLE).toml().theme(CodeTheme::GitHubLight))
                        .child(CodeBlock::new(JSON_SAMPLE).json().github_dark_theme())
                        .child(CodeBlock::new(RUST_SAMPLE).rust().one_dark_theme())
                        .child(CodeBlock::new(SHELL_SAMPLE).shell().nord_theme())
                        .child(CodeBlock::new(RUST_SAMPLE).rust().dracula_theme())
                        .child(
                            CodeBlock::new(TOML_SAMPLE)
                                .toml()
                                .highlighter(CodeHighlighter::Syntect)
                                .theme(CodeTheme::Auto),
                        ),
                ))
                .child(Divider::new())
                .child(section(
                    "Inline 格式",
                    "同一个控件也可作为行内代码样式使用。",
                    Space::new()
                        .wrap()
                        .gap_sm()
                        .child(CodeBlock::new("cargo run -p aura-gallery").shell().inline()),
                )),
        )
    }
}

const RUST_SAMPLE: &str = r#"pub fn render_markdown(md: &str) -> AnyElement {
    CodeBlock::new(md)
        .language("rust")
        .copyable(true)
        .into_any_element()
}
"#;

const JSON_SAMPLE: &str = r#"{
  "name": "aura",
  "native": true,
  "phase": 8
}"#;

const TOML_SAMPLE: &str = r#"[code_block]
theme = "auto"
copyable = true
"#;

const SHELL_SAMPLE: &str = r#"# build and run docs
cargo check -p aura-components
cargo run -p aura-docs
"#;

#[cfg(test)]
mod tests {
    #[test]
    fn code_block_demo_uses_component_api() {
        let source = include_str!("code_block_demo.rs");

        assert!(source.contains("CodeBlock::new"));
        assert!(source.contains(".on_copy("));
        assert!(source.contains("复制成功"));
        assert!(source.contains(".rust()"));
        assert!(source.contains(".shell()"));
        assert!(source.contains(".inline()"));
        assert!(source.contains(".selectable"));
        assert!(source.contains(".light_theme()"));
        assert!(source.contains(".dark_theme()"));
        assert!(source.contains("CodeTheme::Auto"));
        assert!(source.contains("CodeTheme::GitHubLight"));
        assert!(source.contains("CodeHighlighter::Syntect"));
        assert!(source.contains(".github_dark_theme()"));
        assert!(source.contains(".one_dark_theme()"));
        assert!(source.contains(".nord_theme()"));
        assert!(source.contains(".dracula_theme()"));
    }
}
