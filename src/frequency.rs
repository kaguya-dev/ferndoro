use chrono::{Datelike, Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLog {
    pub minutes_studied: u32,
    pub pomodoros_completed: u32,
    pub habits_checked: HashSet<String>,
}

impl Default for DailyLog {
    fn default() -> Self {
        Self {
            minutes_studied: 0,
            pomodoros_completed: 0,
            habits_checked: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitItem {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyTrackerState {
    pub daily_goal_minutes: u32,
    pub logs: HashMap<NaiveDate, DailyLog>,
    pub habits: Vec<HabitItem>,
}

impl Default for FrequencyTrackerState {
    fn default() -> Self {
        let mut logs = HashMap::new();
        let today = Local::now().date_naive();
        
        // Populate sample past 14 days with some activity for immediate visual delight
        let sample_offsets = [0, 1, 2, 4, 5, 7, 8, 9, 11, 12, 13];
        let sample_minutes = [75, 120, 50, 90, 150, 45, 110, 60, 125, 90, 80];
        for (i, &offset) in sample_offsets.iter().enumerate() {
            let date = today - Duration::days(offset);
            let mut habits = HashSet::new();
            habits.insert("Deep Work Block".to_string());
            if offset % 2 == 0 {
                habits.insert("Review Notes".to_string());
            }
            logs.insert(
                date,
                DailyLog {
                    minutes_studied: sample_minutes[i % sample_minutes.len()],
                    pomodoros_completed: (sample_minutes[i % sample_minutes.len()] / 25).max(1),
                    habits_checked: habits,
                },
            );
        }

        Self {
            daily_goal_minutes: 120,
            logs,
            habits: vec![
                HabitItem {
                    id: "h1".into(),
                    name: "Deep Work Block".into(),
                    icon: "🔥".into(),
                },
                HabitItem {
                    id: "h2".into(),
                    name: "Problem Solving / Coding".into(),
                    icon: "💻".into(),
                },
                HabitItem {
                    id: "h3".into(),
                    name: "Review Notes & Flashcards".into(),
                    icon: "📚".into(),
                },
                HabitItem {
                    id: "h4".into(),
                    name: "Physical Stretch & Hydration".into(),
                    icon: "💧".into(),
                },
            ],
        }
    }
}

impl FrequencyTrackerState {
    pub fn today_log_mut(&mut self) -> &mut DailyLog {
        let today = Local::now().date_naive();
        self.logs.entry(today).or_default()
    }

    pub fn today_log(&self) -> DailyLog {
        let today = Local::now().date_naive();
        self.logs.get(&today).cloned().unwrap_or_default()
    }

    pub fn record_study_session(&mut self, minutes: u32, is_pomodoro: bool) {
        let log = self.today_log_mut();
        log.minutes_studied += minutes;
        if is_pomodoro {
            log.pomodoros_completed += 1;
        }
    }

    pub fn toggle_habit_today(&mut self, habit_name: &str) {
        let log = self.today_log_mut();
        if log.habits_checked.contains(habit_name) {
            log.habits_checked.remove(habit_name);
        } else {
            log.habits_checked.insert(habit_name.to_string());
        }
    }

    pub fn current_streak_days(&self) -> u32 {
        let today = Local::now().date_naive();
        let mut streak = 0;
        let mut check_date = today;

        // If today has activity, include it, otherwise check starting from yesterday
        let today_has_activity = self.logs.get(&today).map_or(false, |l| l.minutes_studied > 0);
        if !today_has_activity {
            check_date = today - Duration::days(1);
        }

        while let Some(log) = self.logs.get(&check_date) {
            if log.minutes_studied > 0 {
                streak += 1;
                check_date = check_date - Duration::days(1);
            } else {
                break;
            }
        }

        streak
    }

    pub fn best_streak_days(&self) -> u32 {
        if self.logs.is_empty() {
            return 0;
        }

        let mut sorted_dates: Vec<NaiveDate> = self
            .logs
            .iter()
            .filter(|(_, l)| l.minutes_studied > 0)
            .map(|(d, _)| *d)
            .collect();
        sorted_dates.sort();

        let mut max_streak = 0;
        let mut current = 0;
        let mut prev_date: Option<NaiveDate> = None;

        for date in sorted_dates {
            match prev_date {
                Some(prev) if date == prev + Duration::days(1) => {
                    current += 1;
                }
                _ => {
                    current = 1;
                }
            }
            prev_date = Some(date);
            if current > max_streak {
                max_streak = current;
            }
        }

        max_streak
    }

    pub fn total_minutes_all_time(&self) -> u32 {
        self.logs.values().map(|l| l.minutes_studied).sum()
    }

    pub fn total_active_days(&self) -> usize {
        self.logs.values().filter(|l| l.minutes_studied > 0).count()
    }

    #[allow(dead_code)]
    pub fn weekly_minutes(&self) -> u32 {
        let today = Local::now().date_naive();
        let start_of_week = today - Duration::days(today.weekday().num_days_from_monday() as i64);
        
        self.logs
            .iter()
            .filter(|(date, _)| **date >= start_of_week && **date <= today)
            .map(|(_, log)| log.minutes_studied)
            .sum()
    }

    pub fn get_heatmap_data(&self) -> HashMap<NaiveDate, u32> {
        self.logs
            .iter()
            .map(|(date, log)| (*date, log.minutes_studied))
            .collect()
    }

    pub fn today_goal_fraction(&self) -> f32 {
        if self.daily_goal_minutes == 0 {
            return 0.0;
        }
        let today = self.today_log();
        (today.minutes_studied as f32 / self.daily_goal_minutes as f32).clamp(0.0, 1.0)
    }
}
