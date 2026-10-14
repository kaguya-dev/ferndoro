use egui::{epaint::CornerRadius, epaint::Stroke, Color32, Pos2, Rect, Response, Sense, StrokeKind, Ui, Vec2, Widget};

pub struct CustomProgressBar<'a> {
    progress: f32,
    label: Option<&'a str>,
    detail: Option<&'a str>,
    height: f32,
    fill_color: Color32,
    glow_color: Color32,
    bg_color: Color32,
    show_percentage: bool,
}

impl<'a> CustomProgressBar<'a> {
    pub fn new(progress: f32, fill_color: Color32, glow_color: Color32, bg_color: Color32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            label: None,
            detail: None,
            height: 14.0,
            fill_color,
            glow_color,
            bg_color,
            show_percentage: true,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn detail(mut self, detail: &'a str) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    #[allow(dead_code)]
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }
}

impl<'a> Widget for CustomProgressBar<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let total_width = ui.available_width().max(100.0);
        let header_height = if self.label.is_some() || self.detail.is_some() {
            20.0
        } else {
            0.0
        };
        let total_height = self.height + header_height + 4.0;

        let (rect, response) = ui.allocate_exact_size(Vec2::new(total_width, total_height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            let mut bar_top = rect.min.y;

            if let Some(label) = self.label {
                painter.text(
                    Pos2::new(rect.min.x, bar_top),
                    egui::Align2::LEFT_TOP,
                    label,
                    egui::FontId::proportional(13.0),
                    Color32::WHITE,
                );

                if let Some(detail) = self.detail {
                    painter.text(
                        Pos2::new(rect.max.x, bar_top),
                        egui::Align2::RIGHT_TOP,
                        detail,
                        egui::FontId::proportional(12.0),
                        self.glow_color,
                    );
                } else if self.show_percentage {
                    painter.text(
                        Pos2::new(rect.max.x, bar_top),
                        egui::Align2::RIGHT_TOP,
                        format!("{:.0}%", self.progress * 100.0),
                        egui::FontId::monospace(12.0),
                        self.fill_color,
                    );
                }
                bar_top += header_height;
            }

            let bar_rect = Rect::from_min_max(
                Pos2::new(rect.min.x, bar_top),
                Pos2::new(rect.max.x, bar_top + self.height),
            );

            let radius_u8 = (self.height / 2.0).round().clamp(1.0, 255.0) as u8;

            // Draw track background
            painter.rect_filled(bar_rect, CornerRadius::same(radius_u8), self.bg_color);
            painter.rect_stroke(
                bar_rect,
                CornerRadius::same(radius_u8),
                Stroke::new(1.0_f32, self.fill_color.linear_multiply(0.2)),
                StrokeKind::Outside,
            );

            // Draw filled progress
            let filled_width = (bar_rect.width() * self.progress).max(0.0);
            if filled_width > 2.0 {
                let fill_rect = Rect::from_min_max(
                    bar_rect.min,
                    Pos2::new(bar_rect.min.x + filled_width, bar_rect.max.y),
                );

                // Glow shadow
                painter.rect_filled(
                    fill_rect,
                    CornerRadius::same(radius_u8),
                    self.fill_color,
                );

                // Highlight strip on top edge
                let highlight_rect = Rect::from_min_max(
                    fill_rect.min + Vec2::new(2.0, 1.0),
                    Pos2::new(fill_rect.max.x - 2.0, fill_rect.min.y + self.height * 0.4),
                );
                if highlight_rect.width() > 1.0 {
                    let small_rad = (self.height / 4.0).round().clamp(1.0, 255.0) as u8;
                    painter.rect_filled(
                        highlight_rect,
                        CornerRadius::same(small_rad),
                        self.glow_color.linear_multiply(0.5),
                    );
                }
            }
        }

        response
    }
}
