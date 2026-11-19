use super::clock::StoryClock;
use crate::badges;
use crate::progress::Progress;
use chrono::{Duration, NaiveDate};

pub const BARE_VILLAGE_DAYS: usize = 7;
const MAIL_BAN_STEP: u32 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Punishment {
    Cheater,
    BareVillage,
    MailBan(u32),
}

impl Punishment {
    pub fn for_cheats(cheats: u32) -> Option<Self> {
        match cheats {
            0 => None,
            1 => Some(Self::Cheater),
            2 => Some(Self::BareVillage),
            cheats => Some(Self::MailBan((cheats - 2) * MAIL_BAN_STEP)),
        }
    }
}

pub fn sentence(
    progress: &mut Progress,
    clock: &mut StoryClock,
    mail_date: NaiveDate,
) -> Option<Punishment> {
    if progress.resyncs <= progress.punished_cheats {
        return None;
    }
    progress.punished_cheats = progress.resyncs;
    let punishment = Punishment::for_cheats(progress.resyncs)?;
    progress.badges.insert(badges::CHEATER.to_string());
    let day = clock.day_of(mail_date);
    match punishment {
        Punishment::Cheater => {}
        Punishment::BareVillage => progress.bare_village_until = Some(day + BARE_VILLAGE_DAYS),
        Punishment::MailBan(days) => ban_mail(progress, clock, day, mail_date, days),
    }
    Some(punishment)
}

fn ban_mail(
    progress: &mut Progress,
    clock: &mut StoryClock,
    day: usize,
    mail_date: NaiveDate,
    days: u32,
) {
    let resume = mail_date + Duration::days(i64::from(days) + 1);
    let missing = (resume - clock.date_of(day + 1)).num_days();
    if let Ok(missing @ 1..) = u32::try_from(missing) {
        clock.pause_for(day, missing);
    }
    let resume = clock.date_of(day + 1);
    progress.paused_days = clock.pauses().clone();
    progress.mark_pause_checked(resume - Duration::days(1));
    progress.set_mail_ban_end(resume);
}

pub fn is_mail_banned(progress: &Progress, mail_date: NaiveDate) -> bool {
    progress.mail_ban_end().is_some_and(|end| mail_date < end)
}

pub fn is_village_bare(progress: &Progress, day: usize, mail_date: NaiveDate) -> bool {
    progress.bare_village_until.is_some_and(|until| day < until)
        || is_mail_banned(progress, mail_date)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::pause;
    use crate::story::sources;
    use crate::story::timeline::Timeline;
    use std::collections::BTreeMap;

    const MAIL_HOUR: u32 = 9;
    const VILLAGE_COUNT: usize = 60;

    fn date(month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, month, day).unwrap()
    }

    fn clock() -> StoryClock {
        StoryClock::new(date(10, 1), MAIL_HOUR)
    }

    fn caught(cheats: u32, punished: u32) -> Progress {
        let mut progress = Progress::default();
        progress.resyncs = cheats;
        progress.punished_cheats = punished;
        progress
    }

    fn delivered(clock: &StoryClock, today: usize) -> usize {
        Timeline::new(&sources::registered(VILLAGE_COUNT), clock, today).delivered_by(today)
    }

    #[test]
    fn test_each_cheat_gets_its_punishment() {
        let punishments: Vec<_> = (0..=5).map(Punishment::for_cheats).collect();
        assert_eq!(
            punishments,
            [
                None,
                Some(Punishment::Cheater),
                Some(Punishment::BareVillage),
                Some(Punishment::MailBan(3)),
                Some(Punishment::MailBan(6)),
                Some(Punishment::MailBan(9)),
            ]
        );
    }

    #[test]
    fn test_a_cheat_is_punished_once() {
        let mut progress = caught(1, 0);
        let mut clock = clock();
        assert_eq!(
            sentence(&mut progress, &mut clock, date(10, 5)),
            Some(Punishment::Cheater)
        );
        assert_eq!(progress.punished_cheats, 1);
        assert_eq!(sentence(&mut progress, &mut clock, date(10, 6)), None);
        assert_eq!(sentence(&mut caught(0, 0), &mut clock, date(10, 6)), None);
    }

    #[test]
    fn test_cheater_badge_is_earned_once_and_kept() {
        let mut progress = caught(1, 0);
        let mut clock = clock();
        sentence(&mut progress, &mut clock, date(10, 5));
        assert!(progress.badges.contains(badges::CHEATER));

        for cheats in 2..=4 {
            progress.resyncs = cheats;
            sentence(&mut progress, &mut clock, date(10, 5));
            assert_eq!(progress.badges.len(), 1);
            assert!(progress.badges.contains(badges::CHEATER));
        }
    }

    #[test]
    fn test_late_punishment_takes_the_latest_level_and_the_badge() {
        let mut progress = caught(3, 0);
        assert_eq!(
            sentence(&mut progress, &mut clock(), date(10, 10)),
            Some(Punishment::MailBan(3))
        );
        assert!(progress.badges.contains(badges::CHEATER));
    }

    #[test]
    fn test_bare_village_lasts_seven_story_days() {
        let mut progress = caught(2, 1);
        let clock_before = clock();
        let mut clock = clock();
        assert_eq!(
            sentence(&mut progress, &mut clock, date(10, 10)),
            Some(Punishment::BareVillage)
        );
        assert_eq!(clock.pauses(), clock_before.pauses());
        assert_eq!(progress.mail_ban_end(), None);

        let bare_on = |day: usize| is_village_bare(&progress, day, clock.date_of(day));
        for day in 10..10 + BARE_VILLAGE_DAYS {
            assert!(bare_on(day), "day {day}");
        }
        assert!(!bare_on(17));
    }

    #[test]
    fn test_mail_ban_freezes_the_story_then_resumes_where_it_was() {
        let mut progress = caught(3, 2);
        let mut clock = clock();
        let before = delivered(&clock, 10);
        assert_eq!(
            sentence(&mut progress, &mut clock, date(10, 10)),
            Some(Punishment::MailBan(3))
        );

        for day in 10..=13 {
            assert_eq!(clock.day_of(date(10, day)), 10, "10/{day}");
            assert!(is_mail_banned(&progress, date(10, day)), "10/{day}");
            assert!(is_village_bare(&progress, 10, date(10, day)), "10/{day}");
        }
        assert_eq!(delivered(&clock, 10), before);

        assert_eq!(clock.day_of(date(10, 14)), 11);
        assert!(!is_mail_banned(&progress, date(10, 14)));
        assert!(!is_village_bare(&progress, 11, date(10, 14)));
        assert!(delivered(&clock, 11) > before);
        assert_eq!(progress.paused_days, BTreeMap::from([(10, 3)]));
    }

    #[test]
    fn test_mail_bans_grow_by_three_days_per_cheat() {
        for (cheats, resume) in [(3, date(10, 14)), (4, date(10, 17)), (5, date(10, 20))] {
            let mut progress = caught(cheats, cheats - 1);
            sentence(&mut progress, &mut clock(), date(10, 10));
            assert_eq!(progress.mail_ban_end(), Some(resume), "{cheats} cheats");
        }
    }

    #[test]
    fn test_new_ban_runs_its_full_length_from_today() {
        let mut progress = caught(3, 2);
        let mut clock = clock();
        sentence(&mut progress, &mut clock, date(10, 10));
        progress.resyncs = 4;
        sentence(&mut progress, &mut clock, date(10, 12));
        assert_eq!(progress.mail_ban_end(), Some(date(10, 19)));
        assert_eq!(clock.day_of(date(10, 18)), 10);
        assert_eq!(clock.day_of(date(10, 19)), 11);
    }

    #[test]
    fn test_ban_during_a_full_box_counts_from_today() {
        let mut progress = caught(3, 2);
        let mut clock = clock();
        clock.pause_for(10, 2);
        assert_eq!(clock.day_of(date(10, 12)), 10);
        sentence(&mut progress, &mut clock, date(10, 12));
        assert_eq!(progress.mail_ban_end(), Some(date(10, 16)));
        assert_eq!(clock.day_of(date(10, 15)), 10);
        assert_eq!(clock.day_of(date(10, 16)), 11);
    }

    #[test]
    fn test_full_box_never_stretches_a_mail_ban() {
        let mut progress = caught(3, 2);
        let mut clock = clock();
        sentence(&mut progress, &mut clock, date(10, 10));
        let timeline = Timeline::new(&sources::registered(VILLAGE_COUNT), &clock, 10);
        assert!(pause::is_box_full(&timeline, 10, 0));

        let checked = progress.pause_checked_date();
        assert_eq!(checked, Some(date(10, 13)));
        for today in [date(10, 11), date(10, 13)] {
            assert_eq!(
                pause::hold_since(&mut clock, &timeline, 0, checked, today),
                None
            );
        }
        assert_eq!(clock.pauses(), &BTreeMap::from([(10, 3)]));

        pause::hold_since(&mut clock, &timeline, 0, checked, date(10, 14));
        assert_eq!(clock.day_of(date(10, 14)), 10);
    }
}
