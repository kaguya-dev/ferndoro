use std::collections::BTreeMap;
use std::sync::Arc;
use egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle};

const INTER_MEDIUM: &[u8] = include_bytes!("../assets/fonts/Inter-Medium.ttf");
const JETBRAINS_NERD: &[u8] = include_bytes!("../assets/fonts/JetBrainsMonoNerdFont-Regular.ttf");

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Register Inter for clean modern UI typography
    fonts.font_data.insert(
        "Inter".to_owned(),
        Arc::new(FontData::from_static(INTER_MEDIUM)),
    );

    // Register JetBrains Mono with full Nerd Font icons
    fonts.font_data.insert(
        "JetBrainsNerd".to_owned(),
        Arc::new(FontData::from_static(JETBRAINS_NERD)),
    );

    // Proportional font family: prioritize Inter for text, JetBrainsNerd for icons/fallback
    if let Some(prop) = fonts.families.get_mut(&FontFamily::Proportional) {
        prop.insert(0, "JetBrainsNerd".to_owned());
        prop.insert(0, "Inter".to_owned());
    }

    // Monospace font family: prioritize JetBrains Mono
    if let Some(mono) = fonts.families.get_mut(&FontFamily::Monospace) {
        mono.insert(0, "JetBrainsNerd".to_owned());
    }

    ctx.set_fonts(fonts);

    // Modern typography scale
    let mut text_styles = BTreeMap::new();
    text_styles.insert(TextStyle::Heading, FontId::new(20.0, FontFamily::Proportional));
    text_styles.insert(TextStyle::Body, FontId::new(13.5, FontFamily::Proportional));
    text_styles.insert(TextStyle::Button, FontId::new(13.0, FontFamily::Proportional));
    text_styles.insert(TextStyle::Small, FontId::new(11.0, FontFamily::Proportional));
    text_styles.insert(TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace));

    let mut style = (*ctx.style()).clone();
    style.text_styles = text_styles;
    ctx.set_style(style);
}
