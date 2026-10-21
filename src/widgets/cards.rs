use egui::{epaint::CornerRadius, epaint::Stroke, Color32, Frame, Margin, Response, Ui, Vec2};
use crate::theme::ThemePalette;

pub fn card_frame(palette: &ThemePalette) -> Frame {
    Frame::new()
        .fill(palette.bg_card)
        .stroke(Stroke::new(1.0_f32, palette.bg_card_border))
        .corner_radius(CornerRadius::same(18))
        .inner_margin(Margin::same(16))
}

pub fn elevated_card_frame(palette: &ThemePalette) -> Frame {
    Frame::new()
        .fill(palette.bg_card_hover)
        .stroke(Stroke::new(1.0_f32, palette.bg_card_border))
        .corner_radius(CornerRadius::same(18))
        .inner_margin(Margin::same(16))
}

pub fn glowing_card_frame(palette: &ThemePalette, border_color: Color32) -> Frame {
    Frame::new()
        .fill(palette.bg_card)
        .stroke(Stroke::new(1.5_f32, border_color))
        .corner_radius(CornerRadius::same(18))
        .inner_margin(Margin::same(16))
}

pub fn stat_card(
    ui: &mut Ui,
    palette: &ThemePalette,
    icon: &str,
    label: &str,
    value: &str,
    sub_note: Option<&str>,
) {
    card_frame(palette).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(icon)
                    .size(24.0)
                    .color(palette.accent_primary),
            );
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(label)
                        .size(12.0)
                        .color(palette.text_secondary),
                );
                ui.label(
                    egui::RichText::new(value)
                        .size(19.0)
                        .strong()
                        .color(palette.text_primary),
                );
                if let Some(sub) = sub_note {
                    ui.label(
                        egui::RichText::new(sub)
                            .size(11.0)
                            .color(palette.text_muted),
                    );
                }
            });
        });
    });
}

pub fn status_pill(ui: &mut Ui, text: &str, bg: Color32, fg: Color32) -> Response {
    let padding = Vec2::new(12.0, 5.0);
    let font_id = egui::FontId::proportional(12.0);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, fg);
    let desired_size = galley.size() + padding * 2.0;

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, CornerRadius::same(14), bg);
        painter.galley(rect.min + padding, galley, fg);
    }
    response
}
