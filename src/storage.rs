use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::theme::ThemeMode;
use crate::pomodoro::PomodoroState;
use crate::cycle::StudyCycleState;
use crate::frequency::FrequencyTrackerState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub theme: ThemeMode,
    pub pomodoro: PomodoroState,
    pub cycle: StudyCycleState,
    pub frequency: FrequencyTrackerState,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            theme: ThemeMode::PurpleDark,
            pomodoro: PomodoroState::default(),
            cycle: StudyCycleState::default(),
            frequency: FrequencyTrackerState::default(),
        }
    }
}

pub struct Storage;

impl Storage {
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("ferndoro").join("ferndoro_state.json"))
    }

    pub fn load() -> AppData {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<AppData>(&content) {
                        return data;
                    }
                }
            }
        }
        AppData::default()
    }

    pub fn save(data: &AppData) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(data) {
                let _ = fs::write(path, json);
            }
        }
    }
}
