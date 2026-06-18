use gpui::{AnyView, App, Context, Render, Window, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{SegmentLegendPosition, SegmentRatioBar, SegmentRatioItem, Space};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SegmentRatioBarDemo).into()
}

struct SegmentRatioBarDemo;

impl Render for SegmentRatioBarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "SegmentRatioBar 分段比例条",
            "用一个水平条展示多个部分的占比，并支持标签放在上方、下方、两侧或隐藏。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "下方图例",
                    "默认在下方水平排列；每个文本块与对应分段等宽，左侧对齐分段左边，右侧对齐分段右边。",
                    SegmentRatioBar::new(items())
                        .height(px(14.0))
                        .radius(px(8.0))
                        .segment_radius(px(3.0))
                        .legend_inset_x(px(8.0))
                        .percentage_decimals(0)
                        .split_legend(true),
                ))
                .child(section(
                    "上方文本",
                    "文本也可以统一放在条形图上方，满足截图中的排版需求。",
                    SegmentRatioBar::new(items())
                        .legend_position(SegmentLegendPosition::Top)
                        .height(px(16.0))
                        .radius(px(8.0))
                        .rounded_segments(px(4.0))
                        .legend_inset_x(px(10.0))
                        .percentage_decimals(1),
                ))
                .child(section(
                    "上下同时显示",
                    "legend_both 可在分段条上下同时显示与分段等宽对齐的文本。",
                    SegmentRatioBar::new(items())
                        .legend_both()
                        .height(px(14.0))
                        .radius(px(7.0))
                        .segment_radius(px(3.0))
                        .legend_text_inset(px(8.0))
                        .percentage_decimals(1),
                ))
                .child(section(
                    "隐藏文本",
                    "仅展示分段比例条，适合紧凑卡片或旁边已有说明的场景。",
                    SegmentRatioBar::new(items())
                        .hide_legend()
                        .height(px(18.0))
                        .radius(px(9.0))
                        .segment_radius(px(4.0)),
                ))
                .child(section(
                    "自定义 pattern",
                    "每段的 label、右侧比例/值 pattern、整体圆角、分段圆角和文本左右缩进都可单独配置。",
                    SegmentRatioBar::new(pattern_items())
                        .legend_both()
                        .radius(px(7.0))
                        .segment_radius(px(3.0))
                        .legend_text_inset(px(10.0))
                        .percentage_decimals(1)
                        .split_legend(true),
                ))
                .child(section(
                    "细条与宽缩进",
                    "可控制高度、整体圆角、分段圆角和文本缩进，满足不同视觉密度。",
                    SegmentRatioBar::new(compact_items())
                        .height(px(8.0))
                        .radius(px(4.0))
                        .rounded_segments(px(2.0))
                        .legend_inset_x(px(14.0))
                        .percentage_decimals(2),
                )),
        )
    }
}

fn items() -> Vec<SegmentRatioItem> {
    vec![
        SegmentRatioItem::new("Direct", 42.0, rgb(0x3b82f6).into()),
        SegmentRatioItem::new("Proxy", 51.0, rgb(0x22c55e).into()),
        SegmentRatioItem::new("Reject", 7.0, rgb(0xef4444).into()),
    ]
}

fn pattern_items() -> Vec<SegmentRatioItem> {
    vec![
        SegmentRatioItem::new("Direct", 42.0, rgb(0x3b82f6).into())
            .label_pattern("{label}")
            .value_pattern("{value} req / {percent}"),
        SegmentRatioItem::new("Proxy", 51.0, rgb(0x22c55e).into()).value_pattern("{percent}"),
        SegmentRatioItem::new("Reject", 7.0, rgb(0xef4444).into()).value_pattern("{value}"),
    ]
}

fn compact_items() -> Vec<SegmentRatioItem> {
    vec![
        SegmentRatioItem::new("API", 18.0, rgb(0x8b5cf6).into()),
        SegmentRatioItem::new("Web", 33.0, rgb(0x06b6d4).into()),
        SegmentRatioItem::new("Jobs", 29.0, rgb(0xf59e0b).into()),
        SegmentRatioItem::new("Other", 20.0, rgb(0x64748b).into()),
    ]
}
