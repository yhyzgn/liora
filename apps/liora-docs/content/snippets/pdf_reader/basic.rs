//! The host supplies bytes and awaits decoding before creating a reader.
use gpui::{App, AppContext, Entity, Task};
use liora_components::{PdfDecodeError, PdfDocument, PdfReader};

// Keep this task alive and await its Result; surface errors in the host UI.
fn decode_pdf(
    title: String,
    bytes: Vec<u8>,
    cx: &App,
) -> Task<Result<PdfDocument, PdfDecodeError>> {
    cx.background_executor()
        .spawn(async move { PdfDocument::from_bytes(title, bytes) })
}

// Call on the UI thread with the successful decode result.
fn create_reader(document: PdfDocument, cx: &mut App) -> Entity<PdfReader> {
    cx.new(|cx| PdfReader::new(document, cx))
}

fn main() {}
