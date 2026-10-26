use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::content::chronicles::ARCHIVES;
use crate::story::event::{Beat, Entry, Thing};

const ARCHIVE_FIRST_DAY: usize = 28;
const ARCHIVE_INTERVAL_DAYS: usize = 6;
const BOTTLES_ARCHIVE_NUMBER: usize = 3;

pub struct Archives;

fn archive_day(index: usize) -> usize {
    ARCHIVE_FIRST_DAY + index * ARCHIVE_INTERVAL_DAYS
}

impl StorySource for Archives {
    fn beats(&self, _clock: &StoryClock, _today: usize) -> Vec<Beat> {
        (0..ARCHIVES.len())
            .map(|index| Beat::mail(archive_day(index), Entry::Archive(index)))
            .chain(std::iter::once(Beat::arrival(
                archive_day(BOTTLES_ARCHIVE_NUMBER - 1),
                Thing::Bottles,
            )))
            .collect()
    }
}
