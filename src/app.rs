use std::time::Instant;
use chrono::{Duration, Local};
use egui::{epaint::CornerRadius, Color32, Margin, RichText, ScrollArea, Ui, Vec2};

use crate::cycle::{CyclePreset, StudyCycleState, StudySubject};
use crate::exam::{ExamState, ExamTest};
use crate::frequency::FrequencyTrackerState;
use crate::icons::*;
use crate::pomodoro::{PomodoroPhase, PomodoroState, TimerStatus};
use crate::rogue::{AnteTier, FateModifier, RogueDungeonState, RoguePhase};
use crate::storage::{AppData, Storage};
use crate::theme::{ThemeMode, ThemePalette};
use crate::widgets::{
    card_frame, elevated_card_frame, glowing_card_frame, stat_card, status_pill,
    CustomProgressBar, FrequencyHeatmap, TimerDial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Pomodoro,
    StudyCycle,
    TestMode,
    RogueDungeon,
    FrequencyTracker,
    Settings,
}

pub struct FernDoroApp {
    pub theme: ThemeMode,
    pub pomodoro: PomodoroState,
    pub cycle: StudyCycleState,
    pub frequency: FrequencyTrackerState,
    pub exams: ExamState,
    pub rogue: RogueDungeonState,
    pub active_tab: ActiveTab,
    pub last_tick: Instant,
    
    // UI state for adding new subjects
    pub new_subject_name: String,
    pub new_subject_mins: u32,
    pub new_subject_color: String,

    // UI state for preset builder drawer
    pub show_presets_drawer: bool,
    pub custom_preset_name: String,
    pub custom_preset_desc: String,

    // UI state for adding exams
    pub new_exam_subject: String,
    pub new_exam_title: String,
    pub new_exam_days: i32,
    pub new_exam_weight: u32,
    pub new_exam_diff: u32,

    // UI state for rogue shop drawer
    pub show_relic_shop: bool,

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
            rogue: data.rogue,
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
            show_relic_shop: false,
            toast_message: None,
        };

        // Sync exam boosts to cycle targets on startup
        app.exams.sync_all_exam_boosts(&mut app.cycle);

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
            rogue: self.rogue.clone(),
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

        let window_focused = ctx.input(|i| i.focused);

        // Tick standard pomodoro timer
        if let Some(event) = self.pomodoro.tick(dt) {
            if event.phase == PomodoroPhase::Work {
                self.cycle.add_study_time(&event.subject_name, event.duration_minutes);
                self.frequency.record_study_session(event.duration_minutes, true);
                
                // Gamification reward for completing focus block: +30 XP & +10 Gold!
                let earned_gold = 10;
                let earned_xp = 30;
                self.rogue.vault.vault_gold += earned_gold;
                let level_up = self.rogue.vault.add_xp(earned_xp);

                if let Some(new_lvl) = level_up {
                    self.set_toast(&format!("🎉 LEVEL UP! You reached Level {} ({})!", new_lvl, self.rogue.vault.equipped_title));
                } else {
                    self.set_toast(&format!("Session finished! +{}m logged • +{} XP • +{}g Gold", event.duration_minutes, earned_xp, earned_gold));
                }
            }
            self.save_state();
        }

        // Tick rogue dungeon game engine
        if let Some(event) = self.rogue.tick(dt, window_focused) {
            self.cycle.add_study_time(&event.task_name, event.duration_minutes);
            self.frequency.record_study_session(event.duration_minutes, true);
            
            if let Some(new_lvl) = event.leveled_up {
                self.set_toast(&format!("⚔️ Floor {} Cleared! 🎉 LEVEL UP: Level {}!", event.floor, new_lvl));
            } else {
                self.set_toast(&format!("Floor {} cleared! +{} Gold in Pot • +{} XP", event.floor, event.gold_earned, event.xp_earned));
            }
            self.save_state();
        }

        let is_running = self.pomodoro.status == TimerStatus::Running 
            || self.rogue.phase == RoguePhase::FocusActive 
            || self.rogue.phase == RoguePhase::WarmupLock
            || self.rogue.phase == RoguePhase::BreakDealer
            || self.rogue.phase == RoguePhase::BreakRest;

        if is_running {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }

        let palette = self.theme.colors();

        // Top Navigation Bar (Material You 3 with Gamified HUD)
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::new().fill(palette.bg_deep).inner_margin(Margin::symmetric(20, 14)))
            .show(ctx, |ui| {
                self.render_top_bar(ui, &palette);
            });

        // Main Content Area
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(palette.bg_deep).inner_margin(Margin::symmetric(24, 16)))
            .show(ctx, |ui| {
                match self.active_tab {
                    ActiveTab::Pomodoro => self.render_pomodoro_tab(ui, &palette),
                    ActiveTab::StudyCycle => self.render_cycle_tab(ui, &palette),
                    ActiveTab::TestMode => self.render_test_mode_tab(ui, &palette),
                    ActiveTab::RogueDungeon => self.render_rogue_tab(ui, &palette),
                    ActiveTab::FrequencyTracker => self.render_frequency_tab(ui, &palette),
                    ActiveTab::Settings => self.render_settings_tab(ui, ctx, &palette),
                }

                if let Some((msg, created_at)) = &self.toast_message {
                    if created_at.elapsed().as_secs() < 4 {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            status_pill(ui, &format!("{} {}", SPARKLE, msg), palette.accent_container, palette.on_accent_container);
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
            // Material 3 Brand Heading
            ui.label(
                RichText::new(format!("{} FernDoro", LOGO_LEAF))
                    .size(19.0)
                    .strong()
                    .color(palette.accent_primary),
            );

            ui.add_space(24.0);

            // Material 3 Navigation Pill Tabs
            let tabs = [
                (ActiveTab::Pomodoro, TAB_POMODORO, "Pomodoro"),
                (ActiveTab::StudyCycle, TAB_CYCLE, "Cycle"),
                (ActiveTab::TestMode, TEST_PENCIL, "Tests"),
                (ActiveTab::RogueDungeon, SWORD, "Dungeon"),
                (ActiveTab::FrequencyTracker, TAB_FREQUENCY, "Stats"),
                (ActiveTab::Settings, TAB_SETTINGS, "Settings"),
            ];

            for (tab, icon, title) in tabs {
                let is_active = self.active_tab == tab;
                let bg_color = if is_active { palette.accent_container } else { Color32::TRANSPARENT };
                let text_color = if is_active { palette.on_accent_container } else { palette.text_secondary };
                let stroke = if is_active {
                    egui::Stroke::new(1.0_f32, palette.accent_primary.linear_multiply(0.4))
                } else {
                    egui::Stroke::NONE
                };

                let label_text = format!("{} {}", icon, title);
                let btn = egui::Button::new(RichText::new(label_text).strong().size(13.0).color(text_color))
                    .fill(bg_color)
                    .stroke(stroke)
                    .min_size(Vec2::new(0.0, 32.0))
                    .corner_radius(CornerRadius::same(18));

                if ui.add(btn).clicked() {
                    self.active_tab = tab;
                }
                ui.add_space(8.0);
            }

            // GAMIFIED TOP HUD (Level, Gold, Streaks)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);

                // Gold Display: Prominent, clear, elegant
                let pot_info = if self.rogue.current_run.unbanked_gold > 0 {
                    format!(" (+{}g pot)", self.rogue.current_run.unbanked_gold)
                } else {
                    "".to_string()
                };
                status_pill(
                    ui,
                    &format!("{} {}g{}", COIN, self.rogue.vault.vault_gold, pot_info),
                    palette.accent_tertiary.linear_multiply(0.2),
                    palette.accent_tertiary,
                );

                ui.add_space(12.0);

                // Level & XP Pill
                let lvl = self.rogue.vault.level;
                let xp = self.rogue.vault.current_xp;
                let thresh = self.rogue.vault.xp_threshold();
                status_pill(
                    ui,
                    &format!("⭐ Lv.{} ({}/{} XP)", lvl, xp, thresh),
                    palette.bg_card,
                    palette.accent_primary,
                );

                ui.add_space(12.0);

                // Streak Pill
                let streak = self.frequency.current_streak_days();
                status_pill(
                    ui,
                    &format!("{} {}d", FLAME, streak),
                    palette.bg_card,
                    palette.accent_secondary,
                );

                // Active Exam Indicator if test <= 7 days
                let today = Local::now().date_naive();
                if let Some(urgent_exam) = self.exams.exams.iter().find(|e| e.days_remaining(today) <= 7 && e.days_remaining(today) >= 0) {
                    ui.add_space(12.0);
                    let days = urgent_exam.days_remaining(today);
                    status_pill(
                        ui,
                        &format!("{} {} ({}d)", CALENDAR_ALERT, urgent_exam.subject_name, days),
                        palette.error.linear_multiply(0.2),
                        palette.error,
                    );
                }
            });
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(4.0);
    }

    fn render_pomodoro_tab(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);

            // Gamified Quest Header Banner
            ui.horizontal(|ui| {
                status_pill(
                    ui,
                    "🎯 Focus Quest: +30 XP & +10 Gold upon completion",
                    palette.bg_card,
                    palette.accent_tertiary,
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::from_id_salt("pomodoro_subject_selector")
                        .selected_text(RichText::new(&self.pomodoro.active_subject_name).size(13.5).strong().color(palette.accent_primary))
                        .show_ui(ui, |ui| {
                            for (i, subject) in self.cycle.subjects.iter().enumerate() {
                                if ui.selectable_label(self.pomodoro.active_subject_name == subject.name, &subject.name).clicked() {
                                    self.pomodoro.active_subject_name = subject.name.clone();
                                    self.cycle.active_index = i;
                                    self.save_state();
                                }
                            }
                        });
                    ui.label(RichText::new("Target Course:").size(13.5).color(palette.text_secondary));
                });
            });

            ui.add_space(10.0);

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

            ui.add_space(16.0);

            // Level Progress Bar right below dial
            let xp_frac = self.rogue.vault.xp_progress_fraction();
            let xp_label = format!("Level {} Progress ({} / {} XP)", self.rogue.vault.level, self.rogue.vault.current_xp, self.rogue.vault.xp_threshold());
            ui.add(
                CustomProgressBar::new(xp_frac, palette.accent_primary, palette.accent_glow, palette.track_bg)
                    .label(&xp_label)
                    .detail(&self.rogue.vault.equipped_title)
                    .height(8.0),
            );

            ui.add_space(16.0);

            // Material 3 Pill Action Buttons
            ui.horizontal(|ui| {
                let (play_text, play_color, play_fg) = match self.pomodoro.status {
                    TimerStatus::Running => (format!("{} Pause", PAUSE), palette.warning, palette.bg_deep),
                    TimerStatus::Paused | TimerStatus::Stopped => (format!("{} Start Focus", PLAY), palette.accent_primary, palette.bg_deep),
                };

                let primary_btn = egui::Button::new(RichText::new(play_text).size(15.0).strong().color(play_fg))
                    .fill(play_color)
                    .min_size(Vec2::new(145.0, 44.0))
                    .corner_radius(CornerRadius::same(22));

                if ui.add(primary_btn).clicked() {
                    match self.pomodoro.status {
                        TimerStatus::Running => self.pomodoro.pause(),
                        TimerStatus::Paused | TimerStatus::Stopped => self.pomodoro.start(),
                    }
                    self.save_state();
                }

                ui.add_space(10.0);

                let reset_btn = egui::Button::new(RichText::new(format!("{} Reset", RESET)).size(13.5).color(palette.text_secondary))
                    .fill(palette.bg_card)
                    .stroke(egui::Stroke::new(1.0_f32, palette.bg_card_border))
                    .min_size(Vec2::new(95.0, 44.0))
                    .corner_radius(CornerRadius::same(22));

                if ui.add(reset_btn).clicked() {
                    self.pomodoro.reset();
                    self.save_state();
                }

                ui.add_space(8.0);

                let skip_btn = egui::Button::new(RichText::new(format!("{} Skip", SKIP)).size(13.5).color(palette.text_secondary))
                    .fill(palette.bg_card)
                    .stroke(egui::Stroke::new(1.0_f32, palette.bg_card_border))
                    .min_size(Vec2::new(90.0, 44.0))
                    .corner_radius(CornerRadius::same(22));

                if ui.add(skip_btn).clicked() {
                    let _ = self.pomodoro.skip();
                    self.save_state();
                }
            });

            ui.add_space(18.0);

            // Session Presets row
            ui.horizontal(|ui| {
                ui.label(RichText::new("Rhythms:").size(13.0).color(palette.text_secondary));
                
                let presets = [
                    (25, 5, "25/5 Standard"),
                    (50, 10, "50/10 Lecture Block"),
                    (90, 20, "90/20 Deep Analytical"),
                ];

                for (work, brk, label) in presets {
                    let is_active = self.pomodoro.config.work_mins == work && self.pomodoro.config.short_break_mins == brk;
                    let btn = egui::Button::new(RichText::new(label).size(12.0).color(if is_active { palette.on_accent_container } else { palette.text_secondary }))
                        .fill(if is_active { palette.accent_container } else { palette.bg_card })
                        .stroke(egui::Stroke::new(1.0_f32, if is_active { palette.accent_primary } else { palette.bg_card_border }))
                        .corner_radius(CornerRadius::same(16));

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
                ui.add_space(12.0);
                
                let today_log = self.frequency.today_log();
                stat_card(ui, palette, CLOCK, "Completed Today", &format!("{} Pomodoros", today_log.pomodoros_completed), Some(&format!("{}m focused today", today_log.minutes_studied)));
                ui.add_space(12.0);
                
                let total_sessions = self.pomodoro.total_completed_sessions;
                stat_card(ui, palette, TROPHY, "All-Time Sessions", &format!("{}", total_sessions), Some("total completed"));
            });
        });
    }

    fn render_cycle_tab(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ScrollArea::vertical().show(ui, |ui| {
            card_frame(palette).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{} Study Cycle #{} • Curriculum Quests", TAB_CYCLE, self.cycle.cycle_number))
                                .size(19.0)
                                .strong()
                                .color(palette.text_primary),
                        );
                        ui.label(
                            RichText::new("Clear course blocks to balance your semester and earn +150 XP & +50 Vault Gold upon cycle victory!")
                                .size(12.5)
                                .color(palette.text_secondary),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let complete_btn = egui::Button::new(RichText::new(format!("{} Complete Cycle", CHECK)).strong().color(palette.bg_deep))
                            .fill(palette.accent_primary)
                            .corner_radius(CornerRadius::same(18));

                        if ui.add(complete_btn).clicked() {
                            self.cycle.complete_current_cycle();
                            // Cycle completion rewards!
                            self.rogue.vault.vault_gold += 50;
                            let lvl_up = self.rogue.vault.add_xp(150);
                            if let Some(lvl) = lvl_up {
                                self.set_toast(&format!("🏆 CYCLE COMPLETED! Level Up to Level {}! +50 Gold", lvl));
                            } else {
                                self.set_toast("🏆 Cycle Victory! +150 XP & +50 Vault Gold earned!");
                            }
                            self.save_state();
                        }

                        ui.add_space(8.0);

                        let preset_label = if self.show_presets_drawer { "Hide Presets" } else { "Presets & Templates" };
                        let preset_btn = egui::Button::new(RichText::new(format!("{} {}", PRESET_LIST, preset_label)).color(palette.text_primary))
                            .fill(palette.bg_card_hover)
                            .stroke(egui::Stroke::new(1.0_f32, palette.bg_card_border))
                            .corner_radius(CornerRadius::same(18));

                        if ui.add(preset_btn).clicked() {
                            self.show_presets_drawer = !self.show_presets_drawer;
                        }
                    });
                });

                ui.add_space(10.0);

                let cycle_progress = self.cycle.cycle_progress_fraction();
                let completed_mins = self.cycle.total_completed_minutes();
                let target_mins = self.cycle.total_target_minutes();

                ui.add(
                    CustomProgressBar::new(cycle_progress, palette.accent_primary, palette.accent_glow, palette.track_bg)
                        .label("Overall Cycle Completion")
                        .detail(&format!("{} / {} completed ({:.0}%)", StudySubject::format_time(completed_mins), StudySubject::format_time(target_mins), cycle_progress * 100.0))
                        .height(14.0),
                );
            });

            if self.show_presets_drawer {
                ui.add_space(10.0);
                elevated_card_frame(palette).show(ui, |ui| {
                    ui.label(RichText::new(format!("{} Undergrad Cycle Presets & Template Builder", GRADUATION_CAP)).size(15.0).strong().color(palette.accent_primary));
                    ui.label(RichText::new("Choose a pre-configured university semester template, or save your current rotation:").size(12.0).color(palette.text_secondary));
                    ui.add_space(8.0);

                    let builtins = CyclePreset::builtins();
                    for preset in &builtins {
                        ui.horizontal(|ui| {
                            let apply_btn = egui::Button::new(RichText::new(format!("Apply: {}", preset.name)).strong().color(palette.text_primary))
                                .fill(palette.bg_card)
                                .stroke(egui::Stroke::new(1.0_f32, palette.accent_primary))
                                .corner_radius(CornerRadius::same(14));

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

                    if !self.cycle.custom_presets.is_empty() {
                        ui.add_space(6.0);
                        ui.label(RichText::new("Your Custom Saved Presets:").size(13.0).strong().color(Color32::WHITE));
                        let mut delete_custom = None;
                        let mut apply_custom = None;
                        for (c_idx, custom) in self.cycle.custom_presets.iter().enumerate() {
                            ui.horizontal(|ui| {
                                if ui.button(RichText::new(format!("Apply: {}", custom.name)).strong().color(palette.accent_primary)).clicked() {
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

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Save Current as Preset:").size(12.5).color(palette.text_primary));
                        ui.add(egui::TextEdit::singleline(&mut self.custom_preset_name).hint_text("Preset Name"));
                        ui.add(egui::TextEdit::singleline(&mut self.custom_preset_desc).hint_text("Description"));

                        if ui.button(RichText::new("Save Preset").strong().color(palette.text_primary)).clicked() && !self.custom_preset_name.trim().is_empty() {
                            self.cycle.save_current_as_preset(&self.custom_preset_name, &self.custom_preset_desc);
                            self.custom_preset_name.clear();
                            self.custom_preset_desc.clear();
                            self.save_state();
                            self.set_toast("Custom cycle preset saved successfully!");
                        }
                    });
                });
            }

            ui.add_space(14.0);

            ui.label(RichText::new("Course Quests & Target Setters").size(16.0).strong().color(palette.text_primary));
            ui.add_space(4.0);

            let today = Local::now().date_naive();
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
                    glowing_card_frame(palette, palette.accent_primary)
                } else {
                    card_frame(palette)
                };

                frame.show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let fill_c = parse_hex_color(&subject.color_hex).unwrap_or(palette.accent_secondary);
                            ui.label(RichText::new("●").size(15.0).color(fill_c));
                            ui.add_space(6.0);
                            ui.label(RichText::new(&subject.name).size(15.0).strong().color(palette.text_primary));
                            ui.add_space(10.0);

                            if is_active {
                                status_pill(ui, &format!("{} Active Quest", TARGET), palette.accent_container, palette.on_accent_container);
                                ui.add_space(8.0);
                            }
                            if subject.is_completed() {
                                status_pill(ui, &format!("{} Cleared (+15 XP)", CHECK), palette.success.linear_multiply(0.2), palette.success);
                                ui.add_space(8.0);
                            }

                            if let Some(exam) = self.exams.exams.iter().find(|e| e.subject_name.eq_ignore_ascii_case(&subject.name)) {
                                let days = exam.days_remaining(today);
                                status_pill(ui, &format!("{} Trial in {}d", CALENDAR_ALERT, days), palette.bg_card_hover, palette.accent_tertiary);
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(RichText::new(TRASH).size(13.0).color(palette.text_muted)).clicked() {
                                    to_delete = Some(idx);
                                }
                                ui.add_space(6.0);
                                if ui.button(RichText::new(CHEVRON_DOWN).size(13.0)).clicked() {
                                    to_move_down = Some(idx);
                                }
                                ui.add_space(4.0);
                                if ui.button(RichText::new(CHEVRON_UP).size(13.0)).clicked() {
                                    to_move_up = Some(idx);
                                }
                                ui.add_space(6.0);
                                if !is_active && ui.button("Set Active").clicked() {
                                    set_active = Some(idx);
                                }
                            });
                        });

                        ui.add_space(6.0);

                        let detail_text = if subject.boost_minutes > 0 {
                            format!("{}/{} (Base {}m + Trial Boost {}m)", 
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
                                .height(10.0),
                        );

                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Target:").size(12.0).color(palette.text_secondary));
                            ui.add_space(4.0);
                            ui.label(RichText::new(StudySubject::format_time(subject.base_minutes)).strong().size(13.0).color(palette.accent_primary));
                            ui.add_space(8.0);

                            if ui.button("-15m").clicked() {
                                adjust_base = Some((idx, -15));
                            }
                            ui.add_space(4.0);
                            if ui.button("+15m").clicked() {
                                adjust_base = Some((idx, 15));
                            }

                            ui.add_space(10.0);
                            for &chip_mins in &[25, 45, 60, 90] {
                                let is_current = subject.base_minutes == chip_mins;
                                let chip_btn = egui::Button::new(RichText::new(format!("{}m", chip_mins)).size(11.0).color(if is_current { palette.on_accent_container } else { palette.text_secondary }))
                                    .fill(if is_current { palette.accent_container } else { palette.bg_deep })
                                    .stroke(egui::Stroke::NONE)
                                    .corner_radius(CornerRadius::same(12));

                                if ui.add(chip_btn).clicked() {
                                    set_preset_time = Some((idx, chip_mins));
                                }
                                ui.add_space(6.0);
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(RichText::new("+15m Logged").size(11.5)).clicked() {
                                    add_minutes = Some((idx, 15));
                                }
                            });
                        });
                    });
                });

                ui.add_space(6.0);
            }

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
                    self.rogue.vault.add_xp(15);
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

            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new("+ Add Course to Rotation").size(14.0).strong().color(palette.accent_secondary));
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_subject_name).hint_text("Course name"));

                    ui.label("Target:");
                    ui.label(RichText::new(StudySubject::format_time(self.new_subject_mins)).strong().color(palette.accent_primary));

                    for &chip in &[30, 45, 60, 90] {
                        if ui.selectable_label(self.new_subject_mins == chip, format!("{}m", chip)).clicked() {
                            self.new_subject_mins = chip;
                        }
                    }

                    let add_btn = egui::Button::new(RichText::new("Add Subject").strong().color(palette.bg_deep))
                        .fill(palette.accent_primary)
                        .corner_radius(CornerRadius::same(16));

                    if ui.add(add_btn).clicked() && !self.new_subject_name.trim().is_empty() {
                        self.cycle.add_subject(&self.new_subject_name, self.new_subject_mins, &self.new_subject_color);
                        self.exams.sync_all_exam_boosts(&mut self.cycle);
                        self.new_subject_name.clear();
                        self.save_state();
                        self.set_toast("Course added to cycle!");
                    }
                });
            });
        });
    }

    fn render_test_mode_tab(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ScrollArea::vertical().show(ui, |ui| {
            card_frame(palette).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{} Test Mode • Raid Trials & Exam Booster", TEST_PENCIL))
                                .size(19.0)
                                .strong()
                                .color(palette.text_primary),
                        );
                        ui.label(
                            RichText::new("Track upcoming exam trials. FernDoro automatically injects urgency study points into your cycle!")
                                .size(12.5)
                                .color(palette.text_secondary),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let rebalance_btn = egui::Button::new(RichText::new(format!("{} Auto-Balance", BOLT)).strong().color(palette.bg_deep))
                            .fill(palette.accent_primary)
                            .corner_radius(CornerRadius::same(16));

                        if ui.add(rebalance_btn).clicked() {
                            self.exams.sync_all_exam_boosts(&mut self.cycle);
                            self.save_state();
                            self.set_toast("Cycle balanced for upcoming tests!");
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

            ui.label(RichText::new("Scheduled University Trials & Exams").size(16.0).strong().color(palette.text_primary));
            ui.add_space(4.0);

            let today = Local::now().date_naive();
            let mut to_remove_id = None;

            if self.exams.exams.is_empty() {
                card_frame(palette).show(ui, |ui| {
                    ui.label(RichText::new("No upcoming exams scheduled. Add an exam below to boost your cycle.").color(palette.text_muted));
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
                                    ui.label(RichText::new(&exam.subject_name).size(15.5).strong().color(palette.accent_primary));
                                    ui.add_space(6.0);
                                    ui.label(RichText::new("•").color(palette.text_muted));
                                    ui.add_space(6.0);
                                    ui.label(RichText::new(&exam.test_title).size(14.5).strong().color(palette.text_primary));
                                    ui.add_space(10.0);
                                    status_pill(ui, urgency_label, badge_color.linear_multiply(0.2), badge_color);
                                });

                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    let date_str = exam.exam_date.format("%a, %b %d").to_string();
                                    let countdown = if days < 0 { "Past Due".to_string() } else if days == 0 { "TODAY!".to_string() } else { format!("In {} days", days) };

                                    ui.label(RichText::new(format!("{} {} ({})", CALENDAR, date_str, countdown)).size(12.0).color(palette.text_secondary));
                                    ui.label(RichText::new("•").color(palette.text_muted));
                                    ui.label(RichText::new(format!("Diff: {}/5 • Weight: {}/5", exam.difficulty, exam.weight)).size(12.0).color(palette.text_muted));
                                });
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(RichText::new(TRASH).size(13.0).color(palette.text_muted)).clicked() {
                                    to_remove_id = Some(exam.id.clone());
                                }

                                ui.add_space(8.0);
                                status_pill(
                                    ui,
                                    &format!("{} +{}m", BOLT, auto_points),
                                    palette.accent_container,
                                    palette.on_accent_container,
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
                self.set_toast("Exam removed and cycle updated.");
            }

            ui.add_space(14.0);

            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new(format!("{} Schedule an Exam", GRADUATION_CAP)).size(14.5).strong().color(palette.accent_secondary));
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.label("Course:");
                    egui::ComboBox::from_id_salt("exam_subject_dropdown")
                        .selected_text(&self.new_exam_subject)
                        .show_ui(ui, |ui| {
                            for sub in &self.cycle.subjects {
                                if ui.selectable_label(self.new_exam_subject == sub.name, &sub.name).clicked() {
                                    self.new_exam_subject = sub.name.clone();
                                }
                            }
                        });
                    ui.add(egui::TextEdit::singleline(&mut self.new_exam_subject).hint_text("Custom course name"));
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_exam_title).hint_text("e.g. P1 Integrals Exam"));
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Days until exam:");
                    ui.add(egui::Slider::new(&mut self.new_exam_days, 1..=60).suffix(" days"));

                    ui.add_space(8.0);
                    for &chip_d in &[3, 7, 14, 21] {
                        if ui.selectable_label(self.new_exam_days == chip_d, format!("{}d", chip_d)).clicked() {
                            self.new_exam_days = chip_d;
                        }
                        ui.add_space(6.0);
                    }
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Difficulty:");
                    ui.add(egui::Slider::new(&mut self.new_exam_diff, 1..=5));

                    ui.add_space(8.0);
                    ui.label("Importance / Weight:");
                    ui.add(egui::Slider::new(&mut self.new_exam_weight, 1..=5));
                });

                ui.add_space(8.0);

                let exam_preview = ExamTest::new(
                    &self.new_exam_subject,
                    &self.new_exam_title,
                    today + Duration::days(self.new_exam_days as i64),
                    self.new_exam_weight,
                    self.new_exam_diff,
                );
                let preview_points = exam_preview.calculate_auto_points(today);

                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("Auto-boost: +{}m for {}", preview_points, self.new_exam_subject)).color(palette.accent_tertiary));

                    let add_btn = egui::Button::new(RichText::new(format!("{} Add Exam", BOLT)).strong().color(palette.bg_deep))
                        .fill(palette.accent_primary)
                        .corner_radius(CornerRadius::same(16));

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
                        self.set_toast(&format!("Exam scheduled! +{}m added to {}", preview_points, self.new_exam_subject));
                    }
                });
            });
        });
    }

    // ROGUE DUNGEON GAME MODE TAB WITH ORGANIZED SECTORS & PROMINENT GOLD HUD
    fn render_rogue_tab(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ScrollArea::vertical().show(ui, |ui| {
            // SECTOR 1: THE GOLD & RUN VAULT HUD (PROMINENT & MINIMALIST)
            egui::Frame::new()
                .fill(palette.bg_card)
                .stroke(egui::Stroke::new(1.0_f32, palette.bg_card_border))
                .corner_radius(CornerRadius::same(18))
                .inner_margin(Margin::symmetric(24, 18))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Left: Vault Gold Reserve
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(COIN).size(30.0).color(palette.accent_tertiary));
                            ui.add_space(14.0);
                            ui.vertical(|ui| {
                                ui.label(RichText::new("VAULT RESERVE (SAFE)").size(11.0).strong().color(palette.text_secondary));
                                ui.label(
                                    RichText::new(format!("{} Gold", self.rogue.vault.vault_gold))
                                        .size(24.0)
                                        .strong()
                                        .color(palette.accent_tertiary),
                                );
                            });
                        });

                        ui.add_space(32.0);

                        // Center: Run Multiplier & Soul Shields
                        ui.vertical(|ui| {
                            ui.label(RichText::new(format!("Floor {} Encounter", self.rogue.current_run.floor_number)).size(11.0).color(palette.text_secondary));
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                status_pill(ui, &format!("⚡ {:.2}x Mult", self.rogue.current_run.current_multiplier), palette.bg_card_hover, palette.accent_primary);
                                ui.add_space(10.0);
                                status_pill(ui, &format!("{} {} Shields", SHIELD, self.rogue.current_run.soul_shields), palette.bg_card_hover, palette.text_primary);
                            });
                        });

                        // Right: Unbanked Pot & Instant Bank Button
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(4.0);
                            if self.rogue.current_run.unbanked_gold > 0 {
                                let cash_out_btn = egui::Button::new(RichText::new("💰 Bank Gold to Vault").strong().color(palette.bg_deep))
                                    .fill(palette.accent_tertiary)
                                    .min_size(Vec2::new(0.0, 36.0))
                                    .corner_radius(CornerRadius::same(18));

                                if ui.add(cash_out_btn).clicked() {
                                    let banked = self.rogue.cash_out();
                                    self.save_state();
                                    self.set_toast(&format!("Safely secured {} Gold into Vault!", banked));
                                }
                                ui.add_space(16.0);
                            }

                            ui.vertical(|ui| {
                                ui.label(RichText::new("UNBANKED POT (AT RISK)").size(11.0).strong().color(palette.text_secondary));
                                ui.label(
                                    RichText::new(format!("+{} Gold", self.rogue.current_run.unbanked_gold))
                                        .size(22.0)
                                        .strong()
                                        .color(palette.accent_primary),
                                );
                            });
                        });
                    });
                });

            ui.add_space(16.0);

            // SECTOR 2: RELIC SHOP & PERMANENT PROGRESSION (EXPANDABLE)
            ui.horizontal(|ui| {
                let shop_label = if self.show_relic_shop { "Hide Relic Armory" } else { "Open Relic Armory & Artifacts" };
                let shop_btn = egui::Button::new(RichText::new(format!("{}  {}", VAULT, shop_label)).size(13.0).color(palette.text_primary))
                    .fill(palette.bg_card)
                    .stroke(egui::Stroke::new(1.0_f32, palette.bg_card_border))
                    .min_size(Vec2::new(0.0, 34.0))
                    .corner_radius(CornerRadius::same(16));

                if ui.add(shop_btn).clicked() {
                    self.show_relic_shop = !self.show_relic_shop;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    status_pill(
                        ui,
                        &format!("👑 Player Rank: {}", self.rogue.vault.equipped_title),
                        palette.accent_container.linear_multiply(0.4),
                        palette.accent_primary,
                    );
                });
            });

            if self.show_relic_shop {
                ui.add_space(12.0);
                elevated_card_frame(palette).show(ui, |ui| {
                    ui.label(RichText::new(format!("{} The Dungeon Relic Armory", VAULT)).size(15.5).strong().color(palette.accent_tertiary));
                    ui.label(RichText::new("Invest Vault Gold into permanent passive relics to survive higher floors:").size(12.0).color(palette.text_secondary));
                    ui.add_space(10.0);

                    let relics = [
                        ("loaded_dice", "Loaded Dice", "Shifts fate card draw odds +15% towards Boons", 150),
                        ("merchants_coin", "Merchant's Coin", "+20% bonus Gold on all cleared floors", 200),
                        ("hourglass_of_mercy", "Hourglass of Mercy", "+5s extra grace period on window unfocus (15s total)", 250),
                        ("aegis_of_will", "Aegis of Will", "Start every dungeon run with +1 Soul Shield", 300),
                    ];

                    for (id, name, desc, cost) in relics {
                        let owned = self.rogue.vault.unlocked_relics.contains(id);
                        ui.horizontal(|ui| {
                            if owned {
                                status_pill(ui, "✓ Equipped", palette.success.linear_multiply(0.2), palette.success);
                            } else {
                                let can_afford = self.rogue.vault.vault_gold >= cost;
                                let btn = egui::Button::new(RichText::new(format!("Buy: {}g", cost)).color(if can_afford { palette.bg_deep } else { palette.text_muted }))
                                    .fill(if can_afford { palette.accent_primary } else { palette.bg_deep })
                                    .corner_radius(CornerRadius::same(12));

                                if ui.add_enabled(can_afford, btn).clicked() {
                                    if self.rogue.buy_relic(id, cost) {
                                        self.save_state();
                                        self.set_toast(&format!("Acquired '{}'!", name));
                                    }
                                }
                            }

                            ui.add_space(10.0);
                            ui.label(RichText::new(name).strong().color(palette.text_primary));
                            ui.label(RichText::new(format!("— {}", desc)).size(11.5).color(palette.text_secondary));
                        });
                        ui.add_space(6.0);
                    }
                });
            }

            ui.add_space(16.0);

            // SECTOR 3: ACTIVE GAME STAGE ENGINE
            match self.rogue.phase {
                RoguePhase::IdleAnte => {
                    self.render_rogue_ante_sector(ui, palette);
                }
                RoguePhase::WarmupLock => {
                    card_frame(palette).show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(format!("{} Sudden Death Warmup Lock", SKULL)).size(18.0).strong().color(palette.error));
                            ui.label(RichText::new("Curse Active: Prepare your focus. The encounter will begin shortly.").color(palette.text_secondary));
                            ui.add_space(10.0);

                            let warmup_secs = self.rogue.remaining_warmup_seconds.ceil() as u32;
                            ui.label(RichText::new(format!("{:02}s", warmup_secs)).size(48.0).monospace().color(palette.error));
                        });
                    });
                }
                RoguePhase::FocusActive => {
                    self.render_rogue_focus_sector(ui, palette);
                }
                RoguePhase::FocusFailed => {
                    card_frame(palette).show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(format!("{} ENCOUNTER BUSTED!", SKULL)).size(22.0).strong().color(palette.error));
                            ui.add_space(6.0);
                            ui.label(RichText::new("You lost focus during a strict Ante tier. Your unbanked gold was lost to the dungeon.").color(palette.text_secondary));
                            ui.add_space(14.0);

                            let retry_btn = egui::Button::new(RichText::new("Return to Floor 1").strong().color(palette.bg_deep))
                                .fill(palette.accent_primary)
                                .corner_radius(CornerRadius::same(18));

                            if ui.add(retry_btn).clicked() {
                                self.rogue.current_run.floor_number = 1;
                                self.rogue.current_run.current_multiplier = 1.0;
                                self.rogue.phase = RoguePhase::IdleAnte;
                                self.save_state();
                            }
                        });
                    });
                }
                RoguePhase::BreakDealer => {
                    self.render_rogue_dealer_sector(ui, palette);
                }
                RoguePhase::BreakRest => {
                    self.render_rogue_sanctuary_sector(ui, palette);
                }
            }
        });
    }

    fn render_rogue_ante_sector(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        card_frame(palette).show(ui, |ui| {
            ui.label(RichText::new(format!("{} Floor Contract & Discipline Wager", COIN)).size(16.5).strong().color(palette.text_primary));
            ui.add_space(4.0);
            ui.label(RichText::new("Select your risk tier. Harder tiers strictly lock pauses and penalize leaving the window, but multiply payouts:").size(12.0).color(palette.text_secondary));
            ui.add_space(14.0);

            // Active Modifier Banner if any
            if self.rogue.current_run.active_modifier != FateModifier::None {
                let m = self.rogue.current_run.active_modifier;
                let badge_color = if m.is_boon() { palette.success } else { palette.warning };
                ui.horizontal(|ui| {
                    status_pill(ui, &format!("Active Modifier: {}", m.name()), badge_color.linear_multiply(0.2), badge_color);
                    ui.add_space(10.0);
                    ui.label(RichText::new(m.description()).size(12.0).color(palette.text_secondary));
                });
                ui.add_space(14.0);
            }

            let antes = [
                AnteTier::SafeBet1x,
                AnteTier::DoubleDown2x,
                AnteTier::AllInBoss3x,
                AnteTier::AllInBoss5x,
            ];

            for ante in antes {
                let is_selected = self.rogue.current_run.selected_ante == ante;
                let bg_color = if is_selected {
                    palette.accent_container.linear_multiply(0.4)
                } else {
                    palette.bg_card_hover
                };
                let border_color = if is_selected { palette.accent_primary } else { palette.bg_card_border };
                let border_width = if is_selected { 1.5_f32 } else { 1.0_f32 };

                let frame = egui::Frame::new()
                    .fill(bg_color)
                    .stroke(egui::Stroke::new(border_width, border_color))
                    .corner_radius(CornerRadius::same(16))
                    .inner_margin(Margin::symmetric(18, 14));

                let response = frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Badge Pill on the left
                        let (badge_bg, badge_fg) = if is_selected {
                            (palette.accent_primary, palette.bg_deep)
                        } else {
                            (palette.bg_card, palette.text_primary)
                        };
                        status_pill(ui, ante.name(), badge_bg, badge_fg);

                        ui.add_space(16.0);

                        // Description in center
                        let text_color = if is_selected { palette.text_primary } else { palette.text_secondary };
                        ui.label(RichText::new(ante.description()).size(12.0).color(text_color));

                        // Multiplier and rules pill on the right
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let (rule_text, rule_color) = match ante {
                                AnteTier::SafeBet1x => ("⚡ 1.0x • 2 Pauses", palette.text_secondary),
                                AnteTier::DoubleDown2x => ("⚡ 2.0x • Zero Pauses", palette.accent_secondary),
                                AnteTier::AllInBoss3x => ("⚡ 3.0x • Boss Strict", palette.warning),
                                AnteTier::AllInBoss5x => ("⚡ 5.0x • Nightmare", palette.error),
                            };
                            status_pill(ui, rule_text, palette.bg_deep, rule_color);
                        });
                    });
                });

                if response.response.interact(egui::Sense::click()).clicked() {
                    self.rogue.current_run.selected_ante = ante;
                    if self.rogue.current_run.floor_duration_mins < ante.min_minutes() {
                        self.rogue.current_run.floor_duration_mins = ante.min_minutes();
                    }
                }
                ui.add_space(10.0);
            }

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(14.0);

            // Row 1: Floor Objective
            ui.horizontal(|ui| {
                ui.label(RichText::new("Floor Objective:").strong().size(13.5).color(palette.text_primary));
                ui.add_space(10.0);
                let edit = egui::TextEdit::singleline(&mut self.rogue.current_run.selected_task)
                    .hint_text("Study Task, Topic, or Boss...")
                    .desired_width(320.0)
                    .margin(Margin::symmetric(10, 8));
                ui.add(edit);
            });

            ui.add_space(14.0);

            // Row 2: Duration Selector
            ui.horizontal(|ui| {
                ui.label(RichText::new("Session Duration:").strong().size(13.5).color(palette.text_primary));
                ui.add_space(12.0);
                for &mins in &[25, 45, 50, 60] {
                    if mins >= self.rogue.current_run.selected_ante.min_minutes() {
                        let is_sel = self.rogue.current_run.floor_duration_mins == mins;
                        let dur_btn = egui::Button::new(
                            RichText::new(format!("{}m", mins))
                                .strong()
                                .size(13.0)
                                .color(if is_sel { palette.on_accent_container } else { palette.text_secondary })
                        )
                        .fill(if is_sel { palette.accent_container } else { palette.bg_card_hover })
                        .stroke(egui::Stroke::new(1.0_f32, if is_sel { palette.accent_primary } else { palette.bg_card_border }))
                        .corner_radius(CornerRadius::same(16))
                        .min_size(Vec2::new(56.0, 32.0));

                        if ui.add(dur_btn).clicked() {
                            self.rogue.current_run.floor_duration_mins = mins;
                        }
                        ui.add_space(10.0);
                    }
                }
            });

            ui.add_space(18.0);

            // Row 3: Rewards & Enter Encounter CTA
            let potential_payout = self.rogue.calculate_floor_payout();
            let potential_xp = self.rogue.calculate_floor_xp();
            ui.horizontal(|ui| {
                status_pill(
                    ui,
                    &format!("🎁 Est. Encounter Yield: ~{} Gold  •  +{} XP", potential_payout, potential_xp),
                    palette.accent_tertiary.linear_multiply(0.18),
                    palette.accent_tertiary,
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let enter_btn = egui::Button::new(RichText::new(format!("{}  Enter Encounter", SWORD)).size(15.0).strong().color(palette.bg_deep))
                        .fill(palette.accent_primary)
                        .min_size(Vec2::new(180.0, 44.0))
                        .corner_radius(CornerRadius::same(22));

                    if ui.add(enter_btn).clicked() {
                        self.rogue.start_encounter();
                        self.save_state();
                    }
                });
            });
        });
    }

    fn render_rogue_focus_sector(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ui.vertical_centered(|ui| {
            if self.rogue.is_grace_active {
                elevated_card_frame(palette).show(ui, |ui| {
                    ui.label(
                        RichText::new(format!("🚨 RETURN TO FERNDORO! Focus Lock active: {:.1}s remaining before BUST!", self.rogue.grace_period_seconds))
                            .size(15.0)
                            .strong()
                            .color(palette.error),
                    );
                });
                ui.add_space(10.0);
            }

            let is_blind = self.rogue.current_run.active_modifier == FateModifier::BlindRun 
                && self.rogue.remaining_focus_seconds > 60.0;

            let total_secs = (self.rogue.current_run.floor_duration_mins * 60) as f32;
            let progress = if total_secs > 0.0 {
                (1.0 - (self.rogue.remaining_focus_seconds / total_secs)).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let remaining = self.rogue.remaining_focus_seconds.ceil().max(0.0) as u32;
            let time_str = format!("{:02}:{:02}", remaining / 60, remaining % 60);
            let sub_title = format!("Floor {} • {}", self.rogue.current_run.floor_number, self.rogue.current_run.selected_ante.name());

            let dial = TimerDial::new(
                progress,
                &time_str,
                "DUNGEON ENCOUNTER",
                &sub_title,
                palette.accent_primary,
                palette.accent_glow,
                palette.track_bg,
                true,
            ).blind(is_blind);

            ui.add(dial);

            ui.add_space(16.0);

            ui.horizontal(|ui| {
                let pause_text = if self.rogue.current_run.selected_ante == AnteTier::SafeBet1x {
                    format!("{} Pause ({} left)", PAUSE, 2 - self.rogue.current_run.pauses_used)
                } else if self.rogue.current_run.soul_shields > 0 {
                    format!("{} Soul Shield Pause", SHIELD)
                } else {
                    format!("{} Locked (Hard Mode)", PAUSE)
                };

                let can_pause = self.rogue.current_run.selected_ante.max_pauses() > self.rogue.current_run.pauses_used 
                    || self.rogue.current_run.soul_shields > 0;

                let pause_btn = egui::Button::new(RichText::new(pause_text).color(if can_pause { palette.text_primary } else { palette.text_muted }))
                    .fill(palette.bg_card)
                    .corner_radius(CornerRadius::same(18));

                if ui.add_enabled(can_pause, pause_btn).clicked() {
                    self.rogue.pause_encounter();
                    self.save_state();
                }

                ui.add_space(10.0);

                let abandon_btn = egui::Button::new(RichText::new("Forfeit Encounter").color(palette.error))
                    .fill(palette.bg_card)
                    .corner_radius(CornerRadius::same(18));

                if ui.add(abandon_btn).clicked() {
                    self.rogue.trigger_bust();
                    self.save_state();
                }
            });
        });
    }

    fn render_rogue_dealer_sector(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        card_frame(palette).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(format!("{} The Floor Dealer (Break Casino)", CARDS)).size(19.0).strong().color(palette.accent_tertiary));
                ui.label(RichText::new("Pick 1 of 3 Mystery Fate Cards to seal your modifier for the next floor:").color(palette.text_secondary));
                ui.add_space(12.0);

                let break_secs = self.rogue.remaining_break_seconds.ceil() as u32;
                ui.label(RichText::new(format!("Sanctuary Rest Running: {:02}:{:02}", break_secs / 60, break_secs % 60)).color(palette.break_phase));
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    for i in 0..self.rogue.dealer_cards.len() {
                        let card = self.rogue.dealer_cards[i];
                        let is_flipped = self.rogue.dealer_resolved;

                        let (card_bg, card_text, card_desc) = if is_flipped {
                            (palette.bg_card_hover, card.name(), card.description())
                        } else {
                            (palette.accent_container, "🂠 Fate Card", "Click to Reveal")
                        };

                        let frame = egui::Frame::new()
                            .fill(card_bg)
                            .stroke(egui::Stroke::new(1.0_f32, palette.accent_primary))
                            .corner_radius(CornerRadius::same(16))
                            .inner_margin(Margin::same(16));

                        frame.show(ui, |ui| {
                            ui.set_min_size(Vec2::new(180.0, 110.0));
                            ui.vertical_centered(|ui| {
                                ui.label(RichText::new(card_text).strong().size(14.0).color(Color32::WHITE));
                                ui.add_space(4.0);
                                ui.label(RichText::new(card_desc).size(11.0).color(palette.text_secondary));

                                if !is_flipped {
                                    ui.add_space(8.0);
                                    if ui.button(RichText::new("Reveal").strong().color(palette.bg_deep)).clicked() {
                                        self.rogue.draw_card(i);
                                        self.save_state();
                                    }
                                }
                            });
                        });
                        ui.add_space(6.0);
                    }
                });

                ui.add_space(14.0);

                let rest_btn = egui::Button::new(RichText::new("Step into Sanctuary Rest →").strong().color(palette.text_primary))
                    .fill(palette.bg_card)
                    .corner_radius(CornerRadius::same(18));

                if ui.add(rest_btn).clicked() {
                    self.rogue.phase = RoguePhase::BreakRest;
                    self.save_state();
                }
            });
        });
    }

    fn render_rogue_sanctuary_sector(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        card_frame(palette).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("🌿 The Sanctuary (Guaranteed Rest)").size(20.0).strong().color(palette.break_phase));
                ui.label(RichText::new("Step away from the screen, hydrate, and breathe. Your rest cannot be lost.").color(palette.text_secondary));
                ui.add_space(16.0);

                let break_secs = self.rogue.remaining_break_seconds.ceil() as u32;
                ui.label(RichText::new(format!("{:02}:{:02}", break_secs / 60, break_secs % 60)).size(54.0).monospace().color(palette.break_phase));

                ui.add_space(16.0);

                let next_floor_btn = egui::Button::new(RichText::new(format!("Advance to Floor {} →", self.rogue.current_run.floor_number + 1)).size(15.0).strong().color(palette.bg_deep))
                    .fill(palette.accent_primary)
                    .corner_radius(CornerRadius::same(20));

                if ui.add(next_floor_btn).clicked() {
                    self.rogue.proceed_to_next_floor();
                    self.save_state();
                }
            });
        });
    }

    fn render_frequency_tab(&mut self, ui: &mut Ui, palette: &ThemePalette) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                let streak = self.frequency.current_streak_days();
                stat_card(ui, palette, FLAME, "Current Streak", &format!("{} Days", streak), Some("consecutive study days"));
                ui.add_space(10.0);

                let best = self.frequency.best_streak_days();
                stat_card(ui, palette, TROPHY, "Best Streak", &format!("{} Days", best), Some("all-time record"));
                ui.add_space(10.0);

                let total_mins = self.frequency.total_minutes_all_time();
                let hours = total_mins / 60;
                let mins = total_mins % 60;
                stat_card(ui, palette, CLOCK, "Total Study Time", &format!("{}h {}m", hours, mins), Some("across all sessions"));
                ui.add_space(10.0);

                let active_days = self.frequency.total_active_days();
                stat_card(ui, palette, CALENDAR, "Active Days", &format!("{} Days", active_days), Some("days with logged work"));
            });

            ui.add_space(16.0);

            card_frame(palette).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Consistency Matrix (Past 20 Weeks)").size(15.0).strong().color(palette.text_primary));
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

            card_frame(palette).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Today's Daily Target").size(15.0).strong().color(palette.text_primary));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("+15m").clicked() {
                            self.frequency.record_study_session(15, false);
                            self.rogue.vault.add_xp(15);
                            self.save_state();
                        }
                        if ui.button("+30m").clicked() {
                            self.frequency.record_study_session(30, false);
                            self.rogue.vault.add_xp(30);
                            self.save_state();
                        }
                        if ui.button("+60m").clicked() {
                            self.frequency.record_study_session(60, false);
                            self.rogue.vault.add_xp(60);
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
                        .height(13.0),
                );
            });

            ui.add_space(16.0);

            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new("Daily Habits & Attendance Checklist").size(15.0).strong().color(palette.text_primary));
                ui.label(RichText::new("Complete all 4 daily routines to claim a +50 XP bonus:").size(12.5).color(palette.text_secondary));
                ui.add_space(8.0);

                let today_log = self.frequency.today_log();
                let mut toggled = None;

                for habit in &self.frequency.habits {
                    let is_checked = today_log.habits_checked.contains(&habit.name);
                    ui.horizontal(|ui| {
                        let text = format!("{}  {}", habit.icon, habit.name);
                        let mut checked = is_checked;
                        if ui.checkbox(&mut checked, RichText::new(text).size(13.5).color(if is_checked { palette.accent_primary } else { palette.text_primary })).changed() {
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
            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new(format!("{} Visual Themes (Material You)", PALETTE)).size(16.0).strong().color(palette.text_primary));
                ui.label(RichText::new("Material 3 tonal surface palettes with soft lavender, violet, and rose accents:").size(12.5).color(palette.text_secondary));
                ui.add_space(10.0);

                for mode in ThemeMode::all() {
                    let is_selected = self.theme == *mode;
                    let text = mode.name();
                    
                    let btn = egui::Button::new(RichText::new(text).size(13.0).color(if is_selected { palette.on_accent_container } else { palette.text_secondary }))
                        .fill(if is_selected { palette.accent_container } else { palette.bg_deep })
                        .stroke(egui::Stroke::new(1.0_f32, if is_selected { palette.accent_primary } else { palette.bg_card_border }))
                        .corner_radius(CornerRadius::same(16));

                    if ui.add(btn).clicked() {
                        self.theme = *mode;
                        self.theme.apply(ctx);
                        self.save_state();
                    }
                    ui.add_space(4.0);
                }
            });

            ui.add_space(16.0);

            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new(format!("{} Pomodoro Preferences", TAB_POMODORO)).size(16.0).strong().color(palette.text_primary));
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

            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new("Data & History Management").size(16.0).strong().color(palette.text_primary));
                ui.label(RichText::new("Clear recorded study sessions, streaks, and heatmap logs to start with a fresh slate.").size(12.5).color(palette.text_secondary));
                ui.add_space(8.0);

                let clear_btn = egui::Button::new(RichText::new(format!("{} Clear Study History & Streaks", TRASH)).strong().color(palette.error))
                    .fill(palette.bg_deep)
                    .stroke(egui::Stroke::new(1.0_f32, palette.error))
                    .corner_radius(CornerRadius::same(16));

                if ui.add(clear_btn).clicked() {
                    self.frequency.clear_history();
                    self.pomodoro.total_completed_sessions = 0;
                    self.pomodoro.current_cycle_count = 0;
                    self.save_state();
                    self.set_toast("Study history and streaks have been cleared!");
                }
            });

            ui.add_space(16.0);

            card_frame(palette).show(ui, |ui| {
                ui.label(RichText::new(format!("{} About FernDoro", LOGO_LEAF)).size(15.0).strong().color(palette.accent_primary));
                ui.label(RichText::new("Version 0.3.5 • Material You Design • Gamified Study Suite").size(12.5).color(palette.text_secondary));
                ui.label(RichText::new("Built with Rust, egui, Inter & JetBrains Mono for clean, gamified mastery.").size(12.0).color(palette.text_muted));
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
