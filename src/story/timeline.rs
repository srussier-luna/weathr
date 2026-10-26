use super::clock::StoryClock;
use super::event::{Beat, Entry, StoryEvent};
use super::sources::StorySource;

/// Every story event, sorted by Jour de jeu then by kind.
pub struct Timeline {
    beats: Vec<Beat>,
}

impl Timeline {
    /// Gathers the beats of every source, up to Jour de jeu `today` for open-ended ones.
    pub fn new(sources: &[Box<dyn StorySource>], clock: &StoryClock, today: usize) -> Self {
        let mut beats: Vec<Beat> = sources
            .iter()
            .flat_map(|source| source.beats(clock, today))
            .collect();
        beats.sort_by_key(|beat| (beat.day, beat.event.order()));
        Self { beats }
    }

    /// Lettres in delivery order with their Jour de jeu; `Progress::letters_read` is a prefix length into it.
    pub fn mail(&self) -> impl Iterator<Item = (usize, Entry)> + '_ {
        self.beats.iter().map(|beat| match beat.event {
            StoryEvent::Mail(entry) => (beat.day, entry),
        })
    }

    /// Number of Lettres delivered by the end of Jour de jeu `day`.
    pub fn delivered_by(&self, day: usize) -> usize {
        self.mail()
            .take_while(|(delivery, _)| *delivery <= day)
            .count()
    }

    /// The `index`-th Lettre with its Jour de jeu.
    pub fn mail_at(&self, index: usize) -> Option<(usize, Entry)> {
        self.mail().nth(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::sources;
    use chrono::NaiveDate;

    const VILLAGE_COUNT: usize = 60;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn timeline(start: NaiveDate, today: usize) -> Timeline {
        let clock = StoryClock::new(start, 0);
        Timeline::new(&sources::registered(VILLAGE_COUNT), &clock, today)
    }

    #[test]
    fn test_matches_pre_refactor_golden_fixture() {
        let mut rendered = String::new();
        for start in [date(2026, 1, 5), date(2026, 3, 31), date(2026, 12, 15)] {
            for offset in [0, 6, 40, 120] {
                rendered.push_str(&format!("case {start} +{offset}\n"));
                for (day, entry) in timeline(start, offset + 1).mail() {
                    rendered.push_str(&format!("{day} {entry:?}\n"));
                }
            }
        }
        assert_eq!(rendered, include_str!("testdata/timeline_golden.txt"));
    }

    #[test]
    fn test_orders_each_day() {
        let timeline = timeline(date(2026, 10, 22), 61);
        let entries: Vec<(usize, Entry)> = timeline.mail().collect();
        let day_5: Vec<Entry> = entries
            .iter()
            .filter(|(day, _)| *day == 5)
            .map(|(_, entry)| *entry)
            .collect();
        assert_eq!(day_5, [Entry::Main(4), Entry::Gazette(0), Entry::Creepy(0)]);
        assert!(entries.contains(&(11, Entry::Recipe(10))));
        assert!(entries.contains(&(40, Entry::Postcard(0))));
        assert!(entries.contains(&(11, Entry::Neighbours(0))));
        assert!(entries.contains(&(12, Entry::Marcel(0))));
        assert!(
            !entries
                .iter()
                .any(|(_, entry)| *entry == Entry::Postcard(4))
        );
    }

    #[test]
    fn test_delivered_by_and_mail_at() {
        let timeline = timeline(date(2026, 10, 22), 61);
        assert_eq!(timeline.delivered_by(1), 1);
        assert_eq!(timeline.delivered_by(5), 7);
        assert_eq!(timeline.mail_at(6), Some((5, Entry::Creepy(0))));
    }

    struct Dummy;

    impl StorySource for Dummy {
        fn beats(&self, _clock: &StoryClock, _today: usize) -> Vec<Beat> {
            vec![Beat::mail(3, Entry::Sam(4))]
        }
    }

    #[test]
    fn test_registering_a_source_adds_its_beats() {
        let clock = StoryClock::new(date(2026, 10, 22), 0);
        let mut registered = sources::registered(VILLAGE_COUNT);
        registered.push(Box::new(Dummy));
        let timeline = Timeline::new(&registered, &clock, 10);

        let day_3: Vec<Entry> = timeline
            .mail()
            .filter(|(day, _)| *day == 3)
            .map(|(_, entry)| entry)
            .collect();
        assert_eq!(day_3, [Entry::Main(2), Entry::Sam(4)]);
    }
}
