//! Assert-driven native TodoList projection, composition, identity, and subscription regression.
//! Run: cargo run -p liora-gallery --example todo_smoke --locked

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        AppContext, InteractiveElement, IntoElement, KeyUpEvent, Keystroke, MouseButton,
        MouseDownEvent, MouseUpEvent, ParentElement, PlatformInput, StatefulInteractiveElement,
        Styled, WindowOptions, div, point, px,
    };
    use liora_components::{
        Text, TodoItem, TodoItemChanged, TodoItemData, TodoList, TodoListRenderContext, init_liora,
    };
    use std::{cell::Cell, rc::Rc, time::Duration};

    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let changes = Rc::new(Cell::new(0));
        let mutations = Rc::new(Cell::new(0));
        let header = Rc::new(Cell::new(None::<TodoListRenderContext>));
        let empty = Rc::new(Cell::new(None::<TodoListRenderContext>));
        let footer = Rc::new(Cell::new(None::<TodoListRenderContext>));
        let todos = cx.new(|cx| {
            TodoList::from_data(
                vec![
                    TodoItemData::new("z", "Zulu"),
                    TodoItemData::new("a", "Alpha").checked(true),
                    TodoItemData::new("b", "Bravo"),
                ],
                cx,
            )
            .on_change({
                let changes = changes.clone();
                move |_, _| changes.set(changes.get() + 1)
            })
            .on_mutation({
                let mutations = mutations.clone();
                move |_, _| mutations.set(mutations.get() + 1)
            })
            .header_renderer({
                let header = header.clone();
                move |context| {
                    header.set(Some(context));
                    Text::new("Release queue").into_any_element()
                }
            })
            .empty_renderer({
                let empty = empty.clone();
                move |context| {
                    empty.set(Some(context));
                    Text::new("No matches").into_any_element()
                }
            })
            .contextual_footer_renderer({
                let footer = footer.clone();
                move |context| {
                    footer.set(Some(context));
                    Text::new("Add task").into_any_element()
                }
            })
        });
        let retained = todos.read(cx).item("z", cx).unwrap();
        retained.update(cx, |item, cx| item.set_editing(true, cx));
        let duplicate = cx.new(|cx| TodoItem::new("z", "Must not replace Zulu", cx));
        todos.update(cx, |list, cx| {
            list.push(duplicate, cx);
            assert_eq!(list.push_data(TodoItemData::new("z", "Duplicate"), cx), retained);
            assert_eq!(list.len(), 3);
            list.set_filter(|item| !item.is_checked(), cx);
            list.set_sort(|left, right| left.title().cmp(&right.title()), cx);
            let visible = list.visible_items(cx);
            assert_eq!(visible.iter().map(|item| item.read(cx).id()).collect::<Vec<_>>(), vec!["b", "z"]);
            assert_eq!(visible[1], retained);
            assert!(visible[1].read(cx).is_editing());
            assert_eq!(list.items()[0], retained, "display sorting must not change storage order");
            list.clear_filter(cx);
            list.clear_sort(cx);
            assert_eq!(list.visible_items(cx), list.items());
            list.set_filter(|_| false, cx);
        });
        assert_eq!(mutations.get(), 0, "projection and duplicate pushes are not mutations");
        let handle = cx.open_window(
            WindowOptions { show: false, focus: false, ..Default::default() },
            |_, _| todos.clone(),
        ).expect("native headless window should open");
        cx.update_window(handle.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("empty projection should render");
        let expected = TodoListRenderContext { total: 3, checked: 1, visible: 0 };
        assert_eq!(header.get(), Some(expected));
        assert_eq!(empty.get(), Some(expected));
        assert_eq!(footer.get(), Some(expected));

        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(10)).await;
            // Separate app updates drain GPUI's deferred event/observation effects.
            cx.update(|cx| {
                todos.update(cx, |list, cx| list.set_filter(|item| !item.is_checked(), cx));
                retained.update(cx, |item, cx| item.set_checked(true, cx));
            });
            cx.update(|cx| {
                assert_eq!(todos.read(cx).visible_items(cx).len(), 1);
                assert!(retained.read(cx).is_editing(), "hidden entity retains editor state");
                retained.update(cx, |_, cx| cx.emit(TodoItemChanged { id: "z".into(), checked: true }));
            });
            cx.update(|cx| {
                assert_eq!(changes.get(), 1, "attached item events reach the list once");
                todos.update(cx, |list, cx| { assert_eq!(list.remove("z", cx), Some(retained.clone())); });
                retained.update(cx, |_, cx| cx.emit(TodoItemChanged { id: "z".into(), checked: false }));
            });
            cx.update(|cx| {
                assert_eq!(changes.get(), 1, "removed item cannot emit list changes");
                let replacement = todos.update(cx, |list, cx| list.push_data(TodoItemData::new("z", "Replacement"), cx));
                assert_ne!(replacement, retained);
                // Even reusing the ID must not reconnect the old entity.
                retained.update(cx, |_, cx| cx.emit(TodoItemChanged { id: "z".into(), checked: true }));
                replacement.update(cx, |_, cx| cx.emit(TodoItemChanged { id: "z".into(), checked: true }));
            });
            cx.update(|cx| {
                assert_eq!(changes.get(), 2, "only the replacement entity is subscribed");
                todos.update(cx, |list, cx| {
                    for item in list.clear(cx) {
                        item.update(cx, |item, cx| cx.emit(TodoItemChanged { id: item.id(), checked: true }));
                    }
                });
            });
            cx.update(|cx| {
                assert_eq!(changes.get(), 2, "clear releases every item subscription");
                assert!(todos.read(cx).is_empty());
                let edits = Rc::new(Cell::new(0));
                let row = cx.new(|cx| {
                    let edits = edits.clone();
                    TodoItem::new("pointer", "Click to complete", cx)
                    .check_renderer(|_| Text::new("☆").selectable(false).into_any_element())
                    .content_renderer(|context| div().flex().flex_col()
                        .child(Text::new(context.title).selectable(false))
                        .child(Text::new("Task description").selectable(false))
                        .into_any_element())
                    .trailing_renderer(move |_| {
                        let edits = edits.clone();
                        div()
                            .id("edit")
                            .w(px(80.))
                            .h(px(28.))
                            .on_click(move |_, _, _| edits.set(edits.get() + 1))
                            .child("Edit")
                            .into_any_element()
                    })
                });
                let pointer_list = cx.new(|cx| TodoList::new(cx));
                pointer_list.update(cx, |list, cx| list.push(row.clone(), cx));
                let window = cx.open_window(WindowOptions { show: false, ..Default::default() }, |_, _| pointer_list.clone()).unwrap();
                cx.update_window(window.into(), |_, window, cx| {
                    window.draw(cx).clear(cx);
                    let position = point(px(12.), px(16.));
                    let down = |position| PlatformInput::MouseDown(MouseDownEvent { button: MouseButton::Left, position, click_count: 1, ..Default::default() });
                    let up = |position| PlatformInput::MouseUp(MouseUpEvent { button: MouseButton::Left, position, click_count: 1, ..Default::default() });
                    window.dispatch_event(down(position), cx);
                    assert!(row.read(cx).is_checked(), "press must toggle before release or repaint");
                    window.draw(cx).clear(cx);
                    window.dispatch_event(up(position), cx);
                    assert!(row.read(cx).is_checked(), "release must not toggle again");

                    let action = point(window.viewport_size().width - px(16.), px(16.));
                    window.dispatch_event(down(action), cx);
                    window.dispatch_event(up(action), cx);
                    assert_eq!(edits.get(), 1, "trailing action must receive its click");
                    assert!(row.read(cx).is_checked(), "trailing action must not toggle the row");

                    for position in [point(px(60.), px(12.)), point(px(60.), px(36.))] {
                        for _ in 0..2 {
                            window.draw(cx).clear(cx);
                            let before = row.read(cx).is_checked();
                            window.dispatch_event(down(position), cx);
                            window.dispatch_event(up(position), cx);
                            assert_eq!(row.read(cx).is_checked(), !before, "title and description must toggle the item");
                        }
                    }

                    for key in ["space", "enter"] {
                        let before = row.read(cx).is_checked();
                        let keystroke = Keystroke::parse(key).unwrap();
                        window.dispatch_keystroke(keystroke.clone(), cx);
                        window.dispatch_event(PlatformInput::KeyUp(KeyUpEvent { keystroke }), cx);
                        assert_eq!(row.read(cx).is_checked(), !before, "{key} must toggle exactly once");
                    }

                    for editing in [false, true] {
                        row.update(cx, |item, cx| {
                            item.set_disabled(!editing, cx);
                            item.set_editing(editing, cx);
                        });
                        window.draw(cx).clear(cx);
                        window.dispatch_event(down(position), cx);
                        window.dispatch_event(up(position), cx);
                        let keystroke = Keystroke::parse("space").unwrap();
                        window.dispatch_keystroke(keystroke.clone(), cx);
                        window.dispatch_event(PlatformInput::KeyUp(KeyUpEvent { keystroke }), cx);
                        assert!(row.read(cx).is_checked(), "disabled or editing items must not toggle");
                    }
                }).unwrap();
                println!("PASS: native TodoList projection, identity, subscriptions, pointer, and keyboard");
                cx.quit();
            });
        }).detach();
    });
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn main() {
    eprintln!("This display-free smoke uses the official Linux/FreeBSD headless backend.");
    std::process::exit(1);
}
