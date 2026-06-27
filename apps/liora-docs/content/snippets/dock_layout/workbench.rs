use gpui::{IntoElement, px};
use liora_components::{DockEdge, DockLayout, DockPanel, DockTab, Space, Text};

pub fn dock_layout_workbench() -> impl IntoElement {
    DockLayout::new()
        .height_lg()
        .panel_gap(px(6.0))
        .panel(
            DockPanel::new(
                "explorer",
                "Explorer",
                DockEdge::Left,
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(Text::new("src"))
                    .child(Text::new("crates"))
                    .child(Text::new("apps")),
            )
            .size(px(220.0)),
        )
        .panel(
            DockPanel::new(
                "terminal",
                "Terminal",
                DockEdge::Bottom,
                Text::new("cargo check --workspace --all-targets"),
            )
            .size(px(132.0)),
        )
        .tab(DockTab::new(
            "main",
            "main.rs",
            Text::new("fn main() { init_liora(cx); }").wrap(),
        ))
        .tab(DockTab::new(
            "readme",
            "README.md",
            Text::new("# Liora\nNative GPUI component library.").wrap(),
        ))
}
