use gpui::{
    AnyView, App, Context, Entity, IntoElement, Render, Subscription, Window, div, prelude::*,
};
use liora_components::{DiffDocument, DiffEvent, DiffView};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(DiffDemo::new).into()
}

struct DiffDemo {
    view: Entity<DiffView>,
    status: String,
    _subscription: Subscription,
}
impl DiffDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let view = cx.new(|cx| {
            DiffView::new(
                DiffDocument::new(
                    "fn greet() {\n    println!(\"Hello\");\n}\n",
                    "fn greet() {\n    println!(\"你好, Liora\");\n}\n",
                )
                .paths(Some("before.rs".into()), Some("after.rs".into())),
                cx,
            )
            .expect("small demo input is within diff limits")
            .on_locate(|location, _, cx| {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(format!(
                    "before={:?}, after={:?}",
                    location.before_line, location.after_line
                )));
            })
        });
        let subscription = cx.subscribe(&view, |this: &mut Self, _, event: &DiffEvent, cx| {
            this.status = format!("Host received {event:?}. Source text is unchanged.");
            cx.notify();
        });
        Self {
            view,
            status: "Accept/reject records review state only. Locate copies source coordinates."
                .into(),
            _subscription: subscription,
        }
    }
}
impl Render for DiffDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .child("Native UTF-8 diff review")
            .child(self.status.clone())
            .child(self.view.clone())
    }
}
