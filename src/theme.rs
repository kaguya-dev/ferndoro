use egui::{epaint::CornerRadius, Color32, Stroke, Vec2, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    PurpleDark,
    MidnightViolet,
    CyberNeon,
    ObsidianAmethyst,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::PurpleDark
    }
}

impl ThemeMode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::PurpleDark => "Purple Dark (Default)",
            Self::MidnightViolet => "Midnight Violet",
            Self::CyberNeon => "Cyber Neon",
            Self::ObsidianAmethyst => "Obsidian Amethyst",
        }
    }

    pub fn all() -> &'static [ThemeMode] {
        &[
            ThemeMode::PurpleDark,
            ThemeMode::MidnightViolet,
            ThemeMode::CyberNeon,
            ThemeMode::ObsidianAmethyst,
        ]
    }

    pub fn colors(&self) -> ThemePalette {
        match self {
            Self::PurpleDark => ThemePalette {
                bg_deep: Color32::from_rgb(15, 12, 24),        // #0F0C18
                bg_card: Color32::from_rgb(26, 20, 42),        // #1A142A
                bg_card_hover: Color32::from_rgb(37, 28, 61),  // #251C3D
                bg_card_border: Color32::from_rgb(79, 46, 128),// #4F2E80
                
                accent_primary: Color32::from_rgb(168, 85, 247),   // Vibrant Amethyst #A855F7
                accent_secondary: Color32::from_rgb(192, 132, 252),// Light Violet #C084FC
                accent_glow: Color32::from_rgb(232, 121, 249),     // Fuchsia Neon #E879F9
                accent_dark: Color32::from_rgb(91, 33, 182),       // Deep Purple #5B21B6
                
                text_primary: Color32::from_rgb(255, 255, 255),
                text_secondary: Color32::from_rgb(209, 196, 233),  // Soft Lilac #D1C4E9
                text_muted: Color32::from_rgb(138, 120, 168),      // Muted Purple #8A78A8
                
                success: Color32::from_rgb(52, 211, 153),          // Emerald
                warning: Color32::from_rgb(251, 191, 36),          // Amber
                break_phase: Color32::from_rgb(56, 189, 248),      // Sky Blue
                track_bg: Color32::from_rgb(33, 25, 54),
            },
            Self::MidnightViolet => ThemePalette {
                bg_deep: Color32::from_rgb(10, 10, 20),
                bg_card: Color32::from_rgb(20, 19, 38),
                bg_card_hover: Color32::from_rgb(30, 28, 56),
                bg_card_border: Color32::from_rgb(60, 52, 104),
                
                accent_primary: Color32::from_rgb(139, 92, 246),
                accent_secondary: Color32::from_rgb(167, 139, 250),
                accent_glow: Color32::from_rgb(217, 70, 239),
                accent_dark: Color32::from_rgb(76, 29, 149),
                
                text_primary: Color32::from_rgb(248, 250, 252),
                text_secondary: Color32::from_rgb(203, 213, 225),
                text_muted: Color32::from_rgb(112, 107, 146),
                
                success: Color32::from_rgb(45, 212, 191),
                warning: Color32::from_rgb(245, 158, 11),
                break_phase: Color32::from_rgb(96, 165, 250),
                track_bg: Color32::from_rgb(27, 25, 50),
            },
            Self::CyberNeon => ThemePalette {
                bg_deep: Color32::from_rgb(8, 7, 13),
                bg_card: Color32::from_rgb(18, 14, 29),
                bg_card_hover: Color32::from_rgb(32, 23, 53),
                bg_card_border: Color32::from_rgb(147, 51, 234),
                
                accent_primary: Color32::from_rgb(217, 70, 239),   // Hot Pink Neon
                accent_secondary: Color32::from_rgb(192, 38, 211),
                accent_glow: Color32::from_rgb(34, 211, 238),      // Cyan Neon
                accent_dark: Color32::from_rgb(112, 26, 117),
                
                text_primary: Color32::from_rgb(255, 255, 255),
                text_secondary: Color32::from_rgb(240, 171, 252),
                text_muted: Color32::from_rgb(140, 105, 168),
                
                success: Color32::from_rgb(74, 222, 128),
                warning: Color32::from_rgb(250, 204, 21),
                break_phase: Color32::from_rgb(34, 211, 238),
                track_bg: Color32::from_rgb(25, 18, 41),
            },
            Self::ObsidianAmethyst => ThemePalette {
                bg_deep: Color32::from_rgb(12, 10, 16),
                bg_card: Color32::from_rgb(22, 18, 30),
                bg_card_hover: Color32::from_rgb(34, 27, 46),
                bg_card_border: Color32::from_rgb(67, 42, 99),
                
                accent_primary: Color32::from_rgb(147, 51, 234),
                accent_secondary: Color32::from_rgb(168, 85, 247),
                accent_glow: Color32::from_rgb(244, 114, 182),
                accent_dark: Color32::from_rgb(88, 28, 135),
                
                text_primary: Color32::from_rgb(253, 244, 255),
                text_secondary: Color32::from_rgb(216, 180, 254),
                text_muted: Color32::from_rgb(124, 101, 153),
                
                success: Color32::from_rgb(52, 211, 153),
                warning: Color32::from_rgb(251, 146, 60),
                break_phase: Color32::from_rgb(129, 140, 248),
                track_bg: Color32::from_rgb(29, 23, 40),
            },
        }
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let palette = self.colors();
        let mut visuals = Visuals::dark();

        visuals.panel_fill = palette.bg_deep;
        visuals.window_fill = palette.bg_card;
        visuals.extreme_bg_color = palette.bg_deep;

        visuals.widgets.noninteractive.bg_fill = palette.bg_card;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, palette.bg_card_border);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, palette.text_secondary);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);

        visuals.widgets.inactive.bg_fill = palette.bg_card;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, palette.bg_card_border);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, palette.text_primary);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(8);

        visuals.widgets.hovered.bg_fill = palette.bg_card_hover;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.5_f32, palette.accent_secondary);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.5_f32, Color32::WHITE);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(8);

        visuals.widgets.active.bg_fill = palette.accent_dark;
        visuals.widgets.active.bg_stroke = Stroke::new(2.0_f32, palette.accent_glow);
        visuals.widgets.active.fg_stroke = Stroke::new(2.0_f32, Color32::WHITE);
        visuals.widgets.active.corner_radius = CornerRadius::same(8);

        visuals.selection.bg_fill = palette.accent_primary;
        visuals.selection.stroke = Stroke::new(1.5_f32, palette.accent_glow);

        visuals.window_corner_radius = CornerRadius::same(12);
        visuals.window_stroke = Stroke::new(1.5_f32, palette.bg_card_border);

        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = Vec2::new(10.0, 10.0);
        style.spacing.button_padding = Vec2::new(12.0, 7.0);
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
    pub accent_secondary: Color32,
    pub accent_glow: Color32,
    pub accent_dark: Color32,
    
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    
    pub success: Color32,
    pub warning: Color32,
    pub break_phase: Color32,
    pub track_bg: Color32,
}
