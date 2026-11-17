use super::clock::StoryClock;
use super::content::arcanes::{ARCANES, FIRST_NUMBER, SAMPLE_NUMBER};
use super::event::Entry;
use super::postman_note::MailStep;
use crate::progress::Progress;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

const HOURS_AFTER_MAIL: u32 = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delivery {
    Nothing,
    Due,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArcaneRound {
    pub hour: u32,
    pub delivery: Delivery,
    pub is_arcane_waiting: bool,
}

fn passage_hour(clock: &StoryClock, date: NaiveDate) -> u32 {
    (clock.mail_hour_on(date) + HOURS_AFTER_MAIL) % 24
}

fn passage_at(clock: &StoryClock, date: NaiveDate) -> NaiveDateTime {
    date.and_time(NaiveTime::MIN) + Duration::hours(i64::from(passage_hour(clock, date)))
}

fn nearest_passage(clock: &StoryClock, now: NaiveDateTime) -> NaiveDate {
    let today = now.date();
    let distance = |date: NaiveDate| (passage_at(clock, date) - now).abs();
    today
        .succ_opt()
        .filter(|tomorrow| distance(*tomorrow) < distance(today))
        .unwrap_or(today)
}

pub fn reopened(courrier: &[(usize, Entry)], selected: usize) -> Option<MailStep> {
    Some(match courrier.get(selected)? {
        (_, Entry::Arcane(number)) => MailStep::Arcane(*number),
        _ => MailStep::Letter(
            courrier[..selected]
                .iter()
                .filter(|(_, entry)| !matches!(entry, Entry::Arcane(_)))
                .count(),
        ),
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArcanePost {
    unlocked_on: Option<NaiveDate>,
    received_on: Vec<NaiveDate>,
    read: usize,
    passed_on: Option<NaiveDate>,
    first_number: usize,
    count: usize,
    in_memory: bool,
}

impl ArcanePost {
    pub fn saved(progress: &Progress) -> Self {
        Self {
            unlocked_on: progress.arcanes_unlocked_date(),
            received_on: progress.arcanes_received_dates(),
            read: progress.arcanes_read,
            passed_on: progress.arcanes_passed_date(),
            first_number: FIRST_NUMBER,
            count: ARCANES.len(),
            in_memory: false,
        }
    }

    pub fn sample(today: NaiveDate) -> Self {
        Self {
            unlocked_on: Some(today),
            received_on: Vec::new(),
            read: 0,
            passed_on: None,
            first_number: SAMPLE_NUMBER,
            count: 1,
            in_memory: true,
        }
    }

    pub fn write_to(&self, progress: &mut Progress) -> bool {
        if self.in_memory {
            return false;
        }
        progress.set_arcanes_unlocked_date(self.unlocked_on);
        progress.set_arcanes_received_dates(&self.received_on);
        progress.arcanes_read = self.read;
        progress.set_arcanes_passed_date(self.passed_on);
        true
    }

    pub fn is_unlocked(&self) -> bool {
        self.unlocked_on.is_some()
    }

    fn is_unlocked_on(&self, date: NaiveDate) -> bool {
        self.unlocked_on
            .is_some_and(|unlocked_on| unlocked_on <= date)
    }

    fn delivers_on(&self, date: NaiveDate) -> bool {
        self.is_unlocked_on(date)
            && self.passed_on != Some(date)
            && self.waiting().is_none()
            && self.received_on.len() < self.count
    }

    fn delivery_on(&self, date: NaiveDate) -> Delivery {
        if self.received_on.last() == Some(&date) {
            Delivery::Done
        } else if self.delivers_on(date) {
            Delivery::Due
        } else {
            Delivery::Nothing
        }
    }

    pub fn round(&self, clock: &StoryClock, now: NaiveDateTime) -> Option<ArcaneRound> {
        let date = nearest_passage(clock, now);
        self.is_unlocked().then(|| ArcaneRound {
            hour: passage_hour(clock, date),
            delivery: self.delivery_on(date),
            is_arcane_waiting: self.waiting().is_some(),
        })
    }

    pub fn waiting(&self) -> Option<usize> {
        (self.read < self.received_on.len()).then_some(self.first_number + self.read)
    }

    pub fn mark_read(&mut self, number: usize) -> bool {
        let is_waiting = self.waiting() == Some(number);
        if is_waiting {
            self.read += 1;
        }
        is_waiting
    }

    pub fn courrier(&self, letters: &[(usize, Entry)], clock: &StoryClock) -> Vec<(usize, Entry)> {
        let mut courrier = letters.to_vec();
        courrier.extend(
            self.received_on
                .iter()
                .take(self.read)
                .zip(self.first_number..)
                .map(|(date, number)| {
                    let day = clock.day_of(clock.mail_date(passage_at(clock, *date)));
                    (day, Entry::Arcane(number))
                }),
        );
        courrier.sort_by_key(|(day, _)| *day);
        courrier
    }

    pub fn pass(&mut self, clock: &StoryClock, now: NaiveDateTime) -> bool {
        let today = now.date();
        if now.hour() < passage_hour(clock, today)
            || !self.is_unlocked_on(today)
            || self.passed_on == Some(today)
        {
            return false;
        }
        if self.delivers_on(today) {
            self.received_on.push(today);
        }
        self.passed_on = Some(today);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRIDAY: u32 = 20;
    const SATURDAY: u32 = 21;
    const SUNDAY: u32 = 22;
    const MONDAY: u32 = 23;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, day).unwrap()
    }

    fn at(day: u32, hour: u32) -> NaiveDateTime {
        date(day).and_hms_opt(hour, 0, 0).unwrap()
    }

    fn unlocked(count: usize) -> ArcanePost {
        ArcanePost {
            unlocked_on: Some(date(1)),
            received_on: Vec::new(),
            read: 0,
            passed_on: None,
            first_number: FIRST_NUMBER,
            count,
            in_memory: false,
        }
    }

    fn clock() -> StoryClock {
        StoryClock::new(date(1), 9)
    }

    fn open(post: &mut ArcanePost, day: u32, hour: u32) {
        post.pass(&clock(), at(day, hour));
    }

    #[test]
    fn test_passage_comes_twelve_hours_after_the_mail() {
        let passage = |mail_hour| passage_hour(&StoryClock::new(date(1), mail_hour), date(FRIDAY));
        assert_eq!(passage(9), 21);
        assert_eq!(passage(12), 0);
        assert_eq!(passage(14), 2);
    }

    #[test]
    fn test_first_arcane_waits_for_the_evening_passage() {
        assert_eq!(date(FRIDAY).format("%A").to_string(), "Friday");
        let mut post = unlocked(3);
        open(&mut post, FRIDAY, 20);
        assert!(post.received_on.is_empty());
        open(&mut post, FRIDAY, 21);
        assert_eq!(post.received_on, [date(FRIDAY)]);
    }

    #[test]
    fn test_unread_arcane_holds_back_the_next_one() {
        let mut post = unlocked(3);
        open(&mut post, FRIDAY, 21);
        open(&mut post, SUNDAY, 22);
        assert_eq!(post.received_on, [date(FRIDAY)]);
        assert_eq!(post.waiting(), Some(1));
        assert!(!post.mark_read(2));
    }

    #[test]
    fn test_reading_after_the_passage_waits_for_the_next_evening() {
        let mut post = unlocked(3);
        open(&mut post, FRIDAY, 21);
        open(&mut post, SUNDAY, 22);
        assert!(post.mark_read(1));
        open(&mut post, SUNDAY, 23);
        assert_eq!(post.received_on.len(), 1);
        open(&mut post, MONDAY, 20);
        assert_eq!(post.received_on.len(), 1);
        open(&mut post, MONDAY, 21);
        assert_eq!(post.received_on, [date(FRIDAY), date(MONDAY)]);
        assert_eq!(post.waiting(), Some(2));
    }

    #[test]
    fn test_closed_days_do_not_pile_up() {
        let mut post = unlocked(3);
        open(&mut post, FRIDAY, 21);
        assert!(post.mark_read(1));
        open(&mut post, SUNDAY, 22);
        open(&mut post, SUNDAY, 23);
        assert_eq!(post.received_on, [date(FRIDAY), date(SUNDAY)]);
        assert!(!post.received_on.contains(&date(SATURDAY)));
    }

    #[test]
    fn test_after_the_last_arcane_the_passage_brings_nothing() {
        let mut post = unlocked(1);
        open(&mut post, FRIDAY, 21);
        assert!(post.mark_read(1));
        open(&mut post, SATURDAY, 21);
        assert_eq!(post.passed_on, Some(date(SATURDAY)));
        assert_eq!(post.received_on, [date(FRIDAY)]);
    }

    #[test]
    fn test_locked_post_never_delivers() {
        let mut post = ArcanePost::saved(&Progress::default());
        assert!(!post.pass(&clock(), at(FRIDAY, 22)));
        assert_eq!(post.round(&clock(), at(FRIDAY, 22)), None);
        assert!(post.received_on.is_empty());
    }

    #[test]
    fn test_old_save_keeps_the_secret_locked() {
        let progress: Progress =
            serde_json::from_str(r#"{"started_on":"2026-09-01","letters_read":42}"#).unwrap();
        let post = ArcanePost::saved(&progress);
        assert_eq!(post.unlocked_on, None);
        assert!(post.received_on.is_empty());
        assert_eq!(post.read, 0);
    }

    #[test]
    fn test_saved_post_round_trips_through_the_save() {
        let mut post = ArcanePost {
            count: ARCANES.len(),
            ..unlocked(0)
        };
        post.received_on = vec![date(FRIDAY), date(MONDAY)];
        post.read = 1;
        post.passed_on = Some(date(MONDAY));
        let mut progress = Progress::default();
        assert!(post.write_to(&mut progress));
        let saved = serde_json::to_string(&progress).unwrap();
        let loaded: Progress = serde_json::from_str(&saved).unwrap();
        assert_eq!(ArcanePost::saved(&loaded), post);
    }

    #[test]
    fn test_sample_arrives_at_the_passage_and_stays_out_of_the_save() {
        let mut post = ArcanePost::sample(date(FRIDAY));
        open(&mut post, FRIDAY, 21);
        assert_eq!(post.waiting(), Some(SAMPLE_NUMBER));
        let mut progress = Progress::default();
        assert!(!post.write_to(&mut progress));
        assert_eq!(progress.arcanes_unlocked_date(), None);
        assert!(progress.arcanes_received_dates().is_empty());
    }

    #[test]
    fn test_midnight_arcane_sorts_with_the_mail_day_of_its_passage() {
        let clock = StoryClock::new(date(1), 12);
        let mut post = unlocked(1);
        assert!(post.pass(&clock, at(SATURDAY, 0)));
        assert!(post.mark_read(1));
        let letters = [
            (clock.day_of(date(FRIDAY)), Entry::Main(19)),
            (clock.day_of(date(SATURDAY)), Entry::Main(20)),
        ];
        let entries: Vec<Entry> = post
            .courrier(&letters, &clock)
            .iter()
            .map(|(_, entry)| *entry)
            .collect();
        assert_eq!(
            entries,
            [Entry::Main(19), Entry::Arcane(1), Entry::Main(20)]
        );
    }

    #[test]
    fn test_courrier_lists_read_arcanes_by_arrival() {
        let clock = StoryClock::new(date(1), 9);
        let mut post = unlocked(3);
        open(&mut post, FRIDAY, 21);
        assert!(post.mark_read(1));
        open(&mut post, SUNDAY, 21);
        let letters = [
            (clock.day_of(date(FRIDAY)), Entry::Main(19)),
            (clock.day_of(date(SATURDAY)), Entry::Main(20)),
            (clock.day_of(date(SUNDAY)), Entry::Main(21)),
        ];
        let courrier = post.courrier(&letters, &clock);
        let entries: Vec<Entry> = courrier.iter().map(|(_, entry)| *entry).collect();
        assert_eq!(
            entries,
            [
                Entry::Main(19),
                Entry::Arcane(1),
                Entry::Main(20),
                Entry::Main(21)
            ]
        );
        assert_eq!(clock.date_of(courrier[1].0), date(FRIDAY));
    }

    #[test]
    fn test_reopening_from_the_courrier_finds_the_right_mail() {
        let courrier = [
            (1, Entry::Main(0)),
            (1, Entry::Arcane(1)),
            (2, Entry::Gazette(0)),
            (3, Entry::Arcane(2)),
        ];
        assert_eq!(reopened(&courrier, 0), Some(MailStep::Letter(0)));
        assert_eq!(reopened(&courrier, 1), Some(MailStep::Arcane(1)));
        assert_eq!(reopened(&courrier, 2), Some(MailStep::Letter(1)));
        assert_eq!(reopened(&courrier, 3), Some(MailStep::Arcane(2)));
        assert_eq!(reopened(&courrier, 4), None);
    }

    #[test]
    fn test_round_tells_the_mage_whether_to_stop() {
        let mut post = unlocked(2);
        let delivery =
            |post: &ArcanePost, day, hour| post.round(&clock(), at(day, hour)).unwrap().delivery;
        assert_eq!(delivery(&post, FRIDAY, 20), Delivery::Due);
        open(&mut post, FRIDAY, 21);
        assert_eq!(delivery(&post, FRIDAY, 21), Delivery::Done);
        assert!(
            post.round(&clock(), at(FRIDAY, 21))
                .unwrap()
                .is_arcane_waiting
        );
        assert_eq!(delivery(&post, SATURDAY, 20), Delivery::Nothing);
        assert!(post.mark_read(1));
        open(&mut post, SATURDAY, 21);
        assert!(post.mark_read(2));
        assert_eq!(delivery(&post, SUNDAY, 20), Delivery::Nothing);
    }

    #[test]
    fn test_midnight_passage_belongs_to_the_coming_day() {
        let clock = StoryClock::new(date(1), 12);
        let post = unlocked(2);
        let just_before = date(FRIDAY).and_hms_opt(23, 59, 50).unwrap();
        let round = post.round(&clock, just_before).unwrap();
        assert_eq!(round.hour, 0);
        assert_eq!(nearest_passage(&clock, just_before), date(SATURDAY));
        assert_eq!(nearest_passage(&clock, at(FRIDAY, 0)), date(FRIDAY));
        assert_eq!(
            nearest_passage(&StoryClock::new(date(1), 9), at(FRIDAY, 23)),
            date(FRIDAY)
        );
    }
}
