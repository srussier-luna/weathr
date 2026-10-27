use super::StorySource;
use crate::story::clock::StoryClock;
use crate::story::event::{Beat, Entry};
use chrono::Datelike;

pub struct Recipes;

impl StorySource for Recipes {
    fn beats(&self, clock: &StoryClock, today: usize) -> Vec<Beat> {
        (2..=today)
            .filter_map(|day| {
                let date = clock.date_of(day);
                (date.day() == 1).then(|| Beat::mail(day, Entry::Recipe(date.month0() as usize)))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_recipes_fall_on_first_of_month() {
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 10, 22).unwrap(), 9);
        let beats = Recipes.beats(&clock, 120);

        assert_eq!(
            beats,
            [
                Beat::mail(11, Entry::Recipe(10)),
                Beat::mail(41, Entry::Recipe(11)),
                Beat::mail(72, Entry::Recipe(0)),
                Beat::mail(103, Entry::Recipe(1)),
            ]
        );
        for beat in beats {
            assert_eq!(clock.date_of(beat.day).day(), 1);
        }
    }

    #[test]
    fn test_no_recipe_on_the_start_day() {
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 11, 1).unwrap(), 9);
        assert_eq!(Recipes.beats(&clock, 30), []);
    }
}
