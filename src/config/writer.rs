use std::fs;
use std::io;
use std::path::Path;
use toml_edit::{Decor, DocumentMut, Item, RawString, Table, TableLike, Value};

use crate::error::ConfigError;

pub fn write_value(path: &Path, keys: &[&str], value: Value) -> Result<(), ConfigError> {
    write_values(path, &[(keys, Some(value))])
}

pub fn write_values(path: &Path, values: &[(&[&str], Option<Value>)]) -> Result<(), ConfigError> {
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
    for (keys, value) in values {
        match value {
            Some(value) => set(document.as_table_mut(), keys, value.clone())?,
            None => remove(&mut document, keys),
        }
    }

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

fn remove(document: &mut DocumentMut, keys: &[&str]) {
    let Some((key, parents)) = keys.split_last() else {
        return;
    };
    let mut item = document.as_item_mut();
    for parent in parents {
        let Some(child) = item.get_mut(parent) else {
            return;
        };
        item = child;
    }
    let position = if parents.is_empty() {
        Some(0)
    } else {
        item.as_table().and_then(Table::position)
    };
    let Some(prefix) = item
        .as_table_like_mut()
        .and_then(|table| remove_key(table, key))
        .filter(|prefix| !prefix.is_empty())
    else {
        return;
    };
    let Some(position) = position else {
        return;
    };
    match next_header_position(document.as_table(), position)
        .and_then(|next| header_at(document.as_table_mut(), next))
    {
        Some(header) => prepend(header.decor_mut(), &prefix),
        None => {
            let trailing = document.trailing().as_str().unwrap_or_default();
            document.set_trailing(format!("{prefix}{trailing}"));
        }
    }
}

fn remove_key(table: &mut dyn TableLike, key: &str) -> Option<String> {
    let index = table.iter().position(|(name, _)| name == key)?;
    let prefix = table
        .key(key)
        .and_then(|removed| removed.leaf_decor().prefix())
        .and_then(RawString::as_str)
        .unwrap_or_default()
        .trim_end_matches([' ', '\t'])
        .to_string();
    table.remove(key);
    match table
        .iter_mut()
        .skip(index)
        .find(|(_, item)| item.is_value())
    {
        Some((mut next, _)) => {
            prepend(next.leaf_decor_mut(), &prefix);
            None
        }
        None => Some(prefix),
    }
}

fn prepend(decor: &mut Decor, prefix: &str) {
    let rest = decor
        .prefix()
        .and_then(RawString::as_str)
        .unwrap_or_default();
    decor.set_prefix(format!("{prefix}{rest}"));
}

fn is_header(table: &Table) -> bool {
    !table.is_implicit() && !table.is_dotted()
}

fn next_header_position(table: &Table, after: isize) -> Option<isize> {
    let own = table
        .position()
        .filter(|position| *position > after && is_header(table));
    let nested = table.iter().filter_map(|(_, item)| match item {
        Item::Table(child) => next_header_position(child, after),
        Item::ArrayOfTables(array) => array
            .iter()
            .filter_map(|child| next_header_position(child, after))
            .min(),
        _ => None,
    });
    own.into_iter().chain(nested).min()
}

fn header_at(table: &mut Table, position: isize) -> Option<&mut Table> {
    if table.position() == Some(position) && is_header(table) {
        return Some(table);
    }
    table.iter_mut().find_map(|(_, item)| match item {
        Item::Table(child) => header_at(child, position),
        Item::ArrayOfTables(array) => array
            .iter_mut()
            .find_map(|child| header_at(child, position)),
        _ => None,
    })
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
    fn test_writes_several_values_and_removes_absent_ones() {
        let original = "[location]\n\
                        latitude = 1.0 # ici\n\
                        \n\
                        # ma ville\n\
                        city = \"Minsk\"\n\
                        # détection\n\
                        auto = true\n";
        let path = temp_path("several");
        let dir = path.parent().unwrap().to_path_buf();
        fs::create_dir_all(&dir).unwrap();
        fs::write(&path, original).unwrap();
        write_values(
            &path,
            &[
                (&["location", "latitude"], Some(45.5.into())),
                (&["location", "city"], None),
                (&["location", "auto"], Some(false.into())),
            ],
        )
        .unwrap();
        let written = fs::read_to_string(&path).unwrap();
        fs::remove_dir_all(&dir).ok();
        assert_eq!(
            written,
            "[location]\nlatitude = 45.5 # ici\n\n# ma ville\n# détection\nauto = false\n"
        );
    }

    #[test]
    fn test_removed_last_key_leaves_its_comment_in_place() {
        let next_header = "hide_hud = true\n\
                           # ma barre\n\
                           silent = false\n\
                           \n\
                           [location]\n\
                           latitude = 1.0\n\
                           \n\
                           # ma ville\n\
                           city = \"Minsk\"\n\
                           \n\
                           # unités\n\
                           [units]\n\
                           temperature = \"celsius\"\n";
        let end_of_file = "[location]\nlatitude = 1.0\n# ma ville\ncity = \"Minsk\"\n";
        let city = "city = \"Minsk\"\n";
        for (name, original, keys, line) in [
            (
                "root_last",
                next_header,
                &["silent"][..],
                "silent = false\n",
            ),
            ("table_last", next_header, &["location", "city"], city),
            ("file_last", end_of_file, &["location", "city"], city),
        ] {
            let path = temp_path(name);
            let dir = path.parent().unwrap().to_path_buf();
            fs::create_dir_all(&dir).unwrap();
            fs::write(&path, original).unwrap();
            write_values(&path, &[(keys, None)]).unwrap();
            let written = fs::read_to_string(&path).unwrap();
            fs::remove_dir_all(&dir).ok();
            assert_eq!(written, original.replace(line, ""));
        }
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
