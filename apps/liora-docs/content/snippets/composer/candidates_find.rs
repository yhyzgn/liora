//! Host-supplied candidates and find events, matching the live Composer companion section.

use gpui::{App, Context, Entity, Render, Subscription, Window, prelude::*};
use liora_components::{
    Button, CandidateBar, CandidateBarEvent, CandidateItem, FindBar, FindBarEvent, Space, Text,
};

fn companions(cx: &mut App) -> Entity<CompanionDemo> {
    cx.new(CompanionDemo::new)
}

struct CompanionDemo {
    candidates: Entity<CandidateBar>,
    find: Entity<FindBar>,
    candidate_status: String,
    find_status: String,
    find_visible: bool,
    restore_candidate_focus: bool,
    _subscriptions: Vec<Subscription>,
}

impl CompanionDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let candidates = cx.new(CandidateBar::new);
        candidates.update(cx, |bar, cx| {
            bar.set_composition("design", cx);
            bar.page_size(3, cx);
            bar.set_items(
                [
                    "Design brief",
                    "Design review",
                    "Design tokens",
                    "Design system",
                    "Design notes",
                ]
                .into_iter()
                .map(CandidateItem::new)
                .collect(),
                cx,
            );
        });
        let find = cx.new(FindBar::new);
        let candidate_subscription = cx.subscribe(&candidates, |this, _, event, cx| {
            this.candidate_status = match event {
                CandidateBarEvent::Selected { index, value } => {
                    format!("Selected {}: {}", index + 1, value)
                }
                CandidateBarEvent::CommittedLiteral { composition } => {
                    format!("Literal: {composition}")
                }
                CandidateBarEvent::Cancelled => "Candidates cancelled".into(),
                CandidateBarEvent::PageChanged { page } => format!("Candidate page: {}", page + 1),
            };
            cx.notify();
        });
        let find_subscription = cx.subscribe(&find, |this, _, event, cx| {
            this.find_status = match event {
                FindBarEvent::QueryChanged { query } => format!("Find query: {query}"),
                FindBarEvent::Previous => "Find previous requested".into(),
                FindBarEvent::Next => "Find next requested".into(),
                FindBarEvent::Closed => {
                    this.find_visible = false;
                    this.restore_candidate_focus = true;
                    "Find closed".into()
                }
            };
            cx.notify();
        });
        Self {
            candidates,
            find,
            candidate_status: "No candidate event".into(),
            find_status: "No find event".into(),
            find_visible: true,
            restore_candidate_focus: false,
            _subscriptions: vec![candidate_subscription, find_subscription],
        }
    }
}

impl Render for CompanionDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if std::mem::take(&mut self.restore_candidate_focus) {
            self.candidates.update(cx, |bar, cx| bar.focus(window, cx));
        }
        Space::new()
            .vertical()
            .gap_sm()
            .child(
                Button::new("Focus candidates").on_click(cx.listener(|this, _, window, cx| {
                    this.candidates.update(cx, |bar, cx| bar.focus(window, cx));
                })),
            )
            .child(self.candidates.clone())
            .child(Text::new(self.candidate_status.clone()))
            .child(
                Button::new("Open find").on_click(cx.listener(|this, _, window, cx| {
                    this.find_visible = true;
                    this.find.update(cx, |bar, cx| bar.focus(window, cx));
                    cx.notify();
                })),
            )
            .when(self.find_visible, |space| space.child(self.find.clone()))
            .child(Text::new(self.find_status.clone()))
    }
}

fn main() {}
