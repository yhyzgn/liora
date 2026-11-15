//! Native Markdown Rich editing, composition, history and clipboard regression.
//! Set LIORA_NATIVE_SMOKE=1 for a real display; otherwise uses Linux headless.
use gpui::{
    AppContext, Bounds, ClipboardItem, Context, Entity, EntityInputHandler, Focusable, IntoElement,
    Keystroke, Render, Window, WindowBounds, WindowOptions, point, prelude::*, px, size,
};
use liora_components::{MarkdownEditMode, MarkdownEditor, MarkdownView, init_liora_with_mode};
use std::time::Duration;

struct Surface {
    editor: Entity<MarkdownEditor>,
    preview: Option<Entity<MarkdownView>>,
}

impl Render for Surface {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;
        gpui::div()
            .id("markdown-editing-smoke")
            .size_full()
            .overflow_y_scroll()
            .p_3()
            .bg(theme.neutral.body)
            .text_color(theme.neutral.text_1)
            .child(match &self.preview {
                Some(preview) => preview.clone().into_any_element(),
                None => self.editor.clone().into_any_element(),
            })
    }
}

fn main() {
    let native = std::env::var_os("LIORA_NATIVE_SMOKE").is_some();
    let application = if native {
        gpui_platform::application()
    } else {
        gpui_platform::headless()
    };
    application.with_assets(liora_icons::IconAssetSource).run(move |cx| {
        let mode = if std::env::var_os("LIORA_SMOKE_DARK").is_some() {
            liora_core::ThemeMode::Dark
        } else {
            liora_core::ThemeMode::Light
        };
        init_liora_with_mode(cx, mode);
        let original = "- **old**\n\n> quoted\n";
        let editor = cx.new(|cx| MarkdownEditor::new(original, cx));
        let window = cx.open_window(WindowOptions {
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Liora Markdown Visibility Smoke".into()),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                point(px(0.), px(0.)), size(px(960.), px(760.)),
            ))),
            show: native,
            focus: native,
            ..Default::default()
        }, |_, cx| cx.new(|_| Surface { editor: editor.clone(), preview: None })).unwrap();
        let input = editor.update(cx, |editor, cx| editor.rich_input_at(original.find("old").unwrap(), cx).unwrap());
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            input.update(cx, |input, cx| {
                input.replace_and_mark_text_in_range(Some(0..3), "候选", None, window, cx);
            });
        }).unwrap();
        assert_eq!(editor.read(cx).source(), original);
        editor.update(cx, |editor, cx| {
            assert!(!editor.undo(cx), "history must not cancel composition");
            editor.set_mode(MarkdownEditMode::Source, cx);
            assert_eq!(editor.mode(), MarkdownEditMode::Rich);
        });
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(input.read(cx).value().as_ref(), "候选");
            assert!(input.read(cx).is_composing());
            input.update(cx, |input, cx| input.unmark_text(window, cx));
        }).unwrap();
        assert_eq!(editor.read(cx).source(), "- **候选**\n\n> quoted\n");
        // No intervening redraw: the next edit must use the new canonical parse.
        input.update(cx, |input, cx| input.insert_text("!", cx));
        assert_eq!(editor.read(cx).source(), "- **候选!**\n\n> quoted\n");
        editor.update(cx, |editor, cx| {
            assert!(editor.undo(cx));
            assert_eq!(editor.source(), "- **候选**\n\n> quoted\n");
            assert!(editor.undo(cx));
            assert_eq!(editor.source(), original);
            assert!(editor.redo(cx));
            assert_eq!(editor.source(), "- **候选**\n\n> quoted\n");
        });
        editor.update(cx, |editor, cx| assert!(editor.undo(cx)));
        input.update(cx, |input, cx| input.set_selection(0..0, cx));
        assert_eq!(editor.read(cx).source(), original, "selection after undo must not resubmit stale text");
        editor.update(cx, |editor, cx| assert!(editor.redo(cx)));
        if native {
            let previous = cx.read_from_clipboard();
            cx.update_window(window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                let input = editor.update(cx, |editor, cx| editor.rich_input_at(4, cx).unwrap());
                window.focus(&input.focus_handle(cx), cx);
                input.update(cx, |input, cx| input.set_selection(0.."候选".len(), cx));
                window.draw(cx).clear(cx);
                window.dispatch_keystroke(Keystroke::parse("ctrl-c").unwrap(), cx);
                assert_eq!(cx.read_from_clipboard().and_then(|item| item.text()).as_deref(), Some("候选"));
                cx.write_to_clipboard(ClipboardItem::new_string("粘贴".to_owned()));
                window.dispatch_keystroke(Keystroke::parse("ctrl-v").unwrap(), cx);
            }).unwrap();
            assert_eq!(editor.read(cx).source(), "- **粘贴**\n\n> quoted\n");
            cx.write_to_clipboard(previous.unwrap_or(ClipboardItem { entries: Vec::new() }));
        }
        let table_source = "| A | B |\n| --- | --- |\n| old | 2 |\n";
        editor.update(cx, |editor, cx| editor.set_source(table_source, cx));
        let cell = editor.update(cx, |editor, cx| editor.rich_input_at(table_source.find("old").unwrap(), cx).unwrap());
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            cell.update(cx, |input, cx| input.replace_and_mark_text_in_range(Some(0..3), "表格", None, window, cx));
        }).unwrap();
        assert_eq!(editor.read(cx).source(), table_source);
        editor.update(cx, |editor, cx| assert!(!editor.undo(cx)));
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            assert!(cell.read(cx).is_composing());
            cell.update(cx, |input, cx| input.unmark_text(window, cx));
        }).unwrap();
        assert_eq!(editor.read(cx).source(), "| A | B |\n| --- | --- |\n| 表格 | 2 |\n");
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx)).unwrap();
        let retained = editor.update(cx, |editor, cx| editor.rich_input_at(editor.source().find("表格").unwrap(), cx).unwrap());
        assert_eq!(retained.entity_id(), cell.entity_id(), "table reconciliation must retain focus/selection entity");
        editor.update(cx, |editor, cx| {
            editor.set_source("# Native Markdown\n\nText with $\\frac{1}{2}$ inline, ^superscript^ and ~subscript~.\n\n```math\n\\sum_{i=1}^{n} i\n```\n\n```mermaid\nflowchart LR\nA[Native] --> B[GPUI]\n```\n\n| Name | Value |\n| :--- | ---: |\n| alpha | 42 |\n", cx);
        });
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx)).unwrap();
        if std::env::var_os("LIORA_MARKDOWN_PREVIEW").is_some() {
            let source = editor.read(cx).source().to_owned();
            let preview = cx.new(|cx| MarkdownView::new(source, cx));
            window.update(cx, |surface, _, cx| {
                surface.preview = Some(preview);
                cx.notify();
            }).unwrap();
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx)).unwrap();
        }
        println!("PASS: Rich nested and table IME commit, redraw preservation, history and consecutive edits; native clipboard={native}");
        println!("READY: native Markdown editing smoke");
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(if native { 10_000 } else { 10 })).await;
            cx.update(|cx| cx.quit());
        }).detach();
    });
}
