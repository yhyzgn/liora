//! Code Block module.
//!
//! This public module implements the Liora syntax-highlighted code block component with copy and selection support. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::gpui_compat::element_id;
use gpui::{
    App, Bounds, ClipboardItem, Component, Context, ElementId, Entity, FocusHandle, Focusable,
    FontStyle, FontWeight, GlobalElementId, Hsla, IntoElement, LayoutId, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point, Render, RenderOnce,
    Rgba, ShapedLine, SharedString, Style, StyledText, TextRun, TextStyle, UnderlineStyle,
    WhiteSpace, Window, actions, div, fill, point, prelude::*, px, relative, size,
};
use liora_core::{Config, stable_unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::{
    collections::{HashMap, VecDeque},
    hash::{Hash, Hasher},
    ops::Range,
    sync::{Arc, Mutex, MutexGuard, OnceLock},
    time::{Duration, Instant},
};
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle as SyntectFontStyle, Style as SyntectStyle, Theme},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};
use two_face::theme::{EmbeddedLazyThemeSet, EmbeddedThemeName};

actions!(
    code_block_actions,
    [
        #[doc = "Keyboard action that selects all code in the active code block."]
        CodeSelectAll,
        #[doc = "Keyboard action that copies the selected code block text."]
        CodeCopy
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumerates the supported code language modes and options.
pub enum CodeLanguage {
    /// Uses the plain text variant.
    PlainText,
    /// Uses the rust variant.
    Rust,
    /// Uses the toml variant.
    Toml,
    /// Reports a json failure.
    Json,
    /// Uses the markdown variant.
    Markdown,
    /// Uses the shell variant.
    Shell,
    /// Uses the type script variant.
    TypeScript,
    /// Uses the java script variant.
    JavaScript,
}

impl CodeLanguage {
    /// Returns the stable user-facing label for this value.
    pub fn label(self) -> &'static str {
        match self {
            Self::PlainText => "text",
            Self::Rust => "rust",
            Self::Toml => "toml",
            Self::Json => "json",
            Self::Markdown => "markdown",
            Self::Shell => "shell",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
        }
    }

    fn syntect_token(self) -> &'static str {
        match self {
            Self::PlainText => "txt",
            Self::Rust => "rs",
            Self::Toml => "toml",
            Self::Json => "json",
            Self::Markdown => "md",
            Self::Shell => "sh",
            Self::TypeScript => "ts",
            Self::JavaScript => "js",
        }
    }

    /// Creates this value from label.
    pub fn from_label(label: &str) -> Self {
        match label.trim().to_ascii_lowercase().as_str() {
            "rs" | "rust" => Self::Rust,
            "toml" => Self::Toml,
            "json" => Self::Json,
            "md" | "markdown" => Self::Markdown,
            "sh" | "bash" | "shell" | "zsh" => Self::Shell,
            "ts" | "tsx" | "typescript" => Self::TypeScript,
            "js" | "jsx" | "javascript" => Self::JavaScript,
            _ => Self::PlainText,
        }
    }
}

impl From<&str> for CodeLanguage {
    fn from(value: &str) -> Self {
        Self::from_label(value)
    }
}

impl From<String> for CodeLanguage {
    fn from(value: String) -> Self {
        Self::from_label(&value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerates the supported code format modes and options.
pub enum CodeFormat {
    /// Uses the block variant.
    Block,
    /// Uses the inline variant.
    Inline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumerates the supported code highlighter modes and options.
pub enum CodeHighlighter {
    /// Uses the syntect variant.
    Syntect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumerates the supported code theme modes and options.
pub enum CodeTheme {
    /// Uses the auto variant.
    Auto,
    /// Uses the light theme mode.
    Light,
    /// Uses the dark theme mode.
    Dark,
    /// Uses the liora light variant.
    LioraLight,
    /// Uses the liora dark variant.
    LioraDark,
    /// Uses the git hub light variant.
    GitHubLight,
    /// Uses the git hub dark variant.
    GitHubDark,
    /// Uses the one dark variant.
    OneDark,
    /// Uses the nord variant.
    Nord,
    /// Uses the dracula variant.
    Dracula,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CodeThemeMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResolvedCodeTheme {
    theme: CodeTheme,
    mode: CodeThemeMode,
}

impl CodeTheme {
    /// Returns the stable user-facing label for this value.
    pub fn label(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Light | Self::LioraLight => "liora-light",
            Self::Dark | Self::LioraDark => "liora-dark",
            Self::GitHubLight => "github-light",
            Self::GitHubDark => "github-dark",
            Self::OneDark => "one-dark",
            Self::Nord => "nord",
            Self::Dracula => "dracula",
        }
    }

    fn mode(self) -> CodeThemeMode {
        match self {
            Self::Auto | Self::Light | Self::LioraLight | Self::GitHubLight => CodeThemeMode::Light,
            Self::Dark
            | Self::LioraDark
            | Self::GitHubDark
            | Self::OneDark
            | Self::Nord
            | Self::Dracula => CodeThemeMode::Dark,
        }
    }

    fn embedded_theme(self) -> EmbeddedThemeName {
        match self {
            Self::Auto | Self::Light | Self::LioraLight => EmbeddedThemeName::CatppuccinLatte,
            Self::Dark | Self::LioraDark => EmbeddedThemeName::CatppuccinMocha,
            Self::GitHubLight => EmbeddedThemeName::Github,
            Self::GitHubDark => EmbeddedThemeName::OneHalfDark,
            Self::OneDark => EmbeddedThemeName::TwoDark,
            Self::Nord => EmbeddedThemeName::Nord,
            Self::Dracula => EmbeddedThemeName::Dracula,
        }
    }
}

/// Public builder and render state for the Liora code block component.
pub struct CodeBlock {
    code: SharedString,
    language: CodeLanguage,
    format: CodeFormat,
    highlighter: CodeHighlighter,
    theme: CodeTheme,
    copyable: bool,
    selectable: bool,
    id: Option<ElementId>,
    on_copy: Option<Arc<CodeCopyCallback>>,
}

type CodeCopyCallback = dyn Fn(&str, &mut Window, &mut App) + 'static;

impl CodeBlock {
    /// Creates a new value with the required baseline configuration.
    pub fn new(code: impl Into<SharedString>) -> Self {
        Self {
            code: code.into(),
            language: CodeLanguage::PlainText,
            format: CodeFormat::Block,
            highlighter: CodeHighlighter::Syntect,
            theme: CodeTheme::Auto,
            copyable: true,
            selectable: true,
            id: None,
            on_copy: None,
        }
    }

    /// Configures the language option.
    pub fn language(mut self, language: impl Into<CodeLanguage>) -> Self {
        self.language = language.into();
        self
    }

    /// Configures the rust option.
    pub fn rust(self) -> Self {
        self.language(CodeLanguage::Rust)
    }

    /// Configures the toml option.
    pub fn toml(self) -> Self {
        self.language(CodeLanguage::Toml)
    }

    /// Configures the json option.
    pub fn json(self) -> Self {
        self.language(CodeLanguage::Json)
    }

    /// Configures the markdown option.
    pub fn markdown(self) -> Self {
        self.language(CodeLanguage::Markdown)
    }

    /// Configures the shell option.
    pub fn shell(self) -> Self {
        self.language(CodeLanguage::Shell)
    }

    /// Configures the typescript option.
    pub fn typescript(self) -> Self {
        self.language(CodeLanguage::TypeScript)
    }

    /// Configures the javascript option.
    pub fn javascript(self) -> Self {
        self.language(CodeLanguage::JavaScript)
    }

    /// Configures the format option.
    pub fn format(mut self, format: CodeFormat) -> Self {
        self.format = format;
        self
    }

    /// Configures the inline option.
    pub fn inline(mut self) -> Self {
        self.format = CodeFormat::Inline;
        self.copyable = false;
        self
    }

    /// Configures the highlighter option.
    pub fn highlighter(mut self, highlighter: CodeHighlighter) -> Self {
        self.highlighter = highlighter;
        self
    }

    /// Configures the syntect option.
    pub fn syntect(self) -> Self {
        self.highlighter(CodeHighlighter::Syntect)
    }

    /// Configures the theme option.
    pub fn theme(mut self, theme: CodeTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Configures the auto theme option.
    pub fn auto_theme(self) -> Self {
        self.theme(CodeTheme::Auto)
    }

    /// Configures the light theme option.
    pub fn light_theme(self) -> Self {
        self.theme(CodeTheme::Light)
    }

    /// Configures the dark theme option.
    pub fn dark_theme(self) -> Self {
        self.theme(CodeTheme::Dark)
    }

    /// Configures the liora light theme option.
    pub fn liora_light_theme(self) -> Self {
        self.theme(CodeTheme::LioraLight)
    }

    /// Configures the liora dark theme option.
    pub fn liora_dark_theme(self) -> Self {
        self.theme(CodeTheme::LioraDark)
    }

    /// Configures the github light theme option.
    pub fn github_light_theme(self) -> Self {
        self.theme(CodeTheme::GitHubLight)
    }

    /// Configures the github dark theme option.
    pub fn github_dark_theme(self) -> Self {
        self.theme(CodeTheme::GitHubDark)
    }

    /// Configures the one dark theme option.
    pub fn one_dark_theme(self) -> Self {
        self.theme(CodeTheme::OneDark)
    }

    /// Configures the nord theme option.
    pub fn nord_theme(self) -> Self {
        self.theme(CodeTheme::Nord)
    }

    /// Configures the dracula theme option.
    pub fn dracula_theme(self) -> Self {
        self.theme(CodeTheme::Dracula)
    }

    /// Configures the copyable option.
    pub fn copyable(mut self, copyable: bool) -> Self {
        self.copyable = copyable;
        self
    }

    /// Registers a callback that runs when copy occurs.
    pub fn on_copy(mut self, callback: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_copy = Some(Arc::new(callback));
        self
    }

    /// Configures the selectable option.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Configures the register key bindings option.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            gpui::KeyBinding::new("cmd-a", CodeSelectAll, Some("CodeBlock")),
            gpui::KeyBinding::new("ctrl-a", CodeSelectAll, Some("CodeBlock")),
            gpui::KeyBinding::new("cmd-c", CodeCopy, Some("CodeBlock")),
            gpui::KeyBinding::new("ctrl-c", CodeCopy, Some("CodeBlock")),
        ]);
        Self::prewarm_highlighter();
    }

    /// Configures the prewarm highlighter option.
    pub fn prewarm_highlighter() {
        let _ = syntax_set();
        let themes = theme_set();
        for theme in [
            CodeTheme::LioraLight,
            CodeTheme::LioraDark,
            CodeTheme::GitHubLight,
            CodeTheme::GitHubDark,
            CodeTheme::OneDark,
            CodeTheme::Nord,
            CodeTheme::Dracula,
        ] {
            let _ = themes.get(theme.embedded_theme());
        }
    }

    /// Returns the stable tray command identifier used for menu event routing.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl RenderOnce for CodeBlock {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self.id.clone().unwrap_or_else(|| {
            stable_unique_id(
                format!(
                    "liora-code-block:{}:{:016x}:{}:{:?}:{:?}:{:?}:copyable={}:selectable={}",
                    self.language.label(),
                    hash_code_text(self.code.as_ref()),
                    self.code.len(),
                    self.format,
                    self.highlighter,
                    self.theme,
                    self.copyable,
                    self.selectable
                ),
                "liora-code-block",
                window,
                cx,
            )
            .into()
        });

        match self.format {
            CodeFormat::Inline => render_inline_code(
                self.code,
                self.language,
                self.highlighter,
                self.theme,
                &theme,
            ),
            CodeFormat::Block => render_block_code(
                id,
                self.code,
                self.language,
                self.copyable,
                self.selectable,
                self.highlighter,
                self.theme,
                &theme,
                self.on_copy,
                window,
                cx,
            ),
        }
    }
}

impl IntoElement for CodeBlock {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn render_inline_code(
    code: SharedString,
    language: CodeLanguage,
    highlighter: CodeHighlighter,
    code_theme: CodeTheme,
    theme: &liora_theme::Theme,
) -> gpui::AnyElement {
    let resolved_theme = resolve_code_theme(code_theme, theme);
    div()
        .rounded(px(theme.radius.sm))
        .bg(code_surface(resolved_theme).opacity(0.72))
        .border_1()
        .border_color(code_border(resolved_theme))
        .px_1()
        .py_0p5()
        .child(render_highlighted_text(
            code,
            language,
            highlighter,
            resolved_theme,
            theme,
            false,
        ))
        .into_any_element()
}

fn render_block_code(
    id: ElementId,
    code: SharedString,
    language: CodeLanguage,
    copyable: bool,
    selectable: bool,
    highlighter: CodeHighlighter,
    code_theme: CodeTheme,
    theme: &liora_theme::Theme,
    on_copy: Option<Arc<CodeCopyCallback>>,
    window: &mut Window,
    cx: &mut App,
) -> gpui::AnyElement {
    let resolved_theme = resolve_code_theme(code_theme, theme);
    let copied_code = code.to_string();
    let scroll_id = format!("{id}-scroll");
    let should_render_code = should_render_code_now(
        id.clone(),
        code.as_ref(),
        language,
        highlighter,
        resolved_theme,
        theme,
        window,
        cx,
    );

    let mut header = div()
        .flex()
        .items_center()
        .justify_between()
        .gap_2()
        .px_4()
        .py_2()
        .border_b_1()
        .border_color(code_border(resolved_theme))
        .bg(code_header_surface(resolved_theme))
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .text_color(code_muted_text(resolved_theme))
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .child(
                    Icon::new(IconName::FileCode)
                        .size(px(14.0))
                        .color(code_muted_text(resolved_theme)),
                )
                .child(language.label()),
        );

    if copyable {
        header = header.child(
            div()
                .id(element_id(format!("{id}-copy")))
                .flex()
                .items_center()
                .gap_1()
                .px_2()
                .py_1()
                .rounded(px(theme.radius.sm))
                .text_xs()
                .text_color(code_muted_text(resolved_theme))
                .cursor_pointer()
                .hover(|style| {
                    style
                        .bg(code_hover_surface(resolved_theme))
                        .text_color(code_accent(theme, resolved_theme))
                })
                .on_click(move |_, window, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(copied_code.clone()));
                    if let Some(on_copy) = on_copy.as_ref() {
                        on_copy(copied_code.as_str(), window, cx);
                    }
                })
                .child(
                    Icon::new(IconName::Copy)
                        .size(px(12.0))
                        .color(code_muted_text(resolved_theme)),
                )
                .child("Copy"),
        );
    }

    div()
        .id(id.clone())
        .w_full()
        .rounded(px(theme.radius.lg))
        .border_1()
        .border_color(code_border(resolved_theme))
        .bg(code_surface(resolved_theme))
        .overflow_hidden()
        .child(header)
        .child(
            div()
                .id(element_id(scroll_id))
                .overflow_x_scroll()
                .p_4()
                .bg(code_surface(resolved_theme))
                .cursor_text()
                .child(if should_render_code {
                    render_code_content(
                        id,
                        code,
                        language,
                        highlighter,
                        resolved_theme,
                        selectable,
                        theme,
                        window,
                        cx,
                    )
                } else {
                    render_code_placeholder(code, resolved_theme, theme)
                }),
        )
        .into_any_element()
}

fn should_render_code_now(
    id: ElementId,
    code: &str,
    language: CodeLanguage,
    highlighter: CodeHighlighter,
    code_theme: ResolvedCodeTheme,
    theme: &liora_theme::Theme,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    let cache_key = HighlightCacheKey::new(code, language, highlighter, code_theme, true, theme);
    if lock_highlight_cache().runs.contains_key(&cache_key) {
        return true;
    }

    let state_key = ElementId::NamedChild(Box::new(id), SharedString::from("deferred-code-ready"));
    let ready = window.use_keyed_state(state_key, cx, |_, _| false);

    if !*ready.read(cx) {
        ready.update(cx, |ready, cx| {
            *ready = true;
            cx.notify();
        });
        return false;
    }

    if take_deferred_highlight_slot() {
        true
    } else {
        ready.update(cx, |_, cx| {
            cx.notify();
        });
        false
    }
}

#[derive(Debug)]
struct DeferredHighlightBudget {
    window_start: Instant,
    remaining: usize,
}

fn take_deferred_highlight_slot() -> bool {
    static BUDGET: OnceLock<Mutex<DeferredHighlightBudget>> = OnceLock::new();
    const FRAME_BUDGET: usize = 1;
    const FRAME_WINDOW: Duration = Duration::from_millis(12);

    let mut budget = BUDGET
        .get_or_init(|| {
            Mutex::new(DeferredHighlightBudget {
                window_start: Instant::now(),
                remaining: FRAME_BUDGET,
            })
        })
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    let now = Instant::now();
    if now.duration_since(budget.window_start) >= FRAME_WINDOW {
        budget.window_start = now;
        budget.remaining = FRAME_BUDGET;
    }

    if budget.remaining > 0 {
        budget.remaining -= 1;
        true
    } else {
        false
    }
}

fn render_code_placeholder(
    code: SharedString,
    code_theme: ResolvedCodeTheme,
    theme: &liora_theme::Theme,
) -> gpui::AnyElement {
    let line_count = code.as_ref().lines().count().max(1).min(6);
    div()
        .flex()
        .flex_col()
        .gap_2()
        .children((0..line_count).map(|index| {
            let width = match index % 4 {
                0 => px(520.0),
                1 => px(380.0),
                2 => px(460.0),
                _ => px(300.0),
            };
            div()
                .h(px(theme.font_size.sm * 1.15))
                .w(width)
                .rounded(px(theme.radius.sm))
                .bg(code_muted_text(code_theme).opacity(0.16))
        }))
        .into_any_element()
}

fn render_highlighted_text(
    code: SharedString,
    language: CodeLanguage,
    highlighter: CodeHighlighter,
    code_theme: ResolvedCodeTheme,
    theme: &liora_theme::Theme,
    block: bool,
) -> StyledText {
    let runs = cached_highlight_runs(
        code.as_ref(),
        language,
        highlighter,
        code_theme,
        theme,
        block,
    );
    StyledText::new(code).with_runs(runs.to_vec())
}

fn render_code_content(
    id: ElementId,
    code: SharedString,
    language: CodeLanguage,
    highlighter: CodeHighlighter,
    code_theme: ResolvedCodeTheme,
    selectable: bool,
    theme: &liora_theme::Theme,
    window: &mut Window,
    cx: &mut App,
) -> gpui::AnyElement {
    let (highlight_key, runs) = cached_highlight_runs_with_key(
        code.as_ref(),
        language,
        highlighter,
        code_theme,
        theme,
        true,
    );

    if selectable {
        let state_key = ElementId::NamedChild(
            Box::new(id.clone()),
            SharedString::from("selectable-code-text"),
        );
        let initial_id = id.clone();
        let initial_code = code.clone();
        let initial_runs = runs.clone();
        let initial_theme = theme.clone();
        let initial_highlight_key = highlight_key.clone();
        let input = window.use_keyed_state(state_key, cx, move |_, cx| {
            SelectableCodeText::new(
                cx,
                initial_id,
                initial_code,
                initial_runs,
                initial_highlight_key,
                &initial_theme,
            )
        });
        input.update(cx, |text, cx| {
            text.update_content(id, code, runs, highlight_key, theme, cx);
        });
        SelectableCodeTextView { input }.into_any_element()
    } else {
        let state_key = ElementId::NamedChild(
            Box::new(id.clone()),
            SharedString::from("read-only-code-text"),
        );
        let initial_code = code.clone();
        let initial_runs = runs.clone();
        let initial_theme = theme.clone();
        let initial_highlight_key = highlight_key.clone();
        let input = window.use_keyed_state(state_key, cx, move |_, _| {
            ReadOnlyCodeText::new(
                initial_code,
                initial_runs,
                initial_highlight_key,
                &initial_theme,
            )
        });
        input.update(cx, |text, cx| {
            text.update_content(code, runs, highlight_key, theme, cx);
        });
        ReadOnlyCodeTextView { input }.into_any_element()
    }
}

struct SelectableCodeTextView {
    input: Entity<SelectableCodeText>,
}

impl IntoElement for SelectableCodeTextView {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for SelectableCodeTextView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.input.into_any_element()
    }
}

struct ReadOnlyCodeTextView {
    input: Entity<ReadOnlyCodeText>,
}

impl IntoElement for ReadOnlyCodeTextView {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for ReadOnlyCodeTextView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.input.into_any_element()
    }
}

fn cached_highlight_runs(
    text: &str,
    language: CodeLanguage,
    highlighter: CodeHighlighter,
    code_theme: ResolvedCodeTheme,
    theme: &liora_theme::Theme,
    block: bool,
) -> Vec<TextRun> {
    cached_highlight_runs_with_key(text, language, highlighter, code_theme, theme, block)
        .1
        .to_vec()
}

fn cached_highlight_runs_with_key(
    text: &str,
    language: CodeLanguage,
    highlighter: CodeHighlighter,
    code_theme: ResolvedCodeTheme,
    theme: &liora_theme::Theme,
    block: bool,
) -> (HighlightCacheKey, HighlightRuns) {
    let key = HighlightCacheKey::new(text, language, highlighter, code_theme, block, theme);
    if let Some(runs) = lock_highlight_cache().runs.get(&key).cloned() {
        return (key, runs);
    }

    let runs = HighlightRuns::from(highlight_runs(
        text,
        language,
        highlighter,
        code_theme,
        theme,
        block,
    ));
    let mut cache = lock_highlight_cache();
    cache.insert(key.clone(), runs.clone());
    (key, runs)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct HighlightCacheKey {
    text_hash: u64,
    text_len: usize,
    language: CodeLanguage,
    highlighter: CodeHighlighter,
    theme: CodeTheme,
    block: bool,
    font_size_bits: u32,
}

impl HighlightCacheKey {
    fn new(
        text: &str,
        language: CodeLanguage,
        highlighter: CodeHighlighter,
        code_theme: ResolvedCodeTheme,
        block: bool,
        theme: &liora_theme::Theme,
    ) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        text.hash(&mut hasher);
        Self {
            text_hash: hasher.finish(),
            text_len: text.len(),
            language,
            highlighter,
            theme: code_theme.theme,
            block,
            font_size_bits: if block {
                theme.font_size.sm
            } else {
                theme.font_size.md
            }
            .to_bits(),
        }
    }
}

const HIGHLIGHT_CACHE_CAPACITY: usize = 256;
type HighlightRuns = Arc<[TextRun]>;

#[derive(Default)]
struct HighlightCache {
    runs: HashMap<HighlightCacheKey, HighlightRuns>,
    order: VecDeque<HighlightCacheKey>,
}

impl HighlightCache {
    fn insert(&mut self, key: HighlightCacheKey, runs: HighlightRuns) {
        if self.runs.contains_key(&key) {
            self.runs.insert(key, runs);
            return;
        }

        self.runs.insert(key.clone(), runs);
        self.order.push_back(key);
        self.evict_over_capacity();
    }

    fn evict_over_capacity(&mut self) {
        while self.runs.len() > HIGHLIGHT_CACHE_CAPACITY {
            let Some(oldest) = self.order.pop_front() else {
                break;
            };
            self.runs.remove(&oldest);
        }
    }
}

fn highlight_cache() -> &'static Mutex<HighlightCache> {
    static CACHE: OnceLock<Mutex<HighlightCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HighlightCache::default()))
}

fn lock_highlight_cache() -> MutexGuard<'static, HighlightCache> {
    highlight_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn highlight_runs(
    text: &str,
    language: CodeLanguage,
    highlighter: CodeHighlighter,
    code_theme: ResolvedCodeTheme,
    theme: &liora_theme::Theme,
    block: bool,
) -> Vec<TextRun> {
    match highlighter {
        CodeHighlighter::Syntect => syntect_runs(text, language, code_theme, theme, block),
    }
}

fn syntect_runs(
    text: &str,
    language: CodeLanguage,
    code_theme: ResolvedCodeTheme,
    theme: &liora_theme::Theme,
    block: bool,
) -> Vec<TextRun> {
    if text.is_empty() {
        return vec![base_style(theme, code_theme, block).to_run(0)];
    }

    let syntax_set = syntax_set();
    let syntax = syntax_set
        .find_syntax_by_token(language.syntect_token())
        .or_else(|| syntax_set.find_syntax_by_extension(language.syntect_token()))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let syntect_theme = syntect_theme(code_theme);
    let mut highlighter = HighlightLines::new(syntax, syntect_theme);
    let mut runs = Vec::new();

    for line in LinesWithEndings::from(text) {
        match highlighter.highlight_line(line, syntax_set) {
            Ok(regions) => {
                for (style, slice) in regions {
                    if !slice.is_empty() {
                        push_run(
                            &mut runs,
                            syntect_style_run(slice.len(), style, theme, code_theme, block),
                        );
                    }
                }
            }
            Err(_) => push_run(
                &mut runs,
                base_style(theme, code_theme, block).to_run(line.len()),
            ),
        }
    }

    if runs.is_empty() {
        runs.push(base_style(theme, code_theme, block).to_run(text.len()));
    }

    runs
}

fn push_run(runs: &mut Vec<TextRun>, run: TextRun) {
    if run.len == 0 {
        return;
    }

    if let Some(last) = runs.last_mut() {
        if last.font == run.font
            && last.color == run.color
            && last.background_color == run.background_color
            && last.underline == run.underline
            && last.strikethrough == run.strikethrough
        {
            last.len += run.len;
            return;
        }
    }

    runs.push(run);
}

fn syntect_style_run(
    len: usize,
    syntect_style: SyntectStyle,
    theme: &liora_theme::Theme,
    code_theme: ResolvedCodeTheme,
    block: bool,
) -> TextRun {
    let mut style = base_style(theme, code_theme, block);
    style.color = syntect_color(syntect_style.foreground);

    if syntect_style.font_style.contains(SyntectFontStyle::BOLD) {
        style.font_weight = FontWeight::BOLD;
    }

    if syntect_style.font_style.contains(SyntectFontStyle::ITALIC) {
        style.font_style = FontStyle::Italic;
    }

    if syntect_style
        .font_style
        .contains(SyntectFontStyle::UNDERLINE)
    {
        style.underline = Some(UnderlineStyle {
            thickness: px(1.0),
            color: Some(style.color),
            ..Default::default()
        });
    }

    style.to_run(len)
}

fn base_style(theme: &liora_theme::Theme, code_theme: ResolvedCodeTheme, block: bool) -> TextStyle {
    let mut style = TextStyle::default();
    style.font_family = "Monospace".into();
    style.font_size = px(if block {
        theme.font_size.sm
    } else {
        theme.font_size.md
    })
    .into();
    style.line_height = px(theme.font_size.md * 1.7).into();
    style.white_space = WhiteSpace::Nowrap;
    style.color = code_text(code_theme);
    style
}

fn syntax_set() -> &'static SyntaxSet {
    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    SYNTAX_SET.get_or_init(two_face::syntax::extra_newlines)
}

fn resolve_code_theme(code_theme: CodeTheme, theme: &liora_theme::Theme) -> ResolvedCodeTheme {
    let resolved = match code_theme {
        CodeTheme::Auto if theme.name.eq_ignore_ascii_case("dark") => CodeTheme::LioraDark,
        CodeTheme::Auto => CodeTheme::LioraLight,
        CodeTheme::Light => CodeTheme::LioraLight,
        CodeTheme::Dark => CodeTheme::LioraDark,
        theme => theme,
    };

    ResolvedCodeTheme {
        theme: resolved,
        mode: resolved.mode(),
    }
}

fn theme_set() -> &'static EmbeddedLazyThemeSet {
    static THEME_SET: OnceLock<EmbeddedLazyThemeSet> = OnceLock::new();
    THEME_SET.get_or_init(two_face::theme::extra)
}

fn syntect_theme(code_theme: ResolvedCodeTheme) -> &'static Theme {
    theme_set().get(code_theme.theme.embedded_theme())
}

fn syntect_color(color: syntect::highlighting::Color) -> Hsla {
    Rgba {
        r: color.r as f32 / 255.0,
        g: color.g as f32 / 255.0,
        b: color.b as f32 / 255.0,
        a: color.a as f32 / 255.0,
    }
    .into()
}

fn code_surface(code_theme: ResolvedCodeTheme) -> Hsla {
    match code_theme.mode {
        CodeThemeMode::Light => rgb(0xf7f8fa),
        CodeThemeMode::Dark => rgb(0x1b2b34),
    }
}

fn code_header_surface(code_theme: ResolvedCodeTheme) -> Hsla {
    match code_theme.mode {
        CodeThemeMode::Light => rgb(0xf0f2f5),
        CodeThemeMode::Dark => rgb(0x16242c),
    }
}

fn code_hover_surface(code_theme: ResolvedCodeTheme) -> Hsla {
    match code_theme.mode {
        CodeThemeMode::Light => rgb(0xe8edf3),
        CodeThemeMode::Dark => rgb(0x253c49),
    }
}

fn code_border(code_theme: ResolvedCodeTheme) -> Hsla {
    match code_theme.mode {
        CodeThemeMode::Light => rgb(0xd8dee8),
        CodeThemeMode::Dark => rgb(0x334d5c),
    }
}

fn code_text(code_theme: ResolvedCodeTheme) -> Hsla {
    match code_theme.mode {
        CodeThemeMode::Light => rgb(0x2b303b),
        CodeThemeMode::Dark => rgb(0xc0c5ce),
    }
}

fn code_muted_text(code_theme: ResolvedCodeTheme) -> Hsla {
    match code_theme.mode {
        CodeThemeMode::Light => rgb(0x65737e),
        CodeThemeMode::Dark => rgb(0xa7adba),
    }
}

fn code_accent(theme: &liora_theme::Theme, code_theme: ResolvedCodeTheme) -> Hsla {
    match code_theme.mode {
        CodeThemeMode::Light => theme.info.base,
        CodeThemeMode::Dark => rgb(0x96b5b4),
    }
}

struct ReadOnlyCodeText {
    code: SharedString,
    runs: HighlightRuns,
    highlight_key: HighlightCacheKey,
    theme: liora_theme::Theme,
    layout: Option<Arc<SelectableCodeLayout>>,
}

impl ReadOnlyCodeText {
    fn new(
        code: SharedString,
        runs: HighlightRuns,
        highlight_key: HighlightCacheKey,
        theme: &liora_theme::Theme,
    ) -> Self {
        Self {
            code,
            runs,
            highlight_key,
            theme: theme.clone(),
            layout: None,
        }
    }

    fn update_content(
        &mut self,
        code: SharedString,
        runs: HighlightRuns,
        highlight_key: HighlightCacheKey,
        theme: &liora_theme::Theme,
        cx: &mut Context<Self>,
    ) {
        let changed = self.highlight_key != highlight_key
            || self.theme.name != theme.name
            || self.theme.font_size.sm != theme.font_size.sm
            || self.theme.font_size.md != theme.font_size.md
            || self.theme.primary.base != theme.primary.base;
        if !changed {
            return;
        }

        self.code = code;
        self.runs = runs;
        self.highlight_key = highlight_key;
        self.theme = theme.clone();
        self.layout = None;
        cx.notify();
    }

    fn font_size(&self) -> Pixels {
        px(self.theme.font_size.md)
    }

    fn line_height(&self) -> Pixels {
        px(self.theme.font_size.md * 1.7)
    }

    fn ensure_layout(&mut self, window: &mut Window) -> Arc<SelectableCodeLayout> {
        if let Some(layout) = self.layout.as_ref() {
            return layout.clone();
        }

        let layout = Arc::new(build_code_layout(
            self.code.as_ref(),
            &self.runs,
            self.font_size(),
            self.line_height(),
            window,
        ));
        self.layout = Some(layout.clone());
        layout
    }
}

impl Render for ReadOnlyCodeText {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("read-only-code-text")
            .child(ReadOnlyCodeElement { input: cx.entity() })
    }
}

struct ReadOnlyCodeElement {
    input: Entity<ReadOnlyCodeText>,
}

struct ReadOnlyCodePrepaint {
    layout: Arc<SelectableCodeLayout>,
}

impl IntoElement for ReadOnlyCodeElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ReadOnlyCodeElement {
    type RequestLayoutState = Arc<SelectableCodeLayout>;
    type PrepaintState = ReadOnlyCodePrepaint;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Arc<SelectableCodeLayout>) {
        let layout = self
            .input
            .update(cx, |input, _| input.ensure_layout(window));
        let mut style = Style::default();
        style.size.width = layout.width.into();
        style.min_size.width = relative(1.).into();
        style.size.height = layout.height.into();
        (window.request_layout(style, [], cx), layout)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        layout: &mut Arc<SelectableCodeLayout>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> ReadOnlyCodePrepaint {
        ReadOnlyCodePrepaint {
            layout: layout.clone(),
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Arc<SelectableCodeLayout>,
        prepaint: &mut ReadOnlyCodePrepaint,
        window: &mut Window,
        cx: &mut App,
    ) {
        let line_height = self.input.read(cx).line_height();
        for line in &prepaint.layout.lines {
            let _ = line.shaped.paint(
                point(bounds.left(), bounds.top() + line.y),
                line_height,
                window,
                cx,
            );
        }
    }
}

#[derive(Clone)]
struct SelectableCodeState {
    selected_range: Range<usize>,
    selection_reversed: bool,
    selecting: bool,
    lines: Vec<(ShapedLine, Pixels, usize)>,
    bounds: Option<Bounds<Pixels>>,
}

impl Default for SelectableCodeState {
    fn default() -> Self {
        Self {
            selected_range: 0..0,
            selection_reversed: false,
            selecting: false,
            lines: Vec::new(),
            bounds: None,
        }
    }
}

#[derive(Clone)]
struct SelectableCodeLayout {
    lines: Vec<SelectableCodeLine>,
    width: Pixels,
    height: Pixels,
}

#[derive(Clone)]
struct SelectableCodeLine {
    shaped: ShapedLine,
    start: usize,
    y: Pixels,
}

fn selectable_state_map() -> &'static Mutex<HashMap<String, SelectableCodeState>> {
    static STATES: OnceLock<Mutex<HashMap<String, SelectableCodeState>>> = OnceLock::new();
    STATES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock_selectable_state_map() -> MutexGuard<'static, HashMap<String, SelectableCodeState>> {
    selectable_state_map()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn set_selectable_layout_state(
    id: &ElementId,
    lines: Vec<(ShapedLine, Pixels, usize)>,
    bounds: Bounds<Pixels>,
) {
    with_selectable_state(id, |state| {
        state.lines = lines;
        state.bounds = Some(bounds);
    });
}

fn selectable_key(id: &ElementId) -> String {
    id.to_string()
}

fn with_selectable_state<R>(id: &ElementId, f: impl FnOnce(&mut SelectableCodeState) -> R) -> R {
    let mut states = lock_selectable_state_map();
    f(states.entry(selectable_key(id)).or_default())
}

fn selectable_state_snapshot(id: &ElementId) -> SelectableCodeState {
    lock_selectable_state_map()
        .get(&selectable_key(id))
        .cloned()
        .unwrap_or_default()
}

struct SelectableCodeText {
    id: ElementId,
    focus_handle: FocusHandle,
    code: SharedString,
    runs: HighlightRuns,
    highlight_key: HighlightCacheKey,
    theme: liora_theme::Theme,
    layout: Option<Arc<SelectableCodeLayout>>,
}

impl SelectableCodeText {
    fn new(
        cx: &mut Context<Self>,
        id: ElementId,
        code: SharedString,
        runs: HighlightRuns,
        highlight_key: HighlightCacheKey,
        theme: &liora_theme::Theme,
    ) -> Self {
        Self {
            id,
            focus_handle: cx.focus_handle(),
            code,
            runs,
            highlight_key,
            theme: theme.clone(),
            layout: None,
        }
    }

    fn update_content(
        &mut self,
        id: ElementId,
        code: SharedString,
        runs: HighlightRuns,
        highlight_key: HighlightCacheKey,
        theme: &liora_theme::Theme,
        cx: &mut Context<Self>,
    ) {
        let changed = self.id != id
            || self.highlight_key != highlight_key
            || self.theme.name != theme.name
            || self.theme.font_size.sm != theme.font_size.sm
            || self.theme.font_size.md != theme.font_size.md
            || self.theme.primary.base != theme.primary.base;
        if !changed {
            return;
        }

        let old_id = self.id.clone();
        self.id = id;
        self.code = code;
        self.runs = runs;
        self.highlight_key = highlight_key;
        self.theme = theme.clone();
        self.layout = None;

        if old_id != self.id {
            let old_state = selectable_state_snapshot(&old_id);
            with_selectable_state(&self.id, |state| *state = old_state);
        }

        with_selectable_state(&self.id, |state| {
            state.selected_range.start = self.clamp_boundary(state.selected_range.start);
            state.selected_range.end = self.clamp_boundary(state.selected_range.end);
            if state.selected_range.end < state.selected_range.start {
                state.selected_range = state.selected_range.end..state.selected_range.start;
                state.selection_reversed = !state.selection_reversed;
            }
        });
        cx.notify();
    }

    fn move_to(&self, state: &mut SelectableCodeState, offset: usize) -> bool {
        let offset = self.clamp_boundary(offset);
        if state.selected_range == (offset..offset) && !state.selection_reversed {
            return false;
        }
        state.selected_range = offset..offset;
        state.selection_reversed = false;
        true
    }

    fn select_to(&self, state: &mut SelectableCodeState, offset: usize) -> bool {
        let offset = self.clamp_boundary(offset);
        let previous_range = state.selected_range.clone();
        let previous_reversed = state.selection_reversed;
        if state.selection_reversed {
            state.selected_range.start = offset;
        } else {
            state.selected_range.end = offset;
        }
        if state.selected_range.end < state.selected_range.start {
            state.selection_reversed = !state.selection_reversed;
            state.selected_range = state.selected_range.end..state.selected_range.start;
        }
        if state.selected_range == previous_range && state.selection_reversed == previous_reversed {
            return false;
        }
        true
    }

    fn clamp_boundary(&self, mut offset: usize) -> usize {
        offset = offset.min(self.code.len());
        while offset > 0 && !self.code.is_char_boundary(offset) {
            offset -= 1;
        }
        offset
    }

    fn index_for_point(&self, pt: Point<Pixels>) -> usize {
        let state = selectable_state_snapshot(&self.id);
        let Some(bounds) = state.bounds.as_ref() else {
            return self.code.len();
        };
        if state.lines.is_empty() {
            return 0;
        }

        let mut chosen = 0;
        for (ix, (_line, y, _start)) in state.lines.iter().enumerate() {
            let line_height = self.line_height();
            if pt.y >= *y && pt.y < *y + line_height {
                chosen = ix;
                break;
            }
            if pt.y >= *y {
                chosen = ix;
            }
        }

        let (line, _y, start) = &state.lines[chosen];
        let x = pt.x - bounds.left();
        let line_index = line.index_for_x(x).unwrap_or(line.len());
        self.clamp_boundary(*start + line_index)
    }

    fn select_all(&mut self, _: &CodeSelectAll, _: &mut Window, cx: &mut Context<Self>) {
        let changed = with_selectable_state(&self.id, |state| {
            let changed = state.selected_range != (0..self.code.len()) || state.selection_reversed;
            state.selected_range = 0..self.code.len();
            state.selection_reversed = false;
            changed
        });
        if changed {
            cx.notify();
        }
    }

    fn copy(&mut self, _: &CodeCopy, _: &mut Window, cx: &mut Context<Self>) {
        let selected_range = selectable_state_snapshot(&self.id).selected_range;
        if !selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.code[selected_range].to_string(),
            ));
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle);
        let idx = self.index_for_point(event.position);
        let changed = with_selectable_state(&self.id, |state| {
            let was_selecting = state.selecting;
            state.selecting = true;
            if event.modifiers.shift {
                self.select_to(state, idx) || !was_selecting
            } else if event.click_count >= 3 {
                let changed = state.selected_range != (0..self.code.len())
                    || state.selection_reversed
                    || !was_selecting;
                state.selected_range = 0..self.code.len();
                state.selection_reversed = false;
                changed
            } else if event.click_count == 2 {
                let range = self.word_range_at(idx);
                let changed =
                    state.selected_range != range || state.selection_reversed || !was_selecting;
                state.selected_range = range;
                state.selection_reversed = false;
                changed
            } else {
                self.move_to(state, idx) || !was_selecting
            }
        });
        if changed {
            cx.notify();
        }
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        let dragging = event.pressed_button == Some(MouseButton::Left);
        let idx = dragging.then(|| self.index_for_point(event.position));
        let changed = with_selectable_state(&self.id, |state| {
            if !dragging {
                let changed = state.selecting;
                state.selecting = false;
                changed
            } else if state.selecting {
                self.select_to(state, idx.unwrap_or(self.code.len()))
            } else {
                false
            }
        });
        if changed {
            cx.notify();
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        let changed = with_selectable_state(&self.id, |state| {
            let changed = state.selecting;
            state.selecting = false;
            changed
        });
        if changed {
            cx.notify();
        }
    }

    fn word_range_at(&self, idx: usize) -> Range<usize> {
        let text = self.code.as_ref();
        if text.is_empty() {
            return 0..0;
        }
        let idx = self.clamp_boundary(idx);
        let mut start = idx;
        while start > 0 {
            let prev = self.prev_char(start);
            let ch = text[prev..start].chars().next().unwrap_or(' ');
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            start = prev;
        }
        let mut end = idx;
        while end < text.len() {
            let next = self.next_char(end);
            let ch = text[end..next].chars().next().unwrap_or(' ');
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            end = next;
        }
        start..end
    }

    fn prev_char(&self, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }
        let mut prev = offset - 1;
        while prev > 0 && !self.code.is_char_boundary(prev) {
            prev -= 1;
        }
        prev
    }

    fn next_char(&self, offset: usize) -> usize {
        if offset >= self.code.len() {
            return self.code.len();
        }
        let mut next = offset + 1;
        while next < self.code.len() && !self.code.is_char_boundary(next) {
            next += 1;
        }
        next
    }

    fn font_size(&self) -> Pixels {
        px(self.theme.font_size.md)
    }

    fn line_height(&self) -> Pixels {
        px(self.theme.font_size.md * 1.7)
    }

    fn ensure_layout(&mut self, window: &mut Window) -> Arc<SelectableCodeLayout> {
        if let Some(layout) = self.layout.as_ref() {
            return layout.clone();
        }

        let layout = Arc::new(build_code_layout(
            self.code.as_ref(),
            &self.runs,
            self.font_size(),
            self.line_height(),
            window,
        ));
        self.layout = Some(layout.clone());
        layout
    }
}

impl Focusable for SelectableCodeText {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct SelectableCodeElement {
    id: ElementId,
    input: Entity<SelectableCodeText>,
}

struct SelectableCodePrepaint {
    layout: Arc<SelectableCodeLayout>,
    selection: Vec<PaintQuad>,
    hitbox: gpui::Hitbox,
}

impl IntoElement for SelectableCodeElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for SelectableCodeElement {
    type RequestLayoutState = Arc<SelectableCodeLayout>;
    type PrepaintState = SelectableCodePrepaint;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Arc<SelectableCodeLayout>) {
        let layout = self
            .input
            .update(cx, |input, _| input.ensure_layout(window));
        let mut style = Style::default();
        style.size.width = layout.width.into();
        style.min_size.width = relative(1.).into();
        style.size.height = layout.height.into();
        (window.request_layout(style, [], cx), layout)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Arc<SelectableCodeLayout>,
        window: &mut Window,
        cx: &mut App,
    ) -> SelectableCodePrepaint {
        let input = self.input.read(cx);
        let line_height = input.line_height();
        let mut state_lines = Vec::new();
        let mut selection_quads = Vec::new();
        let selected_range = selectable_state_snapshot(&input.id).selected_range;

        for line in &layout.lines {
            let y = bounds.top() + line.y;
            if !selected_range.is_empty() {
                let line_end = line.start + line.shaped.len();
                let start = selected_range.start.max(line.start);
                let end = selected_range.end.min(line_end);
                if start < end {
                    let x_start = line.shaped.x_for_index(start - line.start);
                    let x_end = line.shaped.x_for_index(end - line.start);
                    selection_quads.push(fill(
                        Bounds::new(
                            point(bounds.left() + x_start, y),
                            size(x_end - x_start, line_height),
                        ),
                        input.theme.primary.base.opacity(0.28),
                    ));
                }
            }

            state_lines.push((line.shaped.clone(), y, line.start));
        }

        let hitbox = window.insert_hitbox(bounds, gpui::HitboxBehavior::Normal);
        set_selectable_layout_state(&input.id, state_lines, bounds);

        SelectableCodePrepaint {
            layout: layout.clone(),
            selection: selection_quads,
            hitbox,
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Arc<SelectableCodeLayout>,
        prepaint: &mut SelectableCodePrepaint,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.set_cursor_style(gpui::CursorStyle::IBeam, &prepaint.hitbox);

        let input = self.input.clone();
        let focus_handle_for_down = focus_handle.clone();
        let hitbox = prepaint.hitbox.clone();
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase.bubble() && event.button == MouseButton::Left && hitbox.is_hovered(window) {
                window.focus(&focus_handle_for_down);
                input.update(cx, |input, cx| input.on_mouse_down(event, window, cx));
                cx.stop_propagation();
            }
        });

        let input = self.input.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, _window, cx| {
            if phase.capture() {
                input.update(cx, |input, cx| input.on_mouse_move(event, cx));
            }
        });

        let input = self.input.clone();
        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            if phase.capture() && event.button == MouseButton::Left {
                input.update(cx, |input, cx| input.on_mouse_up(event, window, cx));
            }
        });

        for selection in prepaint.selection.drain(..) {
            window.paint_quad(selection);
        }

        for line in &prepaint.layout.lines {
            let _ = line.shaped.paint(
                point(bounds.left(), bounds.top() + line.y),
                self.input.read(cx).line_height(),
                window,
                cx,
            );
        }
    }
}

impl Render for SelectableCodeText {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id(element_id(format!("{}-selectable", self.id)))
            .key_context("CodeBlock")
            .track_focus(&self.focus_handle(cx))
            .cursor_text()
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::copy))
            .child(SelectableCodeElement {
                id: element_id(format!("{}-text", self.id)),
                input: cx.entity(),
            })
    }
}

fn build_code_layout(
    code: &str,
    runs: &[TextRun],
    font_size: Pixels,
    line_height: Pixels,
    window: &mut Window,
) -> SelectableCodeLayout {
    let mut max_width = px(1.0);
    let mut offset = 0;
    let mut y = px(0.0);
    let mut lines = Vec::new();
    for line in code_lines(code) {
        let line_len = line.len();
        let line_runs = slice_runs(runs, offset, offset + line_len);
        let shaped = window.text_system().shape_line(
            SharedString::from(line.to_string()),
            font_size,
            &line_runs,
            None,
        );
        max_width = max_width.max(shaped.width);
        lines.push(SelectableCodeLine {
            shaped,
            start: offset,
            y,
        });
        offset += line_len + 1;
        y += line_height;
    }

    SelectableCodeLayout {
        height: line_height * lines.len() as f32,
        lines,
        width: max_width,
    }
}

fn hash_code_text(text: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

fn code_lines(text: &str) -> impl Iterator<Item = &str> {
    text.strip_suffix('\n').unwrap_or(text).split('\n')
}

fn slice_runs(runs: &[TextRun], start: usize, end: usize) -> Vec<TextRun> {
    let mut sliced = Vec::new();
    let mut offset = 0;
    for run in runs {
        let run_start = offset;
        let run_end = offset + run.len;
        let overlap_start = start.max(run_start);
        let overlap_end = end.min(run_end);
        if overlap_start < overlap_end {
            sliced.push(TextRun {
                len: overlap_end - overlap_start,
                ..run.clone()
            });
        }
        offset = run_end;
        if offset >= end {
            break;
        }
    }
    if sliced.is_empty() && start == end {
        return sliced;
    }
    sliced
}

fn rgb(hex: u32) -> Hsla {
    Rgba {
        r: ((hex >> 16) & 0xff) as f32 / 255.0,
        g: ((hex >> 8) & 0xff) as f32 / 255.0,
        b: (hex & 0xff) as f32 / 255.0,
        a: 1.0,
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_labels_parse_common_aliases() {
        assert_eq!(CodeLanguage::from_label("rs"), CodeLanguage::Rust);
        assert_eq!(CodeLanguage::from_label("bash"), CodeLanguage::Shell);
        assert_eq!(CodeLanguage::from_label("tsx"), CodeLanguage::TypeScript);
        assert_eq!(CodeLanguage::from_label("unknown"), CodeLanguage::PlainText);
    }

    #[test]
    fn syntect_highlighter_generates_multiple_styled_runs_for_rust() {
        let theme = liora_theme::Theme::light();
        let code_theme = resolve_code_theme(CodeTheme::Auto, &theme);
        let runs = syntect_runs(
            "fn main() { let n = 42; // ok\n println!(\"hi\"); }",
            CodeLanguage::Rust,
            code_theme,
            &theme,
            true,
        );

        assert!(runs.len() > 3);
        assert_eq!(runs.iter().map(|run| run.len).sum::<usize>(), 48);
        assert!(runs.iter().any(|run| run.color != code_text(code_theme)));
    }

    #[test]
    fn themes_resolve_to_distinct_syntect_and_surface_palettes() {
        let light = liora_theme::Theme::light();
        let dark = liora_theme::Theme::dark();

        assert_eq!(
            resolve_code_theme(CodeTheme::Auto, &light),
            ResolvedCodeTheme {
                theme: CodeTheme::LioraLight,
                mode: CodeThemeMode::Light
            }
        );
        assert_eq!(
            resolve_code_theme(CodeTheme::Auto, &dark),
            ResolvedCodeTheme {
                theme: CodeTheme::LioraDark,
                mode: CodeThemeMode::Dark
            }
        );
        assert_ne!(
            syntect_theme(ResolvedCodeTheme {
                theme: CodeTheme::LioraLight,
                mode: CodeThemeMode::Light,
            })
            .settings
            .background,
            syntect_theme(ResolvedCodeTheme {
                theme: CodeTheme::LioraDark,
                mode: CodeThemeMode::Dark,
            })
            .settings
            .background
        );
        assert_ne!(
            code_surface(ResolvedCodeTheme {
                theme: CodeTheme::LioraLight,
                mode: CodeThemeMode::Light,
            }),
            code_surface(ResolvedCodeTheme {
                theme: CodeTheme::LioraDark,
                mode: CodeThemeMode::Dark,
            })
        );
    }

    #[test]
    fn cached_highlight_runs_reuses_render_runs_for_same_code_and_theme() {
        let theme = liora_theme::Theme::light();
        let code_theme = resolve_code_theme(CodeTheme::Auto, &theme);
        let first = cached_highlight_runs(
            "let cached = true;",
            CodeLanguage::Rust,
            CodeHighlighter::Syntect,
            code_theme,
            &theme,
            true,
        );
        let second = cached_highlight_runs(
            "let cached = true;",
            CodeLanguage::Rust,
            CodeHighlighter::Syntect,
            code_theme,
            &theme,
            true,
        );

        assert_eq!(first, second);
    }

    #[test]
    fn cached_highlight_runs_share_arc_storage_for_block_layouts() {
        let theme = liora_theme::Theme::light();
        let code_theme = resolve_code_theme(CodeTheme::Auto, &theme);
        let (_, first) = cached_highlight_runs_with_key(
            "fn shared_runs() { println!(\"cache\"); }",
            CodeLanguage::Rust,
            CodeHighlighter::Syntect,
            code_theme,
            &theme,
            true,
        );
        let (_, second) = cached_highlight_runs_with_key(
            "fn shared_runs() { println!(\"cache\"); }",
            CodeLanguage::Rust,
            CodeHighlighter::Syntect,
            code_theme,
            &theme,
            true,
        );

        assert!(Arc::ptr_eq(&first, &second));
    }

    #[test]
    fn highlight_cache_evicts_incrementally_without_clearing_all_runs() {
        let mut cache = HighlightCache::default();
        let theme = liora_theme::Theme::light();
        let code_theme = resolve_code_theme(CodeTheme::Auto, &theme);
        let first_key = HighlightCacheKey::new(
            "let item_0 = 0;",
            CodeLanguage::Rust,
            CodeHighlighter::Syntect,
            code_theme,
            true,
            &theme,
        );
        let last_key = HighlightCacheKey::new(
            "let item_256 = 256;",
            CodeLanguage::Rust,
            CodeHighlighter::Syntect,
            code_theme,
            true,
            &theme,
        );

        for index in 0..=HIGHLIGHT_CACHE_CAPACITY {
            let text = format!("let item_{index} = {index};");
            let key = HighlightCacheKey::new(
                &text,
                CodeLanguage::Rust,
                CodeHighlighter::Syntect,
                code_theme,
                true,
                &theme,
            );
            cache.insert(
                key,
                HighlightRuns::from(vec![
                    base_style(&theme, code_theme, true).to_run(text.len()),
                ]),
            );
        }

        assert_eq!(cache.runs.len(), HIGHLIGHT_CACHE_CAPACITY);
        assert!(!cache.runs.contains_key(&first_key));
        assert!(cache.runs.contains_key(&last_key));
    }

    #[test]
    fn block_code_defers_expensive_rendering_for_first_frame() {
        let source = include_str!("code_block.rs");

        assert!(source.contains("should_render_code_now"));
        assert!(source.contains("deferred-code-ready"));
        assert!(source.contains("take_deferred_highlight_slot"));
        assert!(source.contains("FRAME_BUDGET"));
        assert!(source.contains("render_code_placeholder"));
        assert!(source.contains("cx.notify()"));
    }

    #[test]
    fn component_uses_syntect_and_supports_copyable_block_and_inline_format() {
        let source = include_str!("code_block.rs");

        assert!(source.contains("HighlightLines"));
        assert!(source.contains("SyntaxSet::load_defaults_newlines"));
        assert!(source.contains("ThemeSet::load_defaults"));
        assert!(source.contains("ClipboardItem::new_string"));
        assert!(source.contains("on_copy"));
        assert!(source.contains("CodeCopyCallback"));
        assert!(source.contains("CodeFormat::Inline"));
        assert!(source.contains("selectable"));
        assert!(source.contains("SelectableCodeText"));
        assert!(source.contains("SelectableCodeState"));
        assert!(source.contains("selectable_state_map"));
        assert!(source.contains("lines: Vec<(ShapedLine"));
        assert!(source.contains("bounds: Option<Bounds"));
        assert!(source.contains("with_selectable_state(&self.id"));
        assert!(source.contains("prewarm_highlighter"));
        assert!(source.contains("SelectableCodeLayout"));
        assert!(source.contains("ReadOnlyCodeText"));
        assert!(source.contains("build_code_layout"));
        assert!(source.contains("set_selectable_layout_state"));
        assert!(source.contains("fn id(&self) -> Option<ElementId>"));
        assert!(source.contains("fn font_size(&self) -> Pixels"));
        assert!(source.contains("theme.font_size.md"));
        assert!(source.contains("cached_highlight_runs"));
        assert!(source.contains("HighlightCacheKey"));
        assert!(source.contains("CodeHighlighter::Syntect"));
        assert!(source.contains("CodeTheme::Auto"));
        assert!(source.contains("light_theme"));
        assert!(source.contains("dark_theme"));
        assert!(source.contains("github_dark_theme"));
        assert!(source.contains("two_face::syntax::extra_newlines"));
        assert!(source.contains("StyledText::new"));
        assert!(source.contains("with_runs"));
    }
}
