use super::content::chronicles::OBJECTS;
use super::content::letters::{CREEPY_LETTER_COUNT, VILLAGE_LETTER_COUNT};
use super::event::{Entry, Thing};
use super::timeline::Timeline;
use crate::badges;
use crate::progress::Progress;
use chrono::NaiveDate;
use std::collections::BTreeSet;

pub struct Reach {
    letters: usize,
    days: u32,
    gazettes: BTreeSet<usize>,
    finds: bool,
    bottles: bool,
}

fn counts(progress: &Progress) -> [usize; 6] {
    [
        progress.letters_read,
        progress.streak as usize,
        progress.riddles_solved.len(),
        progress.found_objects.len(),
        progress.bottles.len(),
        progress.badges.len(),
    ]
}

impl Reach {
    pub fn on(timeline: &Timeline, day: usize, start: NaiveDate, today: NaiveDate) -> Self {
        let letters = timeline.delivered_by(day);
        let has_arrived = |thing| timeline.arrived_by(day).any(|arrived| arrived == thing);
        Self {
            letters,
            days: u32::try_from((today - start).num_days().max(0) + 2).unwrap_or(u32::MAX),
            gazettes: timeline
                .first_mail(letters)
                .iter()
                .filter_map(|(_, entry)| match entry {
                    Entry::Gazette(week) => Some(*week),
                    _ => None,
                })
                .collect(),
            finds: has_arrived(Thing::FoundObjects),
            bottles: has_arrived(Thing::Bottles),
        }
    }

    pub fn fit(&self, progress: &mut Progress) -> bool {
        let before = counts(progress);
        progress.letters_read = progress.letters_read.min(self.letters);
        progress.streak = progress.streak.min(self.days);
        progress
            .riddles_solved
            .retain(|week| self.gazettes.contains(week));
        if !self.finds {
            progress.found_objects.clear();
        }
        if !self.bottles {
            progress.bottles.clear();
        }
        let unmet = self.unmet(progress);
        progress.badges.retain(|id| !unmet.contains(&id.as_str()));
        counts(progress) != before
    }

    fn unmet(&self, progress: &Progress) -> Vec<&'static str> {
        let has_every_weather =
            badges::weather_badges().all(|badge| progress.badges.contains(badge.id));
        [
            ("streak_3", self.days >= 3),
            ("streak_7", self.days >= 7),
            ("streak_30", self.days >= 30),
            (
                "all_letters",
                progress.letters_read >= VILLAGE_LETTER_COUNT + CREEPY_LETTER_COUNT,
            ),
            ("riddle_1", !progress.riddles_solved.is_empty()),
            ("riddle_10", progress.riddles_solved.len() >= 10),
            ("all_objects", progress.found_objects.len() >= OBJECTS.len()),
            ("first_bottle", !progress.bottles.is_empty()),
            ("summer_holidays", progress.postman_note_day.is_some()),
            ("full_weather", has_every_weather),
        ]
        .into_iter()
        .filter_map(|(id, met)| (!met).then_some(id))
        .collect()
    }
}

pub(crate) fn settle(progress: &mut Progress, out_of_sync: bool, reach: &Reach) -> bool {
    let changed = reach.fit(progress) || out_of_sync;
    if changed {
        progress.resyncs += 1;
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::clock::StoryClock;
    use crate::story::sources;

    fn date(month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, month, day).unwrap()
    }

    fn reach_on(day: usize) -> Reach {
        let start = date(9, 1);
        let clock = StoryClock::new(start, 9);
        let timeline = Timeline::new(&sources::registered(VILLAGE_LETTER_COUNT), &clock, day);
        Reach::on(&timeline, day, start, clock.date_of(day))
    }

    fn badges(ids: &[&str]) -> BTreeSet<String> {
        ids.iter().map(|id| id.to_string()).collect()
    }

    fn honest() -> Progress {
        let mut progress = Progress::default();
        progress.started_on = Some("2026-09-01".to_string());
        progress.letters_read = 80;
        progress.streak = 50;
        progress.riddles_solved = BTreeSet::from([0, 1, 2]);
        progress.found_objects = vec![0, 1];
        progress.bottles = vec![0];
        progress.badges = badges(&[
            "rain",
            "streak_30",
            "riddle_1",
            "first_bottle",
            "all_letters",
        ]);
        progress
    }

    #[test]
    fn test_honest_progress_is_left_alone() {
        let mut progress = honest();
        assert!(!reach_on(60).fit(&mut progress));
        assert_eq!(progress.letters_read, 80);
        assert_eq!(progress.streak, 50);
        assert_eq!(progress.riddles_solved.len(), 3);
        assert_eq!(progress.badges, honest().badges);
    }

    #[test]
    fn test_letters_read_stop_at_the_delivered_ones() {
        let reach = reach_on(10);
        let mut progress = Progress::default();
        progress.letters_read = 500;
        assert!(reach.fit(&mut progress));
        assert_eq!(progress.letters_read, reach.letters);
        assert!(reach.letters > 10);
    }

    #[test]
    fn test_streak_stops_at_the_days_since_the_start() {
        let mut progress = Progress::default();
        progress.streak = 40;
        assert!(reach_on(12).fit(&mut progress));
        assert_eq!(progress.streak, 13);
    }

    #[test]
    fn test_streak_one_day_ahead_of_the_local_date_is_kept() {
        let mut progress = Progress::default();
        progress.streak = 3;
        progress.badges = badges(&["streak_3"]);
        assert!(!reach_on(2).fit(&mut progress));
        assert_eq!(progress.streak, 3);
        assert_eq!(progress.badges, badges(&["streak_3"]));
    }

    #[test]
    fn test_reach_rules_follow_the_current_content() {
        assert_eq!(VILLAGE_LETTER_COUNT, 60);
        assert_eq!(CREEPY_LETTER_COUNT, 15);
        assert_eq!(OBJECTS.len(), 25);
        assert_eq!(badges::weather_badges().count(), 14);
    }

    #[test]
    fn test_settle_adds_one_resync_at_most_per_launch() {
        let reach = reach_on(10);
        let honest = || {
            let mut progress = Progress::default();
            progress.letters_read = 3;
            progress.streak = 2;
            progress
        };

        let mut progress = honest();
        assert!(!settle(&mut progress, false, &reach));
        assert_eq!(progress.resyncs, 0);

        let mut progress = honest();
        assert!(settle(&mut progress, true, &reach));
        assert_eq!(progress.resyncs, 1);
        assert_eq!(progress.letters_read, 3);

        let mut progress = honest();
        progress.letters_read = 900;
        progress.streak = 400;
        progress.resyncs = 2;
        assert!(settle(&mut progress, true, &reach));
        assert_eq!(progress.resyncs, 3);
        assert_eq!(progress.streak, 11);
        assert!(progress.letters_read < 900);
    }

    #[test]
    fn test_riddles_of_gazettes_not_yet_out_are_dropped() {
        let mut progress = Progress::default();
        progress.riddles_solved = BTreeSet::from([0, 25]);
        assert!(reach_on(12).fit(&mut progress));
        assert_eq!(progress.riddles_solved, BTreeSet::from([0]));
    }

    #[test]
    fn test_finds_before_their_arrival_are_dropped() {
        let early = |day| {
            let mut progress = Progress::default();
            progress.found_objects = vec![3];
            progress.bottles = vec![1];
            let changed = reach_on(day).fit(&mut progress);
            (changed, progress)
        };

        let (changed, progress) = early(20);
        assert!(changed);
        assert!(progress.found_objects.is_empty());
        assert!(progress.bottles.is_empty());

        let (changed, progress) = early(50);
        assert!(!changed);
        assert_eq!(progress.found_objects, [3]);
        assert_eq!(progress.bottles, [1]);
    }

    #[test]
    fn test_badges_whose_condition_is_unmet_are_dropped() {
        let mut progress = Progress::default();
        progress.letters_read = 12;
        progress.streak = 2;
        progress.badges = badges(&[
            "rain",
            "night_owl",
            "streak_3",
            "streak_30",
            "all_letters",
            "riddle_1",
            "riddle_10",
            "all_objects",
            "first_bottle",
            "summer_holidays",
            "full_weather",
        ]);
        assert!(reach_on(5).fit(&mut progress));
        assert_eq!(progress.badges, badges(&["night_owl", "rain", "streak_3"]));
    }

    #[test]
    fn test_letters_badge_needs_the_letters_read() {
        let mut progress = Progress::default();
        progress.letters_read = 20;
        progress.badges = badges(&["all_letters"]);
        assert!(reach_on(90).fit(&mut progress));
        assert!(progress.badges.is_empty());
    }
}
