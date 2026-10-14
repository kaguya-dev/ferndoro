use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudySubject {
    pub id: String,
    pub name: String,
    pub target_minutes: u32,
    pub completed_minutes: u32,
    pub color_hex: String,
    pub topics: Vec<String>,
}

impl StudySubject {
    pub fn new(name: &str, target_minutes: u32, color_hex: &str) -> Self {
        Self {
            id: uuid_simple(),
            name: name.to_string(),
            target_minutes: target_minutes.max(5),
            completed_minutes: 0,
            color_hex: color_hex.to_string(),
            topics: Vec::new(),
        }
    }

    pub fn progress_fraction(&self) -> f32 {
        if self.target_minutes == 0 {
            return 0.0;
        }
        (self.completed_minutes as f32 / self.target_minutes as f32).clamp(0.0, 1.0)
    }

    pub fn is_completed(&self) -> bool {
        self.completed_minutes >= self.target_minutes
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    format!("{:x}-{:x}", now.as_secs(), now.subsec_nanos())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyCycleState {
    pub cycle_number: u32,
    pub subjects: Vec<StudySubject>,
    pub active_index: usize,
    pub total_cycles_completed: u32,
    pub total_minutes_all_cycles: u32,
}

impl Default for StudyCycleState {
    fn default() -> Self {
        Self {
            cycle_number: 1,
            subjects: vec![
                StudySubject::new("Rust Systems Programming", 60, "#A855F7"),
                StudySubject::new("Algorithms & Data Structures", 45, "#38BDF8"),
                StudySubject::new("Computer Architecture & OS", 50, "#34D399"),
                StudySubject::new("Mathematics / Foundations", 30, "#FBBF24"),
            ],
            active_index: 0,
            total_cycles_completed: 0,
            total_minutes_all_cycles: 0,
        }
    }
}

impl StudyCycleState {
    pub fn active_subject(&self) -> Option<&StudySubject> {
        self.subjects.get(self.active_index)
    }

    #[allow(dead_code)]
    pub fn active_subject_mut(&mut self) -> Option<&mut StudySubject> {
        self.subjects.get_mut(self.active_index)
    }

    pub fn total_target_minutes(&self) -> u32 {
        self.subjects.iter().map(|s| s.target_minutes).sum()
    }

    pub fn total_completed_minutes(&self) -> u32 {
        self.subjects.iter().map(|s| s.completed_minutes).sum()
    }

    pub fn cycle_progress_fraction(&self) -> f32 {
        let total = self.total_target_minutes();
        if total == 0 {
            return 0.0;
        }
        (self.total_completed_minutes() as f32 / total as f32).clamp(0.0, 1.0)
    }

    pub fn add_study_time(&mut self, subject_name: &str, minutes: u32) {
        self.total_minutes_all_cycles += minutes;
        
        let found = self.subjects.iter_mut().find(|s| s.name.eq_ignore_ascii_case(subject_name));
        if let Some(subject) = found {
            subject.completed_minutes += minutes;
        } else if let Some(active) = self.subjects.get_mut(self.active_index) {
            active.completed_minutes += minutes;
        }

        self.check_auto_advance();
    }

    pub fn check_auto_advance(&mut self) {
        if let Some(subject) = self.subjects.get(self.active_index) {
            if subject.is_completed() {
                // Advance to next uncompleted subject if any
                self.advance_to_next();
            }
        }
    }

    pub fn advance_to_next(&mut self) {
        if self.subjects.is_empty() {
            return;
        }

        // If all subjects completed, wrap cycle!
        let all_done = self.subjects.iter().all(|s| s.is_completed());
        if all_done {
            self.complete_current_cycle();
            return;
        }

        self.active_index = (self.active_index + 1) % self.subjects.len();
    }

    pub fn complete_current_cycle(&mut self) {
        self.cycle_number += 1;
        self.total_cycles_completed += 1;
        for sub in &mut self.subjects {
            sub.completed_minutes = 0;
        }
        self.active_index = 0;
    }

    pub fn move_up(&mut self, index: usize) {
        if index > 0 && index < self.subjects.len() {
            self.subjects.swap(index, index - 1);
            if self.active_index == index {
                self.active_index = index - 1;
            } else if self.active_index == index - 1 {
                self.active_index = index;
            }
        }
    }

    pub fn move_down(&mut self, index: usize) {
        if index + 1 < self.subjects.len() {
            self.subjects.swap(index, index + 1);
            if self.active_index == index {
                self.active_index = index + 1;
            } else if self.active_index == index + 1 {
                self.active_index = index;
            }
        }
    }

    pub fn remove_subject(&mut self, index: usize) {
        if index < self.subjects.len() {
            self.subjects.remove(index);
            if self.active_index >= self.subjects.len() && !self.subjects.is_empty() {
                self.active_index = self.subjects.len() - 1;
            }
        }
    }

    pub fn add_subject(&mut self, name: &str, target_minutes: u32, color_hex: &str) {
        if !name.trim().is_empty() {
            self.subjects.push(StudySubject::new(name.trim(), target_minutes, color_hex));
        }
    }
}
