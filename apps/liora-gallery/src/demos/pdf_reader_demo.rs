use gpui::{
    AnyView, App, Context, Entity, IntoElement, PathPromptOptions, Render, Task, Window, div,
    prelude::*,
};
use liora_components::{Button, PdfDocument, PdfReader};
use std::{fs::File, io::Read, path::PathBuf};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(PdfReaderDemo::new).into()
}

struct PdfReaderDemo {
    reader: Entity<PdfReader>,
    loading: bool,
    error: Option<String>,
    load_task: Option<Task<()>>,
}

impl PdfReaderDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut demo = Self {
            reader: cx.new(|cx| PdfReader::new(PdfDocument::new("Loading PDF", vec![]), cx)),
            loading: false,
            error: None,
            load_task: None,
        };
        demo.load_document(None, cx);
        demo
    }

    fn load_document(&mut self, path: Option<PathBuf>, cx: &mut Context<Self>) {
        self.loading = true;
        self.error = None;
        let decode = cx.background_executor().spawn(async move {
            let (title, bytes) = if let Some(path) = path {
                let title = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let mut bytes = Vec::new();
                File::open(&path)
                    .and_then(|file| file.take(64 * 1024 * 1024 + 1).read_to_end(&mut bytes))
                    .map_err(|error| format!("Could not read PDF: {error}"))?;
                (title, bytes)
            } else {
                (
                    "Liora native PDF sample".to_owned(),
                    include_bytes!("../../../../assets/samples/reader.pdf").to_vec(),
                )
            };
            PdfDocument::from_bytes(title, bytes).map_err(|error| error.to_string())
        });
        self.load_task = Some(cx.spawn(async move |this, cx| {
            let result = decode.await;
            let _ = this.update(cx, |demo, cx| {
                demo.loading = false;
                match result {
                    Ok(document) => demo
                        .reader
                        .update(cx, |reader, cx| reader.set_document(document, cx)),
                    Err(error) => demo.error = Some(error),
                }
                cx.notify();
            });
        }));
        cx.notify();
    }

    fn open_pdf(&mut self, cx: &mut Context<Self>) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Open PDF".into()),
        });
        self.loading = true;
        self.error = None;
        cx.spawn(async move |this, cx| {
            let result = receiver.await;
            let _ = this.update(cx, |demo, cx| {
                demo.loading = false;
                match result {
                    Ok(Ok(Some(mut paths))) if !paths.is_empty() => {
                        demo.load_document(Some(paths.remove(0)), cx)
                    }
                    Ok(Ok(_)) => {}
                    Ok(Err(error)) => {
                        demo.error = Some(format!("Could not open file chooser: {error}"))
                    }
                    Err(error) => demo.error = Some(format!("File chooser disconnected: {error}")),
                }
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }
}

impl Render for PdfReaderDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h(gpui::px(640.0))
            .min_h(gpui::px(0.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("Open PDF")
                            .disabled(self.loading)
                            .on_click(cx.listener(|demo, _, _, cx| demo.open_pdf(cx))),
                    )
                    .child(if self.loading {
                        "Loading PDF…"
                    } else {
                        "Native PDF · files decoded on a background worker"
                    }),
            )
            .when_some(self.error.clone(), |view, error| view.child(error))
            .child(
                div()
                    .flex_1()
                    .min_h(gpui::px(0.0))
                    .child(self.reader.clone()),
            )
    }
}
