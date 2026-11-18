//! Stateful Button activation, matching Gallery and Docs.

use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::{Button, Space, Text};

fn render_states(cx: &mut App) -> Entity<ButtonStatesDemo> {
    cx.new(|_| ButtonStatesDemo {
        activations: 0,
        last_action: "None",
    })
}

struct ButtonStatesDemo {
    activations: usize,
    last_action: &'static str,
}

impl Render for ButtonStatesDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Space::new()
            .vertical()
            .gap_sm()
            .child(Text::new(
                "Tab / Shift+Tab to move; Enter / Space to activate.",
            ))
            .child(
                Space::new().wrap().gap_sm().children(
                    [
                        ("Enabled", false, false),
                        ("Disabled", true, false),
                        ("Loading", false, true),
                        ("After", false, false),
                    ]
                    .into_iter()
                    .map(|(label, disabled, loading)| {
                        Button::new(label)
                            .id(label)
                            .primary()
                            .disabled(disabled)
                            .loading(loading)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.activations += 1;
                                this.last_action = label;
                                cx.notify();
                            }))
                    }),
                ),
            )
            .child(Text::new(format!(
                "Activations: {}; last: {}",
                self.activations, self.last_action
            )))
    }
}

fn main() {}
