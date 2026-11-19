use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const DIR: &str = "tty-sync";
const FILE: &str = "journal";
const DIGEST_LENGTH: usize = 16;
const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x0100_0000_01b3;

pub(super) fn path() -> Option<PathBuf> {
    dirs::data_dir().map(|dir| dir.join(DIR).join(FILE))
}

fn digest(bytes: &[u8]) -> u64 {
    bytes.iter().fold(OFFSET_BASIS, |acc, byte| {
        (acc ^ u64::from(*byte)).wrapping_mul(PRIME)
    })
}

fn encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn decode(text: &str) -> Option<Vec<u8>> {
    if text.len() % 2 != 0 {
        return None;
    }
    (0..text.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(text.get(index..index + 2)?, 16).ok())
        .collect()
}

fn read_digest(text: &str, index: usize) -> Option<u64> {
    let start = index * DIGEST_LENGTH;
    u64::from_str_radix(text.get(start..start + DIGEST_LENGTH)?, 16).ok()
}

pub(super) struct Journal {
    previous: u64,
    current: Vec<u8>,
}

impl Journal {
    pub(super) fn read(path: &Path) -> Option<Self> {
        let text = fs::read_to_string(path).ok()?;
        let previous = read_digest(&text, 0)?;
        let checked = read_digest(&text, 1)?;
        let current = decode(text.get(2 * DIGEST_LENGTH..)?)?;
        (digest(&current) == checked).then_some(Self { previous, current })
    }

    pub(super) fn record(path: &Path, on_disk: &[u8], current: &[u8]) -> io::Result<()> {
        let previous = match Self::read(path) {
            Some(kept) if !kept.holds(on_disk) => digest(&kept.current),
            _ => digest(on_disk),
        };
        let text = format!("{previous:016x}{:016x}{}", digest(current), encode(current));
        super::replace_file(path, text.as_bytes())
    }

    pub(super) fn holds(&self, content: &[u8]) -> bool {
        content == self.current || digest(content) == self.previous
    }

    pub(super) fn current(&self) -> &[u8] {
        &self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("weathr_test_journal_{name}_{}", std::process::id()));
        fs::remove_dir_all(&dir).ok();
        dir
    }

    #[test]
    fn test_digest_is_stable_across_builds() {
        assert_eq!(digest(b""), OFFSET_BASIS);
        assert_eq!(digest(b"a"), 0xaf63_dc4c_8601_ec8c);
    }

    #[test]
    fn test_journal_round_trips_and_holds_both_versions() {
        let dir = scratch_dir("round_trip");
        let path = dir.join("nested").join(FILE);
        Journal::record(&path, b"{\"streak\": 1}", b"{\"streak\": 2}").unwrap();
        let journal = Journal::read(&path);
        fs::remove_dir_all(&dir).ok();

        let journal = journal.unwrap();
        assert_eq!(journal.current(), b"{\"streak\": 2}");
        assert!(journal.holds(b"{\"streak\": 2}"));
        assert!(journal.holds(b"{\"streak\": 1}"));
        assert!(!journal.holds(b"{\"streak\": 3}"));
    }

    #[test]
    fn test_damaged_journal_reads_as_missing() {
        let dir = scratch_dir("damaged");
        let path = dir.join(FILE);
        Journal::record(&path, b"", b"{}").unwrap();
        let mut text = fs::read_to_string(&path).unwrap();
        text.replace_range(text.len() - 2.., "7c");
        fs::write(&path, &text).unwrap();
        let damaged = Journal::read(&path);
        fs::write(&path, "zz").unwrap();
        let garbled = Journal::read(&path);
        fs::remove_dir_all(&dir).ok();

        assert!(damaged.is_none());
        assert!(garbled.is_none());
    }

    #[test]
    fn test_journal_lives_outside_the_game_folder() {
        let (Some(path), Some(data), Some(save)) =
            (path(), dirs::data_dir(), super::super::progress_path())
        else {
            return;
        };
        let inside = path.strip_prefix(&data).unwrap().to_string_lossy();
        assert!(!inside.to_lowercase().contains("weathr"));
        assert!(!path.starts_with(save.parent().unwrap()));
        assert!(dirs::cache_dir().is_none_or(|cache| !path.starts_with(cache)));
    }
}
