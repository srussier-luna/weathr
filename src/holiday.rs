use crate::clock;
use chrono::{Datelike, Days, NaiveDate, Weekday};

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
    pub is_epiphany: bool,
    pub is_neighbours_party: bool,
    pub is_music_day: bool,
    pub is_candlemas: bool,
    pub is_may_day: bool,
    pub meteor_shower: Option<MeteorShower>,
    pub is_winter: bool,
    pub season: Season,
}

impl Holiday {
    pub fn today() -> Self {
        Self::on(clock::now().date_naive())
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
            is_epiphany: month == 1 && day == 6,
            is_neighbours_party: last_friday_of_may(date.year()) == Some(date),
            is_music_day: month == 6 && day == 21,
            is_candlemas: month == 2 && day == 2,
            is_may_day: month == 5 && day == 1,
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

    pub fn neighbours_party() -> Self {
        Self {
            is_spring: true,
            is_neighbours_party: true,
            season: Season::Spring,
            ..Self::default()
        }
    }

    pub fn music_day() -> Self {
        Self {
            is_music_day: true,
            ..Self::default()
        }
    }

    pub fn candlemas() -> Self {
        Self {
            is_candlemas: true,
            is_winter: true,
            season: Season::Winter,
            ..Self::default()
        }
    }

    pub fn may_day() -> Self {
        Self {
            is_spring: true,
            is_may_day: true,
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

    pub fn holiday_box(&self) -> Option<&'static HolidayBox> {
        HOLIDAY_BOXES
            .iter()
            .find(|holiday_box| (holiday_box.is_due)(self))
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

pub struct HolidayBox {
    pub title: &'static str,
    pub lines: &'static [&'static str],
    is_due: fn(&Holiday) -> bool,
}

const HOLIDAY_BOXES: [HolidayBox; 13] = [
    HolidayBox {
        title: "Noël",
        lines: &[
            "Le sapin s'est habillé de lumières.",
            "Regarde à son pied\u{202F}: quelqu'un y a déposé des cadeaux.",
        ],
        is_due: |holiday| holiday.is_christmas,
    },
    HolidayBox {
        title: "Épiphanie",
        lines: &[
            "Au village, on tire les rois.",
            "Lève les yeux vers le toit\u{202F}: ta maison a eu la fève.",
        ],
        is_due: |holiday| holiday.is_epiphany,
    },
    HolidayBox {
        title: "Chandeleur",
        lines: &[
            "Au village, ça sent la pâte à crêpes.",
            "Lève les yeux\u{202F}: dans le ciel, une crêpe géante se retourne.",
        ],
        is_due: |holiday| holiday.is_candlemas,
    },
    HolidayBox {
        title: "Poisson d'avril",
        lines: &[
            "Au village, les farceurs sont de sortie.",
            "Guette ta boîte aux lettres et le dos du Facteur.",
        ],
        is_due: |holiday| holiday.is_april_fools,
    },
    HolidayBox {
        title: "1er mai",
        lines: &[
            "Au village, on s'offre du muguet pour porter bonheur.",
            "Regarde le ciel\u{202F}: un brin géant y a fleuri.",
        ],
        is_due: |holiday| holiday.is_may_day,
    },
    HolidayBox {
        title: "Fête des voisins",
        lines: &[
            "Aujourd'hui, le village dresse la table près de la rivière.",
            "Même le toit s'est paré de fanions.",
        ],
        is_due: |holiday| holiday.is_neighbours_party,
    },
    HolidayBox {
        title: "Fête de la musique",
        lines: &[
            "Ce soir, le village chante.",
            "Ouvre l'œil\u{202F}: des notes s'échappent des fenêtres.",
        ],
        is_due: |holiday| holiday.is_music_day,
    },
    HolidayBox {
        title: "14 juillet",
        lines: &[
            "Ce soir, tout le village a le nez en l'air.",
            "Dès la nuit, des feux d'artifice éclatent dans le ciel.",
        ],
        is_due: |holiday| holiday.is_national_day,
    },
    HolidayBox {
        title: "Halloween",
        lines: &[
            "Ce mois-ci, le village aime se faire peur.",
            "Des citrouilles gardent ta porte, un fantôme rôde la nuit.",
        ],
        is_due: |holiday| holiday.is_halloween,
    },
    HolidayBox {
        title: "Perséides",
        lines: &[
            "Cette semaine, il pleut des étoiles filantes.",
            "Par nuit claire, lève les yeux\u{202F}: de quoi faire mille vœux.",
        ],
        is_due: |holiday| holiday.meteor_shower == Some(MeteorShower::Perseids),
    },
    HolidayBox {
        title: "Géminides",
        lines: &[
            "En décembre aussi, il pleut des étoiles filantes.",
            "Par nuit claire, couvre-toi bien et lève les yeux.",
        ],
        is_due: |holiday| holiday.meteor_shower == Some(MeteorShower::Geminids),
    },
    HolidayBox {
        title: "Nouvel An",
        lines: &[
            "Le village fête la nouvelle année.",
            "Chaque nuit, le ciel s'allume de feux d'artifice.",
        ],
        is_due: |holiday| holiday.new_year.is_some(),
    },
    HolidayBox {
        title: "Pâques",
        lines: &[
            "Les cloches sont passées par le village.",
            "Cherche dans l'herbe\u{202F}: des œufs colorés s'y cachent.",
        ],
        is_due: |holiday| holiday.is_easter,
    },
];

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

fn last_friday_of_may(year: i32) -> Option<NaiveDate> {
    let last_day = NaiveDate::from_ymd_opt(year, 5, 31)?;
    let since_friday =
        (last_day.weekday().num_days_from_monday() + 7 - Weekday::Fri.num_days_from_monday()) % 7;
    last_day.checked_sub_days(Days::new(since_friday.into()))
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

    #[test]
    fn test_epiphany_day() {
        assert!(!Holiday::on(date(2027, 1, 5)).is_epiphany);
        assert!(Holiday::on(date(2027, 1, 6)).is_epiphany);
        assert!(!Holiday::on(date(2027, 1, 7)).is_epiphany);
        assert!(!Holiday::on(date(2027, 6, 6)).is_epiphany);
    }

    #[test]
    fn test_neighbours_party_is_the_last_friday_of_may() {
        assert!(Holiday::on(date(2026, 5, 29)).is_neighbours_party);
        assert!(!Holiday::on(date(2026, 5, 28)).is_neighbours_party);
        assert!(!Holiday::on(date(2026, 5, 30)).is_neighbours_party);
        assert!(!Holiday::on(date(2026, 5, 22)).is_neighbours_party);
        assert!(Holiday::on(date(2027, 5, 28)).is_neighbours_party);
        assert!(!Holiday::on(date(2027, 5, 27)).is_neighbours_party);
        assert!(!Holiday::on(date(2027, 5, 29)).is_neighbours_party);
        assert!(Holiday::on(date(2026, 5, 29)).is_spring);
    }

    #[test]
    fn test_music_day() {
        assert!(!Holiday::on(date(2027, 6, 20)).is_music_day);
        assert!(Holiday::on(date(2027, 6, 21)).is_music_day);
        assert!(!Holiday::on(date(2027, 6, 22)).is_music_day);
        assert!(!Holiday::on(date(2027, 7, 21)).is_music_day);
    }

    fn box_title(holiday: Holiday) -> Option<&'static str> {
        holiday.holiday_box().map(|holiday_box| holiday_box.title)
    }

    #[test]
    fn test_every_holiday_and_meteor_shower_has_its_own_box() {
        let holidays = [
            (Holiday::christmas(), "Noël"),
            (Holiday::new_year(2027), "Nouvel An"),
            (Holiday::on(date(2027, 1, 6)), "Épiphanie"),
            (Holiday::on(date(2025, 4, 1)), "Poisson d'avril"),
            (Holiday::candlemas(), "Chandeleur"),
            (Holiday::may_day(), "1er mai"),
            (Holiday::easter(), "Pâques"),
            (Holiday::neighbours_party(), "Fête des voisins"),
            (Holiday::music_day(), "Fête de la musique"),
            (Holiday::national_day(), "14 juillet"),
            (Holiday::halloween(), "Halloween"),
            (Holiday::perseids(), "Perséides"),
            (Holiday::geminids(), "Géminides"),
        ];

        for (holiday, title) in holidays {
            let holiday_box = holiday.holiday_box().unwrap();
            assert_eq!(holiday_box.title, title);
            assert!((2..=3).contains(&holiday_box.lines.len()), "{title}");
            assert!(holiday_box.lines.iter().all(|line| !line.is_empty()));
        }
        assert_eq!(HOLIDAY_BOXES.len(), holidays.len());
    }

    #[test]
    fn test_plain_day_has_no_box() {
        assert!(Holiday::default().holiday_box().is_none());
        assert!(Holiday::on(date(2026, 11, 19)).holiday_box().is_none());
        assert!(Holiday::on(date(2026, 5, 28)).holiday_box().is_none());
    }

    #[test]
    fn test_box_follows_the_holiday_days() {
        assert_eq!(
            box_title(Holiday::on(date(2026, 7, 13))),
            Some("14 juillet")
        );
        assert_eq!(
            box_title(Holiday::on(date(2026, 10, 31))),
            Some("Halloween")
        );
        assert_eq!(box_title(Holiday::on(date(2026, 8, 12))), Some("Perséides"));
        assert_eq!(
            box_title(Holiday::on(date(2026, 12, 13))),
            Some("Géminides")
        );
        assert_eq!(
            box_title(Holiday::on(date(2026, 12, 30))),
            Some("Nouvel An")
        );
        assert_eq!(box_title(Holiday::on(date(2026, 7, 15))), None);
    }

    #[test]
    fn test_single_day_holiday_wins_over_a_longer_one() {
        assert_eq!(box_title(Holiday::on(date(2026, 12, 25))), Some("Noël"));
        assert_eq!(box_title(Holiday::on(date(2027, 1, 6))), Some("Épiphanie"));
        assert_eq!(
            box_title(Holiday::on(date(2026, 4, 1))),
            Some("Poisson d'avril")
        );
        assert_eq!(box_title(Holiday::on(date(2026, 4, 2))), Some("Pâques"));
        assert_eq!(box_title(Holiday::on(date(2027, 2, 2))), Some("Chandeleur"));
        assert_eq!(box_title(Holiday::on(date(2011, 5, 1))), Some("1er mai"));
    }

    #[test]
    fn test_candlemas_is_only_the_second_of_february() {
        assert!(!Holiday::on(date(2027, 2, 1)).is_candlemas);
        assert!(Holiday::on(date(2027, 2, 2)).is_candlemas);
        assert!(!Holiday::on(date(2027, 2, 3)).is_candlemas);
        assert!(!Holiday::on(date(2027, 3, 2)).is_candlemas);
        assert!(Holiday::on(date(2028, 2, 2)).is_candlemas);
    }

    #[test]
    fn test_forced_candlemas_matches_the_real_day() {
        let real = Holiday::on(date(2027, 2, 2));
        let forced = Holiday::candlemas();
        assert!(forced.is_candlemas);
        assert_eq!(forced.is_winter, real.is_winter);
        assert!(forced.season == real.season);
    }

    #[test]
    fn test_may_day_is_only_the_first_of_may() {
        assert!(!Holiday::on(date(2027, 4, 30)).is_may_day);
        assert!(Holiday::on(date(2027, 5, 1)).is_may_day);
        assert!(!Holiday::on(date(2027, 5, 2)).is_may_day);
        assert!(!Holiday::on(date(2027, 6, 1)).is_may_day);
        assert!(Holiday::on(date(2028, 5, 1)).is_may_day);
    }

    #[test]
    fn test_forced_may_day_matches_the_real_day() {
        let real = Holiday::on(date(2027, 5, 1));
        let forced = Holiday::may_day();
        assert!(forced.is_may_day);
        assert_eq!(forced.is_spring, real.is_spring);
        assert_eq!(forced.is_winter, real.is_winter);
        assert!(forced.season == real.season);
    }
}
