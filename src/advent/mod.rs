pub mod chocolates;
pub mod view;

use chocolates::{CHOCOLATES, Chocolate};
use chrono::{Datelike, NaiveDate};
use std::collections::BTreeSet;

pub const DAYS: u32 = CHOCOLATES.len() as u32;

#[derive(Clone, Copy)]
pub enum Cell {
    Received(&'static Chocolate),
    Missed,
    Closed(u32),
}

pub fn is_advent(date: NaiveDate) -> bool {
    date.month() == 12 && date.day() <= DAYS
}

pub fn chocolate(day: u32) -> Option<&'static Chocolate> {
    CHOCOLATES.get(usize::try_from(day.checked_sub(1)?).ok()?)
}

pub fn due_day(received: &BTreeSet<u32>, today: NaiveDate) -> Option<u32> {
    (is_advent(today) && !received.contains(&today.day())).then_some(today.day())
}

pub fn cell(day: u32, received: &BTreeSet<u32>, today: NaiveDate) -> Cell {
    match chocolate(day) {
        Some(chocolate) if received.contains(&day) => Cell::Received(chocolate),
        _ if today.month() == 12 && day < today.day() => Cell::Missed,
        _ => Cell::Closed(day),
    }
}

pub fn cells(received: &BTreeSet<u32>, today: NaiveDate) -> Vec<Cell> {
    (1..=DAYS).map(|day| cell(day, received, today)).collect()
}

pub fn opening_day(today: NaiveDate) -> u32 {
    if today.month() == 12 {
        today.day().min(DAYS)
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn december(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 12, day).unwrap()
    }

    #[test]
    fn test_advent_runs_from_the_first_to_the_twenty_fourth_of_december() {
        assert!(!is_advent(NaiveDate::from_ymd_opt(2026, 11, 30).unwrap()));
        assert!(is_advent(december(1)));
        assert!(is_advent(december(24)));
        assert!(!is_advent(december(25)));
        assert!(!is_advent(december(31)));
        assert!(!is_advent(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()));
    }

    #[test]
    fn test_chocolate_of_the_day_is_its_number() {
        assert_eq!(chocolate(1).map(|c| c.name), Some(CHOCOLATES[0].name));
        assert_eq!(chocolate(24).map(|c| c.name), Some(CHOCOLATES[23].name));
        assert!(chocolate(0).is_none());
        assert!(chocolate(25).is_none());
    }

    #[test]
    fn test_chocolate_is_due_once_a_day_during_advent() {
        let mut received = BTreeSet::new();
        assert_eq!(due_day(&received, december(1)), Some(1));
        received.insert(1);
        assert_eq!(due_day(&received, december(1)), None);
        assert_eq!(due_day(&received, december(2)), Some(2));
        assert_eq!(due_day(&received, december(26)), None);
        assert_eq!(
            due_day(&received, NaiveDate::from_ymd_opt(2026, 11, 30).unwrap()),
            None
        );
    }

    #[test]
    fn test_cells_show_received_chocolates_empty_past_days_and_closed_doors() {
        let received = BTreeSet::from([1, 3]);
        let cells = cells(&received, december(4));
        assert_eq!(cells.len(), 24);
        assert!(
            matches!(cells[0], Cell::Received(chocolate) if chocolate.name == CHOCOLATES[0].name)
        );
        assert!(matches!(cells[1], Cell::Missed));
        assert!(matches!(cells[2], Cell::Received(_)));
        assert!(matches!(cells[3], Cell::Closed(4)));
        assert!(matches!(cells[23], Cell::Closed(24)));
    }

    #[test]
    fn test_after_christmas_every_unreceived_day_is_empty() {
        let cells = cells(&BTreeSet::from([24]), december(31));
        assert!(matches!(cells[0], Cell::Missed));
        assert!(matches!(cells[23], Cell::Received(_)));
    }

    #[test]
    fn test_calendar_opens_on_today_or_the_last_door() {
        assert_eq!(opening_day(december(7)), 7);
        assert_eq!(opening_day(december(28)), 24);
        assert_eq!(
            opening_day(NaiveDate::from_ymd_opt(2026, 11, 28).unwrap()),
            1
        );
    }
}
