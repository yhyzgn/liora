use gpui::Hsla;

fn rgb(r: u8, g: u8, b: u8) -> Hsla {
    gpui::hsla(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

pub struct AuraColorPalette {
    pub primary: Hsla,
    pub primary_hover: Hsla,
    pub primary_active: Hsla,
    pub primary_light: Hsla,
    pub success: Hsla,
    pub success_hover: Hsla,
    pub success_active: Hsla,
    pub warning: Hsla,
    pub warning_hover: Hsla,
    pub warning_active: Hsla,
    pub danger: Hsla,
    pub danger_hover: Hsla,
    pub danger_active: Hsla,
    pub info: Hsla,
    pub info_hover: Hsla,
    pub info_active: Hsla,
}

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

pub struct AuraTheme {
    pub name: String,
    pub color: AuraColorPalette,
    pub spacing: AuraSpacing,
    pub radius: AuraRadius,
    pub font_size: AuraFontSize,
    pub background: Hsla,
    pub text_primary: Hsla,
    pub text_secondary: Hsla,
    pub text_on_primary: Hsla,
    pub border: Hsla,
    pub border_light: Hsla,
    pub disabled_bg: Hsla,
    pub disabled_text: Hsla,
    pub disabled_border: Hsla,
}

impl Default for AuraTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl AuraTheme {
    pub fn light() -> Self {
        Self {
            name: "light".into(),
            color: AuraColorPalette {
                primary: rgb(64, 158, 255),
                primary_hover: rgb(121, 187, 255),
                primary_active: rgb(51, 126, 204),
                primary_light: rgb(236, 245, 255),
                success: rgb(103, 194, 58),
                success_hover: rgb(133, 206, 97),
                success_active: rgb(82, 155, 46),
                warning: rgb(230, 162, 60),
                warning_hover: rgb(235, 181, 99),
                warning_active: rgb(207, 146, 54),
                danger: rgb(245, 108, 108),
                danger_hover: rgb(247, 137, 137),
                danger_active: rgb(220, 89, 89),
                info: rgb(144, 147, 153),
                info_hover: rgb(166, 169, 173),
                info_active: rgb(115, 118, 122),
            },
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
            background: rgb(255, 255, 255),
            text_primary: rgb(48, 49, 51),
            text_secondary: rgb(144, 147, 153),
            text_on_primary: rgb(255, 255, 255),
            border: rgb(220, 223, 230),
            border_light: rgb(235, 238, 245),
            disabled_bg: rgb(245, 247, 250),
            disabled_text: rgb(192, 196, 204),
            disabled_border: rgb(228, 231, 237),
        }
    }

    pub fn dark() -> Self {
        let mut dark = Self::light();
        dark.name = "dark".into();
        dark.background = rgb(29, 30, 31);
        dark.text_primary = rgb(229, 234, 243);
        dark.text_secondary = rgb(163, 166, 173);
        dark.border = rgb(76, 77, 79);
        dark.border_light = rgb(54, 54, 55);
        dark.color.primary_light = rgb(31, 45, 61);
        dark.disabled_bg = rgb(54, 54, 55);
        dark.disabled_text = rgb(76, 77, 79);
        dark.disabled_border = rgb(54, 54, 55);
        dark
    }

    pub fn color_by_variant(&self, variant: ButtonVariant) -> ButtonVariantColors {
        match variant {
            ButtonVariant::Default => ButtonVariantColors {
                bg: self.background,
                hover_bg: self.border_light,
                active_bg: self.border,
                text: self.text_primary,
                border: self.border,
            },
            ButtonVariant::Primary => ButtonVariantColors {
                bg: self.color.primary,
                hover_bg: self.color.primary_hover,
                active_bg: self.color.primary_active,
                text: self.text_on_primary,
                border: self.color.primary,
            },
            ButtonVariant::Success => ButtonVariantColors {
                bg: self.color.success,
                hover_bg: self.color.success_hover,
                active_bg: self.color.success_active,
                text: self.text_on_primary,
                border: self.color.success,
            },
            ButtonVariant::Warning => ButtonVariantColors {
                bg: self.color.warning,
                hover_bg: self.color.warning_hover,
                active_bg: self.color.warning_active,
                text: self.text_on_primary,
                border: self.color.warning,
            },
            ButtonVariant::Danger => ButtonVariantColors {
                bg: self.color.danger,
                hover_bg: self.color.danger_hover,
                active_bg: self.color.danger_active,
                text: self.text_on_primary,
                border: self.color.danger,
            },
            ButtonVariant::Info => ButtonVariantColors {
                bg: self.color.info,
                hover_bg: self.color.info_hover,
                active_bg: self.color.info_active,
                text: self.text_on_primary,
                border: self.color.info,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

pub struct ButtonVariantColors {
    pub bg: Hsla,
    pub hover_bg: Hsla,
    pub active_bg: Hsla,
    pub text: Hsla,
    pub border: Hsla,
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
            ButtonSize::Small => 24.0,
            ButtonSize::Default => 32.0,
            ButtonSize::Large => 40.0,
        }
    }

    pub fn padding_x(&self) -> f32 {
        match self {
            ButtonSize::Small => 8.0,
            ButtonSize::Default => 15.0,
            ButtonSize::Large => 19.0,
        }
    }
}
