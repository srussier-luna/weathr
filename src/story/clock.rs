use chrono::{Duration, NaiveDate, NaiveDateTime};

#[derive(Clone, Copy, Debug)]
pub struct StoryClock {
    start: NaiveDate,
    mail_hour: u32,
    frozen_day: Option<usize>,
}

impl StoryClock {
    pub fn new(start: NaiveDate, mail_hour: u32) -> Self {
        Self {
            start,
            mail_hour,
            frozen_day: None,
        }
    }

    pub fn frozen(today: NaiveDate, mail_hour: u32, day: usize) -> Self {
        let day = day.max(1);
        Self {
            start: today - Duration::days(day as i64 - 1),
            mail_hour,
            frozen_day: Some(day),
        }
    }

    /// Never below 1, so a clock set in the future stays on day 1.
    pub fn day_on(&self, now: NaiveDateTime) -> usize {
        if let Some(day) = self.frozen_day {
            return day;
        }
        let date = (now - Duration::hours(i64::from(self.mail_hour))).date();
        (date - self.start).num_days().max(0) as usize + 1
    }

    pub fn date_of(&self, day: usize) -> NaiveDate {
        self.start + Duration::days(day as i64 - 1)
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
        let clock = StoryClock::frozen(today, 9, 8);
        assert_eq!(clock.date_of(1), date(2026, 10, 19));
        assert_eq!(clock.date_of(8), today);
        for (day, hour) in [(26, 8), (26, 9), (26, 23), (27, 10), (30, 12)] {
            let now = date(2026, 10, day).and_hms_opt(hour, 0, 0).unwrap();
            assert_eq!(clock.day_on(now), 8);
        }
    }
}
