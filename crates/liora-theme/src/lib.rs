//! Theme tokens for Liora GPUI components.
//!
//! `liora-theme` defines the semantic colors, spacing, radius, typography, and
//! component size/variant enums consumed by `liora-core` and `liora-components`.
//! Use `Theme::light()` or `Theme::dark()` with `liora_core::init_liora(...)` at
//! app startup.

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

#[derive(Clone)]
/// Semantic color family containing base, state, and subtle background tokens.
pub struct ColorFamily {
    /// Base semantic color for the token family.
    pub base: Hsla,
    /// Color used for hover affordances.
    pub hover: Hsla,
    /// Color used for active or pressed affordances.
    pub active: Hsla,
    /// Supplemental accent color paired with the base token.
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

    fn new_dark(base: Hsla, hover: Hsla, active: Hsla, suppl: Hsla) -> Self {
        Self {
            base,
            hover,
            active,
            suppl,
            // These tokens are used as selected/hover subtle backgrounds. In
            // dark mode they must stay translucent instead of being blended
            // toward white, otherwise table rows and picker chips become
            // visually louder than their foreground content.
            light_9: base.opacity(0.16),
            light_8: base.opacity(0.22),
            light_7: base.opacity(0.30),
        }
    }
}

// ---------------------------------------------------------------------------
// Neutral Tokens
// ---------------------------------------------------------------------------

#[derive(Clone)]
/// Neutral surface, text, border, interaction, and overlay tokens used across components.
pub struct NeutralTokens {
    /// Application body background color.
    pub body: Hsla,
    /// Card and elevated surface background color.
    pub card: Hsla,
    /// Modal panel background color.
    pub modal: Hsla,
    /// Popover and floating-panel background color.
    pub popover: Hsla,
    /// Inverted surface color used behind high-contrast content.
    pub inverted: Hsla,

    /// Primary text color.
    pub text_1: Hsla,
    /// Secondary text color.
    pub text_2: Hsla,
    /// Tertiary text color.
    pub text_3: Hsla,
    /// Text color used for disabled controls.
    pub text_disabled: Hsla,
    /// Placeholder text color for empty inputs.
    pub placeholder: Hsla,
    /// Optional icon rendered with the item.
    pub icon: Hsla,

    /// Border color used in the normal state.
    pub border: Hsla,
    /// Divider line color.
    pub divider: Hsla,

    /// Color used for hover affordances.
    pub hover: Hsla,
    /// Pressed-state background color.
    pub pressed: Hsla,

    /// Track or rail color for slider-like controls.
    pub rail: Hsla,

    /// Translucent overlay color.
    pub overlay: Hsla,
    /// Backdrop mask color for modal layers.
    pub mask: Hsla,
}

// ---------------------------------------------------------------------------
// Spacing / Radius / Font (unchanged structure, refined values)
// ---------------------------------------------------------------------------

#[derive(Clone)]
/// Spacing scale used by layout and component padding presets.
pub struct Spacing {
    /// Extra-small spacing or size token.
    pub xs: f32,
    /// Small spacing, radius, or size token.
    pub sm: f32,
    /// Medium spacing, radius, or size token.
    pub md: f32,
    /// Large spacing, radius, or size token.
    pub lg: f32,
    /// Extra-large spacing or size token.
    pub xl: f32,
}

#[derive(Clone)]
/// Corner-radius scale used by controls, cards, and overlays.
pub struct Radius {
    /// Small spacing, radius, or size token.
    pub sm: f32,
    /// Medium spacing, radius, or size token.
    pub md: f32,
    /// Large spacing, radius, or size token.
    pub lg: f32,
    /// Fully rounded radius token.
    pub full: f32,
}

#[derive(Clone)]
/// Font-size scale used by typography and compact component variants.
pub struct FontSize {
    /// Extra-small spacing or size token.
    pub xs: f32,
    /// Small spacing, radius, or size token.
    pub sm: f32,
    /// Medium spacing, radius, or size token.
    pub md: f32,
    /// Large spacing, radius, or size token.
    pub lg: f32,
    /// Extra-large spacing or size token.
    pub xl: f32,
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

#[derive(Clone)]
/// Color tokens for secondary button and low-emphasis surfaces.
pub struct SecondaryColors {
    /// Base background color used in the normal state.
    pub bg: Hsla,
    /// Color used for hover affordances.
    pub hover: Hsla,
    /// Pressed-state background color.
    pub pressed: Hsla,
}

#[derive(Clone)]
/// Complete Liora visual token set for one color mode.
pub struct Theme {
    /// Human-readable name used for display or package metadata.
    pub name: String,
    /// Spacing for this data model.
    pub spacing: Spacing,
    /// Radius for this data model.
    pub radius: Radius,
    /// Font size for this data model.
    pub font_size: FontSize,

    // Semantic color families
    /// Primary brand semantic color family.
    pub primary: ColorFamily,
    /// Informational semantic color family.
    pub info: ColorFamily,
    /// Success semantic color family.
    pub success: ColorFamily,
    /// Warning semantic color family.
    pub warning: ColorFamily,
    /// Danger semantic color family.
    pub danger: ColorFamily,

    // Neutral tokens
    /// Neutral surface, text, border, and overlay tokens.
    pub neutral: NeutralTokens,

    // Secondary button style (NaiveUI buttonColor2)
    /// Secondary button color tokens.
    pub secondary: SecondaryColors,

    // Shadows
    /// Small elevation shadow token.
    pub shadow_1: &'static str,
    /// Medium elevation shadow token.
    pub shadow_2: &'static str,
    /// Large elevation shadow token.
    pub shadow_3: &'static str,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    // ========================================================================
    // Light Theme
    // ========================================================================
    /// Builds the complete light theme token set.
    pub fn light() -> Self {
        Self {
            name: "light".into(),
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 20.0,
                xl: 32.0,
            },
            radius: Radius {
                sm: 2.0,
                md: 4.0,
                lg: 8.0,
                full: 9999.0,
            },
            font_size: FontSize {
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
                icon: rgba(31, 34, 37, 1.0),

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
    /// Builds the complete dark theme token set.
    pub fn dark() -> Self {
        Self {
            name: "dark".into(),
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 20.0,
                xl: 32.0,
            },
            radius: Radius {
                sm: 2.0,
                md: 4.0,
                lg: 8.0,
                full: 9999.0,
            },
            font_size: FontSize {
                xs: 12.0,
                sm: 14.0,
                md: 14.0,
                lg: 15.0,
                xl: 16.0,
            },

            primary: ColorFamily::new_dark(
                rgb(99, 226, 183),  // #63E2B7 — brighter green for dark
                rgb(127, 231, 196), // #7FE7C4
                rgb(90, 206, 167),  // #5ACEA7
                rgb(42, 148, 125),  // #2A947D (suppl)
            ),
            info: ColorFamily::new_dark(
                rgb(112, 192, 232), // #70C0E8
                rgb(138, 203, 236), // #8ACBEC
                rgb(102, 175, 211), // #66AFD3
                rgb(56, 137, 197),  // #3889C5
            ),
            success: ColorFamily::new_dark(
                rgb(99, 226, 183),  // #63E2B7
                rgb(127, 231, 196), // #7FE7C4
                rgb(90, 206, 167),  // #5ACEA7
                rgb(42, 148, 125),  // #2A947D
            ),
            warning: ColorFamily::new_dark(
                rgb(242, 201, 125), // #F2C97D
                rgb(245, 213, 153), // #F5D599
                rgb(230, 194, 96),  // #E6C260
                rgb(240, 138, 0),   // #F08A00
            ),
            danger: ColorFamily::new_dark(
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
    /// Returns semantic colors for the requested button variant.
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
            ButtonVariant::Text => ButtonVariantColors {
                bg: rgba(0, 0, 0, 0.0),
                hover_bg: self.secondary.hover,
                active_bg: self.secondary.pressed,
                text: self.neutral.text_2,
                border: rgba(0, 0, 0, 0.0),
                text_hover: self.primary.base,
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
            ButtonVariant::Text => ButtonVariantColors {
                bg: rgba(0, 0, 0, 0.0),
                hover_bg: self.secondary.hover,
                active_bg: self.secondary.pressed,
                text: self.neutral.text_2,
                border: rgba(0, 0, 0, 0.0),
                text_hover: self.primary.base,
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
/// Enumerates the supported button variant modes and options.
pub enum ButtonVariant {
    /// Uses the default visual style.
    Default,
    /// Uses the tertiary visual style.
    Tertiary,
    /// Uses the text visual style.
    Text,
    /// Uses the primary visual style.
    Primary,
    /// Uses the info visual style.
    Info,
    /// Uses the success visual style.
    Success,
    /// Uses the warning visual style.
    Warning,
    /// Uses the danger visual style.
    Danger,
}

/// Public design-token group for Liora button variant colors styling.
pub struct ButtonVariantColors {
    /// Base background color used in the normal state.
    pub bg: Hsla,
    /// Background color used while the control is hovered.
    pub hover_bg: Hsla,
    /// Background color used while the control is pressed or active.
    pub active_bg: Hsla,
    /// Foreground text color used in the normal state.
    pub text: Hsla,
    /// Border color used in the normal state.
    pub border: Hsla,
    /// Foreground text color used while the control is hovered.
    pub text_hover: Hsla,
    /// Border color used while the control is hovered.
    pub border_hover: Hsla,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerates the supported button size modes and options.
pub enum ButtonSize {
    /// Uses the small visual style.
    Small,
    /// Uses the default visual style.
    Default,
    /// Uses the large visual style.
    Large,
}

impl ButtonSize {
    /// Returns the height token used for component sizing.
    pub fn height(&self) -> f32 {
        match self {
            ButtonSize::Small => 28.0,   // NaiveUI heightSmall
            ButtonSize::Default => 34.0, // NaiveUI heightMedium
            ButtonSize::Large => 40.0,   // NaiveUI heightLarge
        }
    }

    /// Returns the padding x token used for component sizing.
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
        let theme = Theme::light();
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
    fn dark_semantic_subtle_backgrounds_remain_translucent() {
        let theme = Theme::dark();

        assert!(theme.primary.light_9.a < 0.2);
        assert!(theme.primary.light_8.a > theme.primary.light_9.a);
        assert!(theme.primary.light_7.a > theme.primary.light_8.a);
        assert_eq!(theme.primary.light_9.h, theme.primary.base.h);
    }

    #[test]
    fn light_semantic_subtle_backgrounds_remain_opaque_tints() {
        let theme = Theme::light();

        assert_eq!(theme.primary.light_9.a, 1.0);
        assert!(theme.primary.light_9.l > theme.primary.base.l);
    }

    #[test]
    fn default_button_hover_and_active_backgrounds_are_visible_overlays() {
        let theme = Theme::light();
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
