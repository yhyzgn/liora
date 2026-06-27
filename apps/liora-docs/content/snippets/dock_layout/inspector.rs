use gpui::{IntoElement, px};
use liora_components::{DockEdge, DockLayout, DockPanel, DockTab, Space, Tag, Text};

pub fn dock_layout_inspector() -> impl IntoElement {
    DockLayout::new()
        .height(px(460.0))
        .panel_gap(px(6.0))
        .panel(
            DockPanel::new("outline", "Outline", DockEdge::Left, panel_items(["App", "Shell", "Content"]))
                .size(px(180.0)),
        )
        .panel(
            DockPanel::new("props", "Properties", DockEdge::Right, panel_items(["theme", "state", "events"]))
                .size(px(220.0)),
        )
        .panel(
            DockPanel::new("logs", "Logs", DockEdge::Bottom, Text::new("Ready • 0 errors"))
                .size(px(96.0)),
        )
        .tab(DockTab::new(
            "preview",
            "Preview",
            Space::new()
                .vertical()
                .gap_sm()
                .child(Tag::new("Live").success())
                .child(Text::new("Center content remains flexible.")),
        ))
}

fn panel_items(items: impl IntoIterator<Item = &'static str>) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_xs()
        .children(items.into_iter().map(Text::new))
}
