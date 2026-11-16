use gpui::{App, Context, Entity, IntoElement, Render, ScrollHandle, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide};
use liora_components::{Button, Flex, MarkdownEditor, MarkdownImage, Space, Text};

const SOURCE: &str = r#"# Native Markdown

Click this paragraph to edit it immediately.

Native **bold**, *emphasis*, ~~strike~~ and `inline code` keep their styles.

## Lists and quotes

> Quoted paragraphs can be edited without removing the quote marker.

1. First editable item
2. Second editable item

- [x] Rendered task marker
- [ ] Task markers are changed in source mode

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
    scroll_handle: ScrollHandle,
    chunks: usize,
}

pub fn render(cx: &mut App) -> Entity<MarkdownDemo> {
    cx.new(|cx| {
        let scroll_handle = ScrollHandle::new();
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
            scroll_handle,
            chunks: 0,
        }
    })
}

impl Render for MarkdownDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weak = cx.weak_entity();
        let reset = cx.weak_entity();
        page(
            "Markdown",
            "Native rendered content with immediate paragraph, heading and table-cell editing.",
            section(
                "Live rich document",
                "Click a paragraph or heading (or Tab then Enter) to edit. Formatting actions apply to the active block. Code, formulas, diagrams and images stay rendered; use Edit Markdown source for unsupported structure. Table-cell edits replace inline formatting with literal text.",
                showcase_card_wide(
                    "Editable Markdown",
                    "Rich mode is a bounded native editing projection, not full Typora parity. Changes are kept in this demo until reset.",
                    Space::new().vertical().gap_md()
                        .child(Space::new()
                            .child(Button::new("Append example").on_click(move |_, _, cx| {
                                let _ = weak.update(cx, |demo, cx| {
                                    demo.chunks += 1;
                                    let chunk = format!("\n\n## Appended example {}\n\nThis **complete** paragraph is editable too.\n", demo.chunks);
                                    demo.document.update(cx, |editor, cx| {
                                        let source = format!("{}{chunk}", editor.source());
                                        editor.set_source(source, cx);
                                    });
                                    cx.notify();
                                });
                            }))
                            .child(Button::new("Reset document").on_click(move |_, _, cx| {
                                let _ = reset.update(cx, |demo, cx| {
                                    demo.chunks = 0;
                                    demo.document.update(cx, |editor, cx| editor.set_source(SOURCE, cx));
                                    cx.notify();
                                });
                            })))
                        .child(Text::new(format!("Appended examples: {}", self.chunks)))
                        .child(Flex::new().id("markdown-demo-scroll").column().w_full().height_units(380.0).overflow_y_scroll().track_scroll(&self.scroll_handle).child(self.document.clone())),
                ),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        Focusable, PlatformInput, ScrollDelta, ScrollEvent, TestAppContext, TouchPhase, point, px,
        size,
    };

    #[test]
    fn shared_demo_has_a_bounded_scrollable_document() {
        let mut app = TestAppContext::single();
        app.update(liora_components::init_liora);
        let demo = app.update(render);
        let scroll = app.update(|cx| {
            let (document, scroll) = {
                let demo = demo.read(cx);
                (demo.document.clone(), demo.scroll_handle.clone())
            };
            document.update(cx, |editor, cx| {
                let source = format!("{}{}", editor.source(), "\n\nMore content.\n".repeat(60));
                editor.set_source(source, cx);
            });
            scroll
        });
        let handle = app.open_window(size(px(1200.), px(900.)), |_, _| DemoView(demo));
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(scroll.bounds().size.height, px(380.));
            assert!(
                scroll.max_offset().y > px(100.),
                "document must overflow its bounded viewport"
            );
            window.dispatch_event(
                PlatformInput::Scroll(ScrollEvent {
                    position: scroll.bounds().center(),
                    delta: ScrollDelta::Pixels(point(px(0.), px(-120.))),
                    touch_phase: TouchPhase::Moved,
                    ..Default::default()
                }),
                cx,
            );
            window.draw(cx).clear(cx);
            assert!(
                scroll.offset().y < px(0.),
                "wheel input must scroll the shared demo document"
            );
        })
        .unwrap();
        app.quit();
    }

    #[test]
    fn shared_demo_edits_rich_content_immediately_and_preserves_other_elements() {
        let mut app = TestAppContext::single();
        app.update(liora_components::init_liora);
        let demo = app.update(render);
        let document = app.update(|cx| demo.read(cx).document.clone());
        let input = app.update(|cx| {
            document.update(cx, |editor, cx| {
                assert_eq!(editor.mode(), liora_components::MarkdownEditMode::Rich);
                editor
                    .rich_input_at(SOURCE.find("Click this paragraph").unwrap(), cx)
                    .unwrap()
            })
        });
        let handle = app.open_window(size(px(1200.), px(900.)), |_, _| DemoView(demo));
        app.update_window(handle.into(), |_, window, cx| {
            window.focus(&input.focus_handle(cx), cx);
            window.draw(cx).clear(cx);
        })
        .unwrap();
        app.update(|cx| {
            input.update(cx, |input, cx| {
                input.set_selection(0..0, cx);
                input.insert_text("Edited! ", cx);
            })
        });
        app.update(|cx| input.update(cx, |input, cx| input.insert_text("Now ", cx)));
        app.update_window(handle.into(), |_, window, cx| {
            assert!(
                document
                    .read(cx)
                    .source()
                    .contains("Edited! Now Click this paragraph")
            );
            assert!(
                document
                    .read(cx)
                    .source()
                    .contains("Native **bold**, *emphasis*")
            );
            assert!(
                document
                    .read(cx)
                    .source()
                    .contains("```mermaid\nflowchart LR")
            );
            assert!(
                document
                    .read(cx)
                    .source()
                    .contains("![Local Gallery image](gallery-image)")
            );
            window.draw(cx).clear(cx);
            document.update(cx, |editor, cx| {
                assert!(editor.undo(cx));
                assert!(!editor.source().contains("Edited! Now"));
                assert!(editor.redo(cx));
                assert!(editor.source().contains("Edited! Now"));
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
