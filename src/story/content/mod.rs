pub mod chronicles;
mod gazette;
pub mod letters;
mod night_postscript;
mod postcard;
mod village;
mod written;

use super::clock::StoryClock;
use super::event::Entry;
use crate::overlay::{Frame, Line, View};
use crate::progress::Progress;
use chronicles::{
    ARCHIVES, CITIES, MARCEL_LETTERS, MARCEL_SENDER, NEIGHBOURS_LETTERS, NEIGHBOURS_SENDER,
    RECIPES, SAM_LETTERS, SAM_SENDER,
};
use gazette::Gazette;
use letters::{CREEPY_LETTERS, CREEPY_SENDER, Letter};
use night_postscript::NightPostscript;
use postcard::Postcard;
use village::VillageLetter;
use written::Written;

pub use night_postscript::is_night_hour;
pub use postcard::{postcard_lines, postcard_title};

const LETTER_FOOTER: &str = "[c] suivante  [<] [>] parcourir  [l] sommaire  [Echap] fermer";
pub const MONTHS: [&str; 12] = [
    "janvier",
    "février",
    "mars",
    "avril",
    "mai",
    "juin",
    "juillet",
    "août",
    "septembre",
    "octobre",
    "novembre",
    "décembre",
];

/// Content never fetches anything itself: everything a Lettre may read is passed in here.
pub struct ReadContext<'a> {
    pub letters: &'a [Letter],
    pub progress: &'a Progress,
    pub clock: &'a StoryClock,
    pub reading_hour: u32,
    pub riddle_input: Option<&'a str>,
}

pub trait MailContent {
    fn compose(&self, ctx: &ReadContext) -> View;
}

pub fn text(content: impl Into<String>) -> Line {
    Line::Text(content.into())
}

pub fn entry_view(
    day: usize,
    entry: Entry,
    number: usize,
    total: usize,
    ctx: &ReadContext,
) -> View {
    content_for(day, entry, format!("{number}/{total}")).compose(ctx)
}

fn content_for(day: usize, entry: Entry, counter: String) -> Box<dyn MailContent> {
    match entry {
        Entry::Main(main) => Box::new(NightPostscript::new(VillageLetter::new(main, counter))),
        Entry::Creepy(index) => Box::new(Written::new(
            format!("Lettre {counter} (glissée sous la porte)"),
            CREEPY_LETTERS[index],
            CREEPY_SENDER,
            Frame::Parchment,
        )),
        Entry::Sam(index) => Box::new(Written::new(
            format!("Lettre {counter}"),
            SAM_LETTERS[index],
            SAM_SENDER,
            Frame::Parchment,
        )),
        Entry::Neighbours(index) => Box::new(Written::new(
            format!("Lettre {counter}"),
            NEIGHBOURS_LETTERS[index],
            NEIGHBOURS_SENDER,
            Frame::Parchment,
        )),
        Entry::Marcel(index) => Box::new(Written::new(
            format!("Lettre {counter}"),
            MARCEL_LETTERS[index],
            MARCEL_SENDER,
            Frame::Parchment,
        )),
        Entry::Postcard(index) => Box::new(Postcard::new(index, counter)),
        Entry::Archive(index) => {
            let (source, body) = ARCHIVES[index];
            Box::new(Written::new(
                format!("Archives de la mairie {counter} : {source}"),
                body,
                "Copie transmise par Odile",
                Frame::Folder,
            ))
        }
        Entry::Gazette(week) => Box::new(Gazette::new(week, day)),
        Entry::Recipe(month) => {
            let (name, body) = RECIPES[month];
            Box::new(Written::new(
                format!("Recette du mois {counter} : {name}"),
                body,
                "Rose, la boulangère",
                Frame::Recipe,
            ))
        }
    }
}

pub fn label(entry: Entry, ctx: &ReadContext) -> String {
    match entry {
        Entry::Main(main) => ctx.letters[main].sender.clone(),
        Entry::Sam(_) => SAM_SENDER.to_string(),
        Entry::Neighbours(_) => NEIGHBOURS_SENDER.to_string(),
        Entry::Marcel(_) => MARCEL_SENDER.to_string(),
        Entry::Archive(index) => format!("Archives : {}", ARCHIVES[index].0),
        Entry::Gazette(_) => "La Gazette du lundi".to_string(),
        Entry::Recipe(month) => format!("Recette du mois : {}", RECIPES[month].0),
        Entry::Postcard(index) => format!("Carte postale : {}", CITIES[index % CITIES.len()].name),
        Entry::Creepy(_) => CREEPY_SENDER.to_string(),
    }
}

#[cfg(test)]
pub(crate) mod save_golden {
    use super::ReadContext;
    use super::letters::{self, Letter};
    use crate::overlay::{Line, View};
    use crate::progress::Progress;
    use crate::story::{StoryClock, Timeline, sources};
    use chrono::NaiveDate;

    const SAVE: &str = r#"{
  "started_on": "2026-09-01",
  "letters_read": 37,
  "answers": { "bow": 2, "well": 1 },
  "riddles_solved": [1],
  "postcard_weather": { "0": "Il fait beau à Lisbonne." },
  "gazette_weather": { "0": "Ta semaine : soleil et 21 °C.", "3": "Ta semaine : pluie." }
}"#;
    const FIXTURE: &str = include_str!("../testdata/save_37_golden.txt");
    const FIRST_ENTRY: &str = "== entry 0 night false\n";

    pub struct SavedGame {
        pub letters: Vec<Letter>,
        pub progress: Progress,
        pub clock: StoryClock,
        pub timeline: Timeline,
        pub available: usize,
    }

    impl SavedGame {
        pub fn load() -> Self {
            let letters = letters::defaults();
            let progress: Progress = serde_json::from_str(SAVE).unwrap();
            let clock = StoryClock::new(progress.start_date().unwrap(), 0);
            let today = clock.day_on(
                NaiveDate::from_ymd_opt(2026, 10, 26)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
            );
            let timeline = Timeline::new(&sources::registered(letters.len()), &clock, today);
            let available = timeline.delivered_by(today);
            Self {
                letters,
                progress,
                clock,
                timeline,
                available,
            }
        }

        pub fn context(&self, reading_hour: u32) -> ReadContext<'_> {
            ReadContext {
                letters: &self.letters,
                progress: &self.progress,
                clock: &self.clock,
                reading_hour,
                riddle_input: None,
            }
        }
    }

    pub fn courrier_fixture() -> &'static str {
        &FIXTURE[..FIXTURE.find(FIRST_ENTRY).unwrap()]
    }

    pub fn entries_fixture() -> &'static str {
        &FIXTURE[FIXTURE.find(FIRST_ENTRY).unwrap()..]
    }

    pub fn render(view: &View) -> String {
        let mut out = format!("title: {}\n", view.title);
        for line in &view.lines {
            match line {
                Line::Text(text) => out.push_str(&format!("text: {text}\n")),
                Line::Preformatted(text) => out.push_str(&format!("pre: {text}\n")),
                Line::Centered(text) => out.push_str(&format!("centered: {text}\n")),
                Line::Rule => out.push_str("rule\n"),
                Line::Blank => out.push_str("blank\n"),
            }
        }
        out.push_str(&format!("footer: {}\n", view.footer));
        out.push_str(&format!("frame: {:?}\n", view.frame));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::entry_view;
    use super::save_golden::{SavedGame, entries_fixture, render};

    #[test]
    fn test_save_opens_each_entry_as_before_refactor() {
        let save = SavedGame::load();
        let mut out = String::new();
        for (is_night, reading_hour) in [(false, 12), (true, 23)] {
            let context = save.context(reading_hour);
            for (index, (day, entry)) in save.timeline.mail().take(save.available).enumerate() {
                out.push_str(&format!("== entry {index} night {is_night}\n"));
                out.push_str(&render(&entry_view(
                    day,
                    entry,
                    index + 1,
                    save.available,
                    &context,
                )));
            }
        }
        assert_eq!(out, entries_fixture());
    }
}
