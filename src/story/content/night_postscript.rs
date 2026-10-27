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
                riddle_input: None,
            };
            NightPostscript::new(VillageLetter::new(2, "3/10".to_string())).compose(&ctx)
        };

        assert!(has_line(&compose_at(23), NIGHT_POSTSCRIPTS[2]));
        assert!(!has_line(&compose_at(14), NIGHT_POSTSCRIPTS[2]));
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
