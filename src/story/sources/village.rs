use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::event::{Beat, Entry, Thing};

const ARRIVALS: [(usize, Thing); 25] = [
    (3, Thing::Cat),
    (4, Thing::Bench),
    (6, Thing::CatBow),
    (8, Thing::Footprints),
    (9, Thing::Dog),
    (12, Thing::Owl),
    (13, Thing::Well),
    (15, Thing::Bone),
    (17, Thing::Key),
    (18, Thing::Duck),
    (21, Thing::Snail),
    (22, Thing::Treehouse),
    (24, Thing::OwlFriend),
    (26, Thing::BlueLantern),
    (27, Thing::Squirrel),
    (30, Thing::Frog),
    (31, Thing::Garland),
    (33, Thing::Kittens),
    (35, Thing::XMark),
    (40, Thing::Balloon),
    (42, Thing::Ducklings),
    (45, Thing::FoundObjects),
    (49, Thing::WeatherVane),
    (58, Thing::Bunting),
    (59, Thing::Chest),
];

pub struct Village {
    count: usize,
}

impl Village {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

fn village_day(index: usize) -> usize {
    index + 1
}

impl StorySource for Village {
    fn beats(&self, _clock: &StoryClock, _today: usize) -> Vec<Beat> {
        let mail = (0..self.count).map(|index| Beat::mail(village_day(index), Entry::Main(index)));
        let arrivals = ARRIVALS
            .into_iter()
            .filter(|(number, _)| *number <= self.count)
            .map(|(number, thing)| Beat::arrival(village_day(number - 1), thing));
        mail.chain(arrivals).collect()
    }
}
