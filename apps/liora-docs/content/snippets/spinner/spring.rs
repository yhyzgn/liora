//! A retained native spring with retargeting and explicit unmount/remount.
use gpui::{AnimationExt, AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::{Button, MotionSpring, Space, Text};

pub fn spring(cx: &mut App) -> AnyView {
    cx.new(|_| SpringDemo {
        expanded: false,
        mounted: true,
    })
    .into()
}

struct SpringDemo {
    expanded: bool,
    mounted: bool,
}

impl Render for SpringDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let gap = px(if self.expanded { 48.0 } else { 8.0 });
        Space::new()
            .vertical()
            .gap_md()
            .child(
                Space::new()
                    .gap_md()
                    .wrap()
                    .child(Button::new("Toggle spring target").on_click(cx.listener(
                        |this, _, _, cx| {
                            this.expanded = !this.expanded;
                            cx.notify();
                        },
                    )))
                    .child(
                        Button::new(if self.mounted {
                            "Remove spring"
                        } else {
                            "Restore spring"
                        })
                        .id("spring-mount")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.mounted = !this.mounted;
                            cx.notify();
                        })),
                    ),
            )
            .when(self.mounted, |column| {
                column.child(
                    Space::new()
                        .align_center()
                        .child(Text::new("Origin"))
                        .child(Text::new("Interruptible native spring"))
                        .with_spring(
                            "spring-gap",
                            MotionSpring::Snappy.animation_to(gap),
                            |row, gap| row.gap(gap),
                        ),
                )
            })
    }
}
