use gpui::{AnyView, App, Context, Entity, Render, Subscription, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    Button, CandidateBar, CandidateBarEvent, CandidateItem, Card, Composer, ComposerAttachment,
    ComposerCandidate, ComposerEvent, ComposerReference, FindBar, FindBarEvent, Space, Text,
};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let composer = cx.new(Composer::new);
        composer.update(cx, |composer, cx| {
            composer.set_candidates(
                vec![
                    ComposerCandidate::Reference(ComposerReference::new("docs", "Documentation")),
                    ComposerCandidate::Reference(ComposerReference::new("design", "Design notes")),
                    ComposerCandidate::Command {
                        id: "summarize".into(),
                        label: "summarize".into(),
                        text: "/summarize ".into(),
                    },
                ],
                cx,
            );
        });
        let subscription =
            cx.subscribe(&composer, |this: &mut ComposerDemo, composer, event, cx| {
                match event {
                    ComposerEvent::Send(draft) => {
                        this.status = format!(
                            "Host received: {} ({} references, {} attachments)",
                            draft.text,
                            draft.references.len(),
                            draft.attachments.len()
                        );
                        composer.update(cx, |composer, cx| composer.set_busy(true, cx));
                    }
                    ComposerEvent::Stop => {
                        this.status = "Host received Stop; next draft is preserved".into();
                        composer.update(cx, |composer, cx| composer.set_busy(false, cx));
                    }
                    ComposerEvent::Paste(item) => {
                        this.status = format!(
                            "Host received {} native clipboard entries",
                            item.entries().len()
                        );
                    }
                    ComposerEvent::DraftChanged(_) => {}
                }
                cx.notify();
            });
        ComposerDemo {
            composer,
            status: "No request is executed by this demo".into(),
            companions: cx.new(CompanionDemo::new),
            _subscription: subscription,
        }
    })
    .into()
}
struct ComposerDemo {
    composer: Entity<Composer>,
    status: String,
    companions: Entity<CompanionDemo>,
    _subscription: Subscription,
}
impl Render for ComposerDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        page("Composer 草稿编辑", "原生 Input、结构化引用/附件与宿主发送/停止事件。",
            Space::new().vertical().gap_lg()
                .child(section("Draft", "Enter 发送；Shift-Enter 换行；@ 与 / 支持方向键和 Enter。发送后可继续编辑下一条草稿。",
                    Card::new(self.composer.clone())))
                .child(Button::new("Host: attach design.txt").on_click(cx.listener(|this, _, _, cx| {
                    this.composer.update(cx, |composer, cx| composer.add_attachment(
                        ComposerAttachment::new("design-file", "design.txt"), cx));
                })))
                .child(Button::new("Host: complete request").on_click(cx.listener(|this, _, _, cx| {
                    this.composer.update(cx, |composer, cx| composer.set_busy(false, cx));
                    this.status = "Host completed request; draft preserved".into();
                    cx.notify();
                })))
                .child(Text::new(self.status.clone()))
                .child(section("CandidateBar + FindBar", "Host-provided candidates and find events; no system IME or document index is installed.", self.companions.clone())))
    }
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
