use super::clock::StoryClock;
use super::timeline::Timeline;
use chrono::NaiveDate;

pub const FULL_BOX: usize = 7;

pub fn is_box_full(timeline: &Timeline, day: usize, letters_read: usize) -> bool {
    timeline.unread_village(day, letters_read) >= FULL_BOX
}

pub fn hold_full_box(
    clock: &mut StoryClock,
    timeline: &Timeline,
    letters_read: usize,
    checked: NaiveDate,
    today: NaiveDate,
) {
    for eve in checked.iter_days().take_while(|eve| *eve < today) {
        let day = clock.day_of(eve);
        if eve >= clock.start() && is_box_full(timeline, day, letters_read) {
            clock.pause(day);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::event::{Beat, Entry};
    use crate::story::sources::{self, StorySource};
    use chrono::{Datelike, NaiveDateTime, Weekday};

    const VILLAGE_COUNT: usize = 60;
    const MAIL_HOUR: u32 = 9;

    fn date(month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, month, day).unwrap()
    }

    fn at(date: NaiveDate, hour: u32, minute: u32) -> NaiveDateTime {
        date.and_hms_opt(hour, minute, 0).unwrap()
    }

    fn timeline(clock: &StoryClock, today: usize) -> Timeline {
        Timeline::new(&sources::registered(VILLAGE_COUNT), clock, today)
    }

    fn live(clock: &mut StoryClock, letters_read: usize, checked: NaiveDate, now: NaiveDateTime) {
        let before = timeline(clock, clock.day_of(checked));
        hold_full_box(clock, &before, letters_read, checked, clock.mail_date(now));
    }

    fn back_after_three_weeks() -> StoryClock {
        let mut clock = StoryClock::new(date(10, 1), MAIL_HOUR);
        live(&mut clock, 0, date(10, 1), at(date(10, 22), 12, 0));
        clock
    }

    #[test]
    fn test_three_weeks_away_leave_seven_unread_letters() {
        let clock = back_after_three_weeks();
        let today = clock.day_on(at(date(10, 22), 12, 0));
        let timeline = timeline(&clock, today);

        assert_eq!(today, 7);
        assert_eq!(timeline.unread_village(today, 0), FULL_BOX);
        assert!(is_box_full(&timeline, today, 0));
    }

    #[test]
    fn test_paused_days_deliver_no_other_mail() {
        let clock = back_after_three_weeks();
        let today = clock.day_on(at(date(10, 22), 12, 0));
        let timeline = timeline(&clock, today);
        let delivered = timeline.first_mail(timeline.delivered_by(today));

        assert!(delivered.iter().all(|(day, _)| *day <= 7));
        assert!(
            delivered
                .iter()
                .all(|(day, _)| clock.date_of(*day) <= date(10, 7))
        );
        let gazettes: Vec<Entry> = delivered
            .iter()
            .map(|(_, entry)| *entry)
            .filter(|entry| matches!(entry, Entry::Gazette(_)))
            .collect();
        assert_eq!(gazettes, [Entry::Gazette(0)]);
    }

    #[test]
    fn test_reading_one_letter_moves_the_day_at_mail_hour() {
        let mut clock = back_after_three_weeks();
        let read = 1;
        assert_eq!(timeline(&clock, 7).mail_at(0), Some((1, Entry::Main(0))));

        live(&mut clock, read, date(10, 22), at(date(10, 23), 8, 59));
        assert_eq!(clock.day_on(at(date(10, 23), 8, 59)), 7);

        live(
            &mut clock,
            read,
            date(10, 22),
            at(date(10, 23), MAIL_HOUR, 0),
        );
        let today = clock.day_on(at(date(10, 23), MAIL_HOUR, 0));
        let timeline = timeline(&clock, today);
        let delivered = timeline.first_mail(timeline.delivered_by(today));

        assert_eq!(today, 8);
        assert!(delivered.contains(&(8, Entry::Main(7))));
        assert_eq!(timeline.unread_village(today, read), FULL_BOX);
    }

    #[test]
    fn test_next_gazette_after_a_pause_falls_on_a_real_monday() {
        let mut clock = back_after_three_weeks();
        let mut checked = date(10, 22);
        for day in checked
            .iter_days()
            .skip(1)
            .take_while(|day| *day <= date(11, 3))
        {
            let read = timeline(&clock, clock.day_of(checked)).delivered_by(clock.day_of(checked));
            live(&mut clock, read, checked, at(day, 12, 0));
            checked = day;
        }
        let today = clock.day_on(at(date(11, 3), 12, 0));
        let gazettes: Vec<NaiveDate> = timeline(&clock, today)
            .mail()
            .filter(|(_, entry)| matches!(entry, Entry::Gazette(_)))
            .map(|(day, _)| clock.date_of(day))
            .collect();

        assert_eq!(gazettes, [date(10, 5), date(10, 26), date(11, 2)]);
        assert!(gazettes.iter().all(|day| day.weekday() == Weekday::Mon));
    }

    struct SmallVillage;

    impl StorySource for SmallVillage {
        fn beats(&self, _clock: &StoryClock, _today: usize) -> Vec<Beat> {
            (0..6)
                .map(|index| Beat::mail(index + 1, Entry::Main(index)))
                .chain((0..3).map(|week| Beat::mail(2 * week + 2, Entry::Gazette(week))))
                .collect()
        }
    }

    #[test]
    fn test_gazettes_never_fill_the_box() {
        let mut clock = StoryClock::new(date(10, 1), MAIL_HOUR);
        let timeline = Timeline::new(&[Box::new(SmallVillage)], &clock, 6);
        assert_eq!(timeline.delivered_by(6), 9);
        assert_eq!(timeline.unread_village(6, 0), 6);
        assert!(!is_box_full(&timeline, 6, 0));

        hold_full_box(&mut clock, &timeline, 0, date(10, 1), date(10, 10));
        assert!(clock.pauses().is_empty());
        assert_eq!(clock.day_of(date(10, 10)), 10);
    }

    #[test]
    fn test_reading_a_letter_empties_the_full_box() {
        let clock = back_after_three_weeks();
        let timeline = timeline(&clock, 7);
        assert!(is_box_full(&timeline, 7, 0));
        assert!(!is_box_full(&timeline, 7, 1));
    }

    #[test]
    fn test_checked_days_are_never_judged_again() {
        let mut clock = back_after_three_weeks();
        let paused = clock.pauses().clone();
        live(&mut clock, 0, date(10, 22), at(date(10, 22), 23, 0));
        live(&mut clock, 0, date(10, 22), at(date(10, 20), 12, 0));
        assert_eq!(clock.pauses(), &paused);
    }
}
