use egui::{epaint::CornerRadius, epaint::Stroke, Color32, Pos2, Rect, Sense, StrokeKind, Ui, Vec2, Widget};
use chrono::{Datelike, Duration, Local, NaiveDate};
use std::collections::HashMap;

pub struct FrequencyHeatmap<'a> {
    pub daily_data: &'a HashMap<NaiveDate, u32>, // Date -> study minutes
    pub primary_color: Color32,
    pub track_color: Color32,
    pub weeks_to_show: usize,
}

impl<'a> FrequencyHeatmap<'a> {
    pub fn new(
        daily_data: &'a HashMap<NaiveDate, u32>,
        primary_color: Color32,
        track_color: Color32,
    ) -> Self {
        Self {
            daily_data,
            primary_color,
            track_color,
            weeks_to_show: 20,
        }
    }

    pub fn weeks(mut self, weeks: usize) -> Self {
        self.weeks_to_show = weeks;
        self
    }
}

impl<'a> Widget for FrequencyHeatmap<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let cell_size = 14.0;
        let spacing = 4.0;
        let day_label_width = 30.0;
        let header_height = 20.0;

        let total_cols = self.weeks_to_show;
        let total_rows = 7; // Mon..Sun

        let width = day_label_width + (total_cols as f32 * (cell_size + spacing));
        let height = header_height + (total_rows as f32 * (cell_size + spacing));

        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            let today = Local::now().date_naive();
            
            // Align to current week's Sunday/Monday
            let weekday_num = today.weekday().num_days_from_monday(); // 0 = Mon, 6 = Sun
            let start_date = today - Duration::days((self.weeks_to_show as i64 - 1) * 7 + weekday_num as i64);

            let day_names = ["M", "T", "W", "T", "F", "S", "S"];
            for (row, label) in day_names.iter().enumerate() {
                let y = rect.min.y + header_height + row as f32 * (cell_size + spacing) + cell_size / 2.0;
                painter.text(
                    Pos2::new(rect.min.x + 12.0, y),
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(10.0),
                    Color32::from_rgb(140, 130, 170),
                );
            }

            // Draw month labels on top row
            let mut last_month = 0;
            for col in 0..total_cols {
                let col_date = start_date + Duration::days((col * 7) as i64);
                let month = col_date.month();
                if month != last_month {
                    last_month = month;
                    let month_str = match month {
                        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
                        5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
                        9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
                        _ => "",
                    };
                    let x = rect.min.x + day_label_width + col as f32 * (cell_size + spacing);
                    painter.text(
                        Pos2::new(x, rect.min.y + 8.0),
                        egui::Align2::LEFT_CENTER,
                        month_str,
                        egui::FontId::proportional(11.0),
                        Color32::from_rgb(180, 170, 210),
                    );
                }
            }

            // Draw cells
            let pointer_pos = ui.ctx().pointer_latest_pos();

            for col in 0..total_cols {
                for row in 0..total_rows {
                    let date = start_date + Duration::days((col * 7 + row) as i64);
                    if date > today {
                        continue;
                    }

                    let x = rect.min.x + day_label_width + col as f32 * (cell_size + spacing);
                    let y = rect.min.y + header_height + row as f32 * (cell_size + spacing);
                    let cell_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::splat(cell_size));

                    let minutes = self.daily_data.get(&date).copied().unwrap_or(0);
                    
                    // Intensity color calculation
                    let cell_color = if minutes == 0 {
                        self.track_color
                    } else if minutes < 30 {
                        self.primary_color.linear_multiply(0.35)
                    } else if minutes < 60 {
                        self.primary_color.linear_multiply(0.60)
                    } else if minutes < 120 {
                        self.primary_color.linear_multiply(0.85)
                    } else {
                        self.primary_color
                    };

                    painter.rect_filled(cell_rect, CornerRadius::same(3), cell_color);

                    // Highlighting today
                    if date == today {
                        painter.rect_stroke(
                            cell_rect,
                            CornerRadius::same(3),
                            Stroke::new(1.5_f32, Color32::WHITE),
                            StrokeKind::Outside,
                        );
                    }

                    // Tooltip on hover
                    if let Some(pos) = pointer_pos {
                        if cell_rect.contains(pos) {
                            egui::show_tooltip(ui.ctx(), ui.layer_id(), response.id.with(date), |ui| {
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{}", date.format("%A, %b %d, %Y")))
                                            .strong()
                                            .color(Color32::WHITE),
                                    );
                                    let hrs = minutes / 60;
                                    let mins = minutes % 60;
                                    let time_label = if hrs > 0 {
                                        format!("{}h {}m studied", hrs, mins)
                                    } else {
                                        format!("{}m studied", mins)
                                    };
                                    ui.label(
                                        egui::RichText::new(time_label)
                                            .color(self.primary_color),
                                    );
                                });
                            });
                        }
                    }
                }
            }
        }

        response
    }
}
