use chrono::{Duration, NaiveDate, NaiveDateTime};

/// Counts Jours de jeu: day 1 is the start date, each next day begins at the mail hour.
#[derive(Clone, Copy, Debug)]
pub struct StoryClock {
    start: NaiveDate,
    mail_hour: u32,
}

impl StoryClock {
    pub fn new(start: NaiveDate, mail_hour: u32) -> Self {
        Self { start, mail_hour }
    }

    /// Jour de jeu at `now`; never below 1, so a clock set in the future stays on day 1.
    pub fn day_on(&self, now: NaiveDateTime) -> usize {
        let date = (now - Duration::hours(i64::from(self.mail_hour))).date();
        (date - self.start).num_days().max(0) as usize + 1
    }

    /// Calendar date of a Jour de jeu.
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
}
