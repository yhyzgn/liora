//! Real native keyboard/pointer, UTF-8 completion, search, IME and history assertions.
//! Run: cargo run -p liora-gallery --example editor_interaction_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        AppContext, Bounds, EntityInputHandler, Focusable, Keystroke, MouseButton, MouseDownEvent,
        MouseUpEvent, PlatformInput, WindowOptions, point, px,
    };
    use liora_components::{
        CodeCompletionEdit, CodeCompletionItem, CodeDiagnostic, CodeEditor, CodeEditorFindNext,
        CodeEditorOptions, init_liora,
    };
    use std::{cell::Cell, rc::Rc};
    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let unhandled_searches = Rc::new(Cell::new(0));
        cx.on_action({
            let unhandled_searches = unhandled_searches.clone();
            move |_: &CodeEditorFindNext, _| unhandled_searches.set(unhandled_searches.get() + 1)
        });
        let editor = cx.new(|cx| CodeEditor::new("pr", cx).header(false).status_bar(false)
            .cursor_blink(false).height(px(100.)).completions([
                CodeCompletionItem::new("print"), CodeCompletionItem::new("println!")
            ]));
        let handle = cx.open_window(WindowOptions { show: false, ..Default::default() }, |_, _| editor.clone()).unwrap();
        cx.update_window(handle.into(), |_, window, cx| {
            window.focus(&editor.read(cx).focus_handle(cx), cx);
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("down").unwrap(), cx);
            window.dispatch_keystroke(Keystroke::parse("tab").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "println!");
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "pr");
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-shift-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "println!");
            window.draw(cx).clear(cx);
            let before_delete = editor.read(cx).selected_range();
            window.dispatch_keystroke(Keystroke::parse("backspace").unwrap(), cx);
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "println!");
            assert_eq!(editor.read(cx).selected_range(), before_delete, "undo restores caret, not deletion selection");
            editor.update(cx, |editor, cx| {
                editor.set_value("pr", cx);
                let token = editor.request_token();
                editor.set_completions_for_request(&token, [CodeCompletionItem::new("print")], cx);
            });
            window.draw(cx).clear(cx);
            let _ = window.dispatch_keystroke(Keystroke::parse("escape").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "pr");
            editor.update(cx, |editor, cx| {
                assert!(!editor.accept_completion(0, cx));
                let token = editor.request_token();
                editor.set_completions_for_request(&token, [CodeCompletionItem::new("print")], cx);
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "print", "Enter accepts instead of inserting newline");
            editor.update(cx, |editor, cx| {
                editor.set_value("a😀 pr", cx);
                let token = editor.request_token();
                assert!(!editor.set_completion_edits_for_request(&token, [CodeCompletionEdit::new(
                    CodeCompletionItem::new("invalid"), 2..3, "x")], cx));
                assert!(editor.set_completion_edits_for_request(&token, [CodeCompletionEdit::new(
                    CodeCompletionItem::new("Replace emoji"), 1..5, "世界")], cx));
                assert!(editor.accept_completion(0, cx));
                assert_eq!(editor.value(cx).as_ref(), "a世界 pr");
                assert_eq!(editor.selected_range(), 7..7);
                assert!(!editor.set_completions_for_request(&token, [CodeCompletionItem::new("stale")], cx));
                let token = editor.request_token();
                editor.set_selection(0..0, cx);
                assert!(!editor.set_completions_for_request(&token, [CodeCompletionItem::new("moved")], cx));
                editor.set_search_query("", cx);
                assert!(editor.search_matches().is_empty());
                assert_eq!(editor.replace_all_matches("x", cx), 0);
                editor.set_value("γ γ γ", cx);
                editor.set_selection(0..0, cx);
                editor.set_search_query("γ", cx);
                assert_eq!(editor.find_next(cx), Some(0..2));
                assert_eq!(editor.find_next(cx), Some(3..5));
                assert_eq!(editor.find_previous(cx), Some(0..2));
                assert_eq!(editor.find_previous(cx), Some(6..8), "previous wraps at first match");
                assert_eq!(editor.find_next(cx), Some(0..2), "next wraps at final match");
                assert!(editor.replace_current_match("😀", cx));
                assert_eq!(editor.value(cx).as_ref(), "😀 γ γ");
                assert_eq!(editor.replace_all_matches("世界", cx), 2);
                assert_eq!(editor.value(cx).as_ref(), "😀 世界 世界");
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "😀 γ γ", "replace-all is one undo step");
            editor.update(cx, |editor, cx| {
                editor.set_read_only(true, cx);
                assert_eq!(editor.replace_all_matches("x", cx), 0);
                let token = editor.request_token();
                editor.set_completions_for_request(&token, [CodeCompletionItem::new("x")], cx);
                assert!(!editor.accept_completion(0, cx));
                editor.set_read_only(false, cx);
                editor.set_value("pr", cx);
                editor.replace_and_mark_text_in_range(Some(0..2), "拼", Some(1..1), window, cx);
                editor.replace_and_mark_text_in_range(None, "拼", None, window, cx);
                assert_eq!(editor.marked_text_range(window, cx), Some(0..1));
                let token = editor.request_token();
                assert!(!editor.set_completions_for_request(&token, [CodeCompletionItem::new("wrong")], cx));
                assert!(!editor.accept_completion(0, cx));
                editor.replace_text_in_range(None, "拼音", window, cx);
                assert_eq!(editor.value(cx).as_ref(), "拼音");
                editor.set_value("pr", cx);
                let token = editor.request_token();
                assert!(editor.set_completions_for_request(&token, [CodeCompletionItem::new("print")], cx));
            });
            window.draw(cx).clear(cx);
            // Header/status are disabled; the first candidate follows the 100px editor and panel title.
            window.dispatch_event(PlatformInput::MouseDown(MouseDownEvent {
                button: MouseButton::Left, position: point(px(40.), px(145.)), click_count: 1,
                ..Default::default()
            }), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "print", "pointer accepts candidate text edit");
            // Fallback word range extends right of the caret: mid-word accept replaces the whole word.
            editor.update(cx, |editor, cx| {
                editor.set_value("prnt", cx);
                editor.set_selection(2..2, cx);
                let token = editor.request_token();
                editor.set_completions_for_request(&token, [CodeCompletionItem::new("print")], cx);
                assert!(editor.accept_completion(0, cx));
                assert_eq!(editor.value(cx).as_ref(), "print", "mid-word accept replaces the full word");
            });
            editor.update(cx, |editor, cx| {
                editor.set_value("target x target", cx);
                editor.set_selection(0..0, cx);
                editor.set_search_query("target", cx);
            });
            window.dispatch_keystroke(Keystroke::parse("f3").unwrap(), cx);
            assert_eq!(editor.read(cx).selected_range(), 0..6);
            window.dispatch_keystroke(Keystroke::parse("f3").unwrap(), cx);
            assert_eq!(editor.read(cx).selected_range(), 9..15);
            window.dispatch_keystroke(Keystroke::parse("shift-f3").unwrap(), cx);
            assert_eq!(editor.read(cx).selected_range(), 0..6);
            assert_eq!(unhandled_searches.get(), 0, "matched searches stay inside the editor");
            editor.update(cx, |editor, cx| editor.set_search_query("", cx));
            window.dispatch_keystroke(Keystroke::parse("f3").unwrap(), cx);
            assert_eq!(unhandled_searches.get(), 1, "unhandled searches reach the host");

            let options = CodeEditorOptions {
                header: false, status_bar: false, cursor_blink: false,
                ..Default::default()
            };
            editor.update(cx, |editor, cx| {
                editor.dismiss_completions(cx);
                editor.set_options(options, cx);
                editor.set_value("ab", cx);
                editor.replace_text_in_range(None, "c", window, cx);
                editor.set_options(CodeEditorOptions { history: false, ..options }, cx);
                editor.replace_text_in_range(Some(0..0), "X", window, cx);
                editor.set_options(options, cx);
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "Xabc", "unrecorded text invalidates delta undo");
            window.dispatch_keystroke(Keystroke::parse("ctrl-shift-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "Xabc");

            // A real redo must survive both completion and replacement no-ops.
            let changes = Rc::new(Cell::new(0));
            editor.update(cx, |editor, cx| {
                editor.set_value("print", cx);
                editor.set_on_change({
                    let changes = changes.clone();
                    move |_, _| changes.set(changes.get() + 1)
                }, cx);
                editor.replace_text_in_range(None, "!", window, cx);
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            let before_noop = changes.get();
            editor.update(cx, |editor, cx| {
                editor.set_selection(2..2, cx);
                let revision = editor.request_token().revision();
                let token = editor.request_token();
                editor.set_completions_for_request(&token, [CodeCompletionItem::new("print")], cx);
                assert!(editor.accept_completion(0, cx));
                assert_eq!(editor.selected_range(), 5..5);
                assert_eq!(editor.request_token().revision(), revision);
                editor.set_search_query("print", cx);
                editor.set_selection(0..5, cx);
                assert!(editor.replace_current_match("print", cx));
                assert_eq!(editor.replace_all_matches("print", cx), 1);
                assert_eq!(editor.selected_range(), 5..5);
                assert_eq!(editor.request_token().revision(), revision);
            });
            assert_eq!(changes.get(), before_noop, "selection-only transactions do not emit text changes");
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-shift-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "print!", "no-op transactions preserve redo");

            // Selection-only work while history is disabled must preserve history too.
            editor.update(cx, |editor, cx| {
                editor.set_options(CodeEditorOptions { history: false, ..options }, cx);
                editor.set_selection(0..0, cx);
                editor.set_options(options, cx);
            });
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "print");
            editor.update(cx, |editor, cx| {
                editor.set_options(CodeEditorOptions { history: false, ..options }, cx);
                assert_eq!(editor.replace_all_matches("print", cx), 1);
                editor.set_options(options, cx);
            });
            window.dispatch_keystroke(Keystroke::parse("ctrl-shift-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "print!", "disabled-history no-op preserves redo");
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            editor.update(cx, |editor, cx| {
                editor.set_options(CodeEditorOptions { history: false, ..options }, cx);
                editor.replace_text_in_range(Some(0..0), "X", window, cx);
                editor.set_options(options, cx);
            });
            window.dispatch_keystroke(Keystroke::parse("ctrl-shift-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "Xprint", "unrecorded mutation also clears redo");

            for (selection, expected) in [(2..2, 2..2), (4..4, 2..2), (0..7, 0..3)] {
                editor.update(cx, |editor, cx| {
                    editor.set_value("a\n    b", cx);
                    editor.set_selection(selection.clone(), cx);
                });
                window.draw(cx).clear(cx);
                window.dispatch_keystroke(Keystroke::parse("shift-tab").unwrap(), cx);
                assert_eq!(editor.read(cx).value(cx).as_ref(), "a\nb");
                assert_eq!(editor.read(cx).selected_range(), expected);
                window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
                assert_eq!(editor.read(cx).value(cx).as_ref(), "a\n    b");
                assert_eq!(editor.read(cx).selected_range(), selection);
            }
            editor.update(cx, |editor, cx| editor.set_value("    α\n    β\n    γ", cx));
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-shift-home").unwrap(), cx);
            window.dispatch_keystroke(Keystroke::parse("shift-tab").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "α\nβ\nγ");
            editor.update(cx, |editor, cx| {
                let selection = editor.selected_text_range(false, window, cx).unwrap();
                assert_eq!(selection.range, 0..5);
                assert!(selection.reversed, "outdent preserves selection direction");
            });
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "    α\n    β\n    γ");
            window.dispatch_keystroke(Keystroke::parse("ctrl-shift-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "α\nβ\nγ");
            window.dispatch_keystroke(Keystroke::parse("tab").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "    α\n    β\n    γ");
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "α\nβ\nγ");
            window.dispatch_keystroke(Keystroke::parse("ctrl-shift-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "    α\n    β\n    γ");
            editor.update(cx, |editor, cx| {
                editor.set_search_query("    ", cx);
                assert_eq!(editor.replace_all_matches("世\n界", cx), 3);
                assert_eq!(editor.value(cx).as_ref(), "世\n界α\n世\n界β\n世\n界γ");
            });
            window.dispatch_keystroke(Keystroke::parse("ctrl-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "    α\n    β\n    γ");
            window.dispatch_keystroke(Keystroke::parse("ctrl-shift-z").unwrap(), cx);
            assert_eq!(editor.read(cx).value(cx).as_ref(), "世\n界α\n世\n界β\n世\n界γ");

            let source: String = (0..240).map(|row| format!("row{row:05}\n")).collect();
            editor.update(cx, |editor, cx| {
                editor.set_value(source.clone(), cx);
                editor.set_selection(0..0, cx);
                editor.set_search_query("", cx);
            });
            window.draw(cx).clear(cx);
            for row in [60, 120, 180] {
                let offset = row * 9;
                editor.update(cx, |editor, cx| editor.set_selection(offset..offset, cx));
                window.draw(cx).clear(cx);
                let bounds = editor.update(cx, |editor, cx| {
                    assert!(editor.bounds_for_range(0..0, Bounds::default(), window, cx).is_none(),
                        "offscreen rows cannot supply stale IME bounds");
                    editor.bounds_for_range(offset..offset, Bounds::default(), window, cx).unwrap()
                });
                let position = point(bounds.left(), bounds.top() + bounds.size.height / 2.);
                window.dispatch_event(PlatformInput::MouseDown(MouseDownEvent {
                    button: MouseButton::Left, position, click_count: 1, ..Default::default()
                }), cx);
                window.dispatch_event(PlatformInput::MouseUp(MouseUpEvent {
                    button: MouseButton::Left, position, ..Default::default()
                }), cx);
                assert_eq!(editor.read(cx).cursor_point().row, row,
                    "clicking a freshly scrolled row must not hit a stale row");
            }
            let insertion = editor.read(cx).selected_range().start;
            editor.update(cx, |editor, cx| editor.replace_text_in_range(None, "X", window, cx));
            let mut expected = source;
            expected.insert(insertion, 'X');
            assert_eq!(editor.read(cx).value(cx).as_ref(), expected);
        }).unwrap();
        let diagnostic_calls = Rc::new(Cell::new(0));
        let read_only = cx.new(|cx| CodeEditor::new("initial", cx).read_only(true)
            .diagnostics_provider({
                let calls = diagnostic_calls.clone();
                move |text| {
                    assert_eq!(text, "initial");
                    calls.set(calls.get() + 1);
                    vec![CodeDiagnostic::warning(1, 1, "initial diagnostic")]
                }
            }));
        assert_eq!(diagnostic_calls.get(), 1, "builder evaluates initial read-only diagnostics");
        let diagnostic_window = cx.open_window(WindowOptions { show: false, ..Default::default() },
            |_, _| read_only.clone()).unwrap();
        cx.update_window(diagnostic_window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            window.draw(cx).clear(cx);
        }).unwrap();
        assert_eq!(diagnostic_calls.get(), 1, "drawing does not rerun diagnostics providers");
        println!("PASS: editor keyboard/pointer completion, UTF-8, stale results, search/history, read-only and IME");
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(std::time::Duration::from_millis(10)).await;
            cx.update(|cx| cx.quit());
        }).detach();
    });
}
#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn main() {
    eprintln!("Requires the official Linux/FreeBSD headless backend");
    std::process::exit(1);
}
