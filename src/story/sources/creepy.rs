use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::content::letters::CREEPY_LETTER_COUNT;
use crate::story::event::{Beat, Entry};

const CREEPY_FIRST_DAY: usize = 5;
const CREEPY_INTERVAL_DAYS: usize = 4;
const CREEPY_EPILOGUE_DAY: usize = 61;

/// Lettres slipped under the door by the neighbour at n°13; the last one waits for the end of the story.
pub struct Creepy;

fn creepy_day(index: usize) -> usize {
    if index + 1 == CREEPY_LETTER_COUNT {
        CREEPY_EPILOGUE_DAY
    } else {
        CREEPY_FIRST_DAY + index * CREEPY_INTERVAL_DAYS
    }
}

impl StorySource for Creepy {
    fn beats(&self, _clock: &StoryClock, _today: usize) -> Vec<Beat> {
        (0..CREEPY_LETTER_COUNT)
            .map(|index| Beat::mail(creepy_day(index), Entry::Creepy(index)))
            .collect()
    }
}
