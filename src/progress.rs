use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Serialize, Deserialize, Default)]
pub struct Progress {
    #[serde(default)]
    pub started_on: Option<String>,
    #[serde(default)]
    pub letters_read: usize,
    #[serde(default)]
    pub badges: BTreeSet<String>,
    #[serde(default)]
    pub answers: BTreeMap<String, u8>,
    #[serde(default)]
    pub last_seen_on: Option<String>,
    #[serde(default)]
    pub streak: u32,
    #[serde(default)]
    pub riddles_solved: BTreeSet<usize>,
    #[serde(default)]
    pub found_objects: Vec<usize>,
    #[serde(default)]
    pub bottles: Vec<usize>,
    #[serde(default)]
    pub postcard_weather: BTreeMap<usize, String>,
    #[serde(default)]
    pub gazette_weather: BTreeMap<usize, String>,
}

impl Progress {
    pub fn load() -> Self {
        let mut progress: Self = progress_path()
            .and_then(|path| fs::read_to_string(path).ok())
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default();

        if progress.start_date().is_none() {
            progress.started_on = Some(Local::now().format(DATE_FORMAT).to_string());
            progress.save();
        }
        progress
    }

    pub fn save(&self) {
        let Some(path) = progress_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, content);
        }
    }

    pub fn record_visit(&mut self) -> bool {
        let today = Local::now().date_naive();
        let last_seen = self
            .last_seen_on
            .as_deref()
            .and_then(|date| NaiveDate::parse_from_str(date, DATE_FORMAT).ok());
        if last_seen == Some(today) {
            return false;
        }

        self.streak = match last_seen {
            Some(day) if today.pred_opt() == Some(day) => self.streak + 1,
            _ => 1,
        };
        self.last_seen_on = Some(today.format(DATE_FORMAT).to_string());
        true
    }

    pub fn start_date(&self) -> Option<NaiveDate> {
        self.started_on
            .as_deref()
            .and_then(|date| NaiveDate::parse_from_str(date, DATE_FORMAT).ok())
    }
}

fn progress_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("weathr").join("progress.json"))
}
