use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide};
use liora_components::{Flex, MarkdownEditor, MarkdownImage};

const SOURCE: &str = r#"# Native Markdown

Click this paragraph to edit it immediately.

Native **bold**, *emphasis*, ~~strike~~ and `inline code` keep their styles.

## Lists and quotes

> Quoted paragraphs can be edited without removing the quote marker.

1. First editable item
2. Second editable item

- [x] Rendered task marker
- [ ] Click a task marker to toggle it

## Tables

| Feature | State |
| --- | --- |
| Native table cells | Click to edit |
| Canonical Markdown | Updated immediately |

## Code and formulas

```rust
let document = MarkdownEditor::new(source, cx);
```

Inline formula: $\frac{1}{2}$; ==highlight==, ^superscript^ and ~subscript~.

```math
\sum_{i=1}^{n} i
```

```mermaid
flowchart LR
A[Markdown] --> B[Native GPUI]
```

## Host-approved image

![Local Gallery image](gallery-image)
"#;

pub struct MarkdownDemo {
    document: Entity<MarkdownEditor>,
    fill_height: bool,
}

pub fn render(cx: &mut App) -> Entity<MarkdownDemo> {
    new_with_height(cx, false)
}

pub fn render_filling(cx: &mut App) -> gpui::AnyView {
    new_with_height(cx, true).into()
}

fn new_with_height(cx: &mut App, fill_height: bool) -> Entity<MarkdownDemo> {
    cx.new(|cx| {
        let document = cx.new(|cx| {
            let mut editor = MarkdownEditor::new(SOURCE, cx);
            editor.set_image_resolver(
                |destination| {
                    (destination == "gallery-image").then(|| {
                        MarkdownImage::File(
                            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                                .join("assets/local.jpeg"),
                        )
                    })
                },
                cx,
            );
            editor
        });
        MarkdownDemo {
            document,
            fill_height,
        }
    })
}

impl MarkdownDemo {
    #[cfg(debug_assertions)]
    pub fn document(&self) -> Entity<MarkdownEditor> {
        self.document.clone()
    }
}

impl Render for MarkdownDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.fill_height {
            return Flex::new()
                .id("markdown-demo-scroll")
                .column()
                .size_full()
                .min_h_0()
                .child(self.document.clone())
                .into_any_element();
        }
        page(
            "Markdown",
            "Type Markdown syntax directly in one native editor; rendering updates as you type.",
            section(
                "Live native Markdown",
                "Type # headings, **strong**, - list items, and ``` code fences directly. Quotes, tables, tasks, formulas, diagrams, and the host-approved image remain in the same document.",
                showcase_card_wide(
                    "Editable Markdown",
                    "The bounded editor owns focus, selection, rendering, and scrolling. No format buttons or mode switch are needed for syntax-driven input.",
                    Flex::new()
                        .id("markdown-demo-scroll")
                        .column()
                        .w_full()
                        .height_units(380.0)
                        .child(self.document.clone()),
                ),
            ),
        )
        .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        EntityInputHandler, PlatformInput, ScrollDelta, ScrollEvent, TestAppContext, TouchPhase,
        point, px, size,
    };

    #[test]
    fn shared_demo_has_a_bounded_scrollable_document() {
        let mut app = TestAppContext::single();
        app.update(liora_components::init_liora);
        let demo = app.update(render);
        let scroll = app.update(|cx| {
            let document = demo.read(cx).document.clone();
            document.update(cx, |editor, cx| {
                let source = format!("{}{}", editor.source(), "\n\nMore content.\n".repeat(60));
                editor.set_source(source, cx);
            });
            document.read(cx).scroll_handle()
        });
        let handle = app.open_window(size(px(1200.), px(900.)), |_, _| DemoView(demo));
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            let before = scroll.logical_scroll_top();
            let bounds = scroll.viewport_bounds();
            assert!(bounds.size.height > px(0.) && bounds.size.height <= px(380.));
            window.dispatch_event(
                PlatformInput::Scroll(ScrollEvent {
                    position: bounds.center(),
                    delta: ScrollDelta::Pixels(point(px(0.), px(-120.))),
                    touch_phase: TouchPhase::Moved,
                    ..Default::default()
                }),
                cx,
            );
            window.draw(cx).clear(cx);
            let after = scroll.logical_scroll_top();
            assert!(
                after.item_ix > before.item_ix || after.offset_in_item > before.offset_in_item,
                "wheel input must scroll virtual Markdown rows"
            );
        })
        .unwrap();
        app.quit();
    }

    #[test]
    fn shared_demo_types_markdown_syntax_in_the_native_surface() {
        let mut app = TestAppContext::single();
        app.update(liora_components::init_liora);
        let demo = app.update(render);
        let document = app.update(|cx| demo.read(cx).document.clone());
        let handle = app.open_window(size(px(1200.), px(900.)), |_, _| DemoView(demo));
        app.update_window(handle.into(), |_, window, cx| {
            document.update(cx, |editor, cx| {
                editor.focus_source_offset(SOURCE.len(), window, cx);
            });
            window.draw(cx).clear(cx);
            document.update(cx, |editor, cx| {
                editor.begin_undo_group();
                for character in "**typed**".chars() {
                    EntityInputHandler::replace_text_in_range(
                        editor,
                        None,
                        &character.to_string(),
                        window,
                        cx,
                    );
                }
                editor.end_undo_group();
            });
            window.draw(cx).clear(cx);
            let source = document.read(cx).source();
            assert!(source.ends_with("\n**typed**"));
            assert!(source.contains("Native **bold**, *emphasis*"));
            assert!(source.contains("```mermaid\nflowchart LR"));
            assert!(source.contains("![Local Gallery image](gallery-image)"));
            document.update(cx, |editor, cx| {
                assert!(editor.undo(cx));
                assert_eq!(editor.source(), SOURCE);
                assert!(editor.redo(cx));
                assert!(editor.source().ends_with("\n**typed**"));
            });
        })
        .unwrap();
        app.quit();
    }

    struct DemoView(Entity<MarkdownDemo>);
    impl Render for DemoView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.0.clone()
        }
    }
}
