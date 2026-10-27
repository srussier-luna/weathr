use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::event::{Beat, Entry};

pub struct Village {
    count: usize,
}

impl Village {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl StorySource for Village {
    fn beats(&self, _clock: &StoryClock, _today: usize) -> Vec<Beat> {
        (0..self.count)
            .map(|index| Beat::mail(index + 1, Entry::Main(index)))
            .collect()
    }
}
