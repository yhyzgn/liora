//! QR code style, logo, color and error-correction configuration.

use gpui::{IntoElement, px, rgb};
use liora_components::{
    QrCode, QrEcLevel, QrFinderStyle, QrGradientDirection, QrModuleStyle, Space,
};

fn qr_code_style() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_lg()
        .child(
            QrCode::new("Liora primary QR")
                .size(px(160.0))
                .colors(rgb(0x2563eb).into(), rgb(0xeff6ff).into()),
        )
        .child(
            QrCode::new("Rounded finder")
                .size(px(170.0))
                .ec_level(QrEcLevel::High)
                .module_style(QrModuleStyle::Square)
                .finder_style(QrFinderStyle::Rounded)
                .colors(rgb(0x16a34a).into(), rgb(0xf0fdf4).into()),
        )
        .child(
            QrCode::new("Gradient diagonal QR")
                .size(px(180.0))
                .dot_modules()
                .circle_finders()
                .foreground_gradient(
                    vec![
                        rgb(0x7c3aed).into(),
                        rgb(0x06b6d4).into(),
                        rgb(0x22c55e).into(),
                    ],
                    QrGradientDirection::ToBottomRight,
                )
                .background(rgb(0xf8fafc).into()),
        )
        .child(
            QrCode::new("Gradient left QR")
                .size(px(180.0))
                .rounded_modules()
                .rounded_finders()
                .foreground_gradient(
                    vec![rgb(0xf97316).into(), rgb(0xec4899).into()],
                    QrGradientDirection::ToLeft,
                )
                .background(rgb(0xfffbeb).into()),
        )
        .child(
            QrCode::new("Rounded modules with custom logo")
                .size(px(180.0))
                .high_recovery()
                .rounded_modules()
                .circle_finders()
                .logo_text("二维")
                .logo_size_ratio(0.28)
                .logo_background(rgb(0x22c55e).into())
                .logo_color(gpui::white())
                .colors(rgb(0x0f172a).into(), rgb(0xf8fafc).into()),
        )
}

fn main() {
    let _ = qr_code_style();
}
