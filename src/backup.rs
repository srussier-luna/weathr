use crate::config::Config;
use crate::progress::{progress_path, write_creating_dir};
use crate::prompt::{PREFIX, confirm};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Paths {
    progress: PathBuf,
    config: PathBuf,
}

impl Paths {
    fn local() -> Option<Self> {
        Some(Self {
            progress: progress_path()?,
            config: Config::get_config_path().ok()?,
        })
    }
}

#[derive(Debug)]
enum CollectError<'a> {
    NoProgress,
    Unreadable(&'a Path),
}

enum Restore {
    Done,
    Declined,
}

#[derive(Serialize, Deserialize)]
struct Backup {
    version: String,
    progress: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    config: Option<String>,
}

impl Backup {
    fn collect(paths: &Paths) -> Result<Self, CollectError<'_>> {
        let progress = fs::read_to_string(&paths.progress).map_err(|error| match error.kind() {
            io::ErrorKind::NotFound => CollectError::NoProgress,
            _ => CollectError::Unreadable(&paths.progress),
        })?;
        Ok(Self {
            version: VERSION.to_string(),
            progress: serde_json::from_str(&progress)
                .map_err(|_| CollectError::Unreadable(&paths.progress))?,
            config: read_if_present(&paths.config)
                .map_err(|_| CollectError::Unreadable(&paths.config))?,
        })
    }

    fn read(file: &Path) -> io::Result<Self> {
        Ok(serde_json::from_str(&fs::read_to_string(file)?)?)
    }

    fn write(&self, file: &Path) -> io::Result<()> {
        fs::write(file, serde_json::to_string_pretty(self)?)
    }

    fn other_version(&self) -> Option<&str> {
        Some(self.version.as_str()).filter(|version| *version != VERSION)
    }

    fn restore<'a>(&self, paths: &'a Paths) -> Result<(), &'a Path> {
        if let Some(config) = self.config.as_deref().filter(|config| !config.is_empty()) {
            write_creating_dir(&paths.config, config).map_err(|_| paths.config.as_path())?;
        }
        serde_json::to_string_pretty(&self.progress)
            .map_err(io::Error::from)
            .and_then(|progress| write_creating_dir(&paths.progress, &progress))
            .map_err(|_| paths.progress.as_path())
    }
}

pub fn export(file: &Path) -> bool {
    let paths = Paths::local();
    let collected = paths
        .as_ref()
        .ok_or(CollectError::NoProgress)
        .and_then(Backup::collect);
    let backup = match collected {
        Ok(backup) => backup,
        Err(CollectError::NoProgress) => {
            eprintln!("{PREFIX} Aucune progression à sauvegarder.");
            return false;
        }
        Err(CollectError::Unreadable(path)) => {
            eprintln!("{PREFIX} Impossible de lire {}.", path.display());
            return false;
        }
    };
    if backup.write(file).is_err() {
        eprintln!("{PREFIX} Impossible d'écrire {}.", file.display());
        return false;
    }
    println!("{PREFIX} Sauvegarde écrite dans {}.", file.display());
    true
}

pub fn import(file: &Path) -> bool {
    let Ok(backup) = Backup::read(file) else {
        eprintln!("{PREFIX} Sauvegarde illisible\u{202F}: {}", file.display());
        return false;
    };
    if let Some(version) = backup.other_version() {
        eprintln!(
            "{PREFIX} Sauvegarde faite avec weathr {version}, tu as la {VERSION}\u{202F}: des données peuvent manquer."
        );
    }
    let Some(paths) = Paths::local() else {
        eprintln!("{PREFIX} Impossible d'écrire la progression.");
        return false;
    };

    match import_into(&backup, &paths) {
        Ok(Restore::Done) => {
            println!("{PREFIX} Progression importée.");
            true
        }
        Ok(Restore::Declined) => true,
        Err(path) => {
            eprintln!("{PREFIX} Impossible d'écrire {}.", path.display());
            false
        }
    }
}

fn import_into<'a>(backup: &Backup, paths: &'a Paths) -> Result<Restore, &'a Path> {
    if paths.progress.exists()
        && !confirm(
            "Ta progression et tes réglages actuels seront remplacés. Continuer\u{202F}?",
            false,
        )
    {
        return Ok(Restore::Declined);
    }
    backup.restore(paths).map(|()| Restore::Done)
}

fn read_if_present(path: &Path) -> io::Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(Some(content)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            let dir = self.0.join(machine).join("weathr");
            Paths {
                progress: dir.join("progress.json"),
                config: dir.join("config.toml"),
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

        Backup::collect(&old_mac).unwrap().write(&file).unwrap();
        Backup::read(&file).unwrap().restore(&new_mac).unwrap();

        assert_eq!(
            json(&fs::read_to_string(&new_mac.progress).unwrap()),
            json(PROGRESS)
        );
        assert_eq!(fs::read_to_string(&new_mac.config).unwrap(), CONFIG);
    }

    #[test]
    fn test_backup_file_holds_version_progress_and_config() {
        let temp = TempDir::new("content");
        let paths = temp.paths("mac");
        let file = temp.file("s.json");
        seed(&paths, Some(CONFIG));

        Backup::collect(&paths).unwrap().write(&file).unwrap();
        let written = json(&fs::read_to_string(&file).unwrap());

        assert_eq!(written["version"], VERSION);
        assert_eq!(written["progress"], json(PROGRESS));
        assert_eq!(written["config"], CONFIG);
    }

    #[test]
    fn test_export_overwrites_an_existing_file() {
        let temp = TempDir::new("overwrite");
        let paths = temp.paths("mac");
        let file = temp.file("s.json");
        seed(&paths, Some(CONFIG));
        fs::write(&file, "old content").unwrap();

        Backup::collect(&paths).unwrap().write(&file).unwrap();

        assert!(Backup::read(&file).is_ok());
    }

    #[test]
    fn test_backup_without_config_leaves_config_untouched() {
        let temp = TempDir::new("no_config");
        let old_mac = temp.paths("old");
        let new_mac = temp.paths("new");
        let file = temp.file("s.json");
        seed(&old_mac, None);

        Backup::collect(&old_mac).unwrap().write(&file).unwrap();
        assert!(
            json(&fs::read_to_string(&file).unwrap())
                .get("config")
                .is_none()
        );

        Backup::read(&file).unwrap().restore(&new_mac).unwrap();
        assert!(new_mac.progress.exists());
        assert!(!new_mac.config.exists());
    }

    #[test]
    fn test_failed_config_write_leaves_progress_untouched() {
        let temp = TempDir::new("config_fails");
        let old_mac = temp.paths("old");
        let new_mac = temp.paths("new");
        let file = temp.file("s.json");
        seed(&old_mac, Some(CONFIG));
        fs::create_dir_all(&new_mac.config).unwrap();

        Backup::collect(&old_mac).unwrap().write(&file).unwrap();
        let failed = Backup::read(&file).unwrap().restore(&new_mac).err();

        assert_eq!(failed, Some(new_mac.config.as_path()));
        assert!(!new_mac.progress.exists());
    }

    #[test]
    fn test_export_without_progress_fails() {
        let temp = TempDir::new("no_progress");
        assert!(matches!(
            Backup::collect(&temp.paths("mac")),
            Err(CollectError::NoProgress)
        ));
    }

    #[test]
    fn test_export_of_a_corrupt_progress_names_the_file() {
        let temp = TempDir::new("corrupt_progress");
        let paths = temp.paths("mac");
        write_creating_dir(&paths.progress, "pas du JSON").unwrap();
        assert!(matches!(
            Backup::collect(&paths),
            Err(CollectError::Unreadable(path)) if path == paths.progress
        ));
    }

    #[test]
    fn test_other_version_is_detected() {
        let backup: Backup =
            serde_json::from_str(r#"{"version":"0.1.0","progress":{"letters_read":3}}"#).unwrap();
        assert_eq!(backup.other_version(), Some("0.1.0"));
    }

    #[test]
    fn test_same_version_is_not_reported() {
        let backup: Backup = serde_json::from_str(&format!(
            r#"{{"version":"{VERSION}","progress":{{"letters_read":3}}}}"#
        ))
        .unwrap();
        assert_eq!(backup.other_version(), None);
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
        ];
        for (name, content) in cases {
            let file = temp.file(name);
            fs::write(&file, content).unwrap();
            assert!(Backup::read(&file).is_err(), "{name}");
        }
        assert!(Backup::read(&temp.file("missing.json")).is_err());
    }
}
