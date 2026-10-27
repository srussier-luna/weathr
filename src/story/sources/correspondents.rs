use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::content::chronicles::{MARCEL_LETTERS, NEIGHBOURS_LETTERS, SAM_LETTERS};
use crate::story::event::{Beat, Entry};

const SAM_DAYS: [usize; SAM_LETTERS.len()] = [8, 14, 20, 26, 33];
const NEIGHBOURS_DAYS: [usize; NEIGHBOURS_LETTERS.len()] = [11, 18, 25, 36, 47];
const MARCEL_DAYS: [usize; MARCEL_LETTERS.len()] = [12, 21, 30, 44, 56];

pub const SAM: Correspondent = Correspondent::new(&SAM_DAYS, Entry::Sam);
pub const NEIGHBOURS: Correspondent = Correspondent::new(&NEIGHBOURS_DAYS, Entry::Neighbours);
pub const MARCEL: Correspondent = Correspondent::new(&MARCEL_DAYS, Entry::Marcel);

pub struct Correspondent {
    days: &'static [usize],
    entry: fn(usize) -> Entry,
}

impl Correspondent {
    const fn new(days: &'static [usize], entry: fn(usize) -> Entry) -> Self {
        Self { days, entry }
    }
}

impl StorySource for Correspondent {
    fn beats(&self, _clock: &StoryClock, _today: usize) -> Vec<Beat> {
        self.days
            .iter()
            .enumerate()
            .map(|(index, day)| Beat::mail(*day, (self.entry)(index)))
            .collect()
    }
}
