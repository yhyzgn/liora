//! Share stable menu IDs and GPUI actions with a native command palette.
use gpui::{App, Entity, actions};
use liora_components::{CommandPalette, CommandPaletteCommand, MenuItem};

actions!(command_palette_docs, [SaveDocument]);

pub fn command_palette_basic(cx: &mut App) -> Entity<CommandPalette> {
    let save = MenuItem::new("document.save", "Save document").shortcut("Ctrl+S");
    CommandPalette::entity(
        vec![
            CommandPaletteCommand::from_menu_item(&save)
                .description("Dispatches the host's SaveDocument GPUI action")
                .with_action(SaveDocument),
            CommandPaletteCommand::new("document.delete", "Delete document")
                .disabled(true)
                .with_action(SaveDocument),
        ],
        cx,
    )
}
