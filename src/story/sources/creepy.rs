use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::content::letters::CREEPY_LETTER_COUNT;
use crate::story::event::{Beat, Entry, Thing};

const CREEPY_FIRST_DAY: usize = 5;
const CREEPY_INTERVAL_DAYS: usize = 4;
const CREEPY_EPILOGUE_DAY: usize = 61;

const ARRIVALS: [(usize, Thing); 4] = [
    (3, Thing::WetFootprints),
    (5, Thing::Silhouette),
    (8, Thing::WhisperingWell),
    (11, Thing::Staring),
];

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
        let mail = (0..CREEPY_LETTER_COUNT)
            .map(|index| Beat::mail(creepy_day(index), Entry::Creepy(index)));
        let arrivals = ARRIVALS
            .iter()
            .map(|(number, thing)| Beat::arrival(creepy_day(number - 1), *thing));
        mail.chain(arrivals).collect()
    }
}
