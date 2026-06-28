//! Ordered lists with multiple custom counter styles and marker patterns.

use gpui::rgb;
use liora_components::{Card, List, ListItem, OrderedCounterStyle, OrderedMarker, Space, Text};

pub fn custom_ordered_list() -> Space {
    Space::new()
        .vertical()
        .gap_lg()
        .child(
            Card::new(
                List::ordered()
                    .start(3)
                    .marker_width(gpui::px(78.0))
                    .indent(4)
                    .marker_colors([
                        rgb(0x2563eb).into(),
                        rgb(0x16a34a).into(),
                        rgb(0xf97316).into(),
                    ])
                    .ordered_markers([
                        OrderedMarker::new(OrderedCounterStyle::UpperAlpha, "Phase {n}"),
                        OrderedMarker::new(OrderedCounterStyle::DecimalLeadingZero, "{n}/"),
                        OrderedMarker::new(OrderedCounterStyle::LowerRoman, "({n})"),
                    ])
                    .item(ListItem::element(Text::new("Audit current UI").bold()))
                    .item(
                        ListItem::new("Implement component")
                            .child(ListItem::new("Write SDK API"))
                            .child(ListItem::new("Add Gallery and Docs")),
                    )
                    .item(ListItem::new("Verify and publish notes")),
            )
            .no_shadow(),
        )
        .child(
            Card::new(
                List::ordered()
                    .marker_width(gpui::px(54.0))
                    .marker_colors([
                        rgb(0x7c3aed).into(),
                        rgb(0x0891b2).into(),
                        rgb(0xdb2777).into(),
                    ])
                    .ordered_markers([
                        OrderedMarker::new(OrderedCounterStyle::Decimal, "§ {n}"),
                        OrderedMarker::new(OrderedCounterStyle::UpperRoman, "{n}."),
                        OrderedMarker::new(OrderedCounterStyle::LowerAlpha, "{n})"),
                    ])
                    .item(
                        ListItem::new("Quick Start")
                            .child(ListItem::new("Install dependencies"))
                            .child(ListItem::new("Open the first window")),
                    )
                    .item(ListItem::new("Component catalog"))
                    .item(ListItem::new("Release checklist")),
            )
            .no_shadow(),
        )
        .child(
            Card::new(
                List::ordered()
                    .marker_width(gpui::px(64.0))
                    .marker_colors([
                        rgb(0xdc2626).into(),
                        rgb(0xea580c).into(),
                        rgb(0x65a30d).into(),
                    ])
                    .ordered_markers([
                        OrderedMarker::new(OrderedCounterStyle::DecimalLeadingZero, "T-{n}"),
                        OrderedMarker::new(OrderedCounterStyle::LowerAlpha, "[{n}]"),
                        OrderedMarker::new(OrderedCounterStyle::LowerRoman, "step {n}"),
                    ])
                    .item(
                        ListItem::new("Prepare")
                            .child(ListItem::new("Lock requirements"))
                            .child(ListItem::new("Create regression checks")),
                    )
                    .item(ListItem::new("Implement"))
                    .item(ListItem::new("Verify")),
            )
            .no_shadow(),
        )
}
