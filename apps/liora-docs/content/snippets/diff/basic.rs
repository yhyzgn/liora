//! Native read-only diff with real host event subscription.
use gpui::{AppContext, Context, Entity, Render, Subscription, Window};
use liora_components::{DiffDocument, DiffEvent, DiffView};

struct Review {
    diff: Entity<DiffView>,
    _subscription: Subscription,
}
impl Review {
    fn new(cx: &mut Context<Self>) -> Self {
        let diff = cx.new(|cx| {
            DiffView::new(
                DiffDocument::new("hello\n", "你好\n")
                    .paths(Some("before.txt".into()), Some("after.txt".into())),
                cx,
            )
            .expect("small example input fits the explicit diff limits")
        });
        let subscription = cx.subscribe(&diff, |_: &mut Self, _, event: &DiffEvent, _| {
            // The host owns persistence; a review decision never applies a patch.
            println!("{event:?}");
        });
        Self {
            diff,
            _subscription: subscription,
        }
    }
}
impl Render for Review {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl gpui::IntoElement {
        self.diff.clone()
    }
}
fn main() {}
