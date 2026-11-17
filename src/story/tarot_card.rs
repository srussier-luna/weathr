use super::arcane::ArcanePost;
use super::clock::StoryClock;
use super::content::chronicles::is_right_answer;
use super::content::secret_trail::{TAROT_RIDDLE, TRAIL_START_DAY};
use crate::progress::Progress;
use chrono::{NaiveDateTime, Timelike};

const MIDNIGHT_HOUR: u32 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TarotCard {
    Hidden,
    InGrass,
    Picked,
}

impl TarotCard {
    pub fn saved(progress: &Progress) -> Self {
        match (progress.tarot_card_appeared, progress.tarot_card_picked) {
            (_, true) => Self::Picked,
            (true, false) => Self::InGrass,
            (false, false) => Self::Hidden,
        }
    }

    pub fn write_to(self, progress: &mut Progress) {
        progress.tarot_card_appeared = self != Self::Hidden;
        progress.tarot_card_picked = self == Self::Picked;
    }

    pub fn appear(&mut self, now: NaiveDateTime, day: usize, is_unlocked: bool) -> bool {
        let is_due = *self == Self::Hidden
            && !is_unlocked
            && day >= TRAIL_START_DAY
            && now.hour() == MIDNIGHT_HOUR;
        if is_due {
            *self = Self::InGrass;
        }
        is_due
    }

    pub fn pick_up(&mut self) -> bool {
        let is_in_grass = *self == Self::InGrass;
        if is_in_grass {
            *self = Self::Picked;
        }
        is_in_grass
    }

    pub fn is_riddle_open(self, arcanes: &ArcanePost) -> bool {
        self == Self::Picked && !arcanes.is_unlocked()
    }

    pub fn answer(
        self,
        attempt: &str,
        arcanes: &mut ArcanePost,
        clock: &StoryClock,
        now: NaiveDateTime,
    ) -> bool {
        self.is_riddle_open(arcanes)
            && is_right_answer(&TAROT_RIDDLE, attempt)
            && arcanes.unlock(clock, now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlay::Line;
    use crate::story::arcane::NextPassage;
    use crate::story::content::secret_trail::tarot_card_view;
    use chrono::NaiveDate;

    fn at(day: u32, hour: u32, minute: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 11, day)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    }

    fn appears(now: NaiveDateTime, day: usize) -> bool {
        TarotCard::Hidden.appear(now, day, false)
    }

    #[test]
    fn test_card_appears_only_between_midnight_and_one() {
        assert!(appears(at(20, 0, 0), 46));
        assert!(appears(at(20, 0, 30), 46));
        assert!(appears(at(20, 0, 59), 46));
        assert!(!appears(at(20, 1, 0), 46));
        assert!(!appears(at(20, 1, 30), 46));
        assert!(!appears(at(20, 23, 59), 46));
        assert!(!appears(at(20, 12, 0), 46));
    }

    #[test]
    fn test_card_waits_for_day_42() {
        assert!(!appears(at(20, 0, 30), 1));
        assert!(!appears(at(20, 0, 30), 41));
        assert!(appears(at(20, 0, 30), 42));
    }

    #[test]
    fn test_card_never_comes_once_the_secret_is_unlocked() {
        assert!(!TarotCard::Hidden.appear(at(20, 0, 30), 60, true));
    }

    #[test]
    fn test_card_appears_once_and_stays_until_picked() {
        let mut card = TarotCard::Hidden;
        assert!(card.appear(at(20, 0, 30), 46, false));
        assert!(!card.appear(at(21, 0, 30), 47, false));
        assert_eq!(card, TarotCard::InGrass);
        assert!(card.pick_up());
        assert_eq!(card, TarotCard::Picked);
        assert!(!card.pick_up());
        assert!(!card.appear(at(22, 0, 30), 48, false));
        assert_eq!(card, TarotCard::Picked);
    }

    #[test]
    fn test_hidden_card_cannot_be_picked() {
        let mut card = TarotCard::Hidden;
        assert!(!card.pick_up());
        assert_eq!(card, TarotCard::Hidden);
    }

    #[test]
    fn test_card_in_the_grass_survives_a_restart() {
        let mut card = TarotCard::Hidden;
        card.appear(at(20, 0, 30), 46, false);
        let mut progress = Progress::default();
        card.write_to(&mut progress);
        let saved = serde_json::to_string(&progress).unwrap();
        let loaded: Progress = serde_json::from_str(&saved).unwrap();
        assert_eq!(TarotCard::saved(&loaded), TarotCard::InGrass);

        let mut card = TarotCard::saved(&loaded);
        card.pick_up();
        card.write_to(&mut progress);
        assert_eq!(TarotCard::saved(&progress), TarotCard::Picked);
    }

    fn locked() -> ArcanePost {
        ArcanePost::saved(&Progress::default())
    }

    fn at_hour(hour: u32) -> NaiveDateTime {
        at(20, hour, 0)
    }

    fn clock() -> StoryClock {
        StoryClock::new(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(), 9)
    }

    #[test]
    fn test_card_riddle_accepts_its_answers_whatever_the_case_or_accents() {
        for attempt in [
            "le mat",
            "mat",
            "le fou",
            "fou",
            "Le Mât",
            "  LE FOU ! ",
            "Fou",
        ] {
            let mut arcanes = locked();
            assert!(
                TarotCard::Picked.answer(attempt, &mut arcanes, &clock(), at_hour(0)),
                "{attempt}"
            );
            assert!(arcanes.is_unlocked());
        }
    }

    #[test]
    fn test_wrong_answer_changes_nothing() {
        let mut arcanes = locked();
        for attempt in ["", "le bateleur", "matin", "la folle", "secret"] {
            assert!(!TarotCard::Picked.answer(attempt, &mut arcanes, &clock(), at_hour(0)));
        }
        assert_eq!(arcanes, locked());
        assert!(TarotCard::Picked.is_riddle_open(&arcanes));
    }

    #[test]
    fn test_card_must_be_picked_before_its_riddle_counts() {
        for card in [TarotCard::Hidden, TarotCard::InGrass] {
            let mut arcanes = locked();
            assert!(!card.is_riddle_open(&arcanes));
            assert!(!card.answer("le mat", &mut arcanes, &clock(), at_hour(0)));
            assert!(!arcanes.is_unlocked());
        }
    }

    #[test]
    fn test_right_answer_brings_the_arcano_postman_that_same_evening() {
        let clock = clock();
        let mut arcanes = locked();
        assert!(TarotCard::Picked.answer("le fou", &mut arcanes, &clock, at_hour(0)));
        assert!(!TarotCard::Picked.is_riddle_open(&arcanes));
        assert!(!TarotCard::Picked.answer("le fou", &mut arcanes, &clock, at_hour(0)));

        let mut progress = Progress::default();
        assert!(arcanes.write_to(&mut progress));
        assert_eq!(progress.arcanes_unlocked_date(), Some(at_hour(0).date()));
        assert_eq!(
            arcanes.round(&clock, at_hour(20)).map(|round| round.hour),
            Some(21)
        );
        assert!(!arcanes.pass(&clock, at_hour(20)));
        assert!(arcanes.pass(&clock, at_hour(21)));
    }

    fn shows_line(
        arcanes: &ArcanePost,
        clock: &StoryClock,
        now: NaiveDateTime,
        expected: &str,
    ) -> bool {
        let view = tarot_card_view(None, arcanes.next_passage(clock, now), false);
        view.lines
            .iter()
            .any(|line| matches!(line, Line::Text(text) if text == expected))
    }

    #[test]
    fn test_right_answer_names_the_evening_hour_on_the_card() {
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(), 8);
        let mut arcanes = locked();
        assert_eq!(arcanes.next_passage(&clock, at_hour(0)), None);
        assert!(TarotCard::Picked.answer("mat", &mut arcanes, &clock, at_hour(0)));
        assert!(shows_line(
            &arcanes,
            &clock,
            at_hour(0),
            "Ce soir, vers 20 h, lève les yeux au-dessus de ta boîte aux lettres."
        ));
    }

    #[test]
    fn test_card_solved_with_a_mail_after_noon_announces_the_coming_night() {
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(), 14);
        let mut arcanes = locked();
        assert!(TarotCard::Picked.answer("le mat", &mut arcanes, &clock, at_hour(10)));
        assert!(shows_line(
            &arcanes,
            &clock,
            at_hour(10),
            "Cette nuit, vers 2 h, lève les yeux au-dessus de ta boîte aux lettres."
        ));
        assert!(!arcanes.pass(&clock, at_hour(23)));
        assert!(!arcanes.pass(&clock, at(21, 1, 0)));
        assert!(arcanes.pass(&clock, at(21, 2, 0)));
    }

    #[test]
    fn test_card_solved_after_the_passage_announces_tomorrow_evening() {
        let clock = clock();
        let mut arcanes = locked();
        assert!(TarotCard::Picked.answer("le fou", &mut arcanes, &clock, at_hour(22)));
        assert_eq!(
            arcanes.next_passage(&clock, at_hour(22)),
            Some(NextPassage::Tomorrow(21))
        );
        assert!(!arcanes.pass(&clock, at_hour(22)));
        assert!(shows_line(
            &arcanes,
            &clock,
            at_hour(22),
            "Demain soir, vers 21 h, lève les yeux au-dessus de ta boîte aux lettres."
        ));
        assert_eq!(
            arcanes.next_passage(&clock, at(21, 20, 0)),
            Some(NextPassage::Tonight(21))
        );
        assert!(!arcanes.pass(&clock, at(21, 20, 0)));
        assert!(arcanes.pass(&clock, at(21, 21, 0)));
    }

    #[test]
    fn test_forced_day_never_saves_the_unlock() {
        let mut progress = Progress::in_memory(NaiveDate::from_ymd_opt(2026, 10, 6).unwrap());
        let mut arcanes = ArcanePost::saved(&progress);
        assert!(TarotCard::Picked.answer("mat", &mut arcanes, &clock(), at_hour(0)));
        assert!(arcanes.write_to(&mut progress));
        assert!(progress.try_save().is_err());
    }

    #[test]
    fn test_old_save_has_no_card() {
        let progress: Progress =
            serde_json::from_str(r#"{"started_on":"2026-09-01","letters_read":50}"#).unwrap();
        assert_eq!(TarotCard::saved(&progress), TarotCard::Hidden);
        assert!(!ArcanePost::saved(&progress).is_unlocked());
    }
}
