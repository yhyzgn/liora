//! Real Linux XIM acceptance; driven only by scripts/markdown_real_ime_smoke.py.
use gpui::{
    AppContext, Bounds, Context, Entity, Focusable, IntoElement, Render, Window, WindowBounds,
    WindowOptions, point, prelude::*, px, size,
};
use liora_components::{MarkdownEditMode, MarkdownEditor, init_liora_with_mode};
use std::{
    io::Write,
    time::{Duration, Instant},
};

const TITLE: &str = "Liora Real Markdown IME Smoke";

struct Surface {
    editor: Entity<MarkdownEditor>,
}

impl Render for Surface {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;
        gpui::div()
            .size_full()
            .p_3()
            .bg(theme.neutral.body)
            .text_color(theme.neutral.text_1)
            .child(self.editor.clone())
    }
}

fn signal(message: &str) {
    println!("{message}");
    std::io::stdout().flush().expect("flush IME handshake");
}

fn main() {
    assert_eq!(
        std::env::var("LIORA_REAL_IME_ISOLATED").as_deref(),
        Ok("1"),
        "use scripts/markdown_real_ime_smoke.py, never a desktop session"
    );
    gpui_platform::application()
        .with_assets(liora_icons::IconAssetSource)
        .run(|cx| {
            init_liora_with_mode(cx, liora_core::ThemeMode::Light);
            let editor = cx.new(|cx| MarkdownEditor::new("", cx));
            let handle = cx.open_window(WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some(TITLE.into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    point(px(0.), px(0.)), size(px(800.), px(600.)),
                ))),
                show: false,
                focus: true,
                ..Default::default()
            }, |_, cx| cx.new(|_| Surface { editor: editor.clone() })).unwrap();
            cx.update_window(handle.into(), |_, window, _| window.activate_window()).unwrap();
            let executor = cx.background_executor().clone();
            cx.spawn(async move |cx| {
                for (name, original, expected) in [
                    ("paragraph", "- **old**\n\n> untouched\n", "- **你好**\n\n> untouched\n"),
                    ("table", "| A | B |\n| --- | --- |\n| old | 2 |\n", "| A | B |\n| --- | --- |\n| 你好 | 2 |\n"),
                ] {
                    let input = cx.update(|cx| {
                        editor.update(cx, |editor, cx| {
                            editor.set_source(original, cx);
                            editor.set_mode(MarkdownEditMode::Rich, cx);
                        });
                        let input = editor.update(cx, |editor, cx| {
                            editor.rich_input_at(original.find("old").unwrap(), cx).unwrap()
                        });
                        cx.update_window(handle.into(), |_, window, cx| {
                            input.update(cx, |input, cx| input.set_selection(0..3, cx));
                            window.focus(&input.read(cx).focus_handle(cx), cx);
                            window.draw(cx).clear(cx);
                        }).unwrap();
                        input
                    });
                    // Let deferred focus and input-handler installation finish before typing.
                    executor.timer(Duration::from_millis(200)).await;
                    signal(&format!("READY: {name}"));
                    let deadline = Instant::now() + Duration::from_secs(15);
                    let mut saw_preedit = false;
                    loop {
                        executor.timer(Duration::from_millis(20)).await;
                        let committed = cx.update(|cx| {
                            if input.read(cx).is_composing() {
                                assert_eq!(editor.read(cx).source(), original,
                                    "{name}: preedit mutated canonical Markdown");
                                assert!(!input.read(cx).value().is_empty(), "empty preedit");
                                if !saw_preedit {
                                    saw_preedit = true;
                                    signal(&format!("PREEDIT: {name}; canonical unchanged"));
                                }
                                false
                            } else if editor.read(cx).source() != original {
                                assert!(saw_preedit, "{name}: commit arrived without observed OS preedit");
                                assert_eq!(editor.read(cx).source(), expected,
                                    "{name}: real IME must commit 你好 and preserve Markdown");
                                assert_eq!(input.read(cx).value().as_ref(), "你好");
                                signal(&format!("COMMIT: {name}; 你好; Markdown preserved"));
                                true
                            } else {
                                false
                            }
                        });
                        if committed { break; }
                        assert!(Instant::now() < deadline, "{name}: timed out awaiting real XIM preedit/commit");
                    }
                    cx.update(|cx| {
                        editor.update(cx, |editor, cx| assert!(editor.undo(cx)));
                        assert_eq!(editor.read(cx).source(), original, "{name}: undo restores exact source");
                        signal(&format!("UNDO: {name}; original restored"));
                    });
                }
                signal("PASS: real XIM paragraph/table preedit, 你好 commit, Markdown preservation and undo");
                cx.update(|cx| cx.quit());
            }).detach();
        });
}
