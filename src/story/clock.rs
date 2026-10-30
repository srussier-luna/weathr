use chrono::{Duration, NaiveDate, NaiveDateTime};
use std::collections::BTreeMap;
use std::num::NonZeroU16;

#[derive(Clone, Debug)]
pub struct StoryClock {
    start: NaiveDate,
    mail_hour: u32,
    next_mail_hour: Option<(NaiveDate, u32)>,
    frozen_day: Option<usize>,
    pauses: BTreeMap<usize, u32>,
}

impl StoryClock {
    pub fn new(start: NaiveDate, mail_hour: u32) -> Self {
        Self {
            start,
            mail_hour,
            next_mail_hour: None,
            frozen_day: None,
            pauses: BTreeMap::new(),
        }
    }

    pub fn with_pauses(self, pauses: BTreeMap<usize, u32>) -> Self {
        Self { pauses, ..self }
    }

    pub fn frozen(today: NaiveDate, mail_hour: u32, day: NonZeroU16) -> Self {
        let day = day.get();
        Self {
            start: today - Duration::days(i64::from(day) - 1),
            mail_hour,
            next_mail_hour: None,
            frozen_day: Some(usize::from(day)),
            pauses: BTreeMap::new(),
        }
    }

    pub fn is_frozen(&self) -> bool {
        self.frozen_day.is_some()
    }

    pub fn start(&self) -> NaiveDate {
        self.start
    }

    pub fn pauses(&self) -> &BTreeMap<usize, u32> {
        &self.pauses
    }

    pub fn pause(&mut self, day: usize) {
        *self.pauses.entry(day).or_default() += 1;
    }

    pub fn mail_hour_on(&self, date: NaiveDate) -> u32 {
        match self.next_mail_hour {
            Some((from, hour)) if date >= from => hour,
            _ => self.mail_hour,
        }
    }

    pub fn change_mail_hour(&mut self, hour: u32, now: NaiveDateTime) {
        let today = now.date();
        self.mail_hour = self.mail_hour_on(today);
        self.next_mail_hour = (hour != self.mail_hour).then(|| (today + Duration::days(1), hour));
    }

    pub fn mail_date(&self, now: NaiveDateTime) -> NaiveDate {
        (now - Duration::hours(i64::from(self.mail_hour_on(now.date())))).date()
    }

    pub fn next_rollover(&self, now: NaiveDateTime) -> NaiveDate {
        self.mail_date(now) + Duration::days(1)
    }

    /// Never below 1, so a clock set in the future stays on day 1.
    pub fn day_on(&self, now: NaiveDateTime) -> usize {
        if let Some(day) = self.frozen_day {
            return day;
        }
        self.day_of(self.mail_date(now))
    }

    pub fn day_of(&self, date: NaiveDate) -> usize {
        let elapsed = (date - self.start).num_days().max(0) as usize;
        let mut paused_before = 0;
        for (&day, &paused) in &self.pauses {
            let reached = day - 1 + paused_before;
            if reached > elapsed {
                break;
            }
            if elapsed <= reached + paused as usize {
                return day;
            }
            paused_before += paused as usize;
        }
        elapsed - paused_before + 1
    }

    pub fn date_of(&self, day: usize) -> NaiveDate {
        let paused: u32 = self.pauses.range(..day).map(|(_, paused)| paused).sum();
        self.start + Duration::days(day as i64 - 1 + i64::from(paused))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn test_day_rolls_over_at_mail_hour() {
        let clock = StoryClock::new(date(2026, 10, 1), 9);
        let morning = date(2026, 10, 8);
        assert_eq!(clock.day_on(morning.and_hms_opt(8, 59, 0).unwrap()), 7);
        assert_eq!(clock.day_on(morning.and_hms_opt(9, 0, 0).unwrap()), 8);
    }

    #[test]
    fn test_next_rollover_is_the_next_mail_hour_date() {
        let clock = StoryClock::new(date(2026, 10, 1), 9);
        let rollover =
            |day, hour| clock.next_rollover(date(2026, 10, day).and_hms_opt(hour, 0, 0).unwrap());
        assert_eq!(rollover(31, 2), date(2026, 10, 31));
        assert_eq!(rollover(31, 8), date(2026, 10, 31));
        assert_eq!(rollover(31, 9), date(2026, 11, 1));
        assert_eq!(rollover(30, 23), date(2026, 10, 31));
    }

    #[test]
    fn test_new_mail_hour_waits_for_the_next_day() {
        let at = |day, hour, minute| date(2026, 10, day).and_hms_opt(hour, minute, 0).unwrap();

        let mut earlier = StoryClock::new(date(2026, 10, 1), 9);
        earlier.change_mail_hour(6, at(8, 10, 0));
        assert_eq!(earlier.day_on(at(8, 10, 0)), 8);
        assert_eq!(earlier.day_on(at(8, 23, 59)), 8);
        assert_eq!(earlier.day_on(at(9, 5, 59)), 8);
        assert_eq!(earlier.day_on(at(9, 6, 0)), 9);
        assert_eq!(earlier.mail_hour_on(date(2026, 10, 8)), 9);
        assert_eq!(earlier.mail_hour_on(date(2026, 10, 9)), 6);

        let mut later = StoryClock::new(date(2026, 10, 1), 9);
        later.change_mail_hour(12, at(8, 8, 0));
        assert_eq!(later.day_on(at(8, 8, 59)), 7);
        assert_eq!(later.day_on(at(8, 9, 0)), 8);
        assert_eq!(later.day_on(at(9, 11, 59)), 8);
        assert_eq!(later.day_on(at(9, 12, 0)), 9);
    }

    #[test]
    fn test_mail_hour_changes_settle_day_after_day() {
        let at = |day, hour| date(2026, 10, day).and_hms_opt(hour, 0, 0).unwrap();
        let mut clock = StoryClock::new(date(2026, 10, 1), 9);
        clock.change_mail_hour(7, at(8, 12));
        clock.change_mail_hour(11, at(9, 12));
        assert_eq!(clock.mail_hour_on(date(2026, 10, 9)), 7);
        assert_eq!(clock.mail_hour_on(date(2026, 10, 10)), 11);
        assert_eq!(clock.day_on(at(10, 10)), 9);
        assert_eq!(clock.day_on(at(10, 11)), 10);

        clock.change_mail_hour(11, at(10, 12));
        clock.change_mail_hour(11, at(10, 13));
        assert_eq!(clock.mail_hour_on(date(2026, 10, 11)), 11);
    }

    #[test]
    fn test_day_never_goes_below_one() {
        let clock = StoryClock::new(date(2026, 10, 1), 9);
        assert_eq!(
            clock.day_on(date(2026, 10, 1).and_hms_opt(3, 0, 0).unwrap()),
            1
        );
        assert_eq!(
            clock.day_on(date(2026, 9, 20).and_hms_opt(12, 0, 0).unwrap()),
            1
        );
    }

    #[test]
    fn test_date_of_maps_days_to_calendar() {
        let clock = StoryClock::new(date(2026, 10, 1), 9);
        assert_eq!(clock.date_of(1), date(2026, 10, 1));
        assert_eq!(clock.date_of(32), date(2026, 11, 1));
    }

    #[test]
    fn test_frozen_clock_stays_on_its_day() {
        let today = date(2026, 10, 26);
        let clock = StoryClock::frozen(today, 9, NonZeroU16::new(8).unwrap());
        assert_eq!(clock.date_of(1), date(2026, 10, 19));
        assert_eq!(clock.date_of(8), today);
        for (day, hour) in [(26, 8), (26, 9), (26, 23), (27, 10), (30, 12)] {
            let now = date(2026, 10, day).and_hms_opt(hour, 0, 0).unwrap();
            assert_eq!(clock.day_on(now), 8);
        }
    }

    #[test]
    fn test_paused_day_holds_and_shifts_later_days_only() {
        let mut clock = StoryClock::new(date(2026, 10, 1), 9);
        clock.pause(7);
        clock.pause(7);
        assert_eq!(clock.date_of(7), date(2026, 10, 7));
        assert_eq!(clock.date_of(8), date(2026, 10, 10));
        for (day, expected) in [(6, 6), (7, 7), (8, 7), (9, 7), (10, 8), (11, 9)] {
            assert_eq!(clock.day_of(date(2026, 10, day)), expected);
        }
    }

    #[test]
    fn test_pauses_round_trip_through_day_of() {
        let clock = StoryClock::new(date(2026, 10, 1), 9).with_pauses(BTreeMap::from([
            (3, 2),
            (10, 5),
            (11, 1),
        ]));
        for day in 1..40 {
            assert_eq!(clock.day_of(clock.date_of(day)), day);
        }
    }

    #[test]
    fn test_frozen_clock_accepts_the_largest_day() {
        let clock = StoryClock::frozen(date(2026, 10, 26), 9, NonZeroU16::MAX);
        assert_eq!(clock.date_of(usize::from(u16::MAX)), date(2026, 10, 26));
    }
}
