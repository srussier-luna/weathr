use chrono::{Local, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub const DATE_FORMAT: &str = "%Y-%m-%d";
const BACKUP_SUFFIX: &str = ".abime-%Y-%m-%d-%H%M";
const TMP_SUFFIX: &str = ".tmp";

fn delivered_recipes<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<BTreeMap<usize, usize>, D::Error> {
    let recipes = BTreeMap::<usize, Option<usize>>::deserialize(deserializer)?;
    Ok(recipes
        .into_iter()
        .filter_map(|(day, recipe)| Some((day, recipe?)))
        .collect())
}

fn parse_date(date: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date, DATE_FORMAT).ok()
}

#[derive(Serialize, Deserialize)]
struct MailHourChange {
    on: String,
    previous_hour: u32,
}

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
    #[serde(default)]
    pub welcome_seen: bool,
    #[serde(default)]
    pub paused_days: BTreeMap<usize, u32>,
    #[serde(default)]
    pub pause_checked_on: Option<String>,
    #[serde(default)]
    pub postman_note_day: Option<usize>,
    #[serde(default)]
    mail_hour_change: Option<MailHourChange>,
    #[serde(default)]
    pub diet_letter_day: Option<usize>,
    #[serde(default)]
    pub diet_answered_day: Option<usize>,
    #[serde(default)]
    pub diets: BTreeSet<String>,
    #[serde(default, deserialize_with = "delivered_recipes")]
    pub diet_recipes: BTreeMap<usize, usize>,
    #[serde(default)]
    pub diet_recipes_until: Option<usize>,
    #[serde(skip)]
    in_memory: bool,
}

impl Progress {
    pub fn exists() -> bool {
        progress_path().is_some_and(|path| path.exists())
    }

    pub fn load() -> (Self, Option<PathBuf>) {
        let (mut progress, backup) = match progress_path() {
            Some(path) => Self::load_from_path(&path, Local::now().naive_local()),
            None => (Self::default(), None),
        };

        if progress.start_date().is_none() {
            progress.started_on = Some(Local::now().format(DATE_FORMAT).to_string());
            progress.save();
        }
        (progress, backup)
    }

    fn load_from_path(path: &Path, now: NaiveDateTime) -> (Self, Option<PathBuf>) {
        let content = match fs::read_to_string(path) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return (Self::default(), None);
            }
            content => content.ok(),
        };
        if let Some(progress) = content.and_then(|content| serde_json::from_str(&content).ok()) {
            return (progress, None);
        }

        let backup = free_backup_path(path, now);
        match fs::rename(path, &backup) {
            Ok(()) => (Self::default(), Some(backup)),
            Err(_) => (
                Self {
                    in_memory: true,
                    ..Self::default()
                },
                None,
            ),
        }
    }

    pub fn in_memory(start: NaiveDate) -> Self {
        Self {
            started_on: Some(start.format(DATE_FORMAT).to_string()),
            in_memory: true,
            ..Self::default()
        }
    }

    pub fn save(&self) {
        let _ = self.try_save();
    }

    pub fn try_save(&self) -> io::Result<()> {
        let path = progress_path()
            .filter(|_| !self.in_memory)
            .ok_or(io::ErrorKind::Unsupported)?;
        self.save_to_path(&path)
    }

    fn save_to_path(&self, path: &Path) -> io::Result<()> {
        write_creating_dir(path, &serde_json::to_string_pretty(self)?)
    }

    pub fn record_visit(&mut self) -> bool {
        let today = Local::now().date_naive();
        let last_seen = self.last_seen_on.as_deref().and_then(parse_date);
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
        self.started_on.as_deref().and_then(parse_date)
    }

    pub fn notes_seen_date(&self) -> Option<NaiveDate> {
        self.notes_seen_on.as_deref().and_then(parse_date)
    }

    pub fn mark_notes_seen(&mut self, date: NaiveDate) {
        self.notes_seen_on = Some(date.format(DATE_FORMAT).to_string());
    }

    pub fn pause_checked_date(&self) -> Option<NaiveDate> {
        self.pause_checked_on.as_deref().and_then(parse_date)
    }

    pub fn mark_pause_checked(&mut self, date: NaiveDate) {
        self.pause_checked_on = Some(date.format(DATE_FORMAT).to_string());
    }

    pub fn previous_mail_hour(&self) -> Option<(NaiveDate, u32)> {
        let change = self.mail_hour_change.as_ref()?;
        Some((parse_date(&change.on)?, change.previous_hour))
    }

    pub fn set_previous_mail_hour(&mut self, previous: Option<(NaiveDate, u32)>) {
        self.mail_hour_change = previous.map(|(until, previous_hour)| MailHourChange {
            on: until.format(DATE_FORMAT).to_string(),
            previous_hour,
        });
    }
}

pub(crate) fn progress_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("weathr").join("progress.json"))
}

pub(crate) fn write_creating_dir(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = with_suffix(path, TMP_SUFFIX);
    let saved = write_synced(&tmp, content.as_bytes()).and_then(|()| fs::rename(&tmp, path));
    if saved.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    saved
}

fn backup_path(path: &Path, now: NaiveDateTime) -> PathBuf {
    with_suffix(path, &now.format(BACKUP_SUFFIX).to_string())
}

fn free_backup_path(path: &Path, now: NaiveDateTime) -> PathBuf {
    let base = backup_path(path, now);
    let free = std::iter::once(base.clone())
        .chain((2..).map(|copy| with_suffix(&base, &format!("-{copy}"))))
        .find(|candidate| !candidate.exists());
    free.unwrap_or(base)
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(suffix);
    name.into()
}

fn write_synced(path: &Path, content: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)?;
    file.sync_all()
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
    fn test_save_without_welcome_field_has_not_seen_the_welcome() {
        let progress: Progress =
            serde_json::from_str(r#"{"started_on":"2026-09-01","letters_read":40}"#).unwrap();
        assert!(!progress.welcome_seen);
    }

    #[test]
    fn test_save_without_pause_fields_loads_with_no_pause() {
        let progress: Progress =
            serde_json::from_str(r#"{"started_on":"2026-09-01","letters_read":37}"#).unwrap();
        assert!(progress.paused_days.is_empty());
        assert_eq!(progress.pause_checked_date(), None);
        assert_eq!(progress.postman_note_day, None);
    }

    #[test]
    fn test_save_without_mail_hour_change_loads_with_no_previous_hour() {
        let progress: Progress =
            serde_json::from_str(r#"{"started_on":"2026-09-01","letters_read":12}"#).unwrap();
        assert_eq!(progress.previous_mail_hour(), None);
    }

    #[test]
    fn test_save_without_diet_fields_has_no_diet_letter_yet() {
        let progress: Progress =
            serde_json::from_str(r#"{"started_on":"2026-09-01","letters_read":29}"#).unwrap();
        assert_eq!(progress.diet_letter_day, None);
        assert_eq!(progress.diet_answered_day, None);
        assert!(progress.diets.is_empty());
        assert!(progress.diet_recipes.is_empty());
        assert_eq!(progress.diet_recipes_until, None);
    }

    #[test]
    fn test_save_with_empty_recipe_slots_still_loads() {
        let progress: Progress = serde_json::from_str(
            r#"{"started_on":"2026-09-01","letters_read":31,"diet_answered_day":10,"diets":["vegan"],"diet_recipes":{"11":2,"21":null}}"#,
        )
        .unwrap();
        assert_eq!(progress.letters_read, 31);
        assert_eq!(progress.start_date(), NaiveDate::from_ymd_opt(2026, 9, 1));
        assert_eq!(progress.diet_answered_day, Some(10));
        assert_eq!(progress.diets, BTreeSet::from(["vegan".to_string()]));
        assert_eq!(progress.diet_recipes, BTreeMap::from([(11, 2)]));
    }

    #[test]
    fn test_diet_answer_and_delivered_recipes_round_trip() {
        let progress = Progress {
            diet_letter_day: Some(30),
            diet_answered_day: Some(31),
            diets: BTreeSet::from(["vegetarian".to_string(), "no_pork".to_string()]),
            diet_recipes: BTreeMap::from([(32, 2)]),
            diet_recipes_until: Some(42),
            ..Progress::default()
        };
        let saved = serde_json::to_string(&progress).unwrap();
        let loaded: Progress = serde_json::from_str(&saved).unwrap();
        assert_eq!(loaded.diet_letter_day, Some(30));
        assert_eq!(loaded.diet_answered_day, Some(31));
        assert_eq!(loaded.diets, progress.diets);
        assert_eq!(loaded.diet_recipes, progress.diet_recipes);
        assert_eq!(loaded.diet_recipes_until, Some(42));
    }

    #[test]
    fn test_previous_mail_hour_round_trips() {
        let mut progress = Progress::default();
        let until = NaiveDate::from_ymd_opt(2026, 10, 9).unwrap();
        progress.set_previous_mail_hour(Some((until, 9)));
        let saved = serde_json::to_string(&progress).unwrap();
        let mut loaded: Progress = serde_json::from_str(&saved).unwrap();
        assert_eq!(loaded.previous_mail_hour(), Some((until, 9)));

        loaded.set_previous_mail_hour(None);
        assert_eq!(loaded.previous_mail_hour(), None);
    }

    #[test]
    fn test_pauses_round_trip() {
        let mut progress = Progress::default();
        let date = NaiveDate::from_ymd_opt(2026, 10, 22).unwrap();
        progress.paused_days.insert(7, 15);
        progress.mark_pause_checked(date);
        let saved = serde_json::to_string(&progress).unwrap();
        let loaded: Progress = serde_json::from_str(&saved).unwrap();
        assert_eq!(loaded.paused_days, BTreeMap::from([(7, 15)]));
        assert_eq!(loaded.pause_checked_date(), Some(date));
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

    #[test]
    fn test_in_memory_progress_starts_fresh() {
        let start = NaiveDate::from_ymd_opt(2026, 10, 19).unwrap();
        let progress = Progress::in_memory(start);
        assert_eq!(progress.start_date(), Some(start));
        assert_eq!(progress.letters_read, 0);
        assert!(progress.answers.is_empty());
        assert!(progress.badges.is_empty());
        assert!(progress.in_memory);
    }

    #[test]
    fn test_in_memory_progress_is_never_saved() {
        let progress = Progress::in_memory(NaiveDate::from_ymd_opt(2026, 10, 19).unwrap());
        assert!(progress.try_save().is_err());
    }

    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "weathr_test_progress_{name}_{}",
            std::process::id()
        ));
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn at(hour: u32, minute: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 11, 7)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    }

    #[test]
    fn test_backup_name_has_the_date_and_the_minute() {
        assert_eq!(
            backup_path(Path::new("/weathr/progress.json"), at(9, 5)),
            Path::new("/weathr/progress.json.abime-2026-11-07-0905")
        );
    }

    #[test]
    fn test_unreadable_save_is_kept_byte_for_byte_and_a_new_game_starts() {
        for (name, content) in [
            ("unparsable", b"{oops".as_slice()),
            ("not_utf8", [0xff, 0xfe].as_slice()),
        ] {
            let dir = scratch_dir(name);
            let path = dir.join("progress.json");
            fs::write(&path, content).unwrap();

            let (progress, backup) = Progress::load_from_path(&path, at(14, 32));
            let kept = backup.as_deref().map(fs::read);
            let was_left = path.exists();
            progress.save_to_path(&path).unwrap();
            let (_, next_backup) = Progress::load_from_path(&path, at(14, 33));
            fs::remove_dir_all(&dir).ok();

            assert_eq!(
                backup,
                Some(dir.join("progress.json.abime-2026-11-07-1432"))
            );
            assert_eq!(kept.unwrap().unwrap(), content);
            assert!(!was_left);
            assert_eq!(progress.letters_read, 0);
            assert_eq!(progress.start_date(), None);
            assert_eq!(next_backup, None);
        }
    }

    #[test]
    fn test_unreadable_saves_in_the_same_minute_keep_every_copy() {
        let dir = scratch_dir("same_minute");
        let path = dir.join("progress.json");
        let mut backups = Vec::new();
        for content in ["{first", "{second", "{third"] {
            fs::write(&path, content).unwrap();
            let (_, backup) = Progress::load_from_path(&path, at(14, 32));
            let backup = backup.unwrap();
            backups.push((backup.clone(), fs::read_to_string(&backup).unwrap()));
        }
        fs::remove_dir_all(&dir).ok();

        let stamp = "progress.json.abime-2026-11-07-1432";
        assert_eq!(
            backups,
            [
                (dir.join(stamp), "{first".to_string()),
                (dir.join(format!("{stamp}-2")), "{second".to_string()),
                (dir.join(format!("{stamp}-3")), "{third".to_string()),
            ]
        );
    }

    #[test]
    fn test_missing_save_is_a_fresh_install_without_backup() {
        let dir = scratch_dir("missing");
        let (progress, backup) = Progress::load_from_path(&dir.join("progress.json"), at(9, 0));
        let files = fs::read_dir(&dir).unwrap().count();
        fs::remove_dir_all(&dir).ok();
        assert_eq!(backup, None);
        assert_eq!(files, 0);
        assert_eq!(progress.letters_read, 0);
    }

    #[test]
    fn test_readable_save_loads_in_place() {
        let dir = scratch_dir("readable");
        let path = dir.join("progress.json");
        fs::write(&path, r#"{"started_on":"2026-09-01","letters_read":4}"#).unwrap();
        let (progress, backup) = Progress::load_from_path(&path, at(9, 0));
        let files = fs::read_dir(&dir).unwrap().count();
        fs::remove_dir_all(&dir).ok();
        assert_eq!(backup, None);
        assert_eq!(files, 1);
        assert_eq!(progress.letters_read, 4);
    }

    #[test]
    fn test_save_replaces_the_whole_file_and_leaves_no_tmp() {
        let dir = scratch_dir("save");
        let path = dir.join("weathr").join("progress.json");
        let mut progress = Progress {
            answers: (0..50).map(|day| (day.to_string(), 1)).collect(),
            ..Progress::default()
        };
        progress.save_to_path(&path).unwrap();
        progress.answers.clear();
        progress.letters_read = 3;
        progress.save_to_path(&path).unwrap();

        let saved = fs::read_to_string(&path).unwrap();
        let tmp_left = with_suffix(&path, TMP_SUFFIX).exists();
        fs::remove_dir_all(&dir).ok();
        assert!(!tmp_left);
        let loaded: Progress = serde_json::from_str(&saved).unwrap();
        assert_eq!(loaded.letters_read, 3);
        assert!(loaded.answers.is_empty());
    }
}
