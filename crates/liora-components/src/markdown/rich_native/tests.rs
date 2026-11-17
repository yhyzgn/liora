use super::*;
use gpui::{Bounds, EntityInputHandler, TestAppContext, point, size};
use std::{cell::RefCell, sync::Arc};

struct Surface(Entity<MarkdownEditor>);
impl Render for Surface {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().w_full().h_full().child(self.0.clone())
    }
}

#[test]
fn native_composition_keeps_committed_source_events_and_one_undo() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("**cat**\n", cx));
    let events = Rc::new(RefCell::new(Vec::new()));
    let capture = events.clone();
    let subscription = app.update(|cx| {
        cx.subscribe(&editor, move |_, event: &MarkdownEditorChanged, _| {
            capture.borrow_mut().push(event.source.clone())
        })
    });
    let window = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(window.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| {
            editor.set_selection(2..5, cx);
            editor.replace_and_mark_text_in_range(None, "候", None, window, cx);
            editor.replace_and_mark_text_in_range(None, "候选", Some(2..2), window, cx);
            assert_eq!(editor.source(), "**cat**\n");
            assert!(editor.is_composing(cx));
            assert!(!editor.undo(cx));
            editor.set_mode(MarkdownEditMode::Source, cx);
            assert_eq!(editor.mode(), MarkdownEditMode::Rich);
            assert!(events.borrow().is_empty());
            editor.replace_text_in_range(None, "猫", window, cx);
            assert_eq!(editor.source(), "**猫**\n");
            assert!(!editor.is_composing(cx));
            assert_eq!(
                events.borrow().len(),
                0,
                "events dispatch after entity update"
            );
        });
    })
    .unwrap();
    app.run_until_parked();
    assert_eq!(events.borrow().len(), 1);
    app.update(|cx| {
        editor.update(cx, |editor, cx| {
            assert!(editor.undo(cx));
            assert_eq!(editor.source(), "**cat**\n");
            assert_eq!(editor.selection(), 2..5);
        })
    });
    drop(subscription);
    app.quit();
}

#[test]
fn styled_wrapped_text_shares_pointer_and_ime_geometry() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = format!(
        "**{}**\n",
        "Native 猫 text wrapping smoothly across visual lines. ".repeat(10)
    );
    let editor = app.new(|cx| MarkdownEditor::new(source.clone(), cx));
    let window = app.open_window(size(px(360.), px(900.)), |_, _| Surface(editor.clone()));
    app.update_window(window.into(), |_, window, cx| {
        window.focus(&editor.focus_handle(cx), cx);
        window.draw(cx).clear(cx);
        editor.update(cx, |editor, cx| {
            let layout = editor
                .layouts
                .values()
                .next()
                .expect("native styled layout")
                .clone();
            assert!(layout.bounds.size.height > layout.line_height * 2.);
            let point = point(
                layout.bounds.left() + px(35.),
                layout.bounds.top() + layout.line_height * 1.5,
            );
            let offset = editor.index_for_point(point).unwrap();
            assert!(source.is_char_boundary(offset));
            editor.set_selection(offset..offset, cx);
            let utf16 = source[..offset].encode_utf16().count();
            let bounds = editor
                .bounds_for_range(utf16..utf16, Bounds::default(), window, cx)
                .unwrap();
            let roundtrip = editor
                .character_index_for_point(
                    gpui::point(bounds.left(), bounds.top() + layout.line_height * 0.5),
                    window,
                    cx,
                )
                .unwrap();
            assert_eq!(roundtrip, utf16);
            editor.insert_text("X", cx);
            assert!(editor.source().starts_with("**"));
            assert!(editor.source().ends_with("**\n"));
        });
    })
    .unwrap();
    app.quit();
}

#[test]
fn native_virtual_rows_keep_scroll_state_and_only_expose_cell_inputs() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = format!(
        "{}\n| A | B |\n| --- | --- |\n| one | two |\n",
        (0..80)
            .map(|index| format!("Paragraph {index}.\n\n"))
            .collect::<String>()
    );
    let editor = app.new(|cx| MarkdownEditor::new(source.clone(), cx));
    app.update(|cx| {
        editor.update(cx, |editor, cx| {
            assert!(editor.table_input_at(2, cx).is_none());
            assert!(
                editor
                    .table_input_at(source.find("one").unwrap(), cx)
                    .is_some()
            );
            assert!(editor.scroll_handle().item_count() > 80);
            editor.scroll_handle().scroll_to(gpui::ListOffset {
                item_ix: 40,
                offset_in_item: px(0.),
            });
            editor.set_selection(
                source.find("Paragraph 40").unwrap() + 4..source.find("Paragraph 40").unwrap() + 4,
                cx,
            );
            editor.insert_text("猫", cx);
            assert!(editor.scroll_handle().logical_scroll_top().item_ix >= 39);
        })
    });
    app.quit();
}

#[test]
fn prose_edits_reuse_unchanged_formula_images() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("Original paragraph.\n\n$x$\n", cx));
    app.update(|cx| {
        editor.update(cx, |editor, cx| {
            let cache = editor.assets.clone();
            let kind = assets::AssetKind::Math(false, [0, 0, 0, 255]);
            let before = cache.render(kind, "x").unwrap();
            editor.set_source("Edited paragraph.\n\n$x$\n", cx);
            let after = cache.render(kind, "x").unwrap();
            assert!(Arc::ptr_eq(&before, &after));
        })
    });
    app.quit();
}

#[test]
fn native_slash_search_and_cross_block_replacement_use_canonical_selection() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("/heading\n", cx));
    app.update(|cx| {
        editor.update(cx, |editor, cx| {
            editor.set_selection(8..8, cx);
            assert_eq!(editor.slash_choices()[0].1, "heading1");
            editor.apply_slash(0, cx);
            assert!(editor.source().starts_with("# "));
            assert!(editor.undo(cx));
            assert_eq!(editor.source(), "/heading\n");
            editor.set_source("first\n\nsecond\n", cx);
            editor.set_selection(2..10, cx);
            editor.insert_text("猫", cx);
            assert_eq!(editor.source(), "fi猫ond\n");
            editor
                .find_input
                .update(cx, |input, cx| input.set_value("猫", cx));
            editor.find_next(false, cx);
            assert_eq!(editor.engine.selected_text(), "猫");
            editor.set_source("", cx);
            editor.set_selection(0..0, cx);
            editor.engine.type_markdown("# ");
            editor.publish(cx);
            assert!(editor.source().starts_with("# "));
            editor.set_selection(2..2, cx);
            editor.set_mode(MarkdownEditMode::Source, cx);
            assert_eq!(editor.input.read(cx).selected_range(), 2..2);
            editor
                .input
                .update(cx, |input, cx| input.set_selection(0..0, cx));
            editor.set_mode(MarkdownEditMode::Rich, cx);
            assert_eq!(editor.selection(), 0..0);
        })
    });
    app.quit();
}

#[test]
fn empty_xim_preedit_does_not_reopen_history_and_syntax_offsets_are_canonical() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| {
            editor.replace_and_mark_text_in_range(None, "*猫", Some(2..2), window, cx);
            assert_eq!(editor.engine.source(), "*猫");
            assert_eq!(editor.selection(), 4..4);
            editor.replace_text_in_range(None, "你好", window, cx);
            editor.replace_and_mark_text_in_range(None, "", None, window, cx);
            assert!(!editor.is_composing(cx));
            assert!(editor.undo(cx));
            assert_eq!(editor.source(), "");
            editor.replace_and_mark_text_in_range(None, "候", None, window, cx);
            editor.replace_and_mark_text_in_range(None, "", None, window, cx);
            assert!(!editor.is_composing(cx));
        })
    })
    .unwrap();
    app.quit();
}

#[test]
fn tall_native_row_keeps_inner_scroll_on_click_and_edit() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| {
        MarkdownEditor::new(
            "A long native paragraph 猫 with styled content. ".repeat(400),
            cx,
        )
    });
    let handle = app.open_window(size(px(520.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        editor.read(cx).list.scroll_to(gpui::ListOffset {
            item_ix: 0,
            offset_in_item: px(500.),
        });
    })
    .unwrap();
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        editor.update(cx, |editor, cx| {
            let viewport = editor.list.viewport_bounds();
            let point = gpui::point(viewport.left() + px(60.), viewport.top() + px(40.));
            editor.mouse_down(
                &gpui::MouseDownEvent {
                    position: point,
                    button: gpui::MouseButton::Left,
                    click_count: 1,
                    ..Default::default()
                },
                window,
                cx,
            );
            assert_eq!(editor.list.logical_scroll_top().offset_in_item, px(500.));
            editor.insert_text("X", cx);
            assert_eq!(editor.list.logical_scroll_top().offset_in_item, px(500.));
        });
        window.draw(cx).clear(cx);
        assert!(editor.read(cx).list.logical_scroll_top().offset_in_item > px(0.));
    })
    .unwrap();
    app.quit();
}

#[test]
fn source_selection_history_and_viewport_follow_native_caret() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = "Long source line 猫\n\n".repeat(100);
    let editor = app.new(|cx| MarkdownEditor::new(source.clone(), cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| {
            editor.set_mode(MarkdownEditMode::Source, cx);
            editor.focus_source_offset(source.len(), window, cx);
        });
        window.draw(cx).clear(cx);
        assert!(editor.read(cx).source_scroll.offset().y < px(0.));
        editor.update(cx, |editor, cx| {
            editor.set_selection(0..4, cx);
            assert_eq!(editor.input.read(cx).selected_range(), 0..4);
            editor.insert_text("New", cx);
            assert!(editor.undo(cx));
            assert_eq!(editor.selection(), 0..4);
            assert_eq!(editor.input.read(cx).selected_range(), 0..4);
        });
    })
    .unwrap();
    app.quit();
}

#[test]
fn table_transaction_keeps_viewport_and_cell_focus_and_ancestor_history() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = format!(
        "{}\n| A |\n| --- |\n| cell |\n",
        (0..60)
            .map(|n| format!("Paragraph {n}\n\n"))
            .collect::<String>()
    );
    let editor = app.new(|cx| MarkdownEditor::new(source.clone(), cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| {
            let cell = editor
                .table_input_at(source.find("cell").unwrap(), cx)
                .unwrap();
            let table = editor.engine.document().tables()[0].id;
            editor.list.scroll_to(gpui::ListOffset {
                item_ix: 60,
                offset_in_item: px(12.),
            });
            window.focus(&cell.focus_handle(cx), cx);
            editor
                .edit_table(
                    table,
                    MarkdownTableEdit::SetCell {
                        row: 1,
                        column: 0,
                        text: "edited".into(),
                    },
                    cx,
                )
                .unwrap();
            assert_eq!(editor.list.logical_scroll_top().item_ix, 60);
            assert_eq!(editor.list.logical_scroll_top().offset_in_item, px(12.));
            assert!(cell.focus_handle(cx).is_focused(window));
            editor.history_key_down(
                &gpui::KeyDownEvent {
                    keystroke: gpui::Keystroke::parse("ctrl-z").unwrap(),
                    is_held: false,
                    prefer_character_input: false,
                },
                window,
                cx,
            );
            assert_eq!(editor.source(), source);
        });
    })
    .unwrap();
    app.quit();
}

#[test]
fn special_inline_gestures_share_formula_link_script_and_ime_geometry() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = "Before $x$ [link](https://example.com) ^up^ after\n";
    let editor = app.new(|cx| MarkdownEditor::new(source, cx));
    let handle = app.open_window(size(px(640.), px(800.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.layouts.len(),
                1,
                "one geometry for the paragraph, no editing companion"
            );
            let layout = editor.layouts.values().next().unwrap().clone();
            let block = &editor.engine.document().blocks[0];
            let link = block.text.find("link").unwrap();
            let point = layout.position(link + 1) + gpui::point(px(0.), layout.line_height * 0.5);
            editor.set_selection(0..0, cx);
            editor.mouse_down(
                &gpui::MouseDownEvent {
                    position: point,
                    button: gpui::MouseButton::Left,
                    click_count: 1,
                    modifiers: gpui::Modifiers {
                        shift: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                window,
                cx,
            );
            assert!(!editor.selection().is_empty());
            editor.mouse_down(
                &gpui::MouseDownEvent {
                    position: point,
                    button: gpui::MouseButton::Left,
                    click_count: 2,
                    ..Default::default()
                },
                window,
                cx,
            );
            assert_eq!(editor.engine.selected_text(), "link");
            editor.mouse_move(
                &gpui::MouseMoveEvent {
                    position: layout.position(layout.index(point) + 2),
                    pressed_button: Some(gpui::MouseButton::Left),
                    ..Default::default()
                },
                window,
                cx,
            );
            assert!(!editor.selection().is_empty());
            let caret = editor.engine.selection().head;
            let units = editor.engine.source()[..caret].encode_utf16().count();
            let bounds = editor
                .bounds_for_range(units..units, Bounds::default(), window, cx)
                .unwrap();
            assert_eq!(
                editor.character_index_for_point(
                    bounds.origin + gpui::point(px(0.), layout.line_height * 0.5),
                    window,
                    cx
                ),
                Some(units)
            );
        });
    })
    .unwrap();
    app.quit();
}

#[test]
fn previously_measured_offscreen_row_is_revealed_again() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = "Paragraph with native text.\n\n".repeat(100);
    let end = source.len() - 2;
    let editor = app.new(|cx| MarkdownEditor::new(source, cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        editor.update(cx, |editor, cx| editor.focus_source_offset(end, window, cx));
        window.draw(cx).clear(cx);
        editor.read(cx).list.scroll_to(gpui::ListOffset::default());
        window.draw(cx).clear(cx);
        editor.update(cx, |editor, cx| editor.focus_source_offset(end, window, cx));
        window.draw(cx).clear(cx);
        let editor = editor.read(cx);
        let (id, offset) = editor.engine.display_position(end).unwrap();
        let caret = editor.layouts[&id].range_bounds(offset..offset);
        let viewport = editor.list.viewport_bounds();
        assert!(caret.bottom() >= viewport.top() && caret.top() < viewport.bottom());
    })
    .unwrap();
    app.quit();
}

#[test]
fn special_newline_and_trailing_break_keep_caret_and_hit_geometry() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("first $x$\nsecond\n", cx));
    let handle = app.open_window(size(px(640.), px(800.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |_, cx| {
            let mut document = crate::MarkdownDocument::new("first $x$\nsecond\n");
            document.flush();
            let block = &mut document.blocks[0];
            block.text = "first x\nsecond\n".into();
            let mut style = super::super::model::Style::default();
            style.link = Some("https://example.com".into());
            block.spans = vec![super::super::model::Span {
                range: 0..block.text.len(),
                source_range: 0..block.text.len(),
                style,
            }];
            let element = special::SpecialElement::new(block, cx.entity());
            let layout = element.shape(px(600.), window, cx);
            let first_break = block.text.find('\n').unwrap();
            let last_break = block.text.len() - 1;
            assert_eq!(layout.position(first_break).y, px(0.));
            assert!(layout.position(first_break).x > px(0.));
            assert!(layout.position(last_break).y > layout.position(first_break).y);
            assert!(layout.position(block.text.len()).y > layout.position(last_break).y);
            for offset in [first_break, last_break, block.text.len()] {
                assert_eq!(layout.index(layout.position(offset)), offset);
            }
        });
    })
    .unwrap();
    app.quit();
}

#[test]
fn typing_markdown_syntax_renders_in_place_without_commands() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("", cx));
    let handle = app.open_window(size(px(640.), px(800.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.focus(&editor.focus_handle(cx), cx);
        for source in ["# Heading", "- item", "**bold**", "*italic*", "~~strike~~", "`code`", "[link](https://example.com)", "$x$", "- [ ] task", "```rust\nlet x = 1;\n```"] {
            editor.update(cx, |editor, cx| editor.set_source("", cx));
            for ch in source.chars() {
                if ch == '\n' {
                    window.dispatch_keystroke(gpui::Keystroke::parse("enter").unwrap(), cx);
                } else {
                    editor.update(cx, |editor, cx| editor.replace_text_in_range(None, &ch.to_string(), window, cx));
                }
                window.draw(cx).clear(cx);
            }
            editor.update(cx, |editor, _| {
                assert_eq!(editor.source(), source, "native typing retains syntax");
                let blocks = &editor.engine.document().blocks;
                match source {
                    "# Heading" => assert!(matches!(blocks[0].kind, Kind::Heading(1))),
                    "- item" | "- [ ] task" => assert!(matches!(blocks[0].kind, Kind::List(_))),
                    "**bold**" => assert!(blocks[0].spans.iter().any(|s| s.style.strong)),
                    "*italic*" => assert!(blocks[0].spans.iter().any(|s| s.style.emphasis)),
                    "~~strike~~" => assert!(blocks[0].spans.iter().any(|s| s.style.strike)),
                    "`code`" => assert!(blocks[0].spans.iter().any(|s| s.style.code)),
                    "$x$" => assert!(blocks[0].spans.iter().any(|s| s.style.math.is_some())),
                    "[link](https://example.com)" => assert!(blocks[0].spans.iter().any(|s| s.style.link.is_some())),
                    _ => assert!(matches!(&blocks[0].kind, Kind::Code(language) if language.as_ref() == "rust")),
                }
                assert!(editor.error.is_none(), "native typing must remain editable");
            });
        }
        editor.update(cx, |editor, cx| editor.set_source("", cx));
        for ch in "**bold**".chars() {
            editor.update(cx, |editor, cx| editor.replace_text_in_range(None, &ch.to_string(), window, cx));
        }
        window.dispatch_keystroke(gpui::Keystroke::parse("backspace").unwrap(), cx);
        assert_eq!(editor.read(cx).source(), "**bold*");
        editor.update(cx, |editor, cx| { assert!(editor.undo(cx)); });
        assert_eq!(editor.read(cx).source(), "**bold**");
    }).unwrap();
    app.quit();
}

#[test]
fn empty_code_fence_click_types_inside_the_content() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("", cx));
    let handle = app.open_window(size(px(640.), px(800.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        for (source, expected) in [
            ("```rust\n", "```rust\n猫\n"),
            ("```rust\n```", "```rust\n猫\n```"),
            ("~~~rust\r\n~~~", "~~~rust\r\n猫\n~~~"),
        ] {
            editor.update(cx, |editor, cx| editor.set_source(source, cx));
            window.draw(cx).clear(cx);
            editor.update(cx, |editor, cx| {
                let id = editor.engine.document().blocks[0].id;
                let layout = &editor.layouts[&id];
                let position = point(
                    layout.bounds.left() + px(1.),
                    layout.bounds.top() + layout.line_height * 0.5,
                );
                editor.mouse_down(
                    &gpui::MouseDownEvent {
                        position,
                        button: gpui::MouseButton::Left,
                        click_count: 1,
                        ..Default::default()
                    },
                    window,
                    cx,
                );
                let content = source.find('\n').unwrap() + 1;
                assert_eq!(editor.selection(), content..content, "{source:?}");
                editor.replace_text_in_range(None, "猫\n", window, cx);
                assert_eq!(editor.source(), expected);
                assert_eq!(editor.engine.document().blocks[0].text.as_ref(), "猫\n");
                assert!(editor.undo(cx));
                assert_eq!(editor.source(), source);
                assert_eq!(editor.selection(), content..content);
            });
        }
    })
    .unwrap();
    app.quit();
}

#[test]
fn backspace_at_inline_opening_marker_deletes_source_and_undo_restores_it() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("", cx));
    let handle = app.open_window(size(px(640.), px(800.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.focus(&editor.focus_handle(cx), cx);
        for (source, caret, expected) in [
            ("**bold**", 2, "*bold**"),
            ("*italic*", 1, "italic*"),
            ("~~strike~~", 2, "~strike~~"),
            ("`code`", 1, "code`"),
            ("猫 **bold**", 6, "猫 *bold**"),
        ] {
            editor.update(cx, |editor, cx| {
                editor.set_source(source, cx);
                editor.set_selection(caret..caret, cx);
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(gpui::Keystroke::parse("backspace").unwrap(), cx);
            editor.update(cx, |editor, cx| {
                assert_eq!(editor.source(), expected, "{source:?}");
                assert_eq!(editor.selection(), caret - 1..caret - 1);
                assert!(editor.undo(cx));
                assert_eq!(editor.source(), source);
                assert_eq!(editor.selection(), caret..caret);
            });
        }
    })
    .unwrap();
    app.quit();
}

#[test]
fn dispatched_mouse_and_keyboard_edit_the_same_document() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("hello world\n\nsecond paragraph", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        let position = {
            let editor = editor.read(cx);
            let id = editor.engine.document().blocks[0].id;
            let layout = &editor.layouts[&id];
            layout.position(5) + point(px(0.), layout.line_height * 0.5)
        };
        window.dispatch_event(
            gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                position,
                button: gpui::MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                position,
                button: gpui::MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
        assert!(
            editor.focus_handle(cx).is_focused(window),
            "click must focus editor"
        );
        assert_eq!(editor.read(cx).selection(), 5..5);
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("x").unwrap(), cx);
        assert_eq!(editor.read(cx).source(), "hellox world\n\nsecond paragraph");
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("left").unwrap(), cx);
        assert_eq!(editor.read(cx).selection(), 5..5);
        window.dispatch_keystroke(gpui::Keystroke::parse("shift-right").unwrap(), cx);
        assert_eq!(editor.read(cx).selection(), 5..6);
        window.dispatch_keystroke(gpui::Keystroke::parse("backspace").unwrap(), cx);
        assert_eq!(editor.read(cx).source(), "hello world\n\nsecond paragraph");
        window.dispatch_keystroke(gpui::Keystroke::parse("ctrl-z").unwrap(), cx);
        assert_eq!(editor.read(cx).source(), "hellox world\n\nsecond paragraph");
    })
    .unwrap();
    app.quit();
}

#[test]
fn dispatched_click_and_navigation_work_in_nested_document_text() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        for (source, text) in [
            ("- list item", "list item"),
            ("1. ordered item", "ordered item"),
            ("- [ ] task item", "task item"),
            ("> quoted text", "quoted text"),
            ("A $x$ formula", "A x formula"),
        ] {
            editor.update(cx, |editor, cx| editor.set_source(source, cx));
            window.draw(cx).clear(cx);
            let position = {
                let editor = editor.read(cx);
                let block = find_text_for_test(&editor.engine.document().blocks, text).unwrap();
                let layout = editor
                    .layouts
                    .get(&block.id)
                    .expect("every editable text row needs unified hit geometry");
                layout.position(2) + point(px(0.), layout.line_height * 0.5)
            };
            window.dispatch_event(
                gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                    position,
                    button: gpui::MouseButton::Left,
                    click_count: 1,
                    ..Default::default()
                }),
                cx,
            );
            window.dispatch_event(
                gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                    position,
                    button: gpui::MouseButton::Left,
                    click_count: 1,
                    ..Default::default()
                }),
                cx,
            );
            assert!(editor.focus_handle(cx).is_focused(window), "{source}");
            let before = editor.read(cx).selection().end;
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(gpui::Keystroke::parse("right").unwrap(), cx);
            assert!(editor.read(cx).selection().end > before, "{source}");
        }
    })
    .unwrap();
    app.quit();
}

fn find_text_for_test<'a>(blocks: &'a [Block], text: &str) -> Option<&'a Block> {
    blocks.iter().find_map(|block| {
        (block.text.as_ref() == text)
            .then_some(block)
            .or_else(|| find_text_for_test(&block.children, text))
    })
}

#[test]
fn blank_editor_click_focuses_and_places_the_caret_at_the_nearest_text() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("hello", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        let position =
            editor.read(cx).list.viewport_bounds().bottom_right() - point(px(10.), px(10.));
        window.dispatch_event(
            gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                position,
                button: gpui::MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
        assert!(editor.focus_handle(cx).is_focused(window));
        assert_eq!(editor.read(cx).selection(), 5..5);
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("x").unwrap(), cx);
        assert_eq!(editor.read(cx).source(), "hellox");
    })
    .unwrap();
    app.quit();
}

#[test]
fn arrow_keys_cross_hidden_delimiters_and_paragraph_boundaries_in_one_step() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("**one**\n\n*two*", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| editor.focus_source_offset(5, window, cx));
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("right").unwrap(), cx);
        assert_eq!(editor.read(cx).selection(), 10..10);
        window.dispatch_keystroke(gpui::Keystroke::parse("left").unwrap(), cx);
        assert_eq!(editor.read(cx).selection(), 5..5);
        editor.update(cx, |editor, cx| {
            editor.set_source("one **two**", cx);
            editor.set_selection(4..4, cx);
        });
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("right").unwrap(), cx);
        assert_eq!(editor.read(cx).selection(), 7..7);
    })
    .unwrap();
    app.quit();
}

#[test]
fn arrows_follow_displayed_graphemes_and_control_right_stops_at_word_end() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("a&#x301;b", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| editor.focus_source_offset(0, window, cx));
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("shift-right").unwrap(), cx);
        assert_eq!(
            &editor.read(cx).source()[editor.read(cx).selection()],
            "a&#x301;"
        );
        editor.update(cx, |editor, cx| {
            editor.set_source("ab cd", cx);
            editor.set_selection(1..1, cx);
            window.focus(&editor.focus, cx);
        });
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("ctrl-right").unwrap(), cx);
        assert_eq!(editor.read(cx).selection(), 2..2);
    })
    .unwrap();
    app.quit();
}

#[test]
fn find_escape_restores_editor_keyboard_focus() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("hello", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| editor.focus_source_offset(5, window, cx));
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("ctrl-f").unwrap(), cx);
        window.draw(cx).clear(cx);
        assert!(
            editor
                .read(cx)
                .find_input
                .focus_handle(cx)
                .is_focused(window)
        );
        window.dispatch_keystroke(gpui::Keystroke::parse("escape").unwrap(), cx);
        window.draw(cx).clear(cx);
        assert!(!editor.read(cx).find_open);
        assert!(editor.focus_handle(cx).is_focused(window));
        window.dispatch_keystroke(gpui::Keystroke::parse("x").unwrap(), cx);
        assert_eq!(editor.read(cx).source(), "hellox");
    })
    .unwrap();
    app.quit();
}

#[test]
fn find_fields_keep_preedit_focus_and_do_not_search_or_replace() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("hello hello", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        for replacement in [false, true] {
            let field = editor.update(cx, |editor, cx| {
                editor.find_open = true;
                editor.set_selection(0..5, cx);
                let field = if replacement {
                    editor.replacement_input.clone()
                } else {
                    editor.find_input.clone()
                };
                window.focus(&field.focus_handle(cx), cx);
                cx.notify();
                field
            });
            field.update(cx, |input, cx| {
                input.replace_and_mark_text_in_range(None, "hello", None, window, cx)
            });
            window.draw(cx).clear(cx);
            assert!(editor.read(cx).is_composing(cx));
            window.dispatch_keystroke(gpui::Keystroke::parse("enter").unwrap(), cx);
            window.dispatch_keystroke(gpui::Keystroke::parse("escape").unwrap(), cx);
            assert!(editor.read(cx).find_open);
            assert!(field.focus_handle(cx).is_focused(window));
            assert_eq!(editor.read(cx).selection(), 0..5);
            assert_eq!(editor.read(cx).source(), "hello hello");
            field.update(cx, |input, cx| {
                input.replace_text_in_range(None, "", window, cx)
            });
        }
    })
    .unwrap();
    app.quit();
}

#[test]
fn vertical_arrows_reveal_unmeasured_rows_and_preserve_visual_column() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = "abcdefghij\n\n".repeat(60);
    let editor = app.new(|cx| MarkdownEditor::new(source, cx));
    let handle = app.open_window(size(px(640.), px(160.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| editor.focus_source_offset(3, window, cx));
        window.draw(cx).clear(cx);
        for row in 1..40 {
            window.dispatch_keystroke(gpui::Keystroke::parse("down").unwrap(), cx);
            window.draw(cx).clear(cx);
            window.draw(cx).clear(cx);
            assert_eq!(
                editor.read(cx).selection(),
                row * 12 + 3..row * 12 + 3,
                "Down to row {row}"
            );
        }
        for row in (0..39).rev() {
            window.dispatch_keystroke(gpui::Keystroke::parse("up").unwrap(), cx);
            window.draw(cx).clear(cx);
            window.draw(cx).clear(cx);
            assert_eq!(
                editor.read(cx).selection(),
                row * 12 + 3..row * 12 + 3,
                "Up to row {row}"
            );
        }
    })
    .unwrap();
    app.quit();
}

#[test]
fn arrows_visit_empty_fenced_code_and_find_shift_enter_goes_backwards() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = "before\n\n```rust\n```\n\nafter before";
    let editor = app.new(|cx| MarkdownEditor::new(source, cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| editor.focus_source_offset(0, window, cx));
        window.draw(cx).clear(cx);
        for (key, offset) in [
            ("down", 16),
            ("down", 21),
            ("up", 16),
            ("up", 0),
            ("right", 1),
        ] {
            window.dispatch_keystroke(gpui::Keystroke::parse(key).unwrap(), cx);
            window.draw(cx).clear(cx);
            window.draw(cx).clear(cx);
            assert_eq!(editor.read(cx).selection(), offset..offset, "{key}");
        }
        for replacement in [false, true] {
            let field = editor.update(cx, |editor, cx| {
                editor.find_open = true;
                editor
                    .find_input
                    .update(cx, |input, cx| input.set_value("before", cx));
                editor.set_selection(27..33, cx);
                let field = if replacement {
                    editor.replacement_input.clone()
                } else {
                    editor.find_input.clone()
                };
                window.focus(&field.focus_handle(cx), cx);
                cx.notify();
                field
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(gpui::Keystroke::parse("shift-enter").unwrap(), cx);
            assert_eq!(editor.read(cx).selection(), 0..6);
            assert!(field.focus_handle(cx).is_focused(window));
            assert_eq!(editor.read(cx).source(), source);
        }
    })
    .unwrap();
    app.quit();
}

#[test]
fn dispatched_space_inserts_text_in_the_document() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("hello", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| editor.focus_source_offset(5, window, cx));
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("space").unwrap(), cx);
        assert_eq!(editor.read(cx).source(), "hello ");
    })
    .unwrap();
    app.quit();
}

#[test]
fn dispatched_task_click_survives_a_frame_between_down_and_up() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("- [ ] task item", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        let position = {
            let editor = editor.read(cx);
            let block = find_text_for_test(&editor.engine.document().blocks, "task item").unwrap();
            let layout = &editor.layouts[&block.id];
            point(layout.bounds.left() - px(16.), layout.bounds.center().y)
        };
        window.dispatch_event(
            gpui::PlatformInput::MouseMove(gpui::MouseMoveEvent {
                position,
                ..Default::default()
            }),
            cx,
        );
        window.draw(cx).clear(cx);
        window.dispatch_event(
            gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                position,
                button: gpui::MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
        assert!(
            !editor.focus_handle(cx).is_focused(window),
            "task control owns pointer focus"
        );
    })
    .unwrap();
    app.run_until_parked();
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        let position = {
            let editor = editor.read(cx);
            let block = find_text_for_test(&editor.engine.document().blocks, "task item").unwrap();
            let layout = &editor.layouts[&block.id];
            point(layout.bounds.left() - px(16.), layout.bounds.center().y)
        };
        window.dispatch_event(
            gpui::PlatformInput::MouseMove(gpui::MouseMoveEvent {
                position,
                pressed_button: Some(gpui::MouseButton::Left),
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                position,
                button: gpui::MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
    })
    .unwrap();
    app.run_until_parked();
    assert_eq!(
        editor.read_with(&app, |editor, _| editor.source().to_owned()),
        "- [x] task item"
    );
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("space").unwrap(), cx);
    })
    .unwrap();
    app.run_until_parked();
    assert_eq!(
        editor.read_with(&app, |editor, _| editor.source().to_owned()),
        "- [ ] task item"
    );
    app.quit();
}

#[test]
fn dispatched_table_double_click_keeps_cell_input_and_commits_replacement() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = "| A |\n| --- |\n| old |\n";
    let editor = app.new(|cx| MarkdownEditor::new(source, cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        let cell = editor.update(cx, |editor, cx| {
            editor
                .table_input_at(source.find("old").unwrap(), cx)
                .unwrap()
        });
        let bounds = cell.update(cx, |input, cx| {
            input
                .bounds_for_range(
                    1..1,
                    Bounds::new(point(px(12.), px(0.)), size(px(216.), px(40.))),
                    window,
                    cx,
                )
                .unwrap()
        });
        let position = bounds.center();
        for click_count in [1, 2] {
            window.dispatch_event(
                gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                    position,
                    button: gpui::MouseButton::Left,
                    click_count,
                    ..Default::default()
                }),
                cx,
            );
            window.draw(cx).clear(cx);
            window.dispatch_event(
                gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                    position,
                    button: gpui::MouseButton::Left,
                    click_count,
                    ..Default::default()
                }),
                cx,
            );
            window.draw(cx).clear(cx);
        }
        assert!(cell.focus_handle(cx).is_focused(window));
        assert_eq!(cell.read(cx).selected_range(), 0..3);
        window.dispatch_keystroke(gpui::Keystroke::parse("x").unwrap(), cx);
        assert_eq!(cell.read(cx).value().as_ref(), "x");
    })
    .unwrap();
    app.run_until_parked();
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("space").unwrap(), cx);
    })
    .unwrap();
    app.run_until_parked();
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("q").unwrap(), cx);
    })
    .unwrap();
    app.run_until_parked();
    app.update(|cx| assert_eq!(editor.read(cx).source(), "| A |\n| --- |\n| x q |\n"));
    app.quit();
}

#[test]
fn table_edge_spaces_follow_history_external_edits_caret_and_row_moves() {
    for action in ["undo", "external", "middle", "move"] {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let source = "| A |\n| --- |\n| abc |\n| other |\n";
        let editor = app.new(|cx| MarkdownEditor::new(source, cx));
        let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            let cell = editor.update(cx, |editor, cx| {
                editor
                    .table_input_at(source.find("abc").unwrap(), cx)
                    .unwrap()
            });
            window.focus(&cell.focus_handle(cx), cx);
            cell.update(cx, |input, cx| input.set_selection(3..3, cx));
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(gpui::Keystroke::parse("space").unwrap(), cx);
        })
        .unwrap();
        app.run_until_parked();
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            match action {
                "undo" => editor.update(cx, |editor, cx| {
                    assert!(editor.undo(cx));
                }),
                "external" => editor.update(cx, |editor, cx| editor.set_source(source, cx)),
                "move" => editor.update(cx, |editor, cx| {
                    let id = editor.engine.document().tables()[0].id;
                    editor
                        .edit_table(id, MarkdownTableEdit::MoveRow { from: 0, to: 1 }, cx)
                        .unwrap();
                }),
                _ => {}
            }
            window.draw(cx).clear(cx);
            let cell = editor.update(cx, |editor, cx| {
                editor
                    .table_input_at(editor.source().find("abc").unwrap(), cx)
                    .unwrap()
            });
            assert_eq!(
                cell.read(cx).value().as_ref(),
                if matches!(action, "undo" | "external") {
                    "abc"
                } else {
                    "abc "
                },
                "{action}"
            );
            window.focus(&cell.focus_handle(cx), cx);
            cell.update(cx, |input, cx| input.set_selection(1..1, cx));
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(gpui::Keystroke::parse("x").unwrap(), cx);
        })
        .unwrap();
        app.run_until_parked();
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(gpui::Keystroke::parse("q").unwrap(), cx);
        })
        .unwrap();
        app.run_until_parked();
        app.update(|cx| {
            let table = editor.read(cx).engine.document().tables().remove(0);
            let row = if action == "move" { 2 } else { 1 };
            assert_eq!(
                table.rows[row].cells[0].text.as_ref(),
                if matches!(action, "undo" | "external") {
                    "axqbc"
                } else {
                    "axqbc "
                },
                "{action}"
            );
        });
        app.quit();
    }
}

#[test]
fn whitespace_keys_update_caret_before_the_next_character() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("hello", cx));
    let handle = app.open_window(size(px(640.), px(700.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| editor.focus_source_offset(5, window, cx));
        window.draw(cx).clear(cx);
        let caret = |editor: &MarkdownEditor| {
            let (id, offset) = editor
                .engine
                .display_position(editor.engine.selection().head)
                .unwrap();
            editor.layouts[&id].position(offset)
        };
        let before = caret(editor.read(cx));
        window.dispatch_keystroke(gpui::Keystroke::parse("space").unwrap(), cx);
        window.draw(cx).clear(cx);
        let after = caret(editor.read(cx));
        assert!(
            after.x > before.x,
            "Space must advance the painted caret immediately"
        );
        window.dispatch_keystroke(gpui::Keystroke::parse("enter").unwrap(), cx);
        window.draw(cx).clear(cx);
        let next = caret(editor.read(cx));
        assert!(
            next.y > after.y,
            "Enter must place the caret on the new empty paragraph"
        );
        window.dispatch_keystroke(gpui::Keystroke::parse("enter").unwrap(), cx);
        window.draw(cx).clear(cx);
        let empty = caret(editor.read(cx));
        assert!(empty.y > next.y);
        assert_eq!(editor.read(cx).source(), "hello \n\n\n\n");
    })
    .unwrap();
    app.quit();
}

#[test]
fn editing_near_the_top_does_not_scroll_to_an_unfocused_table_caret() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = format!("{}\n| A |\n| --- |\n| cell |\n", "Paragraph.\n\n".repeat(7));
    let editor = app.new(|cx| MarkdownEditor::new(source.clone(), cx));
    let handle = app.open_window(size(px(640.), px(400.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| editor.focus_source_offset(3, window, cx));
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("x").unwrap(), cx);
        window.draw(cx).clear(cx);
        assert_eq!(editor.read(cx).list.logical_scroll_top().item_ix, 0);
        assert_eq!(
            editor.read(cx).list.logical_scroll_top().offset_in_item,
            px(0.)
        );
        assert_eq!(editor.read(cx).selection(), 4..4);
    })
    .unwrap();
    app.quit();
}

#[test]
fn rich_caret_blinks_resets_on_edit_and_stops_on_blur() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("hello", cx));
    let handle = app.open_window(size(px(640.), px(400.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| editor.focus_source_offset(5, window, cx));
        window.draw(cx).clear(cx);
    })
    .unwrap();
    app.run_until_parked();
    app.executor()
        .advance_clock(std::time::Duration::from_millis(500));
    app.run_until_parked();
    app.update(|cx| assert!(!editor.read(cx).cursor_visible));
    app.update_window(handle.into(), |_, window, cx| {
        window.dispatch_keystroke(gpui::Keystroke::parse("space").unwrap(), cx);
        window.draw(cx).clear(cx);
        assert!(editor.read(cx).cursor_visible);
        window.blur(cx);
        window.draw(cx).clear(cx);
        assert!(editor.read(cx).blink_task.is_none());
    })
    .unwrap();
    app.quit();
}

#[test]
fn typing_in_a_visible_lower_row_keeps_its_screen_position() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = "Paragraph text.\n\n".repeat(40);
    let offset = source.match_indices("Paragraph").nth(4).unwrap().0 + 3;
    let editor = app.new(|cx| MarkdownEditor::new(source, cx));
    let handle = app.open_window(size(px(640.), px(400.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        editor.update(cx, |editor, cx| {
            editor.focus_source_offset(offset, window, cx)
        });
        window.draw(cx).clear(cx);
        let before = editor.read(cx).list.logical_scroll_top();
        let (id, _) = editor.read(cx).engine.display_position(offset).unwrap();
        let y = editor.read(cx).layouts[&id].bounds.top();
        window.dispatch_keystroke(gpui::Keystroke::parse("x").unwrap(), cx);
        window.draw(cx).clear(cx);
        let after = editor.read(cx).list.logical_scroll_top();
        assert_eq!(
            (after.item_ix, after.offset_in_item),
            (before.item_ix, before.offset_in_item)
        );
        let (id, display) = editor.read(cx).engine.display_position(offset + 1).unwrap();
        let layout = &editor.read(cx).layouts[&id];
        assert_eq!(layout.bounds.top(), y);
        let point = layout.position(display) + point(px(0.), layout.line_height * 0.5);
        assert_eq!(editor.read(cx).index_for_point(point), Some(offset + 1));
    })
    .unwrap();
    app.quit();
}

#[test]
fn source_backward_selection_keeps_its_active_end_across_modes() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("hello world", cx));
    let handle = app.open_window(size(px(640.), px(400.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| {
            editor.set_selection(8..5, cx);
            editor.set_mode(MarkdownEditMode::Source, cx);
            window.focus(&editor.focus_handle(cx), cx);
        });
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("shift-left").unwrap(), cx);
    })
    .unwrap();
    app.run_until_parked();
    app.update(|cx| {
        editor.update(cx, |editor, cx| {
            assert_eq!(editor.engine.selection(), Selection { anchor: 8, head: 4 });
            editor.set_mode(MarkdownEditMode::Rich, cx);
            assert_eq!(editor.engine.selection(), Selection { anchor: 8, head: 4 });
        })
    });
    app.quit();
}

#[test]
fn caret_autoscroll_leaves_only_final_pointer_geometry() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = format!(
        "First row.\n\n{}\n\n{}",
        "long wrapped paragraph ".repeat(60),
        "Later row.\n\n".repeat(20)
    );
    let offset = source.find("\n\nLater").unwrap();
    let editor = app.new(|cx| MarkdownEditor::new(source, cx));
    let handle = app.open_window(size(px(360.), px(300.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        editor.update(cx, |editor, cx| {
            editor.focus_source_offset(offset, window, cx)
        });
        window.draw(cx).clear(cx);
        let editor = editor.read(cx);
        let viewport = editor.list.viewport_bounds();
        for layout in editor.layouts.values() {
            if layout.bounds.bottom() <= viewport.top() || layout.bounds.top() >= viewport.bottom()
            {
                continue;
            }
            let row = editor
                .rows
                .iter()
                .position(|(id, _)| *id == layout.id)
                .unwrap();
            let bounds = editor
                .list
                .bounds_for_item(row)
                .expect("painted row has final bounds");
            assert!(
                layout.bounds.top() >= bounds.top() && layout.bounds.bottom() <= bounds.bottom()
            );
            let visible = point(
                layout.bounds.left(),
                layout.bounds.top().max(viewport.top()) + px(2.),
            );
            assert_eq!(
                editor.index_for_point(visible),
                editor
                    .engine
                    .source_position(layout.id, layout.index(visible))
            );
        }
        let (id, display) = editor.engine.display_position(offset).unwrap();
        let caret = editor.layouts[&id].position(display);
        assert!(caret.y >= viewport.top() && caret.y < viewport.bottom());
    })
    .unwrap();
    app.quit();
}

#[test]
fn nested_space_and_enter_update_native_caret_geometry_immediately() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("", cx));
    let handle = app.open_window(size(px(640.), px(400.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        let caret = |editor: &MarkdownEditor| {
            let (id, offset) = editor
                .engine
                .display_position(editor.engine.selection().head)
                .unwrap();
            editor.layouts[&id].position(offset)
        };
        for source in ["hello", "- hello", "> hello", "> - hello"] {
            editor.update(cx, |editor, cx| {
                editor.set_source(source, cx);
                editor.focus_source_offset(source.len(), window, cx);
            });
            window.draw(cx).clear(cx);
            let before = caret(editor.read(cx));
            window.dispatch_keystroke(gpui::Keystroke::parse("space").unwrap(), cx);
            window.draw(cx).clear(cx);
            let space = caret(editor.read(cx));
            assert_eq!(space.y, before.y, "{source:?}");
            assert!(space.x > before.x, "{source:?}");
            window.dispatch_keystroke(gpui::Keystroke::parse("enter").unwrap(), cx);
            window.draw(cx).clear(cx);
            let enter = caret(editor.read(cx));
            assert!(enter.y > space.y, "{source:?}");
        }
        editor.update(cx, |editor, cx| {
            editor.set_source("hello", cx);
            editor.focus_source_offset(0, window, cx);
        });
        window.draw(cx).clear(cx);
        let before = caret(editor.read(cx));
        window.dispatch_keystroke(gpui::Keystroke::parse("space").unwrap(), cx);
        window.draw(cx).clear(cx);
        let after = caret(editor.read(cx));
        assert_eq!(after.y, before.y);
        assert!(after.x > before.x);
        assert_eq!(editor.read(cx).engine.document().blocks.len(), 1);
    })
    .unwrap();
    app.quit();
}

#[test]
fn source_selection_notifies_before_typing_and_select_all_resets_direction() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("hello world", cx));
    let handle = app.open_window(size(px(640.), px(400.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        editor.update(cx, |editor, cx| {
            editor.set_selection(8..5, cx);
            editor.set_mode(MarkdownEditMode::Source, cx);
            window.focus(&editor.focus_handle(cx), cx);
        });
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("ctrl-a").unwrap(), cx);
        window.dispatch_keystroke(gpui::Keystroke::parse("shift-left").unwrap(), cx);
        assert_eq!(editor.read(cx).input.read(cx).directed_selection(), 0..10);
        window.blur(cx);
        window.draw(cx).clear(cx);
        editor
            .read(cx)
            .input
            .clone()
            .update(cx, |input, cx| input.set_selection(3..3, cx));
    })
    .unwrap();
    app.run_until_parked();
    app.update_window(handle.into(), |_, window, cx| {
        assert_eq!(
            editor.read(cx).engine.selection(),
            Selection { anchor: 3, head: 3 }
        );
        let input = editor.read(cx).input.clone();
        input.update(cx, |input, cx| {
            input.replace_text_in_range(None, "x", window, cx)
        });
    })
    .unwrap();
    app.run_until_parked();
    app.update(|cx| {
        editor.update(cx, |editor, cx| {
            assert!(editor.undo(cx));
            assert_eq!(editor.source(), "hello world");
            assert_eq!(editor.selection(), 3..3);
        })
    });
    app.quit();
}

#[test]
fn space_in_empty_quotes_advances_the_native_caret() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("", cx));
    let handle = app.open_window(size(px(640.), px(400.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        for source in ["> ", "- > ", "- > hello\r\n  > "] {
            editor.update(cx, |editor, cx| {
                editor.set_source(source, cx);
                editor.focus_source_offset(source.len(), window, cx);
            });
            window.draw(cx).clear(cx);
            let position = |editor: &MarkdownEditor| {
                let (id, offset) = editor
                    .engine
                    .display_position(editor.engine.selection().head)
                    .unwrap();
                assert_eq!(
                    editor.engine.source_position(id, offset),
                    Some(editor.engine.selection().head)
                );
                editor.layouts[&id].position(offset)
            };
            let before = position(editor.read(cx));
            window.dispatch_keystroke(gpui::Keystroke::parse("space").unwrap(), cx);
            window.draw(cx).clear(cx);
            let after = position(editor.read(cx));
            assert_eq!(after.y, before.y, "{source:?}");
            assert!(after.x > before.x, "{source:?}");
        }
    })
    .unwrap();
    app.quit();
}

#[test]
fn syntax_replacement_and_preceding_splices_preserve_visible_anchors() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let source = (0..50)
        .map(|i| format!("Paragraph {i}.\n\n"))
        .collect::<String>();
    let offset = source.find("Paragraph 4.").unwrap();
    let editor = app.new(|cx| MarkdownEditor::new(source.clone(), cx));
    let handle = app.open_window(size(px(640.), px(400.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        editor.update(cx, |editor, cx| {
            editor.focus_source_offset(offset, window, cx)
        });
        window.draw(cx).clear(cx);
        let scroll = editor.read(cx).list.logical_scroll_top();
        editor.update(cx, |editor, cx| {
            editor.replace_text_in_range(None, "# ", window, cx)
        });
        window.draw(cx).clear(cx);
        assert_eq!(
            editor.read(cx).list.logical_scroll_top().item_ix,
            scroll.item_ix
        );
        assert_eq!(
            editor.read(cx).list.logical_scroll_top().offset_in_item,
            scroll.offset_in_item
        );
        editor.update(cx, |editor, _| {
            editor.list.scroll_to(gpui::ListOffset {
                item_ix: 20,
                offset_in_item: px(5.),
            });
        });
        window.draw(cx).clear(cx);
        for insert in [true, false] {
            let before = editor.read(cx).list.logical_scroll_top();
            let id = editor.read(cx).rows[before.item_ix].0;
            editor.update(cx, |editor, cx| {
                let source = editor.engine.source().to_string();
                let next = if insert {
                    format!("Inserted.\n\n{source}")
                } else {
                    source.strip_prefix("Inserted.\n\n").unwrap().to_string()
                };
                editor.engine.set_source(&next);
                editor.reconcile_rows();
                cx.notify();
            });
            window.draw(cx).clear(cx);
            let after = editor.read(cx).list.logical_scroll_top();
            assert_eq!(editor.read(cx).rows[after.item_ix].0, id);
            assert_eq!(after.offset_in_item, before.offset_in_item);
        }
    })
    .unwrap();
    app.quit();
}

#[test]
fn vertical_keys_recover_after_scrolling_away_from_the_caret() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("abcdefghij\n\n".repeat(60), cx));
    let handle = app.open_window(size(px(640.), px(160.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        for (key, expected, extend) in [
            ("down", 27, false),
            ("up", 3, false),
            ("shift-down", 27, true),
            ("shift-up", 3, true),
        ] {
            editor.update(cx, |editor, cx| editor.focus_source_offset(15, window, cx));
            window.draw(cx).clear(cx);
            editor.read(cx).list.scroll_to(gpui::ListOffset {
                item_ix: 35,
                offset_in_item: px(0.),
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(gpui::Keystroke::parse(key).unwrap(), cx);
            for _ in 0..3 {
                window.draw(cx).clear(cx);
            }
            assert_eq!(
                editor.read(cx).engine.selection(),
                Selection {
                    anchor: if extend { 15 } else { expected },
                    head: expected
                },
                "{key}"
            );
        }
    })
    .unwrap();
    app.quit();
}

#[test]
fn intervening_input_cancels_offscreen_vertical_retry() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let editor = app.new(|cx| MarkdownEditor::new("abcdefghij\n\n".repeat(60), cx));
    let handle = app.open_window(size(px(640.), px(160.)), |_, _| Surface(editor.clone()));
    app.update_window(handle.into(), |_, window, cx| {
        for key in ["right", "x"] {
            editor.update(cx, |editor, cx| editor.focus_source_offset(15, window, cx));
            window.draw(cx).clear(cx);
            editor.read(cx).list.scroll_to(gpui::ListOffset {
                item_ix: 35,
                offset_in_item: px(0.),
            });
            window.draw(cx).clear(cx);
            window.dispatch_keystroke(gpui::Keystroke::parse("down").unwrap(), cx);
            window.dispatch_keystroke(gpui::Keystroke::parse(key).unwrap(), cx);
            let expected = editor.read(cx).selection();
            assert!(editor.read(cx).pending_vertical_retry.is_none());
            for _ in 0..3 {
                window.draw(cx).clear(cx);
            }
            assert_eq!(editor.read(cx).selection(), expected, "{key}");
        }
    })
    .unwrap();
    app.quit();
}
