use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::content::chronicles::ARCHIVES;
use crate::story::event::{Beat, Entry};

const ARCHIVE_FIRST_DAY: usize = 28;
const ARCHIVE_INTERVAL_DAYS: usize = 6;

/// Town hall archives, sent by Odile at a steady pace.
pub struct Archives;

impl StorySource for Archives {
    fn beats(&self, _clock: &StoryClock, _today: usize) -> Vec<Beat> {
        (0..ARCHIVES.len())
            .map(|index| {
                Beat::mail(
                    ARCHIVE_FIRST_DAY + index * ARCHIVE_INTERVAL_DAYS,
                    Entry::Archive(index),
                )
            })
            .collect()
    }
}
