use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::event::{Beat, Entry};

const POSTCARD_FIRST_DAY: usize = 40;
const POSTCARD_INTERVAL_DAYS: usize = 7;

/// Sam's Cartes postales once he has left, one a week with no end.
pub struct Postcards;

impl StorySource for Postcards {
    fn beats(&self, _clock: &StoryClock, today: usize) -> Vec<Beat> {
        (0..)
            .map(|index| POSTCARD_FIRST_DAY + index * POSTCARD_INTERVAL_DAYS)
            .take_while(|day| *day <= today)
            .enumerate()
            .map(|(index, day)| Beat::mail(day, Entry::Postcard(index)))
            .collect()
    }
}
