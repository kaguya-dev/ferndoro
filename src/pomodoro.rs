use serde::{Deserialize, Serialize};
use crate::notification::Notifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak,
}

impl PomodoroPhase {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Work => "FOCUS SESSION",
            Self::ShortBreak => "SHORT BREAK",
            Self::LongBreak => "LONG BREAK",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerStatus {
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PomodoroConfig {
    pub work_mins: u32,
    pub short_break_mins: u32,
    pub long_break_mins: u32,
    pub sessions_before_long_break: u32,
    pub auto_advance: bool,
}

impl Default for PomodoroConfig {
    fn default() -> Self {
        Self {
            work_mins: 25,
            short_break_mins: 5,
            long_break_mins: 15,
            sessions_before_long_break: 4,
            auto_advance: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PomodoroState {
    pub config: PomodoroConfig,
    pub phase: PomodoroPhase,
    pub status: TimerStatus,
    pub remaining_seconds: f32,
    pub current_cycle_count: u32,
    pub total_completed_sessions: u32,
    pub active_subject_name: String,
}

impl Default for PomodoroState {
    fn default() -> Self {
        let config = PomodoroConfig::default();
        let initial_seconds = (config.work_mins * 60) as f32;
        Self {
            config,
            phase: PomodoroPhase::Work,
            status: TimerStatus::Stopped,
            remaining_seconds: initial_seconds,
            current_cycle_count: 0,
            total_completed_sessions: 0,
            active_subject_name: "General Study".to_string(),
        }
    }
}

pub struct SessionCompletedEvent {
    pub phase: PomodoroPhase,
    pub duration_minutes: u32,
    pub subject_name: String,
}

impl PomodoroState {
    pub fn total_phase_seconds(&self) -> f32 {
        let mins = match self.phase {
            PomodoroPhase::Work => self.config.work_mins,
            PomodoroPhase::ShortBreak => self.config.short_break_mins,
            PomodoroPhase::LongBreak => self.config.long_break_mins,
        };
        (mins * 60) as f32
    }

    pub fn progress_fraction(&self) -> f32 {
        let total = self.total_phase_seconds();
        if total <= 0.0 {
            return 0.0;
        }
        let elapsed = total - self.remaining_seconds;
        (elapsed / total).clamp(0.0, 1.0)
    }

    pub fn formatted_time(&self) -> String {
        let total_secs = self.remaining_seconds.ceil().max(0.0) as u32;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    pub fn start(&mut self) {
        if self.status == TimerStatus::Stopped {
            self.reset_phase_time();
        }
        self.status = TimerStatus::Running;
    }

    pub fn pause(&mut self) {
        self.status = TimerStatus::Paused;
    }

    pub fn reset(&mut self) {
        self.status = TimerStatus::Stopped;
        self.reset_phase_time();
    }

    pub fn reset_phase_time(&mut self) {
        self.remaining_seconds = self.total_phase_seconds();
    }

    pub fn skip(&mut self) -> Option<SessionCompletedEvent> {
        self.advance_phase(false)
    }

    pub fn advance_phase(&mut self, completed_naturally: bool) -> Option<SessionCompletedEvent> {
        let finished_phase = self.phase;
        let mut completed_event = None;

        if completed_naturally && finished_phase == PomodoroPhase::Work {
            self.current_cycle_count += 1;
            self.total_completed_sessions += 1;
            completed_event = Some(SessionCompletedEvent {
                phase: finished_phase,
                duration_minutes: self.config.work_mins,
                subject_name: self.active_subject_name.clone(),
            });
        }

        // Determine next phase
        match finished_phase {
            PomodoroPhase::Work => {
                if self.current_cycle_count >= self.config.sessions_before_long_break {
                    self.phase = PomodoroPhase::LongBreak;
                    self.current_cycle_count = 0;
                } else {
                    self.phase = PomodoroPhase::ShortBreak;
                }
            }
            PomodoroPhase::ShortBreak | PomodoroPhase::LongBreak => {
                self.phase = PomodoroPhase::Work;
            }
        }

        self.reset_phase_time();

        if self.config.auto_advance && completed_naturally {
            self.status = TimerStatus::Running;
        } else {
            self.status = TimerStatus::Stopped;
        }

        completed_event
    }

    pub fn tick(&mut self, dt: f32) -> Option<SessionCompletedEvent> {
        if self.status != TimerStatus::Running {
            return None;
        }

        self.remaining_seconds -= dt;

        if self.remaining_seconds <= 0.0 {
            self.remaining_seconds = 0.0;
            
            // Notify user
            match self.phase {
                PomodoroPhase::Work => {
                    Notifier::notify(
                        "FernDoro - Focus Complete!",
                        &format!("Great job! You finished a focus session on '{}'. Time for a break!", self.active_subject_name),
                    );
                }
                PomodoroPhase::ShortBreak | PomodoroPhase::LongBreak => {
                    Notifier::notify(
                        "FernDoro - Break Over!",
                        "Ready to dive back into deep work? Let's start the next session!",
                    );
                }
            }

            return self.advance_phase(true);
        }

        None
    }
}
