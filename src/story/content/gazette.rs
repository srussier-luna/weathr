use super::chronicles::{ADS, RIDDLES, RUMORS};
use super::secret_trail::second_rumor;
use super::{LETTER_FOOTER, MONTHS, MailContent, ReadContext, text};
use crate::overlay::{Frame, Line, View};
use chrono::Datelike;

const RIDDLE_FOOTER: &str = "[e] répondre à l'énigme  [c] suivante  [<] [>]  [Echap]";
pub(super) const INPUT_FOOTER: &str = "Tape ta réponse puis [Entrée]  [Echap] annuler";

pub(super) fn typed_answer(input: &str) -> Line {
    text(format!("Ta réponse : {input}_"))
}

pub struct Gazette {
    week: usize,
    day: usize,
}

impl Gazette {
    pub fn new(week: usize, day: usize) -> Self {
        Self { week, day }
    }
}

impl MailContent for Gazette {
    fn compose(&self, ctx: &ReadContext) -> View {
        let week = self.week;
        let date = ctx.clock.date_of(self.day);
        let riddle = &RIDDLES[week % RIDDLES.len()];
        let is_solved = ctx.progress.riddles_solved.contains(&week);
        let weather = ctx.progress.gazette_weather.get(&week).map_or(
            "Les prévisions ne sont pas encore arrivées.",
            String::as_str,
        );

        let mut lines: Vec<Line> = weather.lines().map(text).collect();
        lines.extend([
            Line::Blank,
            text(format!("Les échos : {}", RUMORS[week % RUMORS.len()])),
        ]);
        lines.extend(second_rumor(ctx.reading_day, week, ctx.is_secret_unlocked).map(text));
        lines.extend([
            Line::Blank,
            text(format!(
                "Petite annonce : {}",
                ADS[(week * 7 + 3) % ADS.len()]
            )),
            Line::Rule,
            text(format!("L'énigme de la semaine : {}", riddle.question)),
        ]);

        let footer = if is_solved {
            lines.push(text("Bravo, énigme résolue !"));
            LETTER_FOOTER
        } else if let Some(input) = ctx.riddle_input {
            lines.push(typed_answer(input));
            INPUT_FOOTER
        } else {
            RIDDLE_FOOTER
        };

        if week > 0 {
            let previous = &RIDDLES[(week - 1) % RIDDLES.len()];
            lines.extend([
                Line::Blank,
                text(format!(
                    "Réponse de la semaine dernière : {}",
                    previous.answers[0]
                )),
            ]);
        }

        View {
            title: format!(
                "LA GAZETTE · lundi {} {}",
                date.day(),
                MONTHS[date.month0() as usize]
            ),
            lines,
            footer: footer.to_string(),
            frame: Frame::Newspaper,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::progress::Progress;
    use crate::story::clock::StoryClock;
    use crate::story::content::save_golden::render;
    use chrono::NaiveDate;

    #[test]
    fn test_old_gazette_keeps_its_frozen_weather() {
        let mut progress = Progress::default();
        progress
            .gazette_weather
            .insert(0, "Ta semaine : soleil, 21 °C.".to_string());
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 10, 22).unwrap(), 9);
        let gazette = Gazette::new(0, 5);
        let compose = |progress: &Progress| {
            gazette.compose(&ReadContext {
                letters: &[],
                progress,
                clock: &clock,
                reading_hour: 10,
                reading_day: 30,
                is_secret_unlocked: false,
                riddle_input: None,
            })
        };
        let first_weather = |view: View| match view.lines.into_iter().next() {
            Some(Line::Text(text)) => text,
            _ => String::new(),
        };

        assert_eq!(
            first_weather(compose(&progress)),
            "Ta semaine : soleil, 21 °C."
        );

        progress
            .gazette_weather
            .insert(1, "Ta semaine : orages, 9 °C.".to_string());
        let view = compose(&progress);
        assert_eq!(view.title, "LA GAZETTE · lundi 26 octobre");
        assert_eq!(first_weather(view), "Ta semaine : soleil, 21 °C.");
    }

    #[test]
    fn test_gazette_carries_a_second_echo_from_day_45() {
        let progress = Progress::default();
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 10, 22).unwrap(), 9);
        let gazette = Gazette::new(6, 47);
        let compose = |reading_day, is_secret_unlocked| {
            render(&gazette.compose(&ReadContext {
                letters: &[],
                progress: &progress,
                clock: &clock,
                reading_hour: 10,
                reading_day,
                is_secret_unlocked,
                riddle_input: None,
            }))
        };
        let echoes = |second: Option<&str>| {
            let first = format!("text: Les échos : {}\n", RUMORS[6]);
            match second {
                Some(second) => format!("{first}text: {second}\nblank\n"),
                None => format!("{first}blank\n"),
            }
        };

        assert!(compose(44, false).contains(&echoes(None)));
        let mystery = second_rumor(46, 6, false);
        assert!(mystery.is_some());
        assert!(compose(46, false).contains(&echoes(mystery)));
        let arcane = second_rumor(46, 6, true);
        assert_ne!(arcane, mystery);
        assert!(compose(46, true).contains(&echoes(arcane)));
    }

    #[test]
    fn test_gazette_shows_forecast_lines_once_fetched() {
        let mut progress = Progress::default();
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 10, 22).unwrap(), 9);
        let gazette = Gazette::new(0, 5);
        let compose = |progress: &Progress| {
            render(&gazette.compose(&ReadContext {
                letters: &[],
                progress,
                clock: &clock,
                reading_hour: 10,
                reading_day: 30,
                is_secret_unlocked: false,
                riddle_input: None,
            }))
        };

        assert!(compose(&progress).starts_with(
            "title: LA GAZETTE · lundi 26 octobre\n\
             text: Les prévisions ne sont pas encore arrivées.\n\
             blank\n"
        ));

        progress.gazette_weather.insert(
            0,
            "Prévisions de la semaine\u{202F}: 18 °C au plus, 2 °C au moins.\n\
             ⚠ Alerte orage\u{202F}: vendredi."
                .to_string(),
        );
        assert!(compose(&progress).starts_with(
            "title: LA GAZETTE · lundi 26 octobre\n\
             text: Prévisions de la semaine\u{202F}: 18 °C au plus, 2 °C au moins.\n\
             text: ⚠ Alerte orage\u{202F}: vendredi.\n\
             blank\n"
        ));
    }
}
