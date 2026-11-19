use crate::config::{Config, Mail};
use crate::progress::{Progress, pack, progress_path, sync_path, unpack};
use crate::story::content::letters;
use crate::story::reach::settle_on_launch;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const PREFIX: &str = "[weathr]";
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Paths {
    progress: PathBuf,
    config: PathBuf,
    sync: Option<PathBuf>,
}

impl Paths {
    fn local() -> io::Result<Self> {
        Ok(Self {
            progress: progress_path().ok_or(io::ErrorKind::NotFound)?,
            config: Config::get_config_path().map_err(io::Error::other)?,
            sync: sync_path(),
        })
    }

    fn settle(&self, progress: &mut Progress, out_of_sync: bool, village_count: usize) -> bool {
        let mail_hour = Config::load_from_path(&self.config)
            .map_or(Mail::default().hour, |config| config.mail.hour);
        settle_on_launch(progress, out_of_sync, mail_hour, village_count)
    }

    fn save(&self, progress: &Progress) -> io::Result<()> {
        progress.save_to_path(&self.progress, self.sync.as_deref())
    }
}

#[derive(Serialize, Deserialize)]
struct Backup {
    version: String,
    progress: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    config: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sync: Option<String>,
}

impl Backup {
    fn collect(paths: &Paths, village_count: usize) -> io::Result<Self> {
        let _: Progress = serde_json::from_slice(&fs::read(&paths.progress)?)?;
        let (mut progress, opening) = Progress::load_at(&paths.progress, paths.sync.as_deref());
        if paths.settle(&mut progress, opening.out_of_sync, village_count) {
            paths.save(&progress)?;
        }
        let content = fs::read(&paths.progress)?;
        Ok(Self {
            version: VERSION.to_string(),
            progress: serde_json::from_slice(&content)?,
            config: read_if_present(&paths.config)?,
            sync: Some(pack(&content)),
        })
    }

    fn read(file: &Path) -> io::Result<(Self, Progress, bool)> {
        let backup: Self = serde_json::from_str(&fs::read_to_string(file)?)?;
        let (progress, out_of_sync) = backup.checked()?;
        Ok((backup, progress, out_of_sync))
    }

    fn write(&self, file: &Path) -> io::Result<()> {
        fs::write(file, serde_json::to_string_pretty(self)?)
    }

    fn other_version(&self) -> Option<&str> {
        Some(self.version.as_str()).filter(|version| *version != VERSION)
    }

    fn checked(&self) -> io::Result<(Progress, bool)> {
        let kept = self.sync.as_deref().and_then(unpack).and_then(|content| {
            let progress: Progress = serde_json::from_slice(&content).ok()?;
            let fields: Map<String, Value> = serde_json::from_slice(&content).ok()?;
            Some((progress, fields != self.progress))
        });
        match kept {
            Some(kept) => Ok(kept),
            None => Ok((
                serde_json::from_value(Value::Object(self.progress.clone()))?,
                true,
            )),
        }
    }

    fn restore(
        &self,
        mut progress: Progress,
        out_of_sync: bool,
        paths: &Paths,
        village_count: usize,
    ) -> io::Result<()> {
        if let Some(config) = self.config.as_deref().filter(|config| !config.is_empty()) {
            write_creating_dir(&paths.config, config)?;
        }
        paths.settle(&mut progress, out_of_sync, village_count);
        paths.save(&progress)
    }
}

pub fn export(file: &Path) -> bool {
    match Paths::local()
        .and_then(|paths| Backup::collect(&paths, letters::read().len()))
        .and_then(|backup| backup.write(file))
    {
        Ok(()) => {
            println!("{PREFIX} Sauvegarde écrite dans {}.", file.display());
            true
        }
        Err(error) => {
            eprintln!("{PREFIX} {error}");
            false
        }
    }
}

pub fn import(file: &Path) -> bool {
    let Ok((backup, progress, out_of_sync)) = Backup::read(file) else {
        eprintln!("{PREFIX} Sauvegarde illisible\u{202F}: {}", file.display());
        return false;
    };
    if let Some(version) = backup.other_version() {
        eprintln!(
            "{PREFIX} Sauvegarde faite avec weathr {version}, tu as la {VERSION}\u{202F}: des données peuvent manquer."
        );
    }

    let restored = Paths::local().and_then(|paths| {
        if paths.progress.exists() && !confirm_replacement() {
            return Ok(false);
        }
        backup
            .restore(progress, out_of_sync, &paths, letters::read().len())
            .map(|()| true)
    });
    match restored {
        Ok(true) => {
            println!("{PREFIX} Progression importée.");
            true
        }
        Ok(false) => true,
        Err(error) => {
            eprintln!("{PREFIX} {error}");
            false
        }
    }
}

fn confirm_replacement() -> bool {
    print!(
        "{PREFIX} Ta progression et tes réglages actuels seront remplacés. Continuer\u{202F}? [o/N] "
    );
    let _ = io::stdout().flush();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).is_ok() && accepts(&answer)
}

fn accepts(answer: &str) -> bool {
    matches!(
        answer.trim().to_lowercase().as_str(),
        "o" | "oui" | "y" | "yes"
    )
}

fn read_if_present(path: &Path) -> io::Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(Some(content)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn write_creating_dir(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::content::letters::VILLAGE_LETTER_COUNT;

    const PROGRESS: &str = r#"{"started_on":"2026-09-01","letters_read":12,"badges":["first_rain","music_day"],"answers":{"3":1}}"#;
    const CONFIG: &str = "# Ma config\nhide_hud = false\n\n[location]\ncity = \"Lyon\"\n";

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let dir =
                std::env::temp_dir().join(format!("weathr_backup_{name}_{}", std::process::id()));
            fs::remove_dir_all(&dir).ok();
            fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }

        fn paths(&self, machine: &str) -> Paths {
            let home = self.0.join(machine);
            let dir = home.join("weathr");
            Paths {
                progress: dir.join("progress.json"),
                config: dir.join("config.toml"),
                sync: Some(home.join("elsewhere").join("sync")),
            }
        }

        fn file(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).ok();
        }
    }

    fn seed(paths: &Paths, config: Option<&str>) {
        write_creating_dir(&paths.progress, PROGRESS).unwrap();
        if let Some(config) = config {
            write_creating_dir(&paths.config, config).unwrap();
        }
    }

    fn played(paths: &Paths, letters_read: usize) {
        let mut progress = Progress::default();
        progress.started_on = Some("2026-09-01".to_string());
        progress.letters_read = letters_read;
        paths.save(&progress).unwrap();
    }

    fn export_to(paths: &Paths, file: &Path) {
        Backup::collect(paths, VILLAGE_LETTER_COUNT)
            .unwrap()
            .write(file)
            .unwrap();
    }

    fn import_from(file: &Path, paths: &Paths) {
        let (backup, progress, out_of_sync) = Backup::read(file).unwrap();
        backup
            .restore(progress, out_of_sync, paths, VILLAGE_LETTER_COUNT)
            .unwrap();
    }

    fn reopen(paths: &Paths) -> Progress {
        let (mut progress, opening) = Progress::load_at(&paths.progress, paths.sync.as_deref());
        assert!(!paths.settle(&mut progress, opening.out_of_sync, VILLAGE_LETTER_COUNT));
        progress
    }

    fn edit_backup(file: &Path, change: impl FnOnce(&mut Map<String, Value>)) {
        let mut backup: Map<String, Value> = json(&fs::read_to_string(file).unwrap())
            .as_object()
            .cloned()
            .unwrap();
        change(&mut backup);
        fs::write(file, serde_json::to_string_pretty(&backup).unwrap()).unwrap();
    }

    fn json(text: &str) -> Value {
        serde_json::from_str(text).unwrap()
    }

    #[test]
    fn test_export_then_import_keeps_progress_and_config() {
        let temp = TempDir::new("round_trip");
        let old_mac = temp.paths("old");
        let new_mac = temp.paths("new");
        let file = temp.file("s.json");
        seed(&old_mac, Some(CONFIG));

        export_to(&old_mac, &file);
        import_from(&file, &new_mac);

        assert_eq!(
            fs::read(&new_mac.progress).unwrap(),
            fs::read(&old_mac.progress).unwrap()
        );
        assert_eq!(fs::read_to_string(&new_mac.config).unwrap(), CONFIG);
        let reopened = reopen(&new_mac);
        assert_eq!(reopened.letters_read, 12);
        assert_eq!(reopened.resyncs, 0);
    }

    #[test]
    fn test_import_over_another_game_replaces_it_quietly() {
        let temp = TempDir::new("over_a_game");
        let old_mac = temp.paths("old");
        let new_mac = temp.paths("new");
        let file = temp.file("s.json");
        seed(&old_mac, None);
        played(&new_mac, 3);

        export_to(&old_mac, &file);
        import_from(&file, &new_mac);

        let reopened = reopen(&new_mac);
        assert_eq!(reopened.letters_read, 12);
        assert_eq!(reopened.resyncs, 0);
    }

    #[test]
    fn test_edited_backup_brings_back_the_exported_game() {
        let temp = TempDir::new("edited");
        let old_mac = temp.paths("old");
        let new_mac = temp.paths("new");
        let file = temp.file("s.json");
        seed(&old_mac, None);

        export_to(&old_mac, &file);
        edit_backup(&file, |backup| {
            backup["progress"]["letters_read"] = Value::from(17);
        });
        import_from(&file, &new_mac);

        let reopened = reopen(&new_mac);
        assert_eq!(reopened.letters_read, 12);
        assert_eq!(reopened.resyncs, 1);
    }

    #[test]
    fn test_backup_without_a_readable_copy_counts_once_and_fits_values() {
        let temp = TempDir::new("no_copy");
        let old_mac = temp.paths("old");
        let file = temp.file("s.json");
        seed(&old_mac, None);
        export_to(&old_mac, &file);
        let exported = fs::read_to_string(&file).unwrap();

        let mut far_ahead: Progress = serde_json::from_str(PROGRESS).unwrap();
        far_ahead.letters_read = 900;
        old_mac.settle(&mut far_ahead, false, VILLAGE_LETTER_COUNT);
        let reachable = far_ahead.letters_read;
        assert!(reachable < 900);

        let cases = [
            ("removed", None, 12, 12),
            ("damaged", Some("zz".to_string()), 12, 12),
            (
                "not_a_progress",
                Some(pack(br#"{"letters_read":"douze"}"#)),
                12,
                12,
            ),
            ("removed_and_edited", None, 900, reachable),
        ];
        for (name, copy, letters_read, expected) in cases {
            let new_mac = temp.paths(name);
            fs::write(&file, &exported).unwrap();
            edit_backup(&file, |backup| {
                match copy {
                    Some(copy) => backup["sync"] = Value::from(copy),
                    None => {
                        backup.remove("sync");
                    }
                }
                backup["progress"]["letters_read"] = Value::from(letters_read);
            });
            import_from(&file, &new_mac);

            let reopened = reopen(&new_mac);
            assert_eq!(reopened.resyncs, 1, "{name}");
            assert_eq!(reopened.letters_read, expected, "{name}");
        }
    }

    #[test]
    fn test_export_after_an_edit_carries_the_game_from_before() {
        let temp = TempDir::new("edited_save");
        let old_mac = temp.paths("old");
        let new_mac = temp.paths("new");
        let file = temp.file("s.json");
        played(&old_mac, 12);
        let text = fs::read_to_string(&old_mac.progress).unwrap();
        fs::write(
            &old_mac.progress,
            text.replacen("\"letters_read\": 12", "\"letters_read\": 30", 1),
        )
        .unwrap();

        export_to(&old_mac, &file);

        let written = json(&fs::read_to_string(&file).unwrap());
        assert_eq!(written["progress"]["letters_read"], 12);
        assert_eq!(written["progress"]["resyncs"], 1);
        assert_eq!(reopen(&old_mac).resyncs, 1);

        import_from(&file, &new_mac);
        let reopened = reopen(&new_mac);
        assert_eq!(reopened.letters_read, 12);
        assert_eq!(reopened.resyncs, 1);
    }

    #[test]
    fn test_unreadable_progress_is_neither_exported_nor_touched() {
        let temp = TempDir::new("unreadable");
        let paths = temp.paths("mac");
        write_creating_dir(&paths.progress, "{\"letters_read\": 1").unwrap();

        assert!(Backup::collect(&paths, VILLAGE_LETTER_COUNT).is_err());
        assert_eq!(
            fs::read_to_string(&paths.progress).unwrap(),
            "{\"letters_read\": 1"
        );
        assert_eq!(
            fs::read_dir(paths.progress.parent().unwrap())
                .unwrap()
                .count(),
            1
        );
        assert!(!paths.sync.unwrap().exists());
    }

    #[test]
    fn test_backup_file_holds_version_progress_and_config() {
        let temp = TempDir::new("content");
        let paths = temp.paths("mac");
        let file = temp.file("s.json");
        seed(&paths, Some(CONFIG));

        export_to(&paths, &file);
        let written = json(&fs::read_to_string(&file).unwrap());

        assert_eq!(written["version"], VERSION);
        assert_eq!(
            written["progress"],
            json(&fs::read_to_string(&paths.progress).unwrap())
        );
        assert_eq!(written["progress"]["letters_read"], 12);
        assert_eq!(written["config"], CONFIG);
        assert!(written["sync"].is_string());
    }

    #[test]
    fn test_export_overwrites_an_existing_file() {
        let temp = TempDir::new("overwrite");
        let paths = temp.paths("mac");
        let file = temp.file("s.json");
        seed(&paths, Some(CONFIG));
        fs::write(&file, "old content").unwrap();

        export_to(&paths, &file);

        assert!(Backup::read(&file).is_ok());
    }

    #[test]
    fn test_backup_without_config_leaves_config_untouched() {
        let temp = TempDir::new("no_config");
        let old_mac = temp.paths("old");
        let new_mac = temp.paths("new");
        let file = temp.file("s.json");
        seed(&old_mac, None);

        export_to(&old_mac, &file);
        assert!(
            json(&fs::read_to_string(&file).unwrap())
                .get("config")
                .is_none()
        );

        import_from(&file, &new_mac);
        assert!(new_mac.progress.exists());
        assert!(!new_mac.config.exists());
    }

    #[test]
    fn test_export_without_progress_fails() {
        let temp = TempDir::new("no_progress");
        let paths = temp.paths("mac");
        assert!(Backup::collect(&paths, VILLAGE_LETTER_COUNT).is_err());
        assert!(!paths.progress.exists());
    }

    #[test]
    fn test_other_version_is_detected() {
        let backup: Backup =
            serde_json::from_str(r#"{"version":"0.1.0","progress":{"letters_read":3}}"#).unwrap();
        assert_eq!(backup.other_version(), Some("0.1.0"));
    }

    #[test]
    fn test_same_version_is_not_reported() {
        let temp = TempDir::new("same_version");
        let paths = temp.paths("mac");
        seed(&paths, None);
        assert_eq!(
            Backup::collect(&paths, VILLAGE_LETTER_COUNT)
                .unwrap()
                .other_version(),
            None
        );
    }

    #[test]
    fn test_invalid_backups_are_rejected() {
        let temp = TempDir::new("invalid");
        let cases = [
            ("not_json.json", "pas du JSON"),
            (
                "no_progress.json",
                r#"{"version":"1.7.0","config":"hide_hud = true"}"#,
            ),
            (
                "progress_not_object.json",
                r#"{"version":"1.7.0","progress":[1,2]}"#,
            ),
            (
                "progress_mistyped.json",
                r#"{"version":"1.7.0","progress":{"letters_read":"douze"}}"#,
            ),
        ];
        for (name, content) in cases {
            let file = temp.file(name);
            fs::write(&file, content).unwrap();
            assert!(Backup::read(&file).is_err(), "{name}");
        }
        assert!(Backup::read(&temp.file("missing.json")).is_err());
    }

    #[test]
    fn test_only_an_explicit_yes_replaces_the_progress() {
        for answer in ["o\n", "oui", "Y", " yes \n"] {
            assert!(accepts(answer), "{answer:?}");
        }
        for answer in ["", "\n", "n", "non", "no", "ok"] {
            assert!(!accepts(answer), "{answer:?}");
        }
    }
}
