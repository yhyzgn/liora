use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide};
use liora_components::{Button, MarkdownImage, MarkdownView, Space, Text};

const SOURCE: &str = "# Streaming Markdown\n\nNative **bold**, *emphasis*, ~~strike~~ and `inline code`. [GPUI](https://github.com/zed-industries/zed).\n\n> Text remains selectable while streaming.\n\n1. First item\n2. Second item\n\n| Feature | State |\n| --- | --- |\n| Native tables | Ready |\n\n![Host approved image](gallery-image)\n\n```rust\nlet streamed = ";

pub struct MarkdownDemo {
    document: Entity<MarkdownView>,
    chunks: usize,
}

pub fn render(cx: &mut App) -> Entity<MarkdownDemo> {
    cx.new(|cx| {
        let document = cx.new(|cx| {
            let mut view = MarkdownView::new(SOURCE, cx);
            view.set_image_resolver(
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
            view
        });
        MarkdownDemo {
            document,
            chunks: 0,
        }
    })
}

impl Render for MarkdownDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weak = cx.weak_entity();
        let reset = cx.weak_entity();
        page("Markdown", "Reusable streaming native Markdown; no Docs directives or automatic network images.",
            section("Cached streaming document", "Append preserves IDs and parses one batch at the next render. Use Select across blocks for a continuous native copy range.",
                showcase_card_wide("Live document", "The final code fence intentionally starts unfinished.",
                    Space::new().vertical().gap_md()
                        .child(Space::new().child(Button::new("Append chunk").on_click(move |_, _, cx| {
                            let _ = weak.update(cx, |demo, cx| {
                                demo.chunks += 1;
                                let chunk = format!("{};\n```\n\nChunk **{}** arrived.\n\n```rust\nlet streamed = ", demo.chunks, demo.chunks);
                                demo.document.update(cx, |view, cx| view.append(&chunk, cx));
                                cx.notify();
                            });
                        })).child(Button::new("Reset").on_click(move |_, _, cx| {
                            let _ = reset.update(cx, |demo, cx| {
                                demo.chunks = 0;
                                demo.document.update(cx, |view, cx| view.set_source(SOURCE, cx));
                                cx.notify();
                            });
                        })))
                        .child(Text::new(format!("Appended chunks: {}", self.chunks)))
                        .child(self.document.clone())
                )
            )
        )
    }
}
