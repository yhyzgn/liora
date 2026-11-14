use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Subscription, Window};
use liora_components::{MarkdownChanged, MarkdownImage, MarkdownView, Transcript};

pub fn streaming_document(cx: &mut App) -> Entity<MarkdownView> {
    cx.new(|cx| MarkdownView::new("# Native document\n\nReady for **streaming**.", cx))
}

pub fn append_batch(view: &Entity<MarkdownView>, cx: &mut App) {
    view.update(cx, |view, cx| {
        view.append("\n\n```rust\n", cx);
        view.append("let value = 42;\n", cx);
        view.append("```\n", cx);
        // All three chunks are parsed together at the next render.
    });
}

pub fn enable_approved_asset(view: &Entity<MarkdownView>, cx: &mut App) {
    view.update(cx, |view, cx| {
        view.set_image_resolver(
            |destination| {
                // This documentation example uses the actual Gallery fixture. A product
                // should map an explicit asset ID to its own approved path instead.
                (destination == "approved-logo").then(|| {
                    MarkdownImage::File(
                        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                            .join("../liora-gallery/assets/local.jpeg"),
                    )
                })
            },
            cx,
        );
    });
}

/// Retain this host as an Entity; dropping the host intentionally releases its subscription.
pub struct MarkdownTranscriptExample {
    pub markdown: Entity<MarkdownView>,
    transcript: Entity<Transcript>,
    _markdown_changes: Subscription,
}

impl MarkdownTranscriptExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let markdown = cx.new(|cx| MarkdownView::new("# Message\n\nStreaming content.", cx));
        let row = markdown.clone();
        let transcript = cx.new(|_| {
            let mut transcript = Transcript::new(move |_, _, _| row.clone().into_any_element());
            transcript
                .set_items(vec!["message".into()])
                .expect("one unique message ID");
            transcript
        });
        let measured = transcript.clone();
        let subscription = cx.subscribe(&markdown, move |_, _, _: &MarkdownChanged, cx| {
            let measured = measured.clone();
            // Run after the current render/layout effect, not recursively inside it.
            cx.defer(move |cx| {
                measured.update(cx, |transcript, cx| {
                    transcript
                        .notify_item_changed("message")
                        .expect("host retains the message row");
                    cx.notify();
                })
            });
        });
        Self {
            markdown,
            transcript,
            _markdown_changes: subscription,
        }
    }
}

impl Render for MarkdownTranscriptExample {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.transcript.clone()
    }
}
