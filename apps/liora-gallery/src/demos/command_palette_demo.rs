use gpui::{AnyView, App, Context, Entity, FocusHandle, Render, Window, actions, div, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    Button, CommandPalette, CommandPaletteCommand, Menu, MenuBar, MenuItem, Space, Text,
};

actions!(command_palette_demo, [SaveDocument]);

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let save = MenuItem::new("document.save", "Save document").shortcut("Ctrl+S");
        let mut commands = vec![
            CommandPaletteCommand::from_menu_item(&save)
                .description("Uses the same GPUI action as the File menu")
                .with_action(SaveDocument),
            CommandPaletteCommand::new("document.delete", "Delete document")
                .description("Disabled commands cannot be selected or executed")
                .disabled(true)
                .with_action(SaveDocument),
        ];
        let owner = cx.entity().downgrade();
        commands.extend((0..250).map(|index| {
            let owner = owner.clone();
            CommandPaletteCommand::new(
                format!("workspace.item.{index}"),
                format!("Open workspace item {index}"),
            )
            .description("Virtualized command results; try a fuzzy query")
            .on_execute(move |id, _, cx| {
                let label = format!("Executed {}", id.as_str());
                let _ = owner.update(cx, |view: &mut CommandPaletteDemo, cx| {
                    view.feedback = label;
                    cx.notify();
                });
            })
        }));
        CommandPaletteDemo {
            palette: cx.new(|cx| CommandPalette::new(commands, cx)),
            focus: cx.focus_handle(),
            save,
            saved: 0,
            feedback: "Search, use ↑ / ↓, Enter, or Escape. The original focus is restored.".into(),
        }
    })
    .into()
}

struct CommandPaletteDemo {
    palette: Entity<CommandPalette>,
    focus: FocusHandle,
    save: MenuItem,
    saved: usize,
    feedback: String,
}

impl Render for CommandPaletteDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let menu = MenuBar::new([Menu::new("File").item(self.save.clone())]).on_select(
            |_, item, window, cx| {
                if item.id.as_ref() == "document.save" {
                    window.dispatch_action(Box::new(SaveDocument), cx);
                }
            },
        );
        div()
            .track_focus(&self.focus)
            .on_action(cx.listener(|this, _: &SaveDocument, _, cx| {
                this.saved += 1;
                this.feedback = format!(
                    "Saved {} time(s), via the shared SaveDocument action",
                    this.saved
                );
                cx.notify();
            }))
            .child(page(
                "CommandPalette 命令面板",
                "Stable IDs, metadata, availability, fuzzy ranking and native actions.",
                section(
                    "Menu and palette share a command",
                    "252 commands with viewport-only row rendering.",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(menu)
                        .child(
                            Button::new("Open command palette")
                                .primary()
                                .on_click(cx.listener(|this, _, window, cx| {
                                    window.focus(&this.focus, cx);
                                    this.palette
                                        .update(cx, |palette, cx| palette.open(window, cx));
                                })),
                        )
                        .child(Text::new(self.feedback.clone())),
                ),
            ))
            .child(self.palette.clone())
    }
}
