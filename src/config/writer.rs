use std::fs;
use std::io;
use std::path::Path;
use toml_edit::{DocumentMut, Item, Table, TableLike, Value};

use crate::error::ConfigError;

pub fn write_value(path: &Path, keys: &[&str], value: Value) -> Result<(), ConfigError> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(source) => {
            return Err(ConfigError::ReadError {
                path: path.display().to_string(),
                source,
            });
        }
    };
    let mut document = content.parse::<DocumentMut>()?;
    set(document.as_table_mut(), keys, value)?;

    let write_error = |source| ConfigError::WriteError {
        path: path.display().to_string(),
        source,
    };
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(write_error)?;
    }
    fs::write(path, document.to_string()).map_err(write_error)
}

fn set(table: &mut dyn TableLike, keys: &[&str], value: Value) -> Result<(), ConfigError> {
    match keys {
        [] => Ok(()),
        [key] => {
            match table.get_mut(key) {
                Some(Item::Value(old)) => {
                    let decor = old.decor().clone();
                    *old = value;
                    *old.decor_mut() = decor;
                }
                _ => {
                    table.insert(key, Item::Value(value));
                }
            }
            Ok(())
        }
        [key, rest @ ..] => {
            let child = table
                .entry(key)
                .or_insert_with(|| Item::Table(Table::new()))
                .as_table_like_mut()
                .ok_or_else(|| ConfigError::NotATable(keys.join(".")))?;
            set(child, rest, value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir()
            .join(format!("weathr_writer_{name}_{}", std::process::id()))
            .join("config.toml")
    }

    fn write_and_read(name: &str, original: Option<&str>, keys: &[&str], value: Value) -> String {
        let path = temp_path(name);
        let dir = path.parent().unwrap().to_path_buf();
        fs::remove_dir_all(&dir).ok();
        if let Some(original) = original {
            fs::create_dir_all(&dir).unwrap();
            fs::write(&path, original).unwrap();
        }
        write_value(&path, keys, value).unwrap();
        let written = fs::read_to_string(&path).unwrap();
        fs::remove_dir_all(&dir).ok();
        written
    }

    #[test]
    fn test_changes_only_the_value_and_keeps_comments() {
        let original = "# Ma config\n\
                        hide_hud = false # la barre\n\
                        \n\
                        [units]\n\
                        # unités\n\
                        temperature = \"celsius\"\n\
                        wind_speed = \"kmh\"\n";
        let written = write_and_read(
            "keeps_comments",
            Some(original),
            &["units", "temperature"],
            "fahrenheit".into(),
        );
        assert_eq!(written, original.replace("\"celsius\"", "\"fahrenheit\""));

        let written = write_and_read("keeps_decor", Some(original), &["hide_hud"], true.into());
        assert_eq!(written, original.replace("false", "true"));
    }

    #[test]
    fn test_adds_missing_table_after_existing_lines() {
        let original = "# Ma config\n[location]\nlatitude = 48.85\n";
        let written = write_and_read(
            "missing_table",
            Some(original),
            &["courrier", "heure"],
            7.into(),
        );
        assert!(written.starts_with(original));
        assert!(written.contains("[courrier]\nheure = 7\n"));
    }

    #[test]
    fn test_writes_into_dotted_and_inline_tables() {
        let written = write_and_read(
            "dotted",
            Some("courrier.heure = 9\n"),
            &["courrier", "heure"],
            6.into(),
        );
        assert_eq!(written, "courrier.heure = 6\n");

        let written = write_and_read(
            "inline",
            Some("units = { temperature = \"celsius\", precipitation = \"mm\" }\n"),
            &["units", "precipitation"],
            "inch".into(),
        );
        assert_eq!(
            written,
            "units = { temperature = \"celsius\", precipitation = \"inch\" }\n"
        );
    }

    #[test]
    fn test_creates_absent_file_and_folder() {
        let written = write_and_read("absent", None, &["hide_hud"], true.into());
        assert_eq!(written, "hide_hud = true\n");
    }

    #[test]
    fn test_refuses_a_value_in_place_of_a_table() {
        let path = temp_path("not_a_table");
        let dir = path.parent().unwrap().to_path_buf();
        fs::create_dir_all(&dir).unwrap();
        fs::write(&path, "units = \"metric\"\n").unwrap();
        let error = write_value(&path, &["units", "temperature"], "celsius".into()).unwrap_err();
        assert_eq!(error.kind(), "NotATable");
        assert_eq!(fs::read_to_string(&path).unwrap(), "units = \"metric\"\n");
        fs::remove_dir_all(&dir).ok();
    }
}
