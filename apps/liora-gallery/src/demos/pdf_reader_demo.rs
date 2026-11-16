use gpui::{
    AnyView, App, Context, Entity, IntoElement, PathPromptOptions, Render, Task, Window, div,
    prelude::*,
};
use liora_components::{Button, PdfDecoder, PdfDocument, PdfReader, PdfReaderEvent};
use std::{fs::File, io::Read, path::PathBuf};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(PdfReaderDemo::new).into()
}

pub fn render_filling(cx: &mut App) -> AnyView {
    cx.new(|cx| PdfReaderDemo::new_with_height(cx, true)).into()
}

struct PdfReaderDemo {
    reader: Entity<PdfReader>,
    fill_height: bool,
    loading: bool,
    error: Option<String>,
    load_task: Option<Task<()>>,
    decoder: Option<PdfDecoder>,
    requested_pages: Vec<usize>,
    page_task: Option<Task<()>>,
    reader_subscription: Option<gpui::Subscription>,
    previous_reader: Option<Entity<PdfReader>>,
    previous_decoder: Option<PdfDecoder>,
    previous_subscription: Option<gpui::Subscription>,
    previous_requested_pages: Vec<usize>,
    pending_load: Option<Option<PathBuf>>,
}

impl PdfReaderDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self::new_with_height(cx, false)
    }

    fn new_with_height(cx: &mut Context<Self>, fill_height: bool) -> Self {
        let mut demo = Self {
            reader: cx.new(|cx| PdfReader::new(PdfDocument::new("Loading PDF", vec![]), cx)),
            fill_height,
            loading: false,
            error: None,
            load_task: None,
            decoder: None,
            requested_pages: Vec::new(),
            page_task: None,
            reader_subscription: None,
            previous_reader: None,
            previous_decoder: None,
            previous_subscription: None,
            previous_requested_pages: Vec::new(),
            pending_load: None,
        };
        demo.load_document(None, cx);
        demo
    }

    fn load_document(&mut self, path: Option<PathBuf>, cx: &mut Context<Self>) {
        self.loading = true;
        self.error = None;
        if self.page_task.is_some() {
            self.pending_load = Some(path);
            cx.notify();
            return;
        }
        let previous_reader = self.reader.clone();
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
            let decoder =
                PdfDecoder::new(title.into(), bytes).map_err(|error| error.to_string())?;
            let mut document = decoder.document();
            document.pages[0] = decoder.render_page(1).map_err(|error| error.to_string())?;
            Ok::<_, String>((decoder, document))
        });
        self.load_task = Some(cx.spawn(async move |this, cx| {
            let result = decode.await;
            let _ = this.update(cx, |demo, cx| {
                demo.loading = false;
                match result {
                    Ok((decoder, document)) => {
                        demo.previous_decoder = demo.decoder.replace(decoder);
                        demo.previous_subscription = demo.reader_subscription.take();
                        demo.previous_requested_pages = std::mem::take(&mut demo.requested_pages);
                        demo.previous_reader = Some(previous_reader);
                        demo.show_preview(document, cx);
                        demo.reader_subscription = Some(cx.subscribe(
                            &demo.reader,
                            |demo, _, event: &PdfReaderEvent, cx| {
                                if let PdfReaderEvent::PagesRequested { pages } = event {
                                    demo.requested_pages = pages.clone();
                                    demo.request_page(cx);
                                }
                            },
                        ));
                    }
                    Err(error) => demo.error = Some(error),
                }
                demo.request_page(cx);
                cx.notify();
            });
        }));
        cx.notify();
    }

    fn show_preview(&mut self, document: PdfDocument, cx: &mut Context<Self>) {
        self.reader = cx.new(|cx| PdfReader::new(document, cx));
        cx.notify();
    }

    fn fail_page(&mut self, error: String, cx: &mut Context<Self>) {
        self.loading = false;
        self.error = Some(error);
        self.decoder = self.previous_decoder.take();
        if let Some(previous) = self.previous_reader.take() {
            self.reader = previous;
        }
        self.reader_subscription = self.previous_subscription.take();
        self.requested_pages = std::mem::take(&mut self.previous_requested_pages);
        cx.notify();
    }

    fn request_page(&mut self, cx: &mut Context<Self>) {
        if self.loading || self.page_task.is_some() || self.decoder.is_none() {
            return;
        }
        let page = self.requested_pages.iter().copied().find(|page| {
            self.reader
                .read(cx)
                .document()
                .pages
                .get(page - 1)
                .is_some_and(|page| page.image.is_none())
        });
        let Some(page) = page else {
            return;
        };
        let decoder = self.decoder.take().unwrap();
        let decode = cx.background_executor().spawn(async move {
            let result = decoder.render_page(page).map_err(|error| error.to_string());
            (decoder, result)
        });
        self.page_task = Some(cx.spawn(async move |this, cx| {
            let (decoder, result) = decode.await;
            let _ = this.update(cx, |demo, cx| {
                demo.page_task = None;
                demo.decoder = Some(decoder);
                match result {
                    Ok(page) => {
                        demo.reader
                            .update(cx, |reader, cx| reader.set_decoded_page(page, cx));
                    }
                    Err(error) => demo.fail_page(error, cx),
                }
                if let Some(path) = demo.pending_load.take() {
                    demo.load_document(path, cx);
                    return;
                }
                demo.request_page(cx);
                cx.notify();
            });
        }));
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
                demo.request_page(cx);
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
            .when(self.fill_height, |view| view.h_full())
            .when(!self.fill_height, |view| view.h(gpui::px(640.0)))
            .min_h(gpui::px(0.0))
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("Open PDF")
                            .disabled(self.loading)
                            .on_click(cx.listener(|demo, _, _, cx| demo.open_pdf(cx))),
                    )
                    .child(if self.loading {
                        "Loading first PDF page…"
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, TestAppContext};
    use liora_components::{PdfPageSize, PdfZoom, init_liora};

    #[test]
    fn failed_lazy_page_restores_previous_reader_state_and_decoder() {
        let mut app = TestAppContext::single();
        app.update(init_liora);
        let demo = app.new(|cx| PdfReaderDemo {
            reader: cx.new(|cx| {
                PdfReader::new(
                    PdfDocument::with_page_count("Previous PDF", 2, PdfPageSize::new(600.0, 800.0)),
                    cx,
                )
            }),
            fill_height: false,
            loading: true,
            error: None,
            load_task: None,
            decoder: None,
            requested_pages: Vec::new(),
            page_task: None,
            reader_subscription: None,
            previous_reader: None,
            previous_decoder: None,
            previous_subscription: None,
            previous_requested_pages: Vec::new(),
            pending_load: None,
        });
        demo.update(&mut app, |demo, cx| {
            demo.page_task = Some(cx.spawn(async move |_, _| std::future::pending::<()>().await));
            let replacement = PathBuf::from("pending-replacement.pdf");
            demo.load_document(Some(replacement.clone()), cx);
            assert_eq!(demo.pending_load, Some(Some(replacement)));
            assert!(
                demo.load_task.is_none(),
                "replacement must not start another worker while a page is rendering"
            );
            demo.page_task = None;
            demo.pending_load = None;
            let previous = demo.reader.clone();
            previous.update(cx, |reader, cx| {
                reader.set_page(2, cx);
                reader.set_zoom(PdfZoom::Percent(150), cx);
            });
            demo.show_preview(
                PdfDocument::with_page_count("New PDF preview", 3, PdfPageSize::new(400.0, 300.0)),
                cx,
            );
            assert_ne!(demo.reader.entity_id(), previous.entity_id());
            assert!(demo.loading);
            demo.previous_reader = Some(previous.clone());
            demo.previous_decoder = Some(
                PdfDecoder::new(
                    "Previous PDF".into(),
                    include_bytes!("../../../../assets/samples/reader.pdf").to_vec(),
                )
                .unwrap(),
            );
            demo.previous_requested_pages = vec![1, 2];
            demo.fail_page("Later page failed".to_owned(), cx);
            assert_eq!(demo.reader.entity_id(), previous.entity_id());
            assert_eq!(
                demo.reader.read(cx).document().title.as_ref(),
                "Previous PDF"
            );
            assert_eq!(demo.reader.read(cx).current_page(), 2);
            assert_eq!(demo.reader.read(cx).zoom(), PdfZoom::Percent(150));
            assert_eq!(demo.error.as_deref(), Some("Later page failed"));
            assert!(!demo.loading);
            assert_eq!(
                demo.requested_pages,
                vec![1, 2],
                "rollback preserves the whole continuous visible range"
            );
            let restored = demo
                .decoder
                .as_ref()
                .expect("rollback preserves the lazy decoding session");
            assert!(restored.render_page(2).unwrap().image.is_some());
        });
    }
}
