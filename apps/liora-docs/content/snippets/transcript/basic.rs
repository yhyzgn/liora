use gpui::{App, Entity, IntoElement, div, prelude::*};
use liora_components::{Text, Transcript, TranscriptError, TranscriptPosition};

pub fn build_transcript(cx: &mut App) -> Result<Entity<Transcript>, TranscriptError> {
    let view = cx.new(|_| {
        Transcript::new(|id, _, _| {
            div()
                .p_3()
                .child(Text::new(id.clone()).wrap())
                .into_any_element()
        })
    });
    view.update(cx, |view, cx| {
        view.set_items(vec!["first".into(), "second".into()])?;
        cx.notify();
        Ok::<(), TranscriptError>(())
    })?;
    Ok(view)
}

pub fn restore(
    view: &Entity<Transcript>,
    position: &TranscriptPosition,
    cx: &mut App,
) -> Result<(), TranscriptError> {
    view.update(cx, |view, cx| {
        view.restore_position(position)?;
        cx.notify();
        Ok(())
    })
}
