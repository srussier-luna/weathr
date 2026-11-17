use super::letters::NIGHT_POSTSCRIPTS;
use super::village::VillageLetter;
use super::{MailContent, ReadContext};
use crate::overlay::View;
use std::ops::Range;

const NIGHT_READING_HOURS: [Range<u32>; 2] = [22..24, 0..6];

pub fn is_night_hour(hour: u32) -> bool {
    NIGHT_READING_HOURS
        .iter()
        .any(|hours| hours.contains(&hour))
}

pub struct NightPostscript {
    letter: VillageLetter,
}

impl NightPostscript {
    pub fn new(letter: VillageLetter) -> Self {
        Self { letter }
    }
}

impl MailContent for NightPostscript {
    fn compose(&self, ctx: &ReadContext) -> View {
        let postscript = is_night_hour(ctx.reading_hour)
            .then(|| NIGHT_POSTSCRIPTS[self.letter.main() % NIGHT_POSTSCRIPTS.len()]);
        self.letter.compose_with(ctx, postscript)
    }
}

#[cfg(test)]
mod tests {
    use super::super::letters;
    use super::super::secret_trail::mystery_allusion;
    use super::*;
    use crate::overlay::Line;
    use crate::progress::Progress;
    use crate::story::clock::StoryClock;
    use chrono::NaiveDate;

    fn has_line(view: &View, expected: &str) -> bool {
        view.lines
            .iter()
            .any(|line| matches!(line, Line::Text(text) if text == expected))
    }

    #[test]
    fn test_postscript_only_at_night() {
        let letters = letters::defaults();
        let progress = Progress::default();
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(), 9);
        let compose_at = |reading_hour| {
            let ctx = ReadContext {
                letters: &letters,
                progress: &progress,
                clock: &clock,
                reading_hour,
                reading_day: 3,
                is_secret_unlocked: false,
                riddle_input: None,
            };
            NightPostscript::new(VillageLetter::new(2, 3, "3/10".to_string())).compose(&ctx)
        };

        assert!(has_line(&compose_at(23), NIGHT_POSTSCRIPTS[2]));
        assert!(!has_line(&compose_at(14), NIGHT_POSTSCRIPTS[2]));
    }

    #[test]
    fn test_village_letter_from_day_42_alludes_until_the_unlock() {
        let letters = letters::defaults();
        let progress = Progress::default();
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(), 9);
        let compose = |main: usize, is_secret_unlocked| {
            let ctx = ReadContext {
                letters: &letters,
                progress: &progress,
                clock: &clock,
                reading_hour: 23,
                reading_day: 50,
                is_secret_unlocked,
                riddle_input: None,
            };
            NightPostscript::new(VillageLetter::new(
                main,
                main + 1,
                format!("{}/60", main + 1),
            ))
            .compose(&ctx)
        };
        let position = |view: &View, expected: &str| {
            view.lines
                .iter()
                .position(|line| matches!(line, Line::Text(text) if text == expected))
        };
        let allusion = mystery_allusion(42, false).unwrap();

        let view = compose(41, false);
        let night = NIGHT_POSTSCRIPTS[41 % NIGHT_POSTSCRIPTS.len()];
        assert!(position(&view, allusion).unwrap() < position(&view, night).unwrap());
        assert!(!has_line(&compose(40, false), allusion));
        assert!(!has_line(&compose(41, true), allusion));
    }

    #[test]
    fn test_night_hours() {
        assert!(is_night_hour(22));
        assert!(is_night_hour(0));
        assert!(is_night_hour(5));
        assert!(!is_night_hour(6));
        assert!(!is_night_hour(21));
    }
}
