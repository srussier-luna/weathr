use super::StorySource;
use crate::progress::Progress;
use crate::story::clock::StoryClock;
use crate::story::content::recipes::{Diet, RECIPES, diets_from_ids};
use crate::story::event::{Beat, Entry};
use chrono::{Datelike, NaiveDate};
use std::collections::{BTreeMap, BTreeSet};

const DIET_LETTER_DAY: usize = 10;
const DIET_RECIPE_INTERVAL_DAYS: usize = 10;

pub fn diet_letter_day(today: usize) -> usize {
    (today + 1).max(DIET_LETTER_DAY)
}

pub fn next_recipe(
    diets: &BTreeSet<Diet>,
    date: NaiveDate,
    received: &BTreeSet<usize>,
) -> Option<usize> {
    RECIPES
        .iter()
        .enumerate()
        .find(|(index, recipe)| {
            !received.contains(index) && recipe.suits(diets) && recipe.is_in_season(date)
        })
        .map(|(index, _)| index)
}

#[derive(Clone)]
pub struct Recipes {
    letter_day: usize,
    answered_day: Option<usize>,
    diets: BTreeSet<Diet>,
    delivered: BTreeMap<usize, usize>,
    frozen_until: usize,
}

impl Default for Recipes {
    fn default() -> Self {
        Self {
            letter_day: DIET_LETTER_DAY,
            answered_day: None,
            diets: BTreeSet::new(),
            delivered: BTreeMap::new(),
            frozen_until: 0,
        }
    }
}

impl Recipes {
    pub fn from_progress(progress: &Progress) -> Self {
        Self {
            letter_day: progress.diet_letter_day.unwrap_or(DIET_LETTER_DAY),
            answered_day: progress.diet_answered_day,
            diets: diets_from_ids(&progress.diets),
            delivered: progress.diet_recipes.clone(),
            frozen_until: progress
                .diet_recipes_until
                .or_else(|| progress.diet_recipes.keys().last().copied())
                .unwrap_or_default(),
        }
    }

    fn monthly(&self, clock: &StoryClock, today: usize) -> Vec<(usize, usize)> {
        let last = self
            .answered_day
            .map_or(today, |answered| answered.min(today));
        (2..=last)
            .filter_map(|day| {
                let date = clock.date_of(day);
                (date.day() == 1).then_some((day, date.month0() as usize))
            })
            .collect()
    }

    pub fn deliveries(&self, clock: &StoryClock, today: usize) -> BTreeMap<usize, usize> {
        let Some(answered) = self.answered_day else {
            return BTreeMap::new();
        };
        let mut received: BTreeSet<usize> = self
            .monthly(clock, today)
            .into_iter()
            .map(|(_, recipe)| recipe)
            .collect();
        (answered + 1..=today)
            .step_by(DIET_RECIPE_INTERVAL_DAYS)
            .filter_map(|day| {
                let recipe = if day <= self.frozen_until {
                    self.delivered.get(&day).copied()
                } else {
                    next_recipe(&self.diets, clock.date_of(day), &received)
                }?;
                received.insert(recipe);
                Some((day, recipe))
            })
            .collect()
    }
}

impl StorySource for Recipes {
    fn beats(&self, clock: &StoryClock, today: usize) -> Vec<Beat> {
        let monthly = self
            .monthly(clock, today)
            .into_iter()
            .map(|(day, month)| Beat::mail(day, Entry::Recipe(month)));
        let letter =
            (self.letter_day <= today).then(|| Beat::mail(self.letter_day, Entry::DietLetter));
        let reserve = self
            .deliveries(clock, today)
            .into_iter()
            .map(|(day, recipe)| Beat::mail(day, Entry::DietRecipe(recipe)));
        monthly.chain(letter).chain(reserve).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::event::StoryEvent;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn answered(day: usize, diets: &[Diet]) -> Recipes {
        Recipes {
            answered_day: Some(day),
            diets: diets.iter().copied().collect(),
            ..Recipes::default()
        }
    }

    fn reserve(recipes: &Recipes, clock: &StoryClock, today: usize) -> Vec<(usize, usize)> {
        recipes
            .beats(clock, today)
            .into_iter()
            .filter_map(|beat| match beat.event {
                StoryEvent::Mail(Entry::DietRecipe(recipe)) => Some((beat.day, recipe)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn test_recipes_fall_on_first_of_month() {
        let clock = StoryClock::new(date(2026, 10, 22), 9);
        let recipes = Recipes {
            letter_day: usize::MAX,
            ..Recipes::default()
        };
        let beats = recipes.beats(&clock, 120);

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
        let clock = StoryClock::new(date(2026, 11, 1), 9);
        assert_eq!(Recipes::default().beats(&clock, 9), []);
    }

    #[test]
    fn test_rose_asks_on_day_10_of_a_new_game() {
        let clock = StoryClock::new(date(2026, 11, 2), 9);
        assert_eq!(Recipes::default().beats(&clock, 9), []);
        assert_eq!(
            Recipes::default().beats(&clock, 10),
            [Beat::mail(10, Entry::DietLetter)]
        );
    }

    #[test]
    fn test_a_save_past_day_10_gets_the_letter_on_its_next_day() {
        assert_eq!(diet_letter_day(1), 10);
        assert_eq!(diet_letter_day(9), 10);
        assert_eq!(diet_letter_day(10), 11);
        assert_eq!(diet_letter_day(29), 30);
    }

    #[test]
    fn test_unanswered_letter_keeps_the_monthly_recipe() {
        let clock = StoryClock::new(date(2026, 10, 22), 9);
        let monthly = Recipes::default().monthly(&clock, 120);
        assert_eq!(monthly, [(11, 10), (41, 11), (72, 0), (103, 1)]);
        assert!(Recipes::default().deliveries(&clock, 120).is_empty());
    }

    #[test]
    fn test_monthly_recipes_stop_after_the_answer() {
        let clock = StoryClock::new(date(2026, 10, 22), 9);
        assert_eq!(answered(41, &[]).monthly(&clock, 120), [(11, 10), (41, 11)]);
    }

    #[test]
    fn test_answer_starts_a_recipe_every_10_days_from_the_next_day() {
        let clock = StoryClock::new(date(2026, 11, 2), 9);
        let diets = [Diet::Vegetarian, Diet::NoPork];
        let recipes = answered(10, &diets);
        let reserve = reserve(&recipes, &clock, 60);

        assert_eq!(reserve, [(11, 2), (21, 3), (31, 6), (41, 8), (51, 10)]);
        let diets = BTreeSet::from(diets);
        assert!(
            reserve
                .iter()
                .all(|(_, recipe)| RECIPES[*recipe].suits(&diets))
        );
        assert!(
            !recipes
                .beats(&clock, 60)
                .iter()
                .any(|beat| matches!(beat.event, StoryEvent::Mail(Entry::Recipe(_))))
        );
    }

    #[test]
    fn test_pick_skips_received_and_out_of_season_recipes() {
        let any = BTreeSet::new();
        let none = BTreeSet::new();
        assert_eq!(next_recipe(&any, date(2027, 1, 15), &none), Some(0));
        assert_eq!(next_recipe(&any, date(2027, 2, 15), &none), Some(1));
        assert_eq!(next_recipe(&any, date(2027, 3, 15), &none), Some(2));
        assert_eq!(
            next_recipe(&any, date(2027, 3, 15), &BTreeSet::from([2, 3])),
            Some(6)
        );
    }

    #[test]
    fn test_seasonal_recipe_waits_for_its_month() {
        let clock = StoryClock::new(date(2026, 12, 1), 9);
        let recipes = answered(10, &[Diet::Vegan, Diet::NoVegetable, Diet::NoFruit]);
        let reserve = reserve(&recipes, &clock, 80);

        let (galette_day, _) = reserve.iter().find(|(_, recipe)| *recipe == 0).unwrap();
        assert_eq!(clock.date_of(*galette_day).month(), 1);
        for (day, recipe) in &reserve {
            assert!(RECIPES[*recipe].is_in_season(clock.date_of(*day)));
        }
    }

    #[test]
    fn test_received_monthly_recipes_never_come_twice() {
        let clock = StoryClock::new(date(2026, 10, 22), 9);
        let picks: Vec<usize> = answered(41, &[])
            .deliveries(&clock, 100)
            .into_values()
            .collect();
        assert_eq!(picks, [2, 3, 6, 0, 8, 15]);
    }

    #[test]
    fn test_reserve_runs_out_for_strict_diets() {
        let clock = StoryClock::new(date(2026, 11, 2), 9);
        let recipes = answered(10, &[Diet::Vegan, Diet::NoVegetable, Diet::Protein]);
        let picks: Vec<usize> = reserve(&recipes, &clock, 400)
            .into_iter()
            .map(|(_, recipe)| recipe + 1)
            .collect();
        assert_eq!(picks, [18, 20, 27, 29]);
        assert_eq!(recipes.deliveries(&clock, 400).len(), 4);
    }

    #[test]
    fn test_exhausted_reserve_keeps_the_frozen_recipes_bounded() {
        let clock = StoryClock::new(date(2026, 11, 2), 9);
        let mut recipes = answered(10, &[]);
        for today in [1000, 4000] {
            recipes.delivered = recipes.deliveries(&clock, today);
            recipes.frozen_until = today;
            assert_eq!(recipes.delivered.len(), RECIPES.len());
        }
    }

    #[test]
    fn test_delivered_recipes_stay_when_diets_change() {
        let clock = StoryClock::new(date(2026, 11, 2), 9);
        let delivered = answered(10, &[]).deliveries(&clock, 21);
        assert_eq!(delivered, BTreeMap::from([(11, 2), (21, 3)]));

        let after = Recipes {
            delivered,
            frozen_until: 21,
            ..answered(10, &[Diet::Protein])
        };
        assert_eq!(
            after.deliveries(&clock, 31),
            BTreeMap::from([(11, 2), (21, 3), (31, 16)])
        );
    }

    #[test]
    fn test_empty_slot_stays_empty_when_diets_change() {
        let clock = StoryClock::new(date(2026, 11, 2), 9);
        let recipes = Recipes {
            frozen_until: 11,
            ..answered(10, &[])
        };
        assert_eq!(recipes.deliveries(&clock, 21), BTreeMap::from([(21, 2)]));
    }

    #[test]
    fn test_old_branch_save_keeps_its_delivered_recipes() {
        let clock = StoryClock::new(date(2026, 11, 2), 9);
        let mut progress: Progress = serde_json::from_str(
            r#"{"started_on":"2026-11-02","diet_answered_day":10,"diets":[],"diet_recipes":{"11":2}}"#,
        )
        .unwrap();
        progress.diets = BTreeSet::from(["protein".to_string()]);
        let recipes = Recipes::from_progress(&progress);
        assert_eq!(recipes.frozen_until, 11);
        assert_eq!(
            recipes.deliveries(&clock, 21),
            BTreeMap::from([(11, 2), (21, 16)])
        );
    }

    #[test]
    fn test_progress_feeds_the_recipes() {
        let mut progress = Progress::default();
        progress.diet_letter_day = Some(30);
        progress.diet_answered_day = Some(31);
        progress.diets = BTreeSet::from(["vegan".to_string()]);
        progress.diet_recipes = BTreeMap::from([(32, 2)]);
        progress.diet_recipes_until = Some(42);
        let recipes = Recipes::from_progress(&progress);
        assert_eq!(recipes.letter_day, 30);
        assert_eq!(recipes.answered_day, Some(31));
        assert_eq!(recipes.diets, BTreeSet::from([Diet::Vegan]));
        assert_eq!(recipes.delivered, BTreeMap::from([(32, 2)]));
        assert_eq!(recipes.frozen_until, 42);
    }
}
