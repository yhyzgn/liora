//! Unified native Markdown composition, source fidelity, clipboard and history acceptance.
//! Set LIORA_NATIVE_SMOKE=1 for an isolated real display; otherwise Linux headless.
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
            .flex()
            .flex_col()
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
        } else { liora_core::ThemeMode::Light };
        init_liora_with_mode(cx, mode);
        let original = "- **old**\n\n> quoted\n";
        let editor = cx.new(|cx| MarkdownEditor::new(original, cx));
        let window = cx.open_window(WindowOptions {
            titlebar: Some(gpui::TitlebarOptions { title: Some("Liora Markdown Visibility Smoke".into()), ..Default::default() }),
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(0.), px(0.)), size(px(960.), px(760.))))),
            show: native, focus: native, ..Default::default()
        }, |_, cx| cx.new(|_| Surface { editor: editor.clone(), preview: None })).unwrap();
        cx.update_window(window.into(), |_, window, cx| {
            editor.update(cx, |editor, cx| {
                editor.focus_source_offset(4, window, cx);
                editor.set_selection(4..7, cx);
            });
            window.draw(cx).clear(cx);
            editor.update(cx, |editor, cx| {
                editor.replace_and_mark_text_in_range(Some(4..7), "候选", None, window, cx);
                assert!(editor.is_composing(cx));
                assert!(!editor.undo(cx));
                editor.set_mode(MarkdownEditMode::Source, cx);
                assert_eq!(editor.mode(), MarkdownEditMode::Rich);
            });
            window.draw(cx).clear(cx);
        }).unwrap();
        assert_eq!(editor.read(cx).source(), original, "composition must not publish source");
        cx.update_window(window.into(), |_, window, cx| {
            editor.update(cx, |editor, cx| editor.unmark_text(window, cx));
        }).unwrap();
        assert_eq!(editor.read(cx).source(), "- **候选**\n\n> quoted\n");
        editor.update(cx, |editor, cx| {
            editor.begin_undo_group();
            editor.insert_text("!", cx);
            editor.insert_text("?", cx);
            editor.end_undo_group();
        });
        assert_eq!(editor.read(cx).source(), "- **候选!?**\n\n> quoted\n");
        editor.update(cx, |editor, cx| {
            assert!(editor.undo(cx));
            assert_eq!(editor.source(), "- **候选**\n\n> quoted\n");
            assert!(editor.undo(cx));
            assert_eq!(editor.source(), original);
            assert!(editor.redo(cx));
        });
        if native {
            let previous = cx.read_from_clipboard();
            cx.update_window(window.into(), |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.focus_source_offset(4, window, cx);
                    editor.set_selection(4..4 + "候选".len(), cx);
                });
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
        let cell = editor.update(cx, |editor, cx| editor.table_input_at(table_source.find("old").unwrap(), cx).unwrap());
        cx.update_window(window.into(), |_, window, cx| {
            window.focus(&cell.focus_handle(cx), cx);
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
        let retained = editor.update(cx, |editor, cx| editor.table_input_at(editor.source().find("表格").unwrap(), cx).unwrap());
        assert_eq!(retained.entity_id(), cell.entity_id(), "table edits retain composition/focus owner");
        cx.update_window(window.into(), |_, window, cx| {
            editor.update(cx, |editor, cx| {
                editor.set_source("hello world\n\nlast\n", cx);
                editor.focus_source_offset(5, window, cx);
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
            assert_eq!(editor.read(cx).source(), "hello\n\n world\n\nlast\n");
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("backspace").unwrap(), cx);
            assert_eq!(editor.read(cx).source(), "hello world\n\nlast\n");
            editor.update(cx, |editor, cx| editor.set_selection(0..5, cx));
            window.dispatch_keystroke(Keystroke::parse("ctrl-b").unwrap(), cx);
            assert_eq!(editor.read(cx).source(), "**hello** world\n\nlast\n");
            editor.update(cx, |editor, cx| {
                editor.set_source("first\n\nsecond\n", cx);
                editor.set_selection(2..10, cx);
                editor.insert_text("猫", cx);
            });
            assert_eq!(editor.read(cx).source(), "fi猫ond\n", "replacement spans paragraphs without dropping untouched bytes");
            editor.update(cx, |editor, cx| {
                editor.set_source("/heading\n", cx);
                editor.focus_source_offset(8, window, cx);
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
            assert!(editor.read(cx).source().starts_with("# "), "filtered slash Enter inserts heading");
        }).unwrap();
        let typed = "# Live Markdown\n\nType Markdown directly: **bold**, *emphasis*, ~~strike~~ and `code`.\n\n- A list item\n- [ ] A task\n\n```rust\nlet live = true;\n```";
        cx.update_window(window.into(), |_, window, cx| {
            editor.update(cx, |editor, cx| {
                editor.set_source("", cx);
                editor.focus_source_offset(0, window, cx);
            });
            for ch in typed.chars() {
                editor.update(cx, |editor, cx| editor.replace_text_in_range(None, &ch.to_string(), window, cx));
                window.draw(cx).clear(cx);
            }
            assert_eq!(editor.read(cx).source(), typed, "native typing renders Markdown without formatting controls");
        }).unwrap();
        if std::env::var_os("LIORA_MARKDOWN_PREVIEW").is_some() {
            let preview = cx.new(|cx| MarkdownView::new(editor.read(cx).source().to_owned(), cx));
            window.update(cx, |surface, _, cx| { surface.preview = Some(preview); cx.notify(); }).unwrap();
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx)).unwrap();
        }
        println!("PASS: unified Rich composition, table IME, source fidelity, history and consecutive edits; native clipboard={native}");
        println!("READY: native Markdown editing smoke");
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(if native { 10_000 } else { 10 })).await;
            cx.update(|cx| cx.quit());
        }).detach();
    });
}
