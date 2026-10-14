use chrono::{Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use crate::cycle::StudyCycleState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrgencyLevel {
    Critical, // <= 3 days
    High,     // 4 to 7 days
    Medium,   // 8 to 14 days
    Normal,   // > 14 days
}

impl UrgencyLevel {
    pub fn display_badge(&self) -> (&'static str, egui::Color32) {
        match self {
            Self::Critical => ("CRITICAL (<= 3d)", egui::Color32::from_rgb(239, 68, 68)),   // Red
            Self::High => ("HIGH URGENCY", egui::Color32::from_rgb(249, 115, 22)),          // Orange
            Self::Medium => ("UPCOMING", egui::Color32::from_rgb(234, 179, 8)),             // Amber
            Self::Normal => ("SCHEDULED", egui::Color32::from_rgb(168, 85, 247)),           // Purple
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamTest {
    pub id: String,
    pub subject_name: String,
    pub test_title: String,     // e.g. "P1 Calculus Exam", "Midterm EDA"
    pub exam_date: NaiveDate,
    pub weight: u32,            // 1 to 5 (importance)
    pub difficulty: u32,        // 1 to 5 (difficulty)
}

impl ExamTest {
    pub fn new(subject: &str, title: &str, date: NaiveDate, weight: u32, difficulty: u32) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        Self {
            id: format!("exam-{:x}", now.as_millis()),
            subject_name: subject.trim().to_string(),
            test_title: title.trim().to_string(),
            exam_date: date,
            weight: weight.clamp(1, 5),
            difficulty: difficulty.clamp(1, 5),
        }
    }

    pub fn days_remaining(&self, today: NaiveDate) -> i64 {
        (self.exam_date - today).num_days()
    }

    pub fn urgency(&self, today: NaiveDate) -> UrgencyLevel {
        let days = self.days_remaining(today);
        if days <= 3 {
            UrgencyLevel::Critical
        } else if days <= 7 {
            UrgencyLevel::High
        } else if days <= 14 {
            UrgencyLevel::Medium
        } else {
            UrgencyLevel::Normal
        }
    }

    /// Calculates how many extra minutes should automatically be added to this subject's cycle block
    pub fn calculate_auto_points(&self, today: NaiveDate) -> u32 {
        let days = self.days_remaining(today);
        let base_bonus = if days <= 0 {
            60 // Exam today or past due
        } else if days <= 3 {
            45 // Critical
        } else if days <= 7 {
            30 // High
        } else if days <= 14 {
            15 // Medium
        } else {
            10 // Normal baseline boost
        };

        let diff_mult = (self.difficulty as f32 / 3.0).clamp(0.6, 1.8);
        let weight_mult = (self.weight as f32 / 3.0).clamp(0.6, 1.8);
        let boosted = (base_bonus as f32 * diff_mult * weight_mult).round() as u32;

        // Snap to clean 5-minute increments
        ((boosted + 4) / 5) * 5
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamState {
    pub exams: Vec<ExamTest>,
    pub auto_boost_enabled: bool,
}

impl Default for ExamState {
    fn default() -> Self {
        let today = Local::now().date_naive();
        Self {
            exams: vec![
                ExamTest::new(
                    "Calculos",
                    "P1 Integrals & Derivatives",
                    today + Duration::days(4),
                    4,
                    5,
                ),
                ExamTest::new(
                    "EDA",
                    "Midterm: Trees, Graphs & Sorting",
                    today + Duration::days(9),
                    3,
                    4,
                ),
            ],
            auto_boost_enabled: true,
        }
    }
}

impl ExamState {
    pub fn add_exam(&mut self, exam: ExamTest, cycle: &mut StudyCycleState) {
        let auto_points = exam.calculate_auto_points(Local::now().date_naive());
        let subject_name = exam.subject_name.clone();

        self.exams.push(exam);
        self.exams.sort_by_key(|e| e.exam_date);

        if self.auto_boost_enabled {
            cycle.apply_exam_boost(&subject_name, auto_points);
        }
    }

    pub fn remove_exam(&mut self, id: &str, cycle: &mut StudyCycleState) {
        if let Some(pos) = self.exams.iter().position(|e| e.id == id) {
            let removed = self.exams.remove(pos);
            cycle.remove_exam_boost(&removed.subject_name);
        }
    }

    /// Auto-balance entire cycle based on all current upcoming exams
    pub fn sync_all_exam_boosts(&self, cycle: &mut StudyCycleState) {
        let today = Local::now().date_naive();
        // Reset all boosts first
        cycle.clear_all_exam_boosts();

        if !self.auto_boost_enabled {
            return;
        }

        for exam in &self.exams {
            let boost = exam.calculate_auto_points(today);
            cycle.apply_exam_boost(&exam.subject_name, boost);
        }
    }
}
