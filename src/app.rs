use std::time::Instant;
use chrono::{Duration, Local};
use egui::{epaint::CornerRadius, Color32, Margin, RichText, ScrollArea, Ui, Vec2};

use crate::cycle::{CyclePreset, StudyCycleState, StudySubject};
use crate::exam::{ExamState, ExamTest};
use crate::frequency::FrequencyTrackerState;
use crate::icons::*;
use crate::pomodoro::{PomodoroPhase, PomodoroState, TimerStatus};
use crate::storage::{AppData, Storage};
use crate::theme::{ThemeMode, ThemePalette};
use crate::widgets::{
    card_frame, glowing_card_frame, stat_card, status_pill,
    CustomProgressBar, FrequencyHeatmap, TimerDial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Pomodoro,
    StudyCycle,
    TestMode,
    FrequencyTracker,
    Settings,
}

pub struct FernDoroApp {
    pub theme: ThemeMode,
    pub pomodoro: PomodoroState,
    pub cycle: StudyCycleState,
    pub frequency: FrequencyTrackerState,
    pub exams: ExamState,
    pub active_tab: ActiveTab,
    pub last_tick: Instant,
    
    // UI state for adding new subjects
    pub new_subject_name: String,
    pub new_subject_mins: u32,
    pub new_subject_color: String,

    // UI state for preset builder modal / accordion
    pub show_presets_drawer: bool,
    pub custom_preset_name: String,
    pub custom_preset_desc: String,

    // UI state for adding exams
    pub new_exam_subject: String,
    pub new_exam_title: String,
    pub new_exam_days: i32,
    pub new_exam_weight: u32,
    pub new_exam_diff: u32,

    // Notification toast / message
    pub toast_message: Option<(String, Instant)>,
}

impl FernDoroApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let data = Storage::load();
        data.theme.apply(&cc.egui_ctx);

        let mut app = Self {
            theme: data.theme,
            pomodoro: data.pomodoro,
            cycle: data.cycle,
            frequency: data.frequency,
            exams: data.exams,
            active_tab: ActiveTab::Pomodoro,
            last_tick: Instant::now(),
            new_subject_name: String::new(),
            new_subject_mins: 45,
            new_subject_color: "#A855F7".to_string(),
            show_presets_drawer: false,
            custom_preset_name: String::new(),
            custom_preset_desc: String::new(),
            new_exam_subject: "Calculos".to_string(),
            new_exam_title: "P1 Midterm Exam".to_string(),
            new_exam_days: 7,
            new_exam_weight: 4,
            new_exam_diff: 4,
            toast_message: None,
        };

        // Sync exam boosts to cycle targets on startup
        app.exams.sync_all_exam_boosts(&mut app.cycle);

        // Sync active subject name if cycle has one
        if let Some(sub) = app.cycle.active_subject() {
            app.pomodoro.active_subject_name = sub.name.clone();
        }

        app
    }

    pub fn save_state(&self) {
        let data = AppData {
            theme: self.theme,
            pomodoro: self.pomodoro.clone(),
            cycle: self.cycle.clone(),
            frequency: self.frequency.clone(),
            exams: self.exams.clone(),
        };
        Storage::save(&data);
    }

    pub fn set_toast(&mut self, msg: &str) {
        self.toast_message = Some((msg.to_string(), Instant::now()));
    }
}

impl eframe::App for FernDoroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let dt = (now - self.last_tick).as_secs_f32();
        self.last_tick = now;

        // Tick timer and handle completion
        if let Some(event) = self.pomodoro.tick(dt) {
            if event.phase == PomodoroPhase::Work {
                self.cycle.add_study_time(&event.subject_name, event.duration_minutes);
                self.frequency.record_study_session(event.duration_minutes, true);
                self.set_toast(&format!("Session finished! +{}m logged to {}", event.duration_minutes, event.subject_name));
            }
            self.save_state();
        }

        if self.pomodoro.status == TimerStatus::Running {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }

        let palette = self.theme.colors();

        // Top Navigation Bar
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::new().fill(palette.bg_deep).inner_margin(Margin::symmetric(18, 14)))
            .show(ctx, |ui| {
                self.render_top_bar(ui, &palette);
            });

        // Main Content Area
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(palette.bg_deep).inner_margin(Margin::symmetric(22, 14)))
            .show(ctx, |ui| {
                match self.active_tab {
                    ActiveTab::Pomodoro => self.render_pomodoro_tab(ui, &palette),
                    ActiveTab::StudyCycle => self.render_cycle_tab(ui, &palette),
                    ActiveTab::TestMode => self.render_test_mode_tab(ui, &palette),
                    ActiveTab::FrequencyTracker => self.render_frequency_tab(ui, &palette),
                    ActiveTab::Settings => self.render_settings_tab(ui, ctx, &palette),
                }

                if let Some((msg, created_at)) = &self.toast_message {
                    if created_at.elapsed().as_secs() < 4 {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            status_pill(ui, &format!("{} {}", SPARKLE, msg), palette.accent_primary, Color32::WHITE);
                        });
                    }
                }
            });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_state();
    }
}

impl FernDoroApp {
    fn render_top_bar(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("{} FernDoro", LOGO_LEAF))
                    .size(20.0)
                    .strong()
                    .color(palette.accent_glow),
            );

            ui.add_space(20.0);

            // Tab Navigation with Test Mode included
            let tabs = [
                (ActiveTab::Pomodoro, TAB_POMODORO, "Pomodoro"),
                (ActiveTab::StudyCycle, TAB_CYCLE, "Study Cycle"),
                (ActiveTab::TestMode, TEST_PENCIL, "Test Mode"),
                (ActiveTab::FrequencyTracker, TAB_FREQUENCY, "Frequency Tracker"),
                (ActiveTab::Settings, TAB_SETTINGS, "Settings"),
            ];

            for (tab, icon, title) in tabs {
                let is_active = self.active_tab == tab;
                let bg_color = if is_active { palette.accent_primary } else { palette.bg_card };
                let text_color = if is_active { Color32::WHITE } else { palette.text_secondary };

                let label_text = format!("{} {}", icon, title);
                let btn = egui::Button::new(RichText::new(label_text).strong().size(13.0).color(text_color))
                    .fill(bg_color)
                    .stroke(egui::Stroke::new(1.0_f32, if is_active { palette.accent_glow } else { palette.bg_card_border }))
                    .corner_radius(CornerRadius::same(8));

                if ui.add(btn).clicked() {
                    self.active_tab = tab;
                }
                ui.add_space(3.0);
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Streak Pill
                let streak = self.frequency.current_streak_days();
                status_pill(
                    ui,
                    &format!("{} {}d streak", FLAME, streak),
                    palette.bg_card,
                    palette.accent_secondary,
                );

                ui.add_space(6.0);

                // Active Exam Badge if any test is <= 7 days
                let today = Local::now().date_naive();
                if let Some(urgent_exam) = self.exams.exams.iter().find(|e| e.days_remaining(today) <= 7 && e.days_remaining(today) >= 0) {
                    let days = urgent_exam.days_remaining(today);
                    status_pill(
                        ui,
                        &format!("{} {} in {}d", CALENDAR_ALERT, urgent_exam.subject_name, days),
                        Color32::from_rgb(153, 27, 27),
                        Color32::WHITE,
                    );
                    ui.add_space(6.0);
                }

                // Active Subject Pill
                let active_name = self.cycle.active_subject().map(|s| s.name.as_str()).unwrap_or("General Study");
                status_pill(
                    ui,
                    &format!("{} {}", TARGET, active_name),
                    palette.bg_card,
                    palette.text_primary,
                );
            });
        });

        ui.add_space(6.0);
        ui.separator();
    }

    fn render_pomodoro_tab(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);

            // Active subject selector row
            ui.horizontal(|ui| {
                ui.label(RichText::new("Target Subject:").size(14.0).color(palette.text_secondary));
                egui::ComboBox::from_id_salt("pomodoro_subject_selector")
                    .selected_text(RichText::new(&self.pomodoro.active_subject_name).size(14.0).strong().color(palette.accent_glow))
                    .show_ui(ui, |ui| {
                        for (i, subject) in self.cycle.subjects.iter().enumerate() {
                            if ui.selectable_label(self.pomodoro.active_subject_name == subject.name, &subject.name).clicked() {
                                self.pomodoro.active_subject_name = subject.name.clone();
                                self.cycle.active_index = i;
                                self.save_state();
                            }
                        }
                    });
            });

            ui.add_space(12.0);

            let progress = self.pomodoro.progress_fraction();
            let time_str = self.pomodoro.formatted_time();
            let phase_name = self.pomodoro.phase.display_name();
            let sub_label = format!("Active: {}", self.pomodoro.active_subject_name);

            let progress_color = match self.pomodoro.phase {
                PomodoroPhase::Work => palette.accent_primary,
                PomodoroPhase::ShortBreak => palette.break_phase,
                PomodoroPhase::LongBreak => palette.success,
            };

            let dial = TimerDial::new(
                progress,
                &time_str,
                phase_name,
                &sub_label,
                progress_color,
                palette.accent_glow,
                palette.track_bg,
                self.pomodoro.status == TimerStatus::Running,
            );

            ui.add(dial);

            ui.add_space(18.0);

            // Primary Control Buttons
            ui.horizontal(|ui| {
                let (play_text, play_color) = match self.pomodoro.status {
                    TimerStatus::Running => (format!("{} Pause", PAUSE), palette.warning),
                    TimerStatus::Paused | TimerStatus::Stopped => (format!("{} Start Focus", PLAY), palette.accent_primary),
                };

                let primary_btn = egui::Button::new(RichText::new(play_text).size(15.0).strong().color(Color32::WHITE))
                    .fill(play_color)
                    .min_size(Vec2::new(140.0, 42.0))
                    .corner_radius(CornerRadius::same(10));

                if ui.add(primary_btn).clicked() {
                    match self.pomodoro.status {
                        TimerStatus::Running => self.pomodoro.pause(),
                        TimerStatus::Paused | TimerStatus::Stopped => self.pomodoro.start(),
                    }
                    self.save_state();
                }

                ui.add_space(12.0);

                let reset_btn = egui::Button::new(RichText::new(format!("{} Reset", RESET)).size(13.5).color(palette.text_secondary))
                    .fill(palette.bg_card)
                    .stroke(egui::Stroke::new(1.0_f32, palette.bg_card_border))
                    .min_size(Vec2::new(95.0, 42.0))
                    .corner_radius(CornerRadius::same(10));

                if ui.add(reset_btn).clicked() {
                    self.pomodoro.reset();
                    self.save_state();
                }

                ui.add_space(8.0);

                let skip_btn = egui::Button::new(RichText::new(format!("{} Skip", SKIP)).size(13.5).color(palette.text_secondary))
                    .fill(palette.bg_card)
                    .stroke(egui::Stroke::new(1.0_f32, palette.bg_card_border))
                    .min_size(Vec2::new(90.0, 42.0))
                    .corner_radius(CornerRadius::same(10));

                if ui.add(skip_btn).clicked() {
                    let _ = self.pomodoro.skip();
                    self.save_state();
                }
            });

            ui.add_space(20.0);

            // Presets row
            ui.horizontal(|ui| {
                ui.label(RichText::new("Session Rhythms:").size(13.0).color(palette.text_secondary));
                
                let presets = [
                    (25, 5, "25/5 Standard"),
                    (50, 10, "50/10 Undergrad Lecture Block"),
                    (90, 20, "90/20 Deep Analytical Sprint"),
                ];

                for (work, brk, label) in presets {
                    let is_active = self.pomodoro.config.work_mins == work && self.pomodoro.config.short_break_mins == brk;
                    let btn = egui::Button::new(RichText::new(label).size(12.0).color(if is_active { Color32::WHITE } else { palette.text_secondary }))
                        .fill(if is_active { palette.accent_dark } else { palette.bg_card })
                        .stroke(egui::Stroke::new(1.0_f32, if is_active { palette.accent_primary } else { palette.bg_card_border }))
                        .corner_radius(CornerRadius::same(8));

                    if ui.add(btn).clicked() {
                        self.pomodoro.config.work_mins = work;
                        self.pomodoro.config.short_break_mins = brk;
                        self.pomodoro.reset();
                        self.save_state();
                    }
                }
            });

            ui.add_space(20.0);

            // Session Stats Cards Row
            ui.horizontal(|ui| {
                let session_str = format!("{}/{}", self.pomodoro.current_cycle_count, self.pomodoro.config.sessions_before_long_break);
                stat_card(ui, palette, TARGET, "Long Break In", &session_str, Some("sessions until 15m rest"));
                
                let today_log = self.frequency.today_log();
                stat_card(ui, palette, CLOCK, "Completed Today", &format!("{} Pomodoros", today_log.pomodoros_completed), Some(&format!("{}m focused today", today_log.minutes_studied)));
                
                let total_sessions = self.pomodoro.total_completed_sessions;
                stat_card(ui, palette, TROPHY, "All-Time Sessions", &format!("{}", total_sessions), Some("total completed"));
            });
        });
    }

    fn render_cycle_tab(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ScrollArea::vertical().show(ui, |ui| {
            // Header Card: Cycle Overview & Preset Builder Toggle
            glowing_card_frame(palette, palette.accent_primary).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{} Study Cycle #{}", TAB_CYCLE, self.cycle.cycle_number))
                                .size(20.0)
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.label(
                            RichText::new("Undergrad Rotation Framework • Rotates subjects without rigid calendar schedules")
                                .size(12.5)
                                .color(palette.text_secondary),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let complete_btn = egui::Button::new(RichText::new(format!("{} Complete Cycle", CHECK)).strong().color(Color32::WHITE))
                            .fill(palette.accent_primary)
                            .corner_radius(CornerRadius::same(8));

                        if ui.add(complete_btn).clicked() {
                            self.cycle.complete_current_cycle();
                            self.save_state();
                        }

                        ui.add_space(8.0);

                        // Presets Drawer toggle button
                        let preset_label = if self.show_presets_drawer { "Hide Presets" } else { "Presets & Templates" };
                        let preset_btn = egui::Button::new(RichText::new(format!("{} {}", PRESET_LIST, preset_label)).color(palette.accent_secondary))
                            .fill(palette.bg_deep)
                            .stroke(egui::Stroke::new(1.0_f32, palette.accent_primary))
                            .corner_radius(CornerRadius::same(8));

                        if ui.add(preset_btn).clicked() {
                            self.show_presets_drawer = !self.show_presets_drawer;
                        }
                    });
                });

                ui.add_space(10.0);

                // Overall progress bar
                let cycle_progress = self.cycle.cycle_progress_fraction();
                let completed_mins = self.cycle.total_completed_minutes();
                let target_mins = self.cycle.total_target_minutes();

                ui.add(
                    CustomProgressBar::new(cycle_progress, palette.accent_primary, palette.accent_glow, palette.track_bg)
                        .label("Overall Cycle Completion")
                        .detail(&format!("{} / {} completed ({:.0}%)", StudySubject::format_time(completed_mins), StudySubject::format_time(target_mins), cycle_progress * 100.0))
                        .height(16.0),
                );
            });

            // Preset Builder Drawer (Builtin + Custom presets)
            if self.show_presets_drawer {
                ui.add_space(10.0);
                card_frame(palette).show(ui, |ui| {
                    ui.label(RichText::new(format!("{} Undergrad Cycle Presets & Template Builder", GRADUATION_CAP)).size(15.0).strong().color(palette.accent_glow));
                    ui.label(RichText::new("Choose a pre-configured university semester template, or save your current rotation:").size(12.0).color(palette.text_secondary));
                    ui.add_space(8.0);

                    // Built-in presets
                    ui.label(RichText::new("Curated University Templates:").size(13.0).strong().color(Color32::WHITE));
                    ui.add_space(4.0);

                    let builtins = CyclePreset::builtins();
                    for preset in &builtins {
                        ui.horizontal(|ui| {
                            let apply_btn = egui::Button::new(RichText::new(format!("Apply: {}", preset.name)).strong().color(Color32::WHITE))
                                .fill(palette.bg_deep)
                                .stroke(egui::Stroke::new(1.0_f32, palette.accent_secondary))
                                .corner_radius(CornerRadius::same(6));

                            if ui.add(apply_btn).clicked() {
                                self.cycle.apply_preset(preset);
                                self.exams.sync_all_exam_boosts(&mut self.cycle);
                                if let Some(sub) = self.cycle.active_subject() {
                                    self.pomodoro.active_subject_name = sub.name.clone();
                                }
                                self.save_state();
                                self.set_toast(&format!("Applied preset '{}'", preset.name));
                            }

                            ui.label(RichText::new(&preset.description).size(11.5).color(palette.text_muted));
                        });
                        ui.add_space(4.0);
                    }

                    // Custom presets list if any
                    if !self.cycle.custom_presets.is_empty() {
                        ui.add_space(6.0);
                        ui.label(RichText::new("Your Custom Saved Presets:").size(13.0).strong().color(Color32::WHITE));
                        let mut delete_custom = None;
                        let mut apply_custom = None;
                        for (c_idx, custom) in self.cycle.custom_presets.iter().enumerate() {
                            ui.horizontal(|ui| {
                                if ui.button(RichText::new(format!("Apply: {}", custom.name)).strong().color(palette.accent_glow)).clicked() {
                                    apply_custom = Some(c_idx);
                                }
                                ui.label(RichText::new(&custom.description).size(11.5).color(palette.text_secondary));
                                if ui.button(RichText::new(TRASH).size(11.0).color(palette.text_muted)).clicked() {
                                    delete_custom = Some(c_idx);
                                }
                            });
                        }
                        if let Some(c_idx) = apply_custom {
                            let preset = self.cycle.custom_presets[c_idx].clone();
                            self.cycle.apply_preset(&preset);
                            self.exams.sync_all_exam_boosts(&mut self.cycle);
                            if let Some(sub) = self.cycle.active_subject() {
                                self.pomodoro.active_subject_name = sub.name.clone();
                            }
                            self.save_state();
                            self.set_toast(&format!("Applied preset '{}'", preset.name));
                        }
                        if let Some(c_idx) = delete_custom {
                            self.cycle.custom_presets.remove(c_idx);
                            self.save_state();
                        }
                    }

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(6.0);

                    // Save current cycle as custom preset
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Save Current as Preset:").size(12.5).color(palette.text_primary));
                        ui.add(egui::TextEdit::singleline(&mut self.custom_preset_name).hint_text("Preset Name (e.g. 2nd Semester Finals)"));
                        ui.add(egui::TextEdit::singleline(&mut self.custom_preset_desc).hint_text("Short description..."));

                        if ui.button(RichText::new("Save Preset").strong().color(Color32::WHITE)).clicked() && !self.custom_preset_name.trim().is_empty() {
                            self.cycle.save_current_as_preset(&self.custom_preset_name, &self.custom_preset_desc);
                            self.custom_preset_name.clear();
                            self.custom_preset_desc.clear();
                            self.save_state();
                            self.set_toast("Custom cycle preset saved successfully!");
                        }
                    });
                });
            }

            // Test Mode Boost Summary Banner
            let today = Local::now().date_naive();
            let total_boost: u32 = self.cycle.subjects.iter().map(|s| s.boost_minutes).sum();
            if total_boost > 0 {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    status_pill(
                        ui,
                        &format!("{} Test Mode Active: +{}m automatically boosted across upcoming exams", BOLT, total_boost),
                        palette.accent_dark,
                        palette.accent_glow,
                    );
                    if ui.button(RichText::new(format!("{} Re-balance", BOLT)).size(11.5)).clicked() {
                        self.exams.sync_all_exam_boosts(&mut self.cycle);
                        self.save_state();
                        self.set_toast("Cycle automatically re-balanced for test dates!");
                    }
                });
            }

            ui.add_space(14.0);

            // Subject Cards List with Upgraded Time Setters
            ui.label(RichText::new("Study Queue & Time Setters").size(16.0).strong().color(Color32::WHITE));
            ui.add_space(4.0);

            let mut to_delete = None;
            let mut to_move_up = None;
            let mut to_move_down = None;
            let mut set_active = None;
            let mut add_minutes: Option<(usize, u32)> = None;
            let mut adjust_base: Option<(usize, i32)> = None;
            let mut set_preset_time: Option<(usize, u32)> = None;

            for (idx, subject) in self.cycle.subjects.iter().enumerate() {
                let is_active = idx == self.cycle.active_index;
                let frame = if is_active {
                    glowing_card_frame(palette, palette.accent_glow)
                } else {
                    card_frame(palette)
                };

                frame.show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Top line: Name, Badges, and Reorder/Delete buttons
                        ui.horizontal(|ui| {
                            let fill_c = parse_hex_color(&subject.color_hex).unwrap_or(palette.accent_secondary);
                            ui.label(RichText::new("●").size(16.0).color(fill_c));
                            ui.label(RichText::new(&subject.name).size(15.0).strong().color(Color32::WHITE));

                            if is_active {
                                status_pill(ui, &format!("{} Active Focus", TARGET), palette.accent_dark, Color32::WHITE);
                            }
                            if subject.is_completed() {
                                status_pill(ui, &format!("{} Done", CHECK), palette.success.linear_multiply(0.2), palette.success);
                            }

                            // Show exam badge if subject has an upcoming test
                            if let Some(exam) = self.exams.exams.iter().find(|e| e.subject_name.eq_ignore_ascii_case(&subject.name)) {
                                let days = exam.days_remaining(today);
                                let badge_text = if days <= 3 {
                                    format!("{} {} ({}d left!)", CALENDAR_ALERT, exam.test_title, days)
                                } else {
                                    format!("{} in {}d", exam.test_title, days)
                                };
                                status_pill(ui, &badge_text, Color32::from_rgb(88, 28, 135), palette.accent_glow);
                            }

                            // Right-aligned actions
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(RichText::new(TRASH).size(13.0).color(palette.text_muted)).clicked() {
                                    to_delete = Some(idx);
                                }
                                if ui.button(RichText::new(CHEVRON_DOWN).size(13.0)).clicked() {
                                    to_move_down = Some(idx);
                                }
                                if ui.button(RichText::new(CHEVRON_UP).size(13.0)).clicked() {
                                    to_move_up = Some(idx);
                                }
                                if !is_active && ui.button("Set Active").clicked() {
                                    set_active = Some(idx);
                                }
                            });
                        });

                        ui.add_space(4.0);

                        // Progress Bar with detailed target breakdown
                        let detail_text = if subject.boost_minutes > 0 {
                            format!("{}/{} (Base {}m + Boost {}m)", 
                                StudySubject::format_time(subject.completed_minutes), 
                                StudySubject::format_time(subject.target_minutes()),
                                subject.base_minutes,
                                subject.boost_minutes
                            )
                        } else {
                            format!("{}/{}", StudySubject::format_time(subject.completed_minutes), StudySubject::format_time(subject.target_minutes()))
                        };

                        let fill_c = parse_hex_color(&subject.color_hex).unwrap_or(palette.accent_secondary);
                        ui.add(
                            CustomProgressBar::new(subject.progress_fraction(), fill_c, palette.accent_glow, palette.track_bg)
                                .detail(&detail_text)
                                .height(11.0),
                        );

                        ui.add_space(6.0);

                        // UPGRADED TIME SETTER FOR UNDERGRAD STUDENTS:
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Target Duration:").size(12.0).color(palette.text_secondary));
                            ui.label(RichText::new(StudySubject::format_time(subject.base_minutes)).strong().size(13.0).color(palette.accent_glow));

                            // Quick adjustment buttons
                            if ui.button("-15m").clicked() {
                                adjust_base = Some((idx, -15));
                            }
                            if ui.button("+15m").clicked() {
                                adjust_base = Some((idx, 15));
                            }

                            ui.add_space(4.0);
                            ui.label(RichText::new("Quick Chips:").size(11.5).color(palette.text_muted));

                            // Preset chips (25m, 45m, 60m, 90m)
                            for &chip_mins in &[25, 45, 60, 90] {
                                let is_current = subject.base_minutes == chip_mins;
                                let chip_btn = egui::Button::new(RichText::new(format!("{}m", chip_mins)).size(11.0).color(if is_current { Color32::WHITE } else { palette.text_secondary }))
                                    .fill(if is_current { palette.accent_dark } else { palette.bg_deep })
                                    .stroke(egui::Stroke::new(1.0_f32, if is_current { palette.accent_primary } else { palette.bg_card_border }))
                                    .corner_radius(CornerRadius::same(5));

                                if ui.add(chip_btn).clicked() {
                                    set_preset_time = Some((idx, chip_mins));
                                }
                            }

                            // Quick manual log time button
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(RichText::new("+15m Studied").size(11.5)).clicked() {
                                    add_minutes = Some((idx, 15));
                                }
                            });
                        });
                    });
                });

                ui.add_space(6.0);
            }

            // Apply mutations
            if let Some((idx, delta)) = adjust_base {
                if let Some(sub) = self.cycle.subjects.get_mut(idx) {
                    sub.adjust_base_minutes(delta);
                }
                self.save_state();
            }
            if let Some((idx, target_m)) = set_preset_time {
                if let Some(sub) = self.cycle.subjects.get_mut(idx) {
                    sub.set_base_minutes(target_m);
                }
                self.save_state();
            }
            if let Some(idx) = set_active {
                self.cycle.active_index = idx;
                if let Some(sub) = self.cycle.active_subject() {
                    self.pomodoro.active_subject_name = sub.name.clone();
                }
                self.save_state();
            }
            if let Some((idx, mins)) = add_minutes {
                if let Some(sub) = self.cycle.subjects.get_mut(idx) {
                    sub.completed_minutes += mins;
                    self.frequency.record_study_session(mins, false);
                }
                self.save_state();
            }
            if let Some(idx) = to_move_up {
                self.cycle.move_up(idx);
                self.save_state();
            }
            if let Some(idx) = to_move_down {
                self.cycle.move_down(idx);
                self.save_state();
            }
            if let Some(idx) = to_delete {
                self.cycle.remove_subject(idx);
                self.save_state();
            }

            ui.add_space(14.0);

            // Add New Subject Card with Upgraded Time Setter
            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new("+ Add New Course / Subject to Cycle").size(14.0).strong().color(palette.accent_secondary));
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.label("Course Name:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_subject_name).hint_text("e.g. Physics II"));

                    ui.label("Target:");
                    ui.label(RichText::new(StudySubject::format_time(self.new_subject_mins)).strong().color(palette.accent_glow));

                    for &chip in &[30, 45, 60, 90] {
                        if ui.selectable_label(self.new_subject_mins == chip, format!("{}m", chip)).clicked() {
                            self.new_subject_mins = chip;
                        }
                    }

                    let add_btn = egui::Button::new(RichText::new("Add Subject").strong().color(Color32::WHITE))
                        .fill(palette.accent_primary)
                        .corner_radius(CornerRadius::same(8));

                    if ui.add(add_btn).clicked() && !self.new_subject_name.trim().is_empty() {
                        self.cycle.add_subject(&self.new_subject_name, self.new_subject_mins, &self.new_subject_color);
                        self.exams.sync_all_exam_boosts(&mut self.cycle);
                        self.new_subject_name.clear();
                        self.save_state();
                        self.set_toast("New course added to study cycle!");
                    }
                });
            });
        });
    }

    fn render_test_mode_tab(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ScrollArea::vertical().show(ui, |ui| {
            // Header Card
            glowing_card_frame(palette, palette.accent_glow).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{} Test Mode • Exam Proximity & Auto-Balancer", TEST_PENCIL))
                                .size(20.0)
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.label(
                            RichText::new("Input upcoming university exams. FernDoro automatically boosts required study points in your cycle based on exam proximity and difficulty.")
                                .size(12.5)
                                .color(palette.text_secondary),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let rebalance_btn = egui::Button::new(RichText::new(format!("{} Sync & Auto-Balance Cycle", BOLT)).strong().color(Color32::WHITE))
                            .fill(palette.accent_primary)
                            .corner_radius(CornerRadius::same(8));

                        if ui.add(rebalance_btn).clicked() {
                            self.exams.sync_all_exam_boosts(&mut self.cycle);
                            self.save_state();
                            self.set_toast("Cycle automatically balanced for upcoming tests!");
                        }
                    });
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.exams.auto_boost_enabled, "Auto-add calculated study points directly to the active Study Cycle").changed() {
                        self.exams.sync_all_exam_boosts(&mut self.cycle);
                        self.save_state();
                    }
                });
            });

            ui.add_space(14.0);

            // Active Exams List
            ui.label(RichText::new("Scheduled Tests & University Exams").size(16.0).strong().color(Color32::WHITE));
            ui.add_space(4.0);

            let today = Local::now().date_naive();
            let mut to_remove_id = None;

            if self.exams.exams.is_empty() {
                card_frame(palette).show(ui, |ui| {
                    ui.label(RichText::new("No upcoming exams scheduled. Add an exam below to automatically boost your study cycle.").color(palette.text_muted));
                });
            } else {
                for exam in &self.exams.exams {
                    let days = exam.days_remaining(today);
                    let urgency = exam.urgency(today);
                    let (urgency_label, badge_color) = urgency.display_badge();
                    let auto_points = exam.calculate_auto_points(today);

                    card_frame(palette).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(&exam.subject_name).size(16.0).strong().color(palette.accent_secondary));
                                    ui.label(RichText::new("•").color(palette.text_muted));
                                    ui.label(RichText::new(&exam.test_title).size(15.0).strong().color(Color32::WHITE));
                                    status_pill(ui, urgency_label, badge_color, Color32::WHITE);
                                });

                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    let date_str = exam.exam_date.format("%A, %b %d, %Y").to_string();
                                    let countdown = if days < 0 {
                                        "Past Due".to_string()
                                    } else if days == 0 {
                                        "TODAY!".to_string()
                                    } else if days == 1 {
                                        "Tomorrow (1 day remaining)".to_string()
                                    } else {
                                        format!("In {} days", days)
                                    };

                                    ui.label(RichText::new(format!("{} Date: {} ({})", CALENDAR, date_str, countdown)).size(12.5).color(palette.text_secondary));
                                    ui.label(RichText::new("•").color(palette.text_muted));
                                    ui.label(RichText::new(format!("Difficulty: {}/5 • Importance: {}/5", exam.difficulty, exam.weight)).size(12.0).color(palette.text_muted));
                                });
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(RichText::new(TRASH).size(13.0).color(palette.text_muted)).clicked() {
                                    to_remove_id = Some(exam.id.clone());
                                }

                                ui.add_space(8.0);
                                status_pill(
                                    ui,
                                    &format!("{} +{}m cycle points", BOLT, auto_points),
                                    palette.accent_dark,
                                    palette.accent_glow,
                                );
                            });
                        });
                    });

                    ui.add_space(6.0);
                }
            }

            if let Some(id) = to_remove_id {
                self.exams.remove_exam(&id, &mut self.cycle);
                self.save_state();
                self.set_toast("Exam removed and cycle points updated.");
            }

            ui.add_space(14.0);

            // Add New Exam Form
            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new(format!("{} Schedule an Exam / Test", GRADUATION_CAP)).size(15.0).strong().color(palette.accent_secondary));
                ui.label(RichText::new("FernDoro calculates required preparation time and automatically adds study points to your Study Cycle:").size(12.0).color(palette.text_secondary));
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Course / Subject:");
                    // Dropdown from current cycle subjects or freeform text
                    egui::ComboBox::from_id_salt("exam_subject_dropdown")
                        .selected_text(&self.new_exam_subject)
                        .show_ui(ui, |ui| {
                            for sub in &self.cycle.subjects {
                                if ui.selectable_label(self.new_exam_subject == sub.name, &sub.name).clicked() {
                                    self.new_exam_subject = sub.name.clone();
                                }
                            }
                        });
                    ui.add(egui::TextEdit::singleline(&mut self.new_exam_subject).hint_text("Or custom course name"));
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Exam Title:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_exam_title).hint_text("e.g. P1 Differential & Integral Calculus"));
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Days until exam:");
                    ui.add(egui::Slider::new(&mut self.new_exam_days, 1..=60).suffix(" days"));

                    // Quick chips for days
                    for &chip_d in &[3, 7, 14, 21] {
                        if ui.selectable_label(self.new_exam_days == chip_d, format!("{}d", chip_d)).clicked() {
                            self.new_exam_days = chip_d;
                        }
                    }
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Perceived Difficulty:");
                    ui.add(egui::Slider::new(&mut self.new_exam_diff, 1..=5).text("1=Easy, 5=Extremely Hard"));

                    ui.add_space(8.0);
                    ui.label("Importance / Weight:");
                    ui.add(egui::Slider::new(&mut self.new_exam_weight, 1..=5).text("1=Quiz, 5=Final"));
                });

                ui.add_space(8.0);

                // Preview auto-calculated points before adding
                let exam_preview = ExamTest::new(
                    &self.new_exam_subject,
                    &self.new_exam_title,
                    today + Duration::days(self.new_exam_days as i64),
                    self.new_exam_weight,
                    self.new_exam_diff,
                );
                let preview_points = exam_preview.calculate_auto_points(today);

                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("Calculated boost: +{} minutes will be auto-added to {} in your Study Cycle", preview_points, self.new_exam_subject)).color(palette.accent_glow));

                    let add_btn = egui::Button::new(RichText::new(format!("{} Add Exam & Boost Cycle", BOLT)).strong().color(Color32::WHITE))
                        .fill(palette.accent_primary)
                        .corner_radius(CornerRadius::same(8));

                    if ui.add(add_btn).clicked() && !self.new_exam_subject.trim().is_empty() {
                        let exam = ExamTest::new(
                            &self.new_exam_subject,
                            &self.new_exam_title,
                            today + Duration::days(self.new_exam_days as i64),
                            self.new_exam_weight,
                            self.new_exam_diff,
                        );
                        self.exams.add_exam(exam, &mut self.cycle);
                        self.save_state();
                        self.set_toast(&format!("Exam scheduled! +{}m added to {} in Study Cycle", preview_points, self.new_exam_subject));
                    }
                });
            });
        });
    }

    fn render_frequency_tab(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ScrollArea::vertical().show(ui, |ui| {
            // Stats Row
            ui.horizontal(|ui| {
                let streak = self.frequency.current_streak_days();
                stat_card(ui, palette, FLAME, "Current Streak", &format!("{} Days", streak), Some("consecutive study days"));

                let best = self.frequency.best_streak_days();
                stat_card(ui, palette, TROPHY, "Best Streak", &format!("{} Days", best), Some("all-time record"));

                let total_mins = self.frequency.total_minutes_all_time();
                let hours = total_mins / 60;
                let mins = total_mins % 60;
                stat_card(ui, palette, CLOCK, "Total Study Time", &format!("{}h {}m", hours, mins), Some("across all sessions"));

                let active_days = self.frequency.total_active_days();
                stat_card(ui, palette, CALENDAR, "Active Days", &format!("{} Days", active_days), Some("days with logged work"));
            });

            ui.add_space(16.0);

            // Heatmap Card
            card_frame(palette).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Consistency Matrix (Past 20 Weeks)").size(15.0).strong().color(Color32::WHITE));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("■ High  ■ Med  ■ Low  □ None").size(11.0).color(palette.text_muted));
                    });
                });
                ui.add_space(8.0);

                let heatmap_data = self.frequency.get_heatmap_data();
                let heatmap = FrequencyHeatmap::new(&heatmap_data, palette.accent_primary, palette.track_bg).weeks(22);
                ui.add(heatmap);
            });

            ui.add_space(16.0);

            // Daily Goal Progression Card
            card_frame(palette).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Today's Daily Target").size(15.0).strong().color(Color32::WHITE));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("+15m").clicked() {
                            self.frequency.record_study_session(15, false);
                            self.save_state();
                        }
                        if ui.button("+30m").clicked() {
                            self.frequency.record_study_session(30, false);
                            self.save_state();
                        }
                        if ui.button("+60m").clicked() {
                            self.frequency.record_study_session(60, false);
                            self.save_state();
                        }
                        ui.label(RichText::new("Quick Log:").color(palette.text_muted));
                    });
                });

                ui.add_space(8.0);

                let today_log = self.frequency.today_log();
                let fraction = self.frequency.today_goal_fraction();
                ui.add(
                    CustomProgressBar::new(fraction, palette.success, palette.accent_glow, palette.track_bg)
                        .label("Target Hours Progress")
                        .detail(&format!("{}m / {}m ({:.0}%)", today_log.minutes_studied, self.frequency.daily_goal_minutes, fraction * 100.0))
                        .height(14.0),
                );
            });

            ui.add_space(16.0);

            // Daily Habits Checklist
            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new("Daily Undergrad Habits Checklist").size(15.0).strong().color(Color32::WHITE));
                ui.label(RichText::new("Tick off essential study and wellness routines for today:").size(12.5).color(palette.text_secondary));
                ui.add_space(8.0);

                let today_log = self.frequency.today_log();
                let mut toggled = None;

                for habit in &self.frequency.habits {
                    let is_checked = today_log.habits_checked.contains(&habit.name);
                    ui.horizontal(|ui| {
                        let text = format!("{}  {}", habit.icon, habit.name);
                        let mut checked = is_checked;
                        if ui.checkbox(&mut checked, RichText::new(text).size(13.5).color(if is_checked { palette.accent_secondary } else { palette.text_primary })).changed() {
                            toggled = Some(habit.name.clone());
                        }
                    });
                    ui.add_space(3.0);
                }

                if let Some(habit_name) = toggled {
                    self.frequency.toggle_habit_today(&habit_name);
                    self.save_state();
                }
            });
        });
    }

    fn render_settings_tab(&mut self, ui: &mut Ui, ctx: &egui::Context, palette: &ThemePalette) {
        ScrollArea::vertical().show(ui, |ui| {
            // Theme Section
            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new(format!("{} Visual Themes (Purple Dark Architecture)", PALETTE)).size(16.0).strong().color(Color32::WHITE));
                ui.label(RichText::new("Select from high-contrast dark purple themes. FernDoro uses an extensible palette structure.").size(12.5).color(palette.text_secondary));
                ui.add_space(10.0);

                for mode in ThemeMode::all() {
                    let is_selected = self.theme == *mode;
                    let text = mode.name();
                    
                    let btn = egui::Button::new(RichText::new(text).size(13.0).color(if is_selected { Color32::WHITE } else { palette.text_secondary }))
                        .fill(if is_selected { palette.accent_primary } else { palette.bg_deep })
                        .stroke(egui::Stroke::new(1.0_f32, if is_selected { palette.accent_glow } else { palette.bg_card_border }))
                        .corner_radius(CornerRadius::same(8));

                    if ui.add(btn).clicked() {
                        self.theme = *mode;
                        self.theme.apply(ctx);
                        self.save_state();
                    }
                    ui.add_space(4.0);
                }
            });

            ui.add_space(16.0);

            // Pomodoro Timing Preferences
            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new(format!("{} Pomodoro Preferences", TAB_POMODORO)).size(16.0).strong().color(Color32::WHITE));
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Focus Work Duration:");
                    if ui.add(egui::Slider::new(&mut self.pomodoro.config.work_mins, 5..=120).suffix(" min")).changed() {
                        self.pomodoro.reset_phase_time();
                        self.save_state();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Short Break Duration:");
                    if ui.add(egui::Slider::new(&mut self.pomodoro.config.short_break_mins, 1..=30).suffix(" min")).changed() {
                        self.pomodoro.reset_phase_time();
                        self.save_state();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Long Break Duration:");
                    if ui.add(egui::Slider::new(&mut self.pomodoro.config.long_break_mins, 5..=60).suffix(" min")).changed() {
                        self.pomodoro.reset_phase_time();
                        self.save_state();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Sessions before Long Break:");
                    if ui.add(egui::Slider::new(&mut self.pomodoro.config.sessions_before_long_break, 2..=8)).changed() {
                        self.save_state();
                    }
                });

                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.pomodoro.config.auto_advance, "Auto-advance next session without manual click").changed() {
                        self.save_state();
                    }
                });
            });

            ui.add_space(16.0);

            // Frequency Goals Preferences
            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new(format!("{} Daily Goal Preferences", TAB_FREQUENCY)).size(16.0).strong().color(Color32::WHITE));
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Daily Target Study Time:");
                    if ui.add(egui::Slider::new(&mut self.frequency.daily_goal_minutes, 30..=600).suffix(" min")).changed() {
                        self.save_state();
                    }
                });
            });

            ui.add_space(16.0);

            // Data & History Management
            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new("Data & History Management").size(16.0).strong().color(Color32::WHITE));
                ui.label(RichText::new("Clear recorded study sessions, streaks, and heatmap logs to start with a fresh slate.").size(12.5).color(palette.text_secondary));
                ui.add_space(8.0);

                let clear_btn = egui::Button::new(RichText::new(format!("{} Clear Study History & Streaks", TRASH)).strong().color(Color32::WHITE))
                    .fill(Color32::from_rgb(185, 28, 28))
                    .corner_radius(CornerRadius::same(8));

                if ui.add(clear_btn).clicked() {
                    self.frequency.clear_history();
                    self.pomodoro.total_completed_sessions = 0;
                    self.pomodoro.current_cycle_count = 0;
                    self.save_state();
                    self.set_toast("Study history and streaks have been cleared!");
                }
            });

            ui.add_space(16.0);

            // About FernDoro
            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new(format!("{} About FernDoro", LOGO_LEAF)).size(15.0).strong().color(palette.accent_glow));
                ui.label(RichText::new("Version 0.2.0 • Built with Rust and egui • Powered by Inter & JetBrains Mono").size(12.5).color(palette.text_secondary));
                ui.label(RichText::new("Designed for focused mastery, systematic study rotation, and daily habit consistency.").size(12.0).color(palette.text_muted));
            });
        });
    }
}

fn parse_hex_color(hex: &str) -> Option<Color32> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color32::from_rgb(r, g, b))
    } else {
        None
    }
}
