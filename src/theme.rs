use egui::{epaint::CornerRadius, Color32, Stroke, Vec2, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    MaterialYouViolet,
    MidnightAmethyst,
    CyberVelvet,
    MinimalObsidian,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::MaterialYouViolet
    }
}

impl ThemeMode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::MaterialYouViolet => "Material You Amethyst (Default)",
            Self::MidnightAmethyst => "Midnight Tonal Violet",
            Self::CyberVelvet => "Cyber Velvet (Rose & Violet)",
            Self::MinimalObsidian => "Minimal Obsidian",
        }
    }

    pub fn all() -> &'static [ThemeMode] {
        &[
            ThemeMode::MaterialYouViolet,
            ThemeMode::MidnightAmethyst,
            ThemeMode::CyberVelvet,
            ThemeMode::MinimalObsidian,
        ]
    }

    pub fn colors(&self) -> ThemePalette {
        match self {
            Self::MaterialYouViolet => ThemePalette {
                // Material 3 Dark Surface tonal tiers
                bg_deep: Color32::from_rgb(18, 16, 22),         // #121016 Surface
                bg_card: Color32::from_rgb(29, 25, 38),         // #1D1926 Surface Container
                bg_card_hover: Color32::from_rgb(38, 33, 51),   // #262133 Surface Container High
                bg_card_border: Color32::from_rgb(53, 46, 68),  // #352E44 Outline Variant
                
                // Material 3 Dynamic Lavender/Amethyst
                accent_primary: Color32::from_rgb(208, 188, 255),    // #D0BCFF Primary
                accent_container: Color32::from_rgb(79, 55, 139),    // #4F378B Primary Container (Active pill)
                on_accent_container: Color32::from_rgb(234, 221, 255), // #EADDFF
                accent_secondary: Color32::from_rgb(204, 194, 220),  // #CCC2DC Secondary
                accent_tertiary: Color32::from_rgb(239, 184, 200),   // #EFB8C8 Tertiary Rose/Gold
                accent_glow: Color32::from_rgb(219, 175, 253),       // #DBAFFD Soft glow
                accent_dark: Color32::from_rgb(56, 30, 114),         // #381E72 On Primary

                text_primary: Color32::from_rgb(230, 225, 229),      // #E6E1E5 On Surface
                text_secondary: Color32::from_rgb(202, 196, 208),    // #CAC4D0 On Surface Variant
                text_muted: Color32::from_rgb(147, 143, 153),        // #938F99 Muted
                
                success: Color32::from_rgb(133, 217, 150),          // #85D996 Sage Green
                warning: Color32::from_rgb(247, 186, 85),           // #F7BA55 Warm Amber
                error: Color32::from_rgb(242, 184, 181),             // #F2B8B5 Coral Red
                break_phase: Color32::from_rgb(165, 216, 255),       // #A5D8FF Calm Cyan
                track_bg: Color32::from_rgb(40, 35, 53),            // #282335
            },
            Self::MidnightAmethyst => ThemePalette {
                bg_deep: Color32::from_rgb(14, 13, 20),
                bg_card: Color32::from_rgb(24, 22, 34),
                bg_card_hover: Color32::from_rgb(34, 30, 48),
                bg_card_border: Color32::from_rgb(48, 44, 66),
                
                accent_primary: Color32::from_rgb(194, 175, 255),
                accent_container: Color32::from_rgb(67, 48, 122),
                on_accent_container: Color32::from_rgb(228, 218, 255),
                accent_secondary: Color32::from_rgb(196, 188, 212),
                accent_tertiary: Color32::from_rgb(242, 194, 208),
                accent_glow: Color32::from_rgb(210, 168, 255),
                accent_dark: Color32::from_rgb(48, 26, 96),
                
                text_primary: Color32::from_rgb(238, 234, 242),
                text_secondary: Color32::from_rgb(192, 187, 201),
                text_muted: Color32::from_rgb(136, 132, 148),
                
                success: Color32::from_rgb(140, 220, 165),
                warning: Color32::from_rgb(248, 195, 105),
                error: Color32::from_rgb(244, 168, 164),
                break_phase: Color32::from_rgb(168, 218, 255),
                track_bg: Color32::from_rgb(32, 28, 44),
            },
            Self::CyberVelvet => ThemePalette {
                bg_deep: Color32::from_rgb(16, 12, 20),
                bg_card: Color32::from_rgb(28, 20, 36),
                bg_card_hover: Color32::from_rgb(40, 28, 52),
                bg_card_border: Color32::from_rgb(64, 44, 82),
                
                accent_primary: Color32::from_rgb(234, 155, 245),
                accent_container: Color32::from_rgb(98, 38, 110),
                on_accent_container: Color32::from_rgb(250, 215, 255),
                accent_secondary: Color32::from_rgb(216, 180, 226),
                accent_tertiary: Color32::from_rgb(255, 179, 196),
                accent_glow: Color32::from_rgb(245, 178, 255),
                accent_dark: Color32::from_rgb(68, 22, 78),
                
                text_primary: Color32::from_rgb(245, 238, 248),
                text_secondary: Color32::from_rgb(212, 198, 218),
                text_muted: Color32::from_rgb(148, 130, 156),
                
                success: Color32::from_rgb(130, 225, 160),
                warning: Color32::from_rgb(252, 190, 95),
                error: Color32::from_rgb(248, 160, 160),
                break_phase: Color32::from_rgb(175, 225, 255),
                track_bg: Color32::from_rgb(36, 26, 46),
            },
            Self::MinimalObsidian => ThemePalette {
                bg_deep: Color32::from_rgb(14, 14, 16),
                bg_card: Color32::from_rgb(24, 24, 28),
                bg_card_hover: Color32::from_rgb(34, 34, 40),
                bg_card_border: Color32::from_rgb(46, 46, 54),
                
                accent_primary: Color32::from_rgb(216, 204, 242),
                accent_container: Color32::from_rgb(58, 54, 76),
                on_accent_container: Color32::from_rgb(236, 230, 250),
                accent_secondary: Color32::from_rgb(198, 194, 208),
                accent_tertiary: Color32::from_rgb(230, 196, 208),
                accent_glow: Color32::from_rgb(210, 195, 240),
                accent_dark: Color32::from_rgb(44, 40, 58),
                
                text_primary: Color32::from_rgb(240, 240, 244),
                text_secondary: Color32::from_rgb(196, 196, 204),
                text_muted: Color32::from_rgb(138, 138, 148),
                
                success: Color32::from_rgb(135, 215, 150),
                warning: Color32::from_rgb(244, 188, 90),
                error: Color32::from_rgb(238, 165, 165),
                break_phase: Color32::from_rgb(165, 210, 245),
                track_bg: Color32::from_rgb(32, 32, 38),
            },
        }
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let palette = self.colors();
        let mut visuals = Visuals::dark();

        visuals.panel_fill = palette.bg_deep;
        visuals.window_fill = palette.bg_card;
        visuals.extreme_bg_color = palette.bg_deep;

        // Material 3 pill and rounded corners
        visuals.widgets.noninteractive.bg_fill = palette.bg_card;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, palette.bg_card_border);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, palette.text_secondary);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(16);

        visuals.widgets.inactive.bg_fill = palette.bg_card;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, palette.bg_card_border);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, palette.text_primary);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(16);

        visuals.widgets.hovered.bg_fill = palette.bg_card_hover;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, palette.accent_primary);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(16);

        visuals.widgets.active.bg_fill = palette.accent_container;
        visuals.widgets.active.bg_stroke = Stroke::new(1.5_f32, palette.accent_primary);
        visuals.widgets.active.fg_stroke = Stroke::new(1.5_f32, palette.on_accent_container);
        visuals.widgets.active.corner_radius = CornerRadius::same(16);

        visuals.selection.bg_fill = palette.accent_container;
        visuals.selection.stroke = Stroke::new(1.0_f32, palette.accent_primary);

        visuals.window_corner_radius = CornerRadius::same(24);
        visuals.window_stroke = Stroke::new(1.0_f32, palette.bg_card_border);

        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = Vec2::new(12.0, 12.0);
        style.spacing.button_padding = Vec2::new(16.0, 9.0);
        ctx.set_style(style);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub bg_deep: Color32,
    pub bg_card: Color32,
    pub bg_card_hover: Color32,
    pub bg_card_border: Color32,
    
    pub accent_primary: Color32,
    pub accent_container: Color32,
    pub on_accent_container: Color32,
    pub accent_secondary: Color32,
    pub accent_tertiary: Color32,
    pub accent_glow: Color32,
    #[allow(dead_code)]
    pub accent_dark: Color32,
    
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
    pub break_phase: Color32,
    pub track_bg: Color32,
}
