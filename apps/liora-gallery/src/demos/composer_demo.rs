use gpui::{AnyView, App, Context, Entity, Render, Subscription, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    Button, Card, Composer, ComposerAttachment, ComposerCandidate, ComposerEvent,
    ComposerReference, Space, Text,
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
            _subscription: subscription,
        }
    })
    .into()
}
struct ComposerDemo {
    composer: Entity<Composer>,
    status: String,
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
                .child(Text::new(self.status.clone())))
    }
}
