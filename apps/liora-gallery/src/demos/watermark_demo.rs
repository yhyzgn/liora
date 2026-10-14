use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{Card, Flex, Space, Text, Watermark};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| WatermarkDemo).into()
}

struct WatermarkDemo;

fn document_card(title: &'static str, body: &'static str) -> impl IntoElement {
    Flex::new()
        .column()
        .height_px(180.0)
        .padding_lg()
        .gap_md()
        .child(Text::new(title).bold())
        .child(Text::new(body).sm().wrap())
}

impl Render for WatermarkDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<liora_core::Config>().theme.clone();
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
                        .color(theme.primary.base)
                        .opacity(0.24),
                    ),
                ))
                .child(section(
                    "旋转、倾斜与旋转中心",
                    "rotate 控制角度，skew 控制双轴倾斜，rotation_center 使用每块文字的归一化坐标。",
                    Card::new(
                        Watermark::new(
                            document_card(
                                "Design Draft",
                                "Use stronger watermarking for export screenshots.",
                            ),
                            "DRAFT",
                        )
                        .density(4, 5)
                        .gap(72.0, 48.0)
                        .color(theme.warning.base)
                        .opacity(0.22)
                        .rotate(-32.0)
                        .skew(16.0, 0.0)
                        .rotation_center(0.0, 0.5),
                    ),
                )),
        )
    }
}
