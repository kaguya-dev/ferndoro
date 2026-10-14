use egui::{epaint::Stroke, Color32, Response, Sense, Ui, Vec2, Widget};
use std::f32::consts::PI;

pub struct TimerDial<'a> {
    pub progress: f32, // 0.0 to 1.0
    pub time_str: &'a str,
    pub phase_name: &'a str,
    pub sub_label: &'a str,
    pub radius: f32,
    pub track_color: Color32,
    pub progress_color: Color32,
    pub glow_color: Color32,
    pub is_running: bool,
}

impl<'a> TimerDial<'a> {
    pub fn new(
        progress: f32,
        time_str: &'a str,
        phase_name: &'a str,
        sub_label: &'a str,
        progress_color: Color32,
        glow_color: Color32,
        track_color: Color32,
        is_running: bool,
    ) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            time_str,
            phase_name,
            sub_label,
            radius: 125.0,
            track_color,
            progress_color,
            glow_color,
            is_running,
        }
    }

    #[allow(dead_code)]
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl<'a> Widget for TimerDial<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = Vec2::splat(self.radius * 2.0 + 40.0);
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            let center = rect.center();
            let r = self.radius;

            // Draw soft subtle glow outer ring
            painter.circle_stroke(
                center,
                r + 6.0,
                Stroke::new(1.0_f32, self.glow_color.linear_multiply(0.25)),
            );

            // Draw background track ring
            painter.circle_stroke(
                center,
                r,
                Stroke::new(10.0_f32, self.track_color),
            );

            // Draw tick marks around the perimeter (60 ticks for seconds/minutes)
            let tick_count = 60;
            for i in 0..tick_count {
                let angle = (i as f32 / tick_count as f32) * 2.0 * PI - PI / 2.0;
                let is_major = i % 5 == 0;
                let inner_len = if is_major { 16.0 } else { 8.0 };
                let tick_color = if is_major {
                    self.track_color.linear_multiply(2.2)
                } else {
                    self.track_color.linear_multiply(1.4)
                };
                let inner_r = r - inner_len;
                let outer_r = r - 2.0;

                let p1 = center + Vec2::new(angle.cos() * inner_r, angle.sin() * inner_r);
                let p2 = center + Vec2::new(angle.cos() * outer_r, angle.sin() * outer_r);
                painter.line_segment([p1, p2], Stroke::new(if is_major { 2.0_f32 } else { 1.0_f32 }, tick_color));
            }

            // Draw progress arc
            if self.progress > 0.001 {
                let start_angle = -PI / 2.0;
                let sweep = self.progress * 2.0 * PI;
                let segments = ((sweep / (2.0 * PI) * 120.0).ceil() as usize).max(4);

                let mut points = Vec::with_capacity(segments + 1);
                for i in 0..=segments {
                    let a = start_angle + sweep * (i as f32 / segments as f32);
                    points.push(center + Vec2::new(a.cos() * r, a.sin() * r));
                }

                // Halo glow line
                painter.add(egui::Shape::line(
                    points.clone(),
                    Stroke::new(16.0_f32, self.glow_color.linear_multiply(0.35)),
                ));

                // Core crisp progress line
                painter.add(egui::Shape::line(
                    points,
                    Stroke::new(9.0_f32, self.progress_color),
                ));

                // Indicator needle/dot at arc tip
                let end_angle = start_angle + sweep;
                let tip = center + Vec2::new(end_angle.cos() * r, end_angle.sin() * r);
                
                // Outer pulsing glow dot
                painter.circle_filled(
                    tip,
                    8.0,
                    self.glow_color.linear_multiply(0.5),
                );
                // Inner bright dot
                painter.circle_filled(
                    tip,
                    4.5,
                    Color32::WHITE,
                );
            }

            // Phase indicator badge text
            painter.text(
                center - Vec2::new(0.0, 48.0),
                egui::Align2::CENTER_CENTER,
                self.phase_name,
                egui::FontId::proportional(14.0),
                self.progress_color,
            );

            // Large time readout
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                self.time_str,
                egui::FontId::monospace(48.0),
                Color32::WHITE,
            );

            // Sub-label (e.g. active subject or status)
            painter.text(
                center + Vec2::new(0.0, 46.0),
                egui::Align2::CENTER_CENTER,
                self.sub_label,
                egui::FontId::proportional(13.0),
                self.glow_color.linear_multiply(0.9),
            );

            // Active pulsing indicator symbol when running
            if self.is_running {
                let pulse_dot = center + Vec2::new(0.0, 68.0);
                painter.circle_filled(pulse_dot, 3.5, Color32::from_rgb(52, 211, 153));
            }
        }

        response
    }
}
