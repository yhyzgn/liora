use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*, rgb};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{
    Button, CodeCompletionItem, CodeDiagnostic, CodeEditor, CodeEditorConfig,
    CodeEditorHighlightTheme, CodeEditorInlineDiagnostics, CodeEditorOptions,
    CodeEditorWhitespaceMode, CodeHover, CodeLanguage, CodeTheme, Flex, Input, Space, Text,
    toast_error, toast_info,
};
use std::{fs, path::PathBuf};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| CodeEditorDemo::new(cx)).into()
}

struct CodeEditorDemo {
    basic: Entity<CodeEditor>,
    diagnostics: Entity<CodeEditor>,
    advanced: Entity<CodeEditor>,
    search: Entity<Input>,
    replacement: Entity<Input>,
    configurable: Entity<CodeEditor>,
    file_config_source: Entity<CodeEditor>,
    file_config_preview: Entity<CodeEditor>,
    file_config_path: PathBuf,
    themed: Entity<CodeEditor>,
    line_height_demo: Entity<CodeEditor>,
    indent_guides_demo: Entity<CodeEditor>,
    folding_demo: Entity<CodeEditor>,
    language_matrix: Vec<Entity<CodeEditor>>,
}

impl CodeEditorDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic = cx.new(|cx| {
            CodeEditor::new(RUST_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .tab_size(4)
                .soft_tabs(true)
                .line_numbers(true)
                .theme(CodeTheme::OneDark)
                .on_change(|value, _| {
                    let line_count = value.lines().count().max(1);
                    toast_info!("CodeEditor changed: {} lines", line_count);
                })
        });
        let diagnostics = cx.new(|cx| {
            CodeEditor::new(TS_SAMPLE, cx)
                .language(CodeLanguage::TypeScript)
                .tab_size(2)
                .soft_tabs(true)
                .line_numbers(true)
                .theme(CodeTheme::GitHubDark)
                .diagnostics([
                    CodeDiagnostic::warning(3, 7, "Prefer an explicit return type."),
                    CodeDiagnostic::info(
                        5,
                        3,
                        "Diagnostics are provider-driven and can be replaced by LSP later.",
                    ),
                ])
        });

        let advanced = cx.new(|cx| {
            CodeEditor::new(RUST_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .theme(CodeTheme::OneDark)
                .search_query("Space")
                .completions([
                    CodeCompletionItem::new("Space::new")
                        .kind("struct")
                        .detail("layout container"),
                    CodeCompletionItem::new("Button::new")
                        .kind("function")
                        .detail("action control"),
                    CodeCompletionItem::new("toast_info!")
                        .kind("macro")
                        .detail("show message"),
                ])
                .hover(CodeHover::new(
                    "Space::new",
                    "Creates a flexible native layout container.",
                ))
        });
        let search_editor = advanced.clone();
        let search = cx.new(|cx| {
            Input::new("Space", cx)
                .placeholder("查询文字")
                .on_change(move |value, cx| {
                    search_editor.update(cx, |editor, cx| {
                        editor.set_search_query(value.to_string(), cx)
                    });
                })
        });
        let replacement = cx.new(|cx| Input::new("Flex", cx).placeholder("替换文字"));

        let themed = cx.new(|cx| {
            CodeEditor::new(THEME_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .highlight_theme(
                    CodeEditorHighlightTheme::new(CodeTheme::Nord)
                        .surface(rgb(0x0f172a).into())
                        .chrome_surface(rgb(0x111827).into())
                        .gutter_surface(rgb(0x0b1220).into())
                        .border(rgb(0x334155).into())
                        .text(rgb(0xe5e7eb).into())
                        .muted_text(rgb(0x94a3b8).into())
                        .interaction(
                            rgb(0x38bdf8).into(),
                            rgb(0x2563eb).opacity(0.32).into(),
                            rgb(0x1e293b).into(),
                        )
                        .ruler(rgb(0x475569).opacity(0.72).into())
                        .whitespace(rgb(0x64748b).opacity(0.58).into())
                        .diagnostics(
                            rgb(0x22d3ee).into(),
                            rgb(0xfacc15).into(),
                            rgb(0xfb7185).into(),
                        ),
                )
                .options(CodeEditorOptions {
                    current_line_highlight: true,
                    rulers: true,
                    ruler_column: 88,
                    whitespace: CodeEditorWhitespaceMode::Boundary,
                    inline_diagnostics: CodeEditorInlineDiagnostics::WarningsAndErrors,
                    diagnostics_limit: 4,
                    completion_limit: 4,
                    ..CodeEditorOptions::default()
                })
                .diagnostics([CodeDiagnostic::warning(
                    4,
                    5,
                    "Theme overrides also recolor diagnostics.",
                )])
                .completions([
                    CodeCompletionItem::new("highlight_theme(...)"),
                    CodeCompletionItem::new("whitespace(CodeEditorWhitespaceMode::Boundary)"),
                    CodeCompletionItem::new("ruler_column(88)"),
                ])
        });

        let language_matrix = LANGUAGE_SAMPLES
            .iter()
            .map(|sample| {
                cx.new(move |cx| {
                    CodeEditor::new(sample.source, cx)
                        .language(sample.language)
                        .theme(CodeTheme::OneDark)
                        .rows(7)
                        .line_numbers(true)
                        .status_bar(true)
                        .header(true)
                })
            })
            .collect::<Vec<_>>();

        let line_height_demo = cx.new(|cx| {
            CodeEditor::new(ADVANCED_LAYOUT_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .theme(CodeTheme::OneDark)
                .rows(8)
                .line_height_units(32.0)
                .current_line_highlight(true)
                .rulers(true)
                .ruler_column(96)
        });

        let indent_guides_demo = cx.new(|cx| {
            CodeEditor::new(ADVANCED_LAYOUT_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .theme(CodeTheme::OneDark)
                .rows(8)
                .indent_guides(true)
                .whitespace(CodeEditorWhitespaceMode::Boundary)
                .current_line_highlight(true)
        });

        let folding_demo = cx.new(|cx| {
            CodeEditor::new(FOLDING_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .theme(CodeTheme::OneDark)
                .rows(8)
                .indent_guides(true)
                .code_folding(true)
        });
        let configurable = cx.new(|cx| {
            CodeEditor::new(CONFIG_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .theme(CodeTheme::OneDark)
                .options(CodeEditorOptions {
                    read_only: true,
                    header: false,
                    status_bar: true,
                    line_numbers: false,
                    diagnostics_panel: false,
                    completions_panel: true,
                    hover_panel: false,
                    current_line_highlight: false,
                    completion_limit: 3,
                    ..CodeEditorOptions::default()
                })
                .completions([
                    CodeCompletionItem::new("CodeEditorOptions::default")
                        .kind("config")
                        .detail("start from full chrome"),
                    CodeCompletionItem::new("read_only(true)")
                        .kind("builder")
                        .detail("disable mutation commands"),
                    CodeCompletionItem::new("current_line_highlight(true)")
                        .kind("builder")
                        .detail("highlight the active row"),
                ])
        });

        let file_config_path = gallery_code_editor_config_path();
        if let Some(parent) = file_config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&file_config_path, CONFIG_FILE_TOML);
        let config_source =
            fs::read_to_string(&file_config_path).unwrap_or_else(|_| CONFIG_FILE_TOML.to_string());
        let file_config_preview_path = file_config_path.clone();
        let initial_preview_source = config_preview_source(&config_source);
        let file_config_preview = cx.new(move |cx| {
            let mut editor = CodeEditor::new(initial_preview_source, cx)
                .language(CodeLanguage::Rust)
                .theme(CodeTheme::OneDark)
                .diagnostics([
                    CodeDiagnostic::info(2, 5, "Edit the TOML file on the left to update chrome."),
                    CodeDiagnostic::warning(
                        9,
                        9,
                        "Invalid TOML keeps the last valid editor config.",
                    ),
                ])
                .completions([
                    CodeCompletionItem::new("CodeEditorConfig::load_from_path")
                        .kind("api")
                        .detail("load TOML/JSON once"),
                    CodeCompletionItem::new("set_config")
                        .kind("api")
                        .detail("explicitly apply saved settings"),
                    CodeCompletionItem::new("appearance.syntax.keyword")
                        .kind("toml")
                        .detail("override syntax captures"),
                ]);
            if let Ok(config) = CodeEditorConfig::load_from_path(&file_config_preview_path) {
                editor = editor.config(config);
            }
            editor
        });
        let file_config_source_path = file_config_path.clone();
        let file_config_source = cx.new(move |cx| {
            CodeEditor::new(config_source, cx)
                .language(CodeLanguage::Toml)
                .theme(CodeTheme::OneDark)
                .rows(16)
                .line_numbers(true)
                .status_bar(true)
                .current_line_highlight(true)
                .on_change(move |value, _cx| {
                    let _ = fs::write(&file_config_source_path, value);
                })
        });

        Self {
            basic,
            diagnostics,
            advanced,
            search,
            replacement,
            configurable,
            file_config_source,
            file_config_preview,
            file_config_path,
            themed,
            line_height_demo,
            indent_guides_demo,
            folding_demo,
            language_matrix,
        }
    }
}

impl Render for CodeEditorDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "CodeEditor 代码编辑器",
            "原生 GPUI 代码编辑控件 v2 基座，支持独立 buffer/selection/viewport、可见行渲染、行内实时语法高亮和 diagnostics 扩展点。",
            Space::new().vertical().gap_xl().child(section(
                "Editor showcase",
                "代码编辑示例统一使用宽卡片展示，避免编辑器高度和说明文本打散页面节奏。",
                showcase_stack(vec![
                    showcase_card_wide(
                        "Rust 编辑器",
                        "使用独立 CodeBuffer 与 GPUI list 可见行渲染，不再把 Input 当作编辑核心。",
                        self.basic.clone(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "Diagnostics 扩展点",
                        "业务层可以通过 diagnostics(...) 或 set_diagnostics(...) 注入任意诊断结果。",
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.diagnostics.clone())
                            .child(Text::new("v2 基座已完成基础输入、导航、选择、行号、缩进元数据、虚拟行渲染、诊断渲染、display map 与 undo/redo 事务基座；当前已切到 tree-sitter + Zed-style SyntaxTheme 高亮路线；后续继续增强 LSP bridge、软换行和增量解析。")),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "真实搜索、补全和 hover",
                        "修改查询定位命中；F3/Shift+F3 或按钮前后查找。补全用上下键选择，Tab/Enter 或鼠标接受，Escape 取消。替换可 Ctrl/Cmd+Z 撤销。",
                        Space::new().vertical().gap_md()
                            .child(self.search.clone())
                            .child(self.replacement.clone())
                            .child({
                                let previous = self.advanced.clone();
                                let next = self.advanced.clone();
                                let current = self.advanced.clone();
                                let all = self.advanced.clone();
                                let suggestions = self.advanced.clone();
                                let replacement_current = self.replacement.clone();
                                let replacement_all = self.replacement.clone();
                                Flex::new().gap_sm()
                                    .child(Button::new("上一个").on_click(move |_, _, cx| {
                                        previous.update(cx, |editor, cx| { editor.find_previous(cx); });
                                    }))
                                    .child(Button::new("下一个").on_click(move |_, _, cx| {
                                        next.update(cx, |editor, cx| { editor.find_next(cx); });
                                    }))
                                    .child(Button::new("替换当前").on_click(move |_, _, cx| {
                                        let value = replacement_current.read(cx).value();
                                        current.update(cx, |editor, cx| { editor.replace_current_match(value.as_ref(), cx); });
                                    }))
                                    .child(Button::new("替换全部").on_click(move |_, _, cx| {
                                        let value = replacement_all.read(cx).value();
                                        all.update(cx, |editor, cx| { editor.replace_all_matches(value.as_ref(), cx); });
                                    }))
                                    .child(Button::new("请求补全").on_click(move |_, _, cx| {
                                        suggestions.update(cx, |editor, cx| {
                                            let token = editor.request_token();
                                            editor.set_completions_for_request(&token, [CodeCompletionItem::new("Space::new()"), CodeCompletionItem::new("Button::new(\"Run\")")], cx);
                                        });
                                    }))
                            })
                            .child(self.advanced.clone()),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "默认语言矩阵",
                        "内置 tree-sitter grammar 覆盖 Rust、TypeScript、SQL、Shell、XML、TOML、YAML、INI/conf、JSON 和 Markdown。",
                        Space::new()
                            .vertical()
                            .gap_lg()
                            .children(LANGUAGE_SAMPLES.iter().zip(self.language_matrix.iter()).map(
                                |(sample, editor)| {
                                    Space::new()
                                        .vertical()
                                        .gap_sm()
                                        .child(Text::new(sample.title).bold())
                                        .child(editor.clone())
                                        .into_any_element()
                                },
                            )),
                    )
                    .into_any_element(),

                    showcase_card_wide(
                        "高级布局：行高自定义",
                        "line_height_units(...) 调整可见行高度，适合审计、演示和高密度/低密度编辑场景。",
                        self.line_height_demo.clone(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "高级布局：缩进指示线",
                        "indent_guides(true) 会按缩进层级绘制纵向 guide，并可搭配 whitespace(...) 展示缩进空白。",
                        self.indent_guides_demo.clone(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "高级布局：自动代码块折叠",
                        "CodeEditor 会自动识别多行代码块，在 gutter/行号区显示折叠箭头；点击箭头可展开或收起。",
                        self.folding_demo.clone(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "配置矩阵：只读、Chrome 和扩展面板",
                        "通过 CodeEditorOptions 或 builder 控制 header/status、行号、只读、当前行高亮、诊断/补全/hover 面板和补全数量。",
                        self.configurable.clone(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "配置文件显式应用：TOML 驱动编辑器",
                        "左侧是实际写入的配置文件。编辑会保存 TOML，但右侧不会自动套用视觉配置；点击“应用 set_config”后才 load_from_path(...) 并显式 set_config(...) 到右侧，外部编辑配置文件同理不需要后台 watcher。",
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(Text::new(format!(
                                "当前配置文件：{}",
                                self.file_config_path.display()
                            )))
                            .child({
                                let source = self.file_config_source.clone();
                                let preview = self.file_config_preview.clone();
                                let path = self.file_config_path.clone();
                                Button::new("应用 set_config")
                                    .primary()
                                    .small()
                                    .on_click(move |_, _, cx| {
                                        let source_text = source.read(cx).value(cx).to_string();
                                        if let Err(error) = fs::write(&path, &source_text) {
                                            toast_error!("写入配置失败: {}", error);
                                            return;
                                        }
                                        match CodeEditorConfig::load_from_path(&path) {
                                            Ok(config) => {
                                                let preview_source = config_preview_source(&source_text);
                                                preview.update(cx, |preview, cx| {
                                                    preview.set_value(preview_source, cx);
                                                    preview.set_config(config, cx);
                                                });
                                                toast_info!("已显式应用 CodeEditorConfig::set_config");
                                            }
                                            Err(error) => {
                                                preview.update(cx, |preview, cx| {
                                                    preview.set_value(config_preview_source(&source_text), cx);
                                                });
                                                toast_error!("配置解析失败，保留当前视觉配置: {}", error);
                                            }
                                        }
                                    })
                            })
                            .child(
                                Flex::new()
                                    .row()
                                    .gap_lg()
                                    .child(
                                        Space::new()
                                            .vertical()
                                            .gap_sm()
                                            .child(Text::new("editor.toml（可直接编辑）").bold())
                                            .child(self.file_config_source.clone()),
                                    )
                                    .child(
                                        Space::new()
                                            .vertical()
                                            .gap_sm()
                                            .child(Text::new("应用结果 CodeEditor").bold())
                                            .child(self.file_config_preview.clone()),
                                    ),
                            ),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "自定义高亮主题和高级编辑行为",
                        "CodeEditorHighlightTheme 可覆盖编辑器底色、gutter、selection、caret、ruler、whitespace、诊断色，并可装载 Zed-style syntax capture 主题。",
                        self.themed.clone(),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}

const RUST_SAMPLE: &str = r#"use liora_components::{Button, CodeEditor, Space};

pub fn editor_panel() -> impl gpui::IntoElement {
    Space::new()
        .vertical()
        .child(Button::new("Run").primary())
}
"#;

const TS_SAMPLE: &str = r#"type Metric = { label: string; value: number };

export function summarize(items: Metric[]) {
  return items.map((item) => `${item.label}: ${item.value}`);
}
"#;

const ADVANCED_LAYOUT_SAMPLE: &str = r#"pub fn build_panel() {
    let root = Shell::new()
        .sidebar(|sidebar| {
            sidebar
                .brand("Liora")
                .item("CodeEditor")
                .item("Diagnostics")
        })
        .content(|cx| {
            CodeEditor::new("fn main() {}", cx)
                .line_height_units(32.0)
                .indent_guides(true)
        });
}
"#;

const FOLDING_SAMPLE: &str = r#"pub struct Workspace {
    name: String,
}

impl Workspace {
    pub fn open(path: &str) -> Self {
        Self {
            name: path.to_string(),
        }
    }

    pub fn render(&self) {
        println!("{}", self.name);
    }
}

mod tests {
    #[test]
    fn opens_workspace() {
        let workspace = Workspace::open("liora");
        assert_eq!(workspace.name, "liora");
    }
}
"#;
const CONFIG_SAMPLE: &str = r#"use liora_components::{CodeEditor, CodeEditorOptions};

let options = CodeEditorOptions::default();
// Turn panels on/off per product surface.
// Read-only mode still supports selection, copy, scroll and hover.
"#;

fn gallery_code_editor_config_path() -> PathBuf {
    std::env::temp_dir()
        .join("liora-gallery")
        .join("code-editor-config.toml")
}

const CONFIG_FILE_TOML: &str = r##"language = "rust"
theme = "one-dark"
font_family = "Monospace"
font_size_px = 16
font_weight = 500
line_height_px = 26
height_px = 360
tab_size = 2
soft_tabs = true

[options]
header = true
status_bar = true
line_numbers = true
current_line_highlight = true
indent_guides = true
rulers = true
ruler_column = 72
whitespace = "boundary"
completion_limit = 4
diagnostics_limit = 4

[layout]
gutter_width_px = 72
content_padding_x_px = 18
row_padding_y_px = 2
viewport_padding_y_px = 18
header_padding_x_px = 18
header_padding_y_px = 10
header_gap_px = 16
panel_padding_x_px = 18
panel_padding_y_px = 10
panel_gap_px = 6

[appearance]
surface = "#0f172aff"
chrome_surface = "#111827ff"
gutter_surface = "#0b1220ff"
border = "#334155ff"
text = "#e5e7ebff"
muted_text = "#94a3b8ff"
caret = "#38bdf8ff"
selection = "#2563eb52"
current_line = "#1e293bff"
ruler = "#475569b8"
whitespace = "#64748b94"
warning = "#facc15ff"
error = "#fb7185ff"

[appearance.syntax.keyword]
color = "#ff79c6ff"
font_style = "italic"

[appearance.syntax.string]
color = "#a7f3d0ff"
"##;

fn config_preview_source(source: &str) -> String {
    match CodeEditorConfig::load_toml(source) {
        Ok(config) => {
            let layout = config.layout.unwrap_or_default();
            format!(
                r#"// This preview content is generated from the TOML on the left.
// Editing the config file is the explicit trigger that updates this source and styling.

pub struct EffectiveEditorConfig {{
    pub language: &'static str,
    pub theme: &'static str,
    pub font_family: &'static str,
    pub font_size_px: f32,
    pub font_weight: f32,
    pub line_height_px: f32,
    pub height_px: f32,
    pub tab_size: usize,
    pub content_padding_x_px: f32,
    pub row_padding_y_px: f32,
    pub panel_gap_px: f32,
}}

let effective = EffectiveEditorConfig {{
    language: {language:?},
    theme: {theme:?},
    font_family: {font_family:?},
    font_size_px: {font_size_px:.1},
    font_weight: {font_weight:.1},
    line_height_px: {line_height_px:.1},
    height_px: {height_px:.1},
    tab_size: {tab_size},
    content_padding_x_px: {content_padding_x_px:.1},
    row_padding_y_px: {row_padding_y_px:.1},
    panel_gap_px: {panel_gap_px:.1},
}};
"#,
                language = config.language.unwrap_or_else(|| "plain-text".to_string()),
                theme = config.theme.unwrap_or_else(|| "auto".to_string()),
                font_family = config
                    .font_family
                    .unwrap_or_else(|| "global code font".to_string()),
                font_size_px = config.font_size_px.unwrap_or(14.0),
                font_weight = config.font_weight.unwrap_or(400.0),
                line_height_px = config.line_height_px.unwrap_or(24.0),
                height_px = config.height_px.unwrap_or(0.0),
                tab_size = config.tab_size.unwrap_or(4),
                content_padding_x_px = layout.content_padding_x_px.unwrap_or(14.0),
                row_padding_y_px = layout.row_padding_y_px.unwrap_or(0.0),
                panel_gap_px = layout.panel_gap_px.unwrap_or(4.0),
            )
        }
        Err(error) => format!(
            r#"// The TOML on the left is currently invalid.
// The visual preview keeps the last valid CodeEditor config until this parses again.

pub const CONFIG_ERROR: &str = {error:?};
"#
        ),
    }
}

const THEME_SAMPLE: &str = r#"use liora_components::{
    CodeEditorHighlightTheme, CodeEditorOptions, CodeEditorWhitespaceMode,
};

let options = CodeEditorOptions {
    current_line_highlight: true,
    rulers: true,
    whitespace: CodeEditorWhitespaceMode::Boundary,
    ..CodeEditorOptions::default()
};
"#;

struct LanguageSample {
    title: &'static str,
    language: CodeLanguage,
    source: &'static str,
}

const LANGUAGE_SAMPLES: &[LanguageSample] = &[
    LanguageSample {
        title: "SQL",
        language: CodeLanguage::Sql,
        source: r#"select user_id, count(*) as sessions
from analytics.sessions
where started_at >= now() - interval '7 days'
group by user_id
order by sessions desc
limit 20;
"#,
    },
    LanguageSample {
        title: "Shell",
        language: CodeLanguage::Shell,
        source: r#"#!/usr/bin/env bash
set -euo pipefail
cargo check -p liora-components
printf 'release=%s\n' "$LIORA_VERSION"
"#,
    },
    LanguageSample {
        title: "XML",
        language: CodeLanguage::Xml,
        source: r#"<?xml version="1.0" encoding="UTF-8"?>
<window title="Liora">
  <sidebar collapsed="false" />
  <content kind="docs" />
</window>
"#,
    },
    LanguageSample {
        title: "TOML",
        language: CodeLanguage::Toml,
        source: r#"[package]
name = "liora-app"
edition = "2024"

[features]
default = ["tray", "updater"]
"#,
    },
    LanguageSample {
        title: "YAML",
        language: CodeLanguage::Yaml,
        source: r#"name: release
on:
  push:
    tags: ["v*"]
jobs:
  package:
    runs-on: ubuntu-latest
"#,
    },
    LanguageSample {
        title: "INI / conf",
        language: CodeLanguage::Conf,
        source: r#"[window]
theme=system
remember_choice=true

[updates]
channel=stable
auto_check=true
"#,
    },
    LanguageSample {
        title: "JSON",
        language: CodeLanguage::Json,
        source: r#"{
  "theme": "One Dark",
  "lineNumbers": true,
  "languages": ["rust", "sql", "yaml"]
}
"#,
    },
    LanguageSample {
        title: "Markdown",
        language: CodeLanguage::Markdown,
        source: r#"# Liora

- Native GPUI components
- Tree-sitter code editor
- Zed-style syntax themes
"#,
    },
];

#[cfg(test)]
mod tests {
    #[test]
    fn code_editor_demo_uses_component_api() {
        let source = include_str!("code_editor_demo.rs");
        assert!(source.contains("CodeEditor::new"));
        assert!(source.contains("CodeDiagnostic::warning"));
        assert!(source.contains("line_numbers"));
        assert!(source.contains("tab_size"));
        assert!(source.contains("on_change"));
        assert!(source.contains("CodeCompletionItem"));
        assert!(source.contains("CodeHover"));
        assert!(source.contains("search_query"));
        assert!(source.contains("CodeEditorOptions"));
        assert!(source.contains("read_only(true)"));
        assert!(source.contains("current_line_highlight(true)"));
        assert!(source.contains("CodeEditorHighlightTheme"));
        assert!(source.contains("CodeEditorWhitespaceMode"));
        assert!(source.contains("line_height_units"));
        assert!(source.contains("indent_guides"));
        assert!(source.contains("code_folding(true)"));
        assert!(source.contains("自动识别"));
        assert!(source.contains("ruler_column"));
        assert!(source.contains("set_config"));
        assert!(source.contains("CodeEditorConfig"));
        assert!(source.contains("liora-gallery"));
        assert!(source.contains("appearance.syntax.keyword"));
        assert!(source.contains("font_size_px"));
        assert!(source.contains("row_padding_y_px"));
        assert!(source.contains("height_px"));
        assert!(source.contains("content_padding_x_px"));
        assert!(source.contains("config_preview_source"));
        assert!(source.contains("EffectiveEditorConfig"));
        assert!(source.contains("Button::new(\"应用 set_config\")"));
        assert!(source.contains("CodeEditorConfig::load_from_path(&path)"));
        assert!(source.contains("preview.set_config(config, cx)"));
    }
}
