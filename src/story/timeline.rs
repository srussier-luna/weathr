use super::clock::StoryClock;
use super::event::{Beat, Entry, StoryEvent, Thing};
use super::sources::StorySource;

pub struct Timeline {
    mail: Vec<(usize, Entry)>,
    arrivals: Vec<(usize, Thing)>,
}

impl Timeline {
    pub fn new(sources: &[Box<dyn StorySource>], clock: &StoryClock, today: usize) -> Self {
        let mut beats: Vec<Beat> = sources
            .iter()
            .flat_map(|source| source.beats(clock, today))
            .collect();
        beats.sort_by_key(|beat| (beat.day, beat.event.order()));
        let mut mail = Vec::new();
        let mut arrivals = Vec::new();
        for beat in beats {
            match beat.event {
                StoryEvent::Mail(entry) => mail.push((beat.day, entry)),
                StoryEvent::Arrival(thing) => arrivals.push((beat.day, thing)),
            }
        }
        Self { mail, arrivals }
    }

    /// `Progress::letters_read` is a prefix length into this sequence.
    pub fn mail(&self) -> impl Iterator<Item = (usize, Entry)> + '_ {
        self.mail.iter().copied()
    }

    pub fn first_mail(&self, count: usize) -> &[(usize, Entry)] {
        &self.mail[..count.min(self.mail.len())]
    }

    pub fn arrived_by(&self, day: usize) -> impl Iterator<Item = Thing> + '_ {
        self.arrivals
            .iter()
            .take_while(move |(arrival, _)| *arrival <= day)
            .map(|(_, thing)| *thing)
    }

    pub fn delivered_by(&self, day: usize) -> usize {
        self.mail
            .iter()
            .take_while(|(delivery, _)| *delivery <= day)
            .count()
    }

    pub fn mail_at(&self, index: usize) -> Option<(usize, Entry)> {
        self.mail.get(index).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::sources;
    use chrono::NaiveDate;
    use std::num::NonZeroU16;

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
        assert_eq!(timeline.first_mail(7).last(), Some(&(5, Entry::Creepy(0))));
        assert_eq!(
            timeline.first_mail(usize::MAX).len(),
            timeline.mail().count()
        );
    }

    #[test]
    fn test_arrived_by() {
        let timeline = timeline(date(2026, 10, 1), 61);
        assert_eq!(timeline.arrived_by(0).count(), 0);
        assert!(!timeline.arrived_by(3).any(|thing| thing == Thing::Bench));
        assert!(timeline.arrived_by(4).any(|thing| thing == Thing::Bench));
        assert!(!timeline.arrived_by(5).any(|thing| thing == Thing::CatBow));
        assert!(timeline.arrived_by(6).any(|thing| thing == Thing::CatBow));
    }

    #[test]
    fn test_arrivals_stay_out_of_mail() {
        let timeline = timeline(date(2026, 10, 1), 61);
        let village = timeline
            .mail()
            .filter(|(_, entry)| matches!(entry, Entry::Main(_)))
            .count();
        assert_eq!(village, VILLAGE_COUNT);
        assert_eq!(timeline.delivered_by(4), 4);
    }

    #[test]
    fn test_frozen_clock_delivers_every_mail_up_to_its_day() {
        let today = date(2026, 10, 26);
        let frozen = StoryClock::frozen(today, 9, NonZeroU16::new(12).unwrap());
        let late = date(2026, 10, 27).and_hms_opt(10, 0, 0).unwrap();
        let day = frozen.day_on(late);
        let preview = Timeline::new(&sources::registered(VILLAGE_COUNT), &frozen, day);

        let expected = timeline(frozen.date_of(1), 12)
            .mail()
            .filter(|(delivery, _)| *delivery <= 12)
            .count();
        assert_eq!(day, 12);
        assert_eq!(preview.delivered_by(day), expected);
        assert!(expected > 12);
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
