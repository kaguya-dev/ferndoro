use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudySubject {
    pub id: String,
    pub name: String,
    pub base_minutes: u32,
    pub boost_minutes: u32, // Auto-added from upcoming tests/exams
    pub completed_minutes: u32,
    pub color_hex: String,
    pub topics: Vec<String>,
}

impl StudySubject {
    pub fn new(name: &str, base_minutes: u32, color_hex: &str) -> Self {
        Self {
            id: uuid_simple(),
            name: name.trim().to_string(),
            base_minutes: base_minutes.max(5),
            boost_minutes: 0,
            completed_minutes: 0,
            color_hex: color_hex.to_string(),
            topics: Vec::new(),
        }
    }

    pub fn target_minutes(&self) -> u32 {
        self.base_minutes + self.boost_minutes
    }

    pub fn set_base_minutes(&mut self, mins: u32) {
        self.base_minutes = mins.clamp(5, 360);
    }

    pub fn adjust_base_minutes(&mut self, delta: i32) {
        let new_val = (self.base_minutes as i32 + delta).clamp(5, 360);
        self.base_minutes = new_val as u32;
    }

    pub fn progress_fraction(&self) -> f32 {
        let target = self.target_minutes();
        if target == 0 {
            return 0.0;
        }
        (self.completed_minutes as f32 / target as f32).clamp(0.0, 1.0)
    }

    pub fn is_completed(&self) -> bool {
        self.completed_minutes >= self.target_minutes()
    }

    pub fn format_time(mins: u32) -> String {
        let h = mins / 60;
        let m = mins % 60;
        if h > 0 && m > 0 {
            format!("{}h {}m", h, m)
        } else if h > 0 {
            format!("{}h", h)
        } else {
            format!("{}m", m)
        }
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    format!("{:x}-{:x}", now.as_secs(), now.subsec_nanos())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclePreset {
    pub name: String,
    pub description: String,
    pub subjects: Vec<StudySubject>,
}

impl CyclePreset {
    pub fn builtins() -> Vec<CyclePreset> {
        vec![
            CyclePreset {
                name: "Computer Science (Core)".to_string(),
                description: "Calculus, Data Structures, Architecture, and Discrete Math for CS/Software undergrads".to_string(),
                subjects: vec![
                    StudySubject::new("Calculos", 60, "#A855F7"),
                    StudySubject::new("EDA (Data Structures)", 60, "#38BDF8"),
                    StudySubject::new("Computer Architecture & OS", 45, "#34D399"),
                    StudySubject::new("Linear Algebra & Discrete", 45, "#FBBF24"),
                ],
            },
            CyclePreset {
                name: "STEM & Engineering Heavy".to_string(),
                description: "Deep analytical blocks for engineering problem sets and mathematical theory".to_string(),
                subjects: vec![
                    StudySubject::new("Calculus I / II", 75, "#A855F7"),
                    StudySubject::new("General Physics & Mechanics", 60, "#38BDF8"),
                    StudySubject::new("Differential Equations", 60, "#F43F5E"),
                    StudySubject::new("Linear Algebra", 50, "#FBBF24"),
                ],
            },
            CyclePreset {
                name: "Balanced Semester (4x45m)".to_string(),
                description: "Standard university rhythm: 45-minute balanced intervals across 4 active subjects".to_string(),
                subjects: vec![
                    StudySubject::new("Major Theory Subject", 45, "#A855F7"),
                    StudySubject::new("Major Practical / Lab", 45, "#38BDF8"),
                    StudySubject::new("Secondary Course 1", 45, "#34D399"),
                    StudySubject::new("Secondary Course 2", 45, "#FBBF24"),
                ],
            },
            CyclePreset {
                name: "Exam Rush / Finals Sprint".to_string(),
                description: "High-intensity priority sprint focusing on your 2 hardest upcoming exams".to_string(),
                subjects: vec![
                    StudySubject::new("Priority 1 Hardest Exam", 90, "#EF4444"),
                    StudySubject::new("Priority 2 Exam", 75, "#F97316"),
                    StudySubject::new("Rapid Review & Exercises", 30, "#A855F7"),
                ],
            },
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyCycleState {
    pub cycle_number: u32,
    pub subjects: Vec<StudySubject>,
    pub active_index: usize,
    pub total_cycles_completed: u32,
    pub total_minutes_all_cycles: u32,
    pub custom_presets: Vec<CyclePreset>,
}

impl Default for StudyCycleState {
    fn default() -> Self {
        Self {
            cycle_number: 1,
            subjects: vec![
                StudySubject::new("Calculos", 50, "#A855F7"),
                StudySubject::new("EDA", 50, "#38BDF8"),
                StudySubject::new("Linear Algebra", 45, "#34D399"),
                StudySubject::new("Physics / Lab", 45, "#FBBF24"),
            ],
            active_index: 0,
            total_cycles_completed: 0,
            total_minutes_all_cycles: 0,
            custom_presets: Vec::new(),
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
        self.subjects.iter().map(|s| s.target_minutes()).sum()
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
                self.advance_to_next();
            }
        }
    }

    pub fn advance_to_next(&mut self) {
        if self.subjects.is_empty() {
            return;
        }

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

    pub fn add_subject(&mut self, name: &str, base_minutes: u32, color_hex: &str) {
        if !name.trim().is_empty() {
            self.subjects.push(StudySubject::new(name.trim(), base_minutes, color_hex));
        }
    }

    pub fn apply_preset(&mut self, preset: &CyclePreset) {
        self.subjects = preset.subjects.clone();
        self.active_index = 0;
    }

    pub fn save_current_as_preset(&mut self, name: &str, description: &str) {
        if !name.trim().is_empty() {
            self.custom_presets.push(CyclePreset {
                name: name.trim().to_string(),
                description: description.trim().to_string(),
                subjects: self.subjects.clone(),
            });
        }
    }

    pub fn apply_exam_boost(&mut self, subject_name: &str, boost_minutes: u32) {
        if let Some(sub) = self.subjects.iter_mut().find(|s| s.name.eq_ignore_ascii_case(subject_name)) {
            sub.boost_minutes = boost_minutes;
        } else {
            // If subject not in cycle, automatically add it with boost!
            let mut new_sub = StudySubject::new(subject_name, 45, "#A855F7");
            new_sub.boost_minutes = boost_minutes;
            self.subjects.push(new_sub);
        }
    }

    pub fn remove_exam_boost(&mut self, subject_name: &str) {
        if let Some(sub) = self.subjects.iter_mut().find(|s| s.name.eq_ignore_ascii_case(subject_name)) {
            sub.boost_minutes = 0;
        }
    }

    pub fn clear_all_exam_boosts(&mut self) {
        for sub in &mut self.subjects {
            sub.boost_minutes = 0;
        }
    }
}
