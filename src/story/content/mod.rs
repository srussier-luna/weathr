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

pub use gazette::MONTHS;
pub use night_postscript::is_night_hour;
pub use postcard::{postcard_lines, postcard_title};

const LETTER_FOOTER: &str = "[c] suivante  [<] [>] parcourir  [l] sommaire  [Echap] fermer";

/// Everything a Lettre may read when it is opened; content never fetches anything itself.
pub struct ReadContext<'a> {
    /// Village Lettres, `lettres.txt` overrides included.
    pub letters: &'a [Letter],
    /// Answers, solved riddles and the weather snapshots frozen at delivery.
    pub progress: &'a Progress,
    pub clock: &'a StoryClock,
    /// Local hour at which the Lettre is read.
    pub reading_hour: u32,
    /// Riddle answer being typed, if any.
    pub riddle_input: Option<&'a str>,
}

/// What a Lettre says once opened.
pub trait MailContent {
    fn compose(&self, ctx: &ReadContext) -> View;
}

pub fn text(content: impl Into<String>) -> Line {
    Line::Text(content.into())
}

/// The Lettre screen: Lettre `number` of `total`, delivered on Jour de jeu `day`.
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

/// How a Lettre is listed in the Courrier.
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
