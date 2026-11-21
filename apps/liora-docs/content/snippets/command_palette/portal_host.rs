//! Use this view as the window root after calling init_liora(cx) at startup.
use gpui::{App, Context, Entity, Render, Window, div, prelude::*};
use liora_components::{Button, CommandPalette, CommandPaletteCommand, PortalLayer, Text};

pub fn palette_window_root(cx: &mut App) -> Entity<PaletteWindowRoot> {
    cx.new(|cx| {
        let owner = cx.entity().downgrade();
        let command = CommandPaletteCommand::new("show.status", "Show status").on_execute(
            move |_, _, cx| {
                let _ = owner.update(cx, |host: &mut PaletteWindowRoot, cx| {
                    host.executed = true;
                    cx.notify();
                });
            },
        );
        PaletteWindowRoot {
            palette: CommandPalette::entity(vec![command], cx),
            executed: false,
        }
    })
}

pub struct PaletteWindowRoot {
    palette: Entity<CommandPalette>,
    executed: bool,
}

impl Render for PaletteWindowRoot {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .child(Button::new("Open command palette").on_click(cx.listener(|this, _, window, cx| {
                this.palette.update(cx, |palette, cx| palette.open(window, cx));
            })))
            .child(Text::new(if self.executed { "Command executed" } else { "Ready" }))
            .child(self.palette.clone())
            .child(PortalLayer)
    }
}
