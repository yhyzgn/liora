use std::time::Duration;

use gpui::{AnyView, App, Context, Render, SharedString, Task, Window, prelude::*};
use liora_components::{Button, Popover, Space, Text, Tooltip};

#[derive(Clone, Copy)]
pub(super) enum Kind {
    Tooltip,
    Popover,
}

pub(super) fn render(kind: Kind, cx: &mut App) -> AnyView {
    cx.new(|_| OverlayLifecycle {
        kind,
        id: liora_core::unique_id("overlay-lifecycle"),
        shifted: false,
        mounted: true,
        pending: None,
    })
    .into()
}

struct OverlayLifecycle {
    kind: Kind,
    id: SharedString,
    shifted: bool,
    mounted: bool,
    pending: Option<Task<()>>,
}

impl OverlayLifecycle {
    fn schedule(&mut self, remove: bool, cx: &mut Context<Self>) {
        let executor = cx.background_executor().clone();
        self.pending = Some(cx.spawn(async move |this, cx| {
            executor.timer(Duration::from_secs(2)).await;
            let _ = this.update(cx, |this, cx| {
                if remove {
                    this.mounted = false;
                } else {
                    this.shifted = !this.shifted;
                }
                cx.notify();
            });
        }));
    }

    fn reset(&mut self, cx: &mut Context<Self>) {
        self.pending = None;
        self.shifted = false;
        self.mounted = true;
        cx.notify();
    }
}

impl Render for OverlayLifecycle {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut anchor = Space::new().gap_md();
        if self.shifted {
            anchor = anchor.child(Text::new("Shifted anchor →"));
        }
        if self.mounted {
            let trigger = Button::new("Lifecycle trigger");
            anchor = anchor.child(match self.kind {
                Kind::Tooltip => Tooltip::new(trigger)
                    .id(self.id.clone())
                    .content("Lifecycle help")
                    .into_any_element(),
                Kind::Popover => Popover::new(trigger)
                    .id(self.id.clone())
                    .content(|_, _| Text::new("Lifecycle card"))
                    .into_any_element(),
            });
        } else {
            anchor = anchor.child(Text::new("Trigger removed"));
        }
        Space::new()
            .vertical()
            .gap_md()
            .child(Text::new(
                "Schedule a change, then hover or focus/open the trigger within 2 seconds.",
            ))
            .child(
                Space::new()
                    .gap_sm()
                    .wrap()
                    .child(
                        Button::new("Move in 2 s").on_click(cx.listener(|this, _, _, cx| {
                            this.schedule(false, cx);
                        })),
                    )
                    .child(
                        Button::new("Remove in 2 s").on_click(cx.listener(|this, _, _, cx| {
                            this.schedule(true, cx);
                        })),
                    )
                    .child(
                        Button::new("Reset trigger").on_click(cx.listener(|this, _, _, cx| {
                            this.reset(cx);
                        })),
                    ),
            )
            .child(anchor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacement_and_reset_cancel_scheduled_mutations() {
        for kind in [Kind::Tooltip, Kind::Popover] {
            let mut app = gpui::TestAppContext::single();
            let demo = app.update(|cx| {
                cx.new(|_| OverlayLifecycle {
                    kind,
                    id: liora_core::unique_id("overlay-lifecycle-test"),
                    shifted: false,
                    mounted: true,
                    pending: None,
                })
            });
            demo.update(&mut app, |demo, cx| {
                demo.schedule(false, cx);
                demo.schedule(true, cx);
            });
            app.executor().run_until_parked();
            app.executor().advance_clock(Duration::from_secs(3));
            app.executor().run_until_parked();
            demo.read_with(&app, |demo, _| {
                assert!(!demo.mounted, "replacement removal must execute");
                assert!(!demo.shifted, "replaced movement must not execute");
            });
            demo.update(&mut app, |demo, cx| {
                demo.reset(cx);
                demo.schedule(true, cx);
                demo.reset(cx);
            });
            app.executor().run_until_parked();
            app.executor().advance_clock(Duration::from_secs(3));
            app.executor().run_until_parked();
            demo.read_with(&app, |demo, _| {
                assert!(demo.mounted, "reset must cancel pending removal");
                assert!(!demo.shifted);
            });
            demo.update(&mut app, |demo, cx| demo.schedule(false, cx));
            app.executor().run_until_parked();
            app.executor().advance_clock(Duration::from_secs(3));
            app.executor().run_until_parked();
            demo.read_with(&app, |demo, _| {
                assert!(demo.mounted);
                assert!(demo.shifted, "fresh movement must still execute");
            });
            app.quit();
        }
    }
}
