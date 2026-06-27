use gpui::{AnyView, App, Context, Render, Window, green, prelude::*, px, red, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{CandlestickChart, CandlestickPoint, ChartValueLabelContent, Space};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| CandlestickChartDemo).into()
}

struct CandlestickChartDemo;

impl Render for CandlestickChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "CandlestickChart K 线图",
            "使用 GPUI 原生 canvas 绘制 OHLC 蜡烛图，适合金融行情、交易系统和高低区间监控。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础 OHLC + hover tooltip",
                    "每根蜡烛包含 open/high/low/close；鼠标悬停可查看完整数值。",
                    CandlestickChart::new(market_points())
                        .id("candlestick-demo-basic")
                        .height(px(380.0))
                        .show_legend(true)
                        .tooltip_hit_radius(px(12.0)),
                ))
                .child(section(
                    "自定义涨跌色与实体宽度",
                    "可按产品风格调整上涨/下跌颜色、蜡烛实体宽度和 wick 粗细。",
                    CandlestickChart::new(market_points())
                        .id("candlestick-demo-custom")
                        .height(px(360.0))
                        .up_color(rgb(0x14b8a6).into())
                        .down_color(rgb(0xf43f5e).into())
                        .body_width(px(12.0))
                        .wick_width(px(2.0))
                        .max_axis_labels(6),
                ))
                .child(section(
                    "密集数据降采样 + 收盘价标签",
                    "大量数据会保留首尾与稀疏采样点，避免 K 线挤成一片；必要时可 disable_downsampling。",
                    CandlestickChart::new(dense_points())
                        .id("candlestick-demo-dense")
                        .height(px(400.0))
                        .up_color(green())
                        .down_color(red())
                        .max_render_points(36)
                        .max_axis_labels(8)
                        .max_value_labels(10)
                        .show_value_labels(true)
                        .value_label_content(ChartValueLabelContent::Value),
                )),
        )
    }
}

pub fn market_points() -> Vec<CandlestickPoint> {
    vec![
        CandlestickPoint::new("Mon", 102.0, 112.0, 98.0, 109.0).volume(12_400.0),
        CandlestickPoint::new("Tue", 109.0, 115.0, 104.0, 106.0).volume(15_800.0),
        CandlestickPoint::new("Wed", 106.0, 121.0, 105.0, 118.0).volume(18_600.0),
        CandlestickPoint::new("Thu", 118.0, 124.0, 111.0, 114.0).volume(16_100.0),
        CandlestickPoint::new("Fri", 114.0, 128.0, 113.0, 126.0).volume(21_900.0),
        CandlestickPoint::new("Mon+1", 126.0, 132.0, 120.0, 123.0).volume(19_300.0),
        CandlestickPoint::new("Tue+1", 123.0, 136.0, 122.0, 134.0).volume(24_100.0),
    ]
}

pub fn dense_points() -> Vec<CandlestickPoint> {
    (0..72)
        .map(|index| {
            let base = 110.0 + (index as f64 * 0.42) + ((index % 7) as f64 - 3.0) * 1.8;
            let open = base + ((index % 5) as f64 - 2.0) * 0.9;
            let close = base + (((index + 2) % 5) as f64 - 2.0) * 1.2;
            let high = open.max(close) + 3.0 + (index % 4) as f64;
            let low = open.min(close) - 2.6 - (index % 3) as f64;
            CandlestickPoint::new(format!("D{}", index + 1), open, high, low, close)
                .volume(8_000.0 + index as f64 * 150.0)
        })
        .collect()
}
