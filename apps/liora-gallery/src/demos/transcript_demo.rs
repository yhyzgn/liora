use std::{cell::RefCell, collections::HashMap, rc::Rc};

use gpui::{App, Context, Entity, IntoElement, Render, SharedString, Window, div, prelude::*, px};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    Button, SelectableText, SelectableTextOptions, Space, Text, Transcript, TranscriptPosition,
};
use liora_core::Config;

type Messages = Rc<RefCell<HashMap<SharedString, SharedString>>>;

pub fn render(cx: &mut App) -> Entity<TranscriptDemo> {
    cx.new(TranscriptDemo::new)
}

pub struct TranscriptDemo {
    transcript: Entity<Transcript>,
    messages: Messages,
    sequence: usize,
    saved: Option<TranscriptPosition>,
    status: SharedString,
}

impl TranscriptDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let messages: Messages = Rc::new(RefCell::new(HashMap::new()));
        let items: Vec<SharedString> = (0..40)
            .map(|index| format!("message-{index}").into())
            .collect();
        for (index, id) in items.iter().enumerate() {
            messages.borrow_mut().insert(
                id.clone(),
                format!(
                    "Native transcript row {}. {}",
                    index + 1,
                    "Select and copy this text; scrolling up detaches from the growing tail. "
                        .repeat(index % 4 + 1),
                )
                .into(),
            );
        }
        let content = messages.clone();
        let transcript = cx.new(|_| {
            let mut view = Transcript::new(move |id, window, cx| {
                let text = content.borrow().get(id).cloned().unwrap_or_default();
                let mut options = SelectableTextOptions::new(id.clone(), text);
                options.text_color = cx.global::<Config>().theme.neutral.text_1;
                div()
                    .p_3()
                    .child(
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .child(Text::new(id.clone()).bold())
                            .child(SelectableText::view(options, window, cx)),
                    )
                    .into_any_element()
            });
            view.set_items(items)
                .expect("generated transcript IDs are unique");
            view
        });
        Self {
            transcript,
            messages,
            sequence: 40,
            saved: None,
            status: "Scroll up before appending to verify detachment.".into(),
        }
    }

    fn insert(&mut self, prepend: bool, cx: &mut Context<Self>) {
        let id: SharedString = format!("message-{}", self.sequence).into();
        self.sequence += 1;
        self.messages.borrow_mut().insert(
            id.clone(),
            "New content supplied by the host.\nVariable-height native row.".into(),
        );
        let result = self.transcript.update(cx, |view, cx| {
            let mut items = view.items().to_vec();
            items.insert(if prepend { 0 } else { items.len() }, id);
            let result = view.set_items(items);
            cx.notify();
            result
        });
        self.status = match result {
            Ok(()) => "Roster updated; surviving visible IDs retain their offset.".into(),
            Err(error) => error.to_string().into(),
        };
        cx.notify();
    }
}

impl Render for TranscriptDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        page("Transcript 会话滚动", "Stable item identity, native tail following and host-owned session positions.",
            section("Live viewport", "Append, grow, prepend; save and restore after scrolling.",
                Space::new().vertical().gap_md()
                    .child(Space::new().wrap().gap_sm()
                        .child(Button::new("Append").on_click(cx.listener(|this, _, _, cx| this.insert(false, cx))))
                        .child(Button::new("Prepend history").on_click(cx.listener(|this, _, _, cx| this.insert(true, cx))))
                        .child(Button::new("Grow last row").on_click(cx.listener(|this, _, _, cx| {
                            let id = this.transcript.read(cx).items().last().cloned();
                            if let Some(id) = id {
                                if let Some(text) = this.messages.borrow_mut().get_mut(&id) {
                                    *text = format!("{text}\nMore streamed content supplied by the host.").into();
                                }
                                let result = this.transcript.update(cx, |view, cx| {
                                    let result = view.notify_item_changed(&id);
                                    cx.notify();
                                    result
                                });
                                if let Err(error) = result { this.status = error.to_string().into(); }
                            }
                            cx.notify();
                        })))
                        .child(Button::new("Save position").on_click(cx.listener(|this, _, _, cx| {
                            this.saved = Some(this.transcript.read(cx).position());
                            this.status = "Position saved in host memory.".into();
                            cx.notify();
                        })))
                        .child(Button::new("Restore position").on_click(cx.listener(|this, _, _, cx| {
                            if let Some(position) = &this.saved {
                                let result = this.transcript.update(cx, |view, cx| {
                                    let result = view.restore_position(position);
                                    cx.notify();
                                    result
                                });
                                this.status = match result { Ok(()) => "Position restored by item identity.".into(), Err(error) => error.to_string().into() };
                            } else { this.status = "Save a position first.".into(); }
                            cx.notify();
                        })))
                        .child(Button::new("Latest").on_click(cx.listener(|this, _, _, cx| {
                            this.transcript.update(cx, |view, cx| { view.follow_tail(); cx.notify(); });
                        }))))
                    .child(Text::new(self.status.clone()))
                    .child(div().w_full().h(px(420.0)).child(self.transcript.clone())),
            ),
        )
    }
}
