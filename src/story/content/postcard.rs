use super::chronicles::CITIES;
use super::{LETTER_FOOTER, MailContent, ReadContext, text};
use crate::overlay::{Frame, Line, View};
use crate::progress::Progress;

/// A Carte postale from Sam, with the weather frozen when it arrived.
pub struct Postcard {
    index: usize,
    counter: String,
}

impl Postcard {
    pub fn new(index: usize, counter: String) -> Self {
        Self { index, counter }
    }
}

impl MailContent for Postcard {
    fn compose(&self, ctx: &ReadContext) -> View {
        View {
            title: format!(
                "Carte postale {} : {}",
                self.counter,
                postcard_title(self.index)
            ),
            lines: postcard_lines(self.index, ctx.progress),
            footer: LETTER_FOOTER.to_string(),
            frame: Frame::Postcard,
        }
    }
}

/// Body of Carte postale `index`, as shown in the Lettre and in the Album.
pub fn postcard_lines(index: usize, progress: &Progress) -> Vec<Line> {
    let city = &CITIES[index % CITIES.len()];
    let weather = progress
        .postcard_weather
        .get(&index)
        .map_or("La météo s'est perdue en route.", String::as_str);
    vec![text(city.note), text(weather), Line::Blank, text("— Sam")]
}

/// City and country of Carte postale `index`.
pub fn postcard_title(index: usize) -> String {
    let city = &CITIES[index % CITIES.len()];
    format!("{}, {}", city.name, city.country)
}
