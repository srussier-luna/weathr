use chrono::{Datelike, Local, NaiveDate};

const NEW_YEAR_WINDOW_DAYS: i64 = 7;
const EASTER_WINDOW_DAYS: i64 = 7;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Season {
    Spring,
    #[default]
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn on(month: u32, day: u32) -> Self {
        match (month, day) {
            (3, 20..) | (4 | 5, _) | (6, ..=20) => Self::Spring,
            (6, 21..) | (7 | 8, _) | (9, ..=21) => Self::Summer,
            (9, 22..) | (10 | 11, _) | (12, ..=20) => Self::Autumn,
            _ => Self::Winter,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MeteorShower {
    Perseids,
    Geminids,
}

impl MeteorShower {
    pub fn on(month: u32, day: u32) -> Option<Self> {
        match (month, day) {
            (8, 9..=15) => Some(Self::Perseids),
            (12, 10..=16) => Some(Self::Geminids),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Holiday {
    pub is_december: bool,
    pub is_christmas: bool,
    pub is_national_day: bool,
    pub is_halloween: bool,
    pub is_april_fools: bool,
    pub new_year: Option<i32>,
    pub is_spring: bool,
    pub is_easter: bool,
    pub meteor_shower: Option<MeteorShower>,
    pub is_winter: bool,
    pub season: Season,
}

impl Holiday {
    pub fn today() -> Self {
        Self::on(Local::now().date_naive())
    }

    pub fn on(date: NaiveDate) -> Self {
        let (month, day) = (date.month(), date.day());

        Self {
            is_december: month == 12,
            is_christmas: month == 12 && matches!(day, 24 | 25),
            is_national_day: month == 7 && matches!(day, 13 | 14),
            is_halloween: month == 10,
            is_april_fools: month == 4 && day == 1,
            new_year: Self::new_year_around(date),
            is_spring: (3, 20) <= (month, day) && (month, day) <= (6, 20),
            is_easter: easter_sunday(date.year())
                .is_some_and(|easter| (date - easter).num_days().abs() <= EASTER_WINDOW_DAYS),
            meteor_shower: MeteorShower::on(month, day),
            is_winter: matches!(month, 12 | 1 | 2),
            season: Season::on(month, day),
        }
    }

    pub fn christmas() -> Self {
        Self {
            is_december: true,
            is_christmas: true,
            is_winter: true,
            season: Season::Winter,
            ..Self::default()
        }
    }

    pub fn national_day() -> Self {
        Self {
            is_national_day: true,
            ..Self::default()
        }
    }

    pub fn halloween() -> Self {
        Self {
            is_halloween: true,
            season: Season::Autumn,
            ..Self::default()
        }
    }

    pub fn new_year(year: i32) -> Self {
        Self {
            new_year: Some(year),
            is_winter: true,
            season: Season::Winter,
            ..Self::default()
        }
    }

    pub fn easter() -> Self {
        Self {
            is_spring: true,
            is_easter: true,
            season: Season::Spring,
            ..Self::default()
        }
    }

    pub fn perseids() -> Self {
        Self {
            meteor_shower: Some(MeteorShower::Perseids),
            ..Self::default()
        }
    }

    pub fn geminids() -> Self {
        Self {
            is_december: true,
            meteor_shower: Some(MeteorShower::Geminids),
            is_winter: true,
            season: Season::Autumn,
            ..Self::default()
        }
    }

    pub fn has_fireworks(&self) -> bool {
        self.is_national_day || self.new_year.is_some()
    }

    fn new_year_around(date: NaiveDate) -> Option<i32> {
        let next_year = date.year() + 1;
        let upcoming = NaiveDate::from_ymd_opt(next_year, 1, 1)?;
        let current = NaiveDate::from_ymd_opt(date.year(), 1, 1)?;

        if (upcoming - date).num_days() <= NEW_YEAR_WINDOW_DAYS {
            Some(next_year)
        } else if (date - current).num_days() <= NEW_YEAR_WINDOW_DAYS {
            Some(date.year())
        } else {
            None
        }
    }
}

fn easter_sunday(year: i32) -> Option<NaiveDate> {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = (h + l - 7 * m + 114) % 31 + 1;

    NaiveDate::from_ymd_opt(year, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn test_easter_sunday() {
        assert_eq!(easter_sunday(2024), Some(date(2024, 3, 31)));
        assert_eq!(easter_sunday(2025), Some(date(2025, 4, 20)));
        assert_eq!(easter_sunday(2026), Some(date(2026, 4, 5)));
        assert_eq!(easter_sunday(2027), Some(date(2027, 3, 28)));
    }

    #[test]
    fn test_easter_window() {
        assert!(Holiday::on(date(2026, 3, 29)).is_easter);
        assert!(Holiday::on(date(2026, 4, 12)).is_easter);
        assert!(!Holiday::on(date(2026, 4, 13)).is_easter);
    }

    fn meteor_shower(year: i32, month: u32, day: u32) -> Option<MeteorShower> {
        Holiday::on(date(year, month, day)).meteor_shower
    }

    #[test]
    fn test_perseids_window() {
        let perseids = Some(MeteorShower::Perseids);

        assert!(meteor_shower(2026, 8, 8).is_none());
        assert!(meteor_shower(2026, 8, 9) == perseids);
        assert!(meteor_shower(2026, 8, 15) == perseids);
        assert!(meteor_shower(2026, 8, 16).is_none());
        assert!(meteor_shower(2027, 8, 12) == perseids);
        assert!(meteor_shower(2027, 7, 12).is_none());
    }

    #[test]
    fn test_geminids_window() {
        let geminids = Some(MeteorShower::Geminids);

        assert!(meteor_shower(2026, 12, 9).is_none());
        assert!(meteor_shower(2026, 12, 10) == geminids);
        assert!(meteor_shower(2026, 12, 13) == geminids);
        assert!(meteor_shower(2026, 12, 16) == geminids);
        assert!(meteor_shower(2026, 12, 17).is_none());
        assert!(meteor_shower(2026, 12, 20).is_none());
        assert!(meteor_shower(2027, 12, 14) == geminids);
        assert!(meteor_shower(2027, 11, 13).is_none());
    }

    #[test]
    fn test_new_year_window() {
        assert_eq!(Holiday::on(date(2026, 12, 24)).new_year, None);
        assert_eq!(Holiday::on(date(2026, 12, 25)).new_year, Some(2027));
        assert_eq!(Holiday::on(date(2027, 1, 8)).new_year, Some(2027));
        assert_eq!(Holiday::on(date(2027, 1, 9)).new_year, None);
    }

    #[test]
    fn test_season_boundaries() {
        assert!(Season::on(3, 19) == Season::Winter);
        assert!(Season::on(3, 20) == Season::Spring);
        assert!(Season::on(6, 21) == Season::Summer);
        assert!(Season::on(9, 22) == Season::Autumn);
        assert!(Season::on(12, 21) == Season::Winter);
    }

    #[test]
    fn test_april_fools_is_only_the_first_of_april() {
        assert!(!Holiday::on(date(2026, 3, 31)).is_april_fools);
        assert!(Holiday::on(date(2026, 4, 1)).is_april_fools);
        assert!(!Holiday::on(date(2026, 4, 2)).is_april_fools);
        assert!(Holiday::on(date(2027, 4, 1)).is_april_fools);
    }

    #[test]
    fn test_seasons() {
        assert!(Holiday::on(date(2026, 10, 1)).is_halloween);
        assert!(!Holiday::on(date(2026, 11, 1)).is_halloween);
        assert!(Holiday::on(date(2026, 3, 20)).is_spring);
        assert!(Holiday::on(date(2026, 6, 20)).is_spring);
        assert!(!Holiday::on(date(2026, 6, 21)).is_spring);
    }
}
