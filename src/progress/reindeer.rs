use super::{DATE_FORMAT, Progress, parse_date};
use chrono::{Datelike, NaiveDate};

const DEFAULT_REINDEER_NAME: &str = "Rudolf";

fn is_christmas_day(date: NaiveDate) -> bool {
    date.month() == 12 && date.day() == 25
}

impl Progress {
    pub fn is_santa_due(&self, today: NaiveDate) -> bool {
        is_christmas_day(today) && self.reindeer_arrived_on.is_none()
    }

    pub fn welcome_reindeer(&mut self, today: NaiveDate) {
        self.reindeer_arrived_on = Some(today.format(DATE_FORMAT).to_string());
    }

    pub fn has_reindeer(&self) -> bool {
        self.reindeer_arrived_on.is_some()
    }

    pub fn awaits_reindeer_name(&self, today: NaiveDate) -> bool {
        self.reindeer_name.is_none()
            && self.reindeer_arrived_on.as_deref().and_then(parse_date) == Some(today)
    }

    pub fn name_reindeer(&mut self, draft: &str) {
        let name = draft.trim();
        let name = if name.is_empty() {
            DEFAULT_REINDEER_NAME
        } else {
            name
        };
        self.reindeer_name = Some(name.to_string());
    }

    pub fn reindeer_name(&self) -> Option<&str> {
        self.has_reindeer().then(|| {
            self.reindeer_name
                .as_deref()
                .unwrap_or(DEFAULT_REINDEER_NAME)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, month, day).unwrap()
    }

    #[test]
    fn test_santa_is_due_on_christmas_day_only() {
        let progress = Progress::default();
        assert!(progress.is_santa_due(date(12, 25)));
        assert!(!progress.is_santa_due(date(12, 24)));
        assert!(!progress.is_santa_due(date(12, 26)));
        assert!(!progress.is_santa_due(date(1, 25)));
    }

    #[test]
    fn test_santa_passes_once() {
        let mut progress = Progress::default();
        progress.welcome_reindeer(date(12, 25));
        assert!(!progress.is_santa_due(date(12, 25)));
        assert!(!progress.is_santa_due(NaiveDate::from_ymd_opt(2027, 12, 25).unwrap()));
    }

    #[test]
    fn test_no_reindeer_without_a_christmas_visit() {
        let progress = Progress::default();
        assert!(!progress.has_reindeer());
        assert_eq!(progress.reindeer_name(), None);
        assert!(!progress.awaits_reindeer_name(date(12, 26)));
    }

    #[test]
    fn test_empty_name_is_rudolf() {
        let mut progress = Progress::default();
        progress.welcome_reindeer(date(12, 25));
        progress.name_reindeer("   ");
        assert_eq!(progress.reindeer_name(), Some("Rudolf"));
    }

    #[test]
    fn test_typed_name_is_kept_trimmed() {
        let mut progress = Progress::default();
        progress.welcome_reindeer(date(12, 25));
        progress.name_reindeer("  Comète ");
        assert_eq!(progress.reindeer_name(), Some("Comète"));
        assert!(!progress.awaits_reindeer_name(date(12, 25)));
    }

    #[test]
    fn test_unnamed_reindeer_asks_again_the_same_day_then_is_rudolf() {
        let mut progress = Progress::default();
        progress.welcome_reindeer(date(12, 25));
        assert!(progress.awaits_reindeer_name(date(12, 25)));
        assert!(!progress.awaits_reindeer_name(date(12, 26)));
        assert_eq!(progress.reindeer_name(), Some("Rudolf"));
    }

    #[test]
    fn test_old_save_has_no_reindeer() {
        let progress: Progress =
            serde_json::from_str(r#"{"started_on":"2026-09-01","letters_read":4}"#).unwrap();
        assert!(!progress.has_reindeer());
        assert!(progress.is_santa_due(date(12, 25)));
    }

    #[test]
    fn test_reindeer_survives_a_restart() {
        let mut progress = Progress::default();
        progress.welcome_reindeer(date(12, 25));
        progress.name_reindeer("Comète");
        let saved = serde_json::to_string(&progress).unwrap();
        let loaded: Progress = serde_json::from_str(&saved).unwrap();
        assert_eq!(loaded.reindeer_name(), Some("Comète"));
        assert!(!loaded.is_santa_due(date(12, 25)));
    }
}
