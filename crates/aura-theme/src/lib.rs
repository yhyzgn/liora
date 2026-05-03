use gpui::{Hsla, Rgba};

/// NaiveUI-inspired Forest Green theme.
///
/// Reference: https://github.com/tusen-ai/naive-ui
/// Light theme base: neutralBase=#FFF, primary=#18A058 (green)
/// Dark theme base: neutralBase=#000, primary=#63E2B7 (green)

// ---------------------------------------------------------------------------
// Color helpers — construct via gpui::Rgba then .into() to Hsla
// ---------------------------------------------------------------------------

fn rgb(r: u8, g: u8, b: u8) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
    .into()
}

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
}

/// Lighten: blend with white. factor 0.9 = very light (90% white)
fn lighten(base: Hsla, factor: f32) -> Hsla {
    base.blend(gpui::white().opacity(factor))
}

#[allow(dead_code)]
fn darken(base: Hsla, factor: f32) -> Hsla {
    base.blend(gpui::black().opacity(factor))
}

// ---------------------------------------------------------------------------
// Semantic Color Family
// ---------------------------------------------------------------------------

pub struct ColorFamily {
    pub base: Hsla,
    pub hover: Hsla,
    pub active: Hsla,
    pub suppl: Hsla,
    /// light-9: for subtle backgrounds
    pub light_9: Hsla,
    /// light-8
    pub light_8: Hsla,
    /// light-7: for hover backgrounds
    pub light_7: Hsla,
}

impl ColorFamily {
    fn new(base: Hsla, hover: Hsla, active: Hsla, suppl: Hsla) -> Self {
        Self {
            base,
            hover,
            active,
            suppl,
            light_9: lighten(base, 0.9),
            light_8: lighten(base, 0.8),
            light_7: lighten(base, 0.7),
        }
    }
}

// ---------------------------------------------------------------------------
// Neutral Tokens
// ---------------------------------------------------------------------------

pub struct NeutralTokens {
    pub body: Hsla,
    pub card: Hsla,
    pub modal: Hsla,
    pub popover: Hsla,
    pub inverted: Hsla,

    pub text_1: Hsla,
    pub text_2: Hsla,
    pub text_3: Hsla,
    pub text_disabled: Hsla,
    pub placeholder: Hsla,
    pub icon: Hsla,

    pub border: Hsla,
    pub divider: Hsla,

    pub hover: Hsla,
    pub pressed: Hsla,

    pub rail: Hsla,

    pub overlay: Hsla,
    pub mask: Hsla,
}

// ---------------------------------------------------------------------------
// Spacing / Radius / Font (unchanged structure, refined values)
// ---------------------------------------------------------------------------

pub struct AuraSpacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

pub struct AuraRadius {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub full: f32,
}

pub struct AuraFontSize {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

// ---------------------------------------------------------------------------
// AuraTheme
// ---------------------------------------------------------------------------

pub struct SecondaryColors {
    pub bg: Hsla,
    pub hover: Hsla,
    pub pressed: Hsla,
}

pub struct AuraTheme {
    pub name: String,
    pub spacing: AuraSpacing,
    pub radius: AuraRadius,
    pub font_size: AuraFontSize,

    // Semantic color families
    pub primary: ColorFamily,
    pub info: ColorFamily,
    pub success: ColorFamily,
    pub warning: ColorFamily,
    pub danger: ColorFamily,

    // Neutral tokens
    pub neutral: NeutralTokens,

    // Secondary button style (NaiveUI buttonColor2)
    pub secondary: SecondaryColors,

    // Shadows
    pub shadow_1: &'static str,
    pub shadow_2: &'static str,
    pub shadow_3: &'static str,
}

impl Default for AuraTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl AuraTheme {
    // ========================================================================
    // Light Theme
    // ========================================================================
    pub fn light() -> Self {
        Self {
            name: "light".into(),
            spacing: AuraSpacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 20.0,
                xl: 32.0,
            },
            radius: AuraRadius {
                sm: 2.0,
                md: 4.0,
                lg: 8.0,
                full: 9999.0,
            },
            font_size: AuraFontSize {
                xs: 10.0,
                sm: 12.0,
                md: 14.0,
                lg: 16.0,
                xl: 20.0,
            },

            primary: ColorFamily::new(
                rgb(24, 160, 88),  // #18A058 — NaiveUI primary green
                rgb(54, 173, 106), // #36AD6A
                rgb(12, 122, 67),  // #0C7A43
                rgb(54, 173, 106), // #36AD6A
            ),
            info: ColorFamily::new(
                rgb(32, 128, 240), // #2080F0 — NaiveUI info blue
                rgb(64, 152, 252), // #4098FC
                rgb(16, 96, 201),  // #1060C9
                rgb(64, 152, 252), // #4098FC
            ),
            success: ColorFamily::new(
                rgb(24, 160, 88),  // #18A058
                rgb(54, 173, 106), // #36AD6A
                rgb(12, 122, 67),  // #0C7A43
                rgb(54, 173, 106), // #36AD6A
            ),
            warning: ColorFamily::new(
                rgb(240, 160, 32), // #F0A020 — NaiveUI warning gold
                rgb(252, 176, 64), // #FCB040
                rgb(201, 124, 16), // #C97C10
                rgb(252, 176, 64), // #FCB040
            ),
            danger: ColorFamily::new(
                rgb(208, 48, 80),  // #D03050 — NaiveUI error red
                rgb(222, 87, 109), // #DE576D
                rgb(171, 31, 63),  // #AB1F3F
                rgb(222, 87, 109), // #DE576D
            ),

            neutral: NeutralTokens {
                body: rgb(255, 255, 255),
                card: rgb(255, 255, 255),
                modal: rgb(255, 255, 255),
                popover: rgb(255, 255, 255),
                inverted: rgb(0, 20, 40),

                text_1: rgb(31, 34, 37),
                text_2: rgb(51, 54, 57),
                text_3: rgb(118, 124, 130),
                text_disabled: rgba(194, 194, 194, 1.0),
                placeholder: rgba(194, 194, 194, 1.0),
                icon: rgba(24, 160, 88, 1.0),

                border: rgb(224, 224, 230),
                divider: rgb(239, 239, 245),

                hover: rgb(243, 243, 245),
                pressed: rgb(237, 237, 239),

                rail: rgb(219, 219, 223),

                overlay: rgba(0, 0, 0, 0.50),
                mask: rgba(255, 255, 255, 0.90),
            },
            // NaiveUI button secondary colors
            secondary: SecondaryColors {
                bg: rgba(46, 51, 56, 0.05),
                hover: rgba(46, 51, 56, 0.09),
                pressed: rgba(46, 51, 56, 0.13),
            },

            shadow_1: "0 1px 2px -2px rgba(0,0,0,.08), 0 3px 6px 0 rgba(0,0,0,.06), 0 5px 12px 4px rgba(0,0,0,.04)",
            shadow_2: "0 3px 6px -4px rgba(0,0,0,.12), 0 6px 16px 0 rgba(0,0,0,.08), 0 9px 28px 8px rgba(0,0,0,.05)",
            shadow_3: "0 6px 16px -9px rgba(0,0,0,.08), 0 9px 28px 0 rgba(0,0,0,.05), 0 12px 48px 16px rgba(0,0,0,.03)",
        }
    }

    // ========================================================================
    // Dark Theme
    // ========================================================================
    pub fn dark() -> Self {
        Self {
            name: "dark".into(),
            spacing: AuraSpacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 20.0,
                xl: 32.0,
            },
            radius: AuraRadius {
                sm: 2.0,
                md: 4.0,
                lg: 8.0,
                full: 9999.0,
            },
            font_size: AuraFontSize {
                xs: 12.0,
                sm: 14.0,
                md: 14.0,
                lg: 15.0,
                xl: 16.0,
            },

            primary: ColorFamily::new(
                rgb(99, 226, 183),  // #63E2B7 — brighter green for dark
                rgb(127, 231, 196), // #7FE7C4
                rgb(90, 206, 167),  // #5ACEA7
                rgb(42, 148, 125),  // #2A947D (suppl)
            ),
            info: ColorFamily::new(
                rgb(112, 192, 232), // #70C0E8
                rgb(138, 203, 236), // #8ACBEC
                rgb(102, 175, 211), // #66AFD3
                rgb(56, 137, 197),  // #3889C5
            ),
            success: ColorFamily::new(
                rgb(99, 226, 183),  // #63E2B7
                rgb(127, 231, 196), // #7FE7C4
                rgb(90, 206, 167),  // #5ACEA7
                rgb(42, 148, 125),  // #2A947D
            ),
            warning: ColorFamily::new(
                rgb(242, 201, 125), // #F2C97D
                rgb(245, 213, 153), // #F5D599
                rgb(230, 194, 96),  // #E6C260
                rgb(240, 138, 0),   // #F08A00
            ),
            danger: ColorFamily::new(
                rgb(232, 128, 128), // #E88080
                rgb(233, 139, 139), // #E98B8B
                rgb(229, 114, 114), // #E57272
                rgb(208, 58, 82),   // #D03A52
            ),

            neutral: NeutralTokens {
                body: rgb(16, 16, 20),    // #101014
                card: rgb(24, 24, 28),    // #18181C
                modal: rgb(44, 44, 50),   // #2C2C32
                popover: rgb(72, 72, 78), // #48484E
                inverted: rgb(255, 255, 255),

                text_1: rgba(255, 255, 255, 0.90),
                text_2: rgba(255, 255, 255, 0.82),
                text_3: rgba(255, 255, 255, 0.52),
                text_disabled: rgba(255, 255, 255, 0.38),
                placeholder: rgba(255, 255, 255, 0.38),
                icon: rgba(255, 255, 255, 0.38),

                border: rgba(255, 255, 255, 0.24),
                divider: rgba(255, 255, 255, 0.09),

                hover: rgba(255, 255, 255, 0.09),
                pressed: rgba(255, 255, 255, 0.05),

                rail: rgba(255, 255, 255, 0.20),

                overlay: rgba(0, 0, 0, 0.60),
                mask: rgba(0, 0, 0, 0.70),
            },

            shadow_1: "0 1px 2px -2px rgba(0,0,0,.24), 0 3px 6px 0 rgba(0,0,0,.18), 0 5px 12px 4px rgba(0,0,0,.12)",
            shadow_2: "0 3px 6px -4px rgba(0,0,0,.24), 0 6px 12px 0 rgba(0,0,0,.16), 0 9px 18px 8px rgba(0,0,0,.10)",
            shadow_3: "0 6px 16px -9px rgba(0,0,0,.08), 0 9px 28px 0 rgba(0,0,0,.05), 0 12px 48px 16px rgba(0,0,0,.03)",

            secondary: SecondaryColors {
                bg: rgba(255, 255, 255, 0.08),
                hover: rgba(255, 255, 255, 0.12),
                pressed: rgba(255, 255, 255, 0.16),
            },
        }
    }

    // ========================================================================
    // Convenience: resolve colors for a ButtonVariant
    // ========================================================================
    pub fn color_by_variant(
        &self,
        variant: ButtonVariant,
        secondary: bool,
        background: bool,
        border: bool,
    ) -> ButtonVariantColors {
        if secondary {
            return self.secondary_colors(variant, background, border);
        }

        // Filled (primary) style
        match variant {
            ButtonVariant::Default => ButtonVariantColors {
                bg: rgba(0, 0, 0, 0.0),
                hover_bg: self.secondary.hover,
                active_bg: self.secondary.pressed,
                text: self.neutral.text_2,
                border: self.neutral.border,
                text_hover: self.primary.base,
                border_hover: self.primary.base,
            },
            ButtonVariant::Tertiary => ButtonVariantColors {
                bg: self.secondary.bg,
                hover_bg: self.secondary.hover,
                active_bg: self.secondary.pressed,
                text: self.neutral.text_2,
                border: rgba(0, 0, 0, 0.0),
                text_hover: self.neutral.text_1,
                border_hover: rgba(0, 0, 0, 0.0),
            },
            ButtonVariant::Primary => self.filled_colors(&self.primary),
            ButtonVariant::Info => self.filled_colors(&self.info),
            ButtonVariant::Success => self.filled_colors(&self.success),
            ButtonVariant::Warning => self.filled_colors(&self.warning),
            ButtonVariant::Danger => self.filled_colors(&self.danger),
        }
    }

    /// Secondary (light bg + colored text) for colored variants;
    /// Default/Tertiary stay neutral.
    fn secondary_colors(
        &self,
        variant: ButtonVariant,
        show_bg: bool,
        show_border: bool,
    ) -> ButtonVariantColors {
        match variant {
            ButtonVariant::Default => ButtonVariantColors {
                bg: if show_bg {
                    self.secondary.bg
                } else {
                    rgba(0, 0, 0, 0.0)
                },
                hover_bg: self.secondary.hover,
                active_bg: self.secondary.pressed,
                text: self.neutral.text_2,
                border: if show_border {
                    self.neutral.border
                } else {
                    rgba(0, 0, 0, 0.0)
                },
                text_hover: self.primary.base,
                border_hover: self.primary.base,
            },
            ButtonVariant::Tertiary => ButtonVariantColors {
                bg: if show_bg {
                    self.secondary.bg
                } else {
                    rgba(0, 0, 0, 0.0)
                },
                hover_bg: self.secondary.hover,
                active_bg: self.secondary.pressed,
                text: self.neutral.text_2,
                border: if show_border {
                    self.neutral.border
                } else {
                    rgba(0, 0, 0, 0.0)
                },
                text_hover: self.neutral.text_1,
                border_hover: rgba(0, 0, 0, 0.0),
            },
            ButtonVariant::Primary => self.secondary_family(&self.primary, show_bg, show_border),
            ButtonVariant::Info => self.secondary_family(&self.info, show_bg, show_border),
            ButtonVariant::Success => self.secondary_family(&self.success, show_bg, show_border),
            ButtonVariant::Warning => self.secondary_family(&self.warning, show_bg, show_border),
            ButtonVariant::Danger => self.secondary_family(&self.danger, show_bg, show_border),
        }
    }

    fn secondary_family(
        &self,
        family: &ColorFamily,
        show_bg: bool,
        show_border: bool,
    ) -> ButtonVariantColors {
        ButtonVariantColors {
            bg: if show_bg {
                family.light_9
            } else {
                rgba(0, 0, 0, 0.0)
            },
            hover_bg: family.light_8,
            active_bg: family.light_7,
            text: family.base,
            border: if show_border {
                family.base
            } else {
                rgba(0, 0, 0, 0.0)
            },
            text_hover: family.hover,
            border_hover: family.hover,
        }
    }

    fn filled_colors(&self, family: &ColorFamily) -> ButtonVariantColors {
        let hover = family.base.blend(gpui::black().opacity(0.10));
        let active = family.base.blend(gpui::black().opacity(0.25));
        ButtonVariantColors {
            bg: family.base,
            hover_bg: hover,
            active_bg: active,
            text: rgb(255, 255, 255),
            border: family.base,
            text_hover: rgb(255, 255, 255),
            border_hover: hover,
        }
    }
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Default,
    Tertiary,
    Primary,
    Info,
    Success,
    Warning,
    Danger,
}

pub struct ButtonVariantColors {
    pub bg: Hsla,
    pub hover_bg: Hsla,
    pub active_bg: Hsla,
    pub text: Hsla,
    pub border: Hsla,
    pub text_hover: Hsla,
    pub border_hover: Hsla,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Default,
    Large,
}

impl ButtonSize {
    pub fn height(&self) -> f32 {
        match self {
            ButtonSize::Small => 28.0,   // NaiveUI heightSmall
            ButtonSize::Default => 34.0, // NaiveUI heightMedium
            ButtonSize::Large => 40.0,   // NaiveUI heightLarge
        }
    }

    pub fn padding_x(&self) -> f32 {
        match self {
            ButtonSize::Small => 12.0,   // NaiveUI: 0 12px
            ButtonSize::Default => 14.0, // NaiveUI: 0 14px
            ButtonSize::Large => 18.0,   // NaiveUI: 0 18px
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::Rgba;

    fn rgba_color(color: Hsla) -> Rgba {
        color.into()
    }

    #[test]
    fn filled_button_hover_and_active_backgrounds_get_progressively_darker() {
        let theme = AuraTheme::light();
        let colors = theme.color_by_variant(ButtonVariant::Primary, false, true, true);

        let bg = rgba_color(colors.bg);
        let hover = rgba_color(colors.hover_bg);
        let active = rgba_color(colors.active_bg);

        assert!(hover.r < bg.r, "hover red channel should be darker");
        assert!(hover.g < bg.g, "hover green channel should be darker");
        assert!(hover.b < bg.b, "hover blue channel should be darker");
        assert!(
            active.r < hover.r,
            "active red channel should be darker than hover"
        );
        assert!(
            active.g < hover.g,
            "active green channel should be darker than hover"
        );
        assert!(
            active.b < hover.b,
            "active blue channel should be darker than hover"
        );
    }

    #[test]
    fn default_button_hover_and_active_backgrounds_are_visible_overlays() {
        let theme = AuraTheme::light();
        let colors = theme.color_by_variant(ButtonVariant::Default, false, true, true);

        let bg = rgba_color(colors.bg);
        let hover = rgba_color(colors.hover_bg);
        let active = rgba_color(colors.active_bg);

        assert_eq!(
            bg.a, 0.0,
            "default button base background should stay transparent"
        );
        assert!(hover.a > bg.a, "hover background should be visible");
        assert!(
            active.a > hover.a,
            "active background should be stronger than hover"
        );
    }
}
