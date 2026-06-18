use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{Card, Space, Watermark};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| WatermarkDemo).into()
}

struct WatermarkDemo;

fn document_card(title: &'static str, body: &'static str) -> impl IntoElement {
    div()
        .min_h(px(180.0))
        .rounded_lg()
        .bg(gpui::white())
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .p_5()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .text_lg()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(rgb(0x0f172a))
                .child(title),
        )
        .child(div().text_sm().text_color(rgb(0x475569)).child(body))
}

impl Render for WatermarkDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Watermark 水印",
            "为卡片、文档预览或页面局部区域添加原生文字水印。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "覆盖水印",
                    "默认在内容区域重复铺设水印文本。",
                    Card::new(
                        Watermark::new(
                            document_card(
                                "Quarterly Report",
                                "Revenue and risk notes for internal review.",
                            ),
                            "LIORA CONFIDENTIAL",
                        )
                        .density(3, 4)
                        .opacity(0.18),
                    ),
                ))
                .child(section(
                    "页眉水印",
                    "header/footer 模式适合只在局部边缘展示轻量标识。",
                    Card::new(
                        Watermark::new(
                            document_card(
                                "Preview Asset",
                                "This image is visible but protected by a header watermark.",
                            ),
                            "PREVIEW",
                        )
                        .header()
                        .density(1, 3)
                        .color(rgb(0x2563eb).into())
                        .opacity(0.24),
                    ),
                ))
                .child(section(
                    "自定义密度与颜色",
                    "gap、density、opacity 和 color 共同控制水印强度。",
                    Card::new(
                        Watermark::new(
                            document_card(
                                "Design Draft",
                                "Use stronger watermarking for export screenshots.",
                            ),
                            "DRAFT",
                        )
                        .density(4, 5)
                        .gap(px(72.0), px(48.0))
                        .color(rgb(0xf97316).into())
                        .opacity(0.22)
                        .rotate(-32.0),
                    ),
                )),
        )
    }
}
