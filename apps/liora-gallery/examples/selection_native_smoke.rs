//! Real-display selection regression; preserves GPUI-readable clipboard contents.
//! Run with a disposable clipboard if it contains formats outside GPUI ClipboardItem.
//! cargo run -p liora-gallery --example selection_native_smoke --locked
use gpui::{
    App, AppContext, Bounds, ClipboardItem, Context, IntoElement, Keystroke, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, PlatformInput, Point, Render, TextAlign, Window,
    WindowBounds, WindowOptions, div, point, prelude::*, px, size,
};
use liora_components::{MarkdownView, SelectableText, SelectableTextOptions, init_liora};
use std::{
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    time::Duration,
};

struct AlignedText(TextAlign);
impl Render for AlignedText {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut options = SelectableTextOptions::new("aligned-native-selection", "hello");
        options.text_align = self.0;
        div()
            .w(px(320.))
            .child(SelectableText::view(options, window, cx))
    }
}

fn drag(window: &mut Window, cx: &mut App, start: Point<gpui::Pixels>, end: Point<gpui::Pixels>) {
    window.dispatch_event(
        PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position: start,
            click_count: 1,
            ..Default::default()
        }),
        cx,
    );
    window.dispatch_event(
        PlatformInput::MouseMove(MouseMoveEvent {
            position: end,
            pressed_button: Some(MouseButton::Left),
            ..Default::default()
        }),
        cx,
    );
    window.dispatch_event(
        PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position: end,
            click_count: 1,
            ..Default::default()
        }),
        cx,
    );
}
fn copied_text(window: &mut Window, cx: &mut App) -> Option<String> {
    window.dispatch_keystroke(Keystroke::parse("ctrl-c").unwrap(), cx);
    cx.read_from_clipboard().and_then(|item| item.text())
}
fn options() -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::new(
            point(px(0.), px(0.)),
            size(px(640.), px(400.)),
        ))),
        ..Default::default()
    }
}
fn main() {
    gpui_platform::application().with_assets(liora_icons::IconAssetSource).run(|cx| {
        init_liora(cx);
        let markdown = cx.new(|cx| MarkdownView::new("# Heading\n\nFirst **block**.\n\nLast block.", cx));
        markdown.update(cx, |view, cx| view.set_continuous_selection(true, cx));
        let markdown_window = cx.open_window(options(), |_, _| markdown.clone()).unwrap();
        let aligned = [TextAlign::Center, TextAlign::Right].map(|align| {
            (cx.open_window(options(), |_, cx| cx.new(|_| AlignedText(align))).unwrap(), align)
        });
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(50)).await;
            cx.update(|cx| {
                let previous = cx.read_from_clipboard();
                let result = catch_unwind(AssertUnwindSafe(|| {
                    cx.update_window(markdown_window.into(), |_, window, cx| {
                        window.draw(cx).clear(cx);
                        let expected = markdown.read(cx).document().plain_text();
                        drag(window, cx, point(px(12.), px(46.)), point(px(12.), px(46.)));
                        window.dispatch_keystroke(Keystroke::parse("ctrl-a").unwrap(), cx);
                        let copied = copied_text(window, cx).expect("continuous selection must copy text");
                        assert!(copied.starts_with("Heading\n") && copied.contains("First block.") && copied.ends_with("Last block.\n"));
                        assert_eq!(copied, expected, "Select All must span separate Markdown blocks");
                    }).unwrap();
                    for (handle, align) in aligned {
                        cx.update_window(handle.into(), |_, window, cx| {
                            window.draw(cx).clear(cx);
                            let runs = [window.text_style().to_run("hello".len())];
                            let shaped = window.text_system().shape_text("hello".into(), px(14.), &runs, None, None).unwrap();
                            let line = &shaped[0];
                            let free = px(320.) - line.width();
                            let left = if align == TextAlign::Center { free / 2. } else { free };
                            drag(window, cx, point(left + px(0.01), px(11.)), point(left + line.unwrapped_layout.x_for_index(2), px(11.)));
                            assert_eq!(copied_text(window, cx).as_deref(), Some("he"), "aligned pointer selection must follow the painted glyphs");
                        }).unwrap();
                    }
                }));
                cx.write_to_clipboard(previous.unwrap_or(ClipboardItem { entries: Vec::new() }));
                if let Err(panic) = result { resume_unwind(panic); }
                println!("PASS: real-display Markdown cross-block copy and centered/right-aligned pointer selection; prior GPUI clipboard restored");
                cx.quit();
            });
        }).detach();
    });
}
