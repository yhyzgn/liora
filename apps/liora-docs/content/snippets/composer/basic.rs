use gpui::{App, Entity};
use liora_components::{Composer, ComposerCandidate, ComposerReference};

pub fn composer(cx: &mut App) -> Entity<Composer> {
    let composer = Composer::entity(cx);
    composer.update(cx, |composer, cx| {
        composer.set_candidates(
            vec![
                ComposerCandidate::Reference(ComposerReference::new("design", "Design")),
                ComposerCandidate::Command {
                    id: "summarize".into(),
                    label: "summarize".into(),
                    text: "/summarize ".into(),
                },
            ],
            cx,
        );
    });
    composer
}
