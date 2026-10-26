use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::event::{Beat, Entry};
use chrono::{Datelike, Weekday};

/// The Gazette, every Monday.
pub struct Gazettes;

impl StorySource for Gazettes {
    fn beats(&self, clock: &StoryClock, today: usize) -> Vec<Beat> {
        (1..=today)
            .filter(|day| clock.date_of(*day).weekday() == Weekday::Mon)
            .enumerate()
            .map(|(week, day)| Beat::mail(day, Entry::Gazette(week)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::event::StoryEvent;
    use chrono::NaiveDate;

    #[test]
    fn test_gazettes_fall_on_mondays() {
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 10, 22).unwrap(), 9);
        let beats = Gazettes.beats(&clock, 60);

        assert_eq!(beats.len(), 8);
        assert_eq!(beats[0], Beat::mail(5, Entry::Gazette(0)));
        for (week, beat) in beats.iter().enumerate() {
            assert_eq!(clock.date_of(beat.day).weekday(), Weekday::Mon);
            assert_eq!(beat.event, StoryEvent::Mail(Entry::Gazette(week)));
        }
    }
}
