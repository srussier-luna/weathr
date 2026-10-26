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
    #[serde(default)]
    pub notes_seen_on: Option<String>,
}

impl Progress {
    pub fn exists() -> bool {
        progress_path().is_some_and(|path| path.exists())
    }

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

    pub fn day_on(&self, today: NaiveDate) -> usize {
        let days = self
            .start_date()
            .map_or(0, |start| (today - start).num_days().max(0) as usize);
        days + 1
    }

    pub fn start_date(&self) -> Option<NaiveDate> {
        self.started_on
            .as_deref()
            .and_then(|date| NaiveDate::parse_from_str(date, DATE_FORMAT).ok())
    }

    pub fn notes_seen_date(&self) -> Option<NaiveDate> {
        self.notes_seen_on
            .as_deref()
            .and_then(|date| NaiveDate::parse_from_str(date, DATE_FORMAT).ok())
    }

    pub fn mark_notes_seen(&mut self, date: NaiveDate) {
        self.notes_seen_on = Some(date.format(DATE_FORMAT).to_string());
    }
}

fn progress_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("weathr").join("progress.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_without_notes_field_loads_with_no_seen_date() {
        let progress: Progress =
            serde_json::from_str(r#"{"started_on":"2026-09-01","letters_read":4}"#).unwrap();
        assert_eq!(progress.letters_read, 4);
        assert_eq!(progress.notes_seen_date(), None);
    }

    #[test]
    fn test_seen_notes_date_round_trips() {
        let mut progress = Progress::default();
        let date = NaiveDate::from_ymd_opt(2026, 10, 26).unwrap();
        progress.mark_notes_seen(date);
        let saved = serde_json::to_string(&progress).unwrap();
        let loaded: Progress = serde_json::from_str(&saved).unwrap();
        assert_eq!(loaded.notes_seen_date(), Some(date));
    }
}
