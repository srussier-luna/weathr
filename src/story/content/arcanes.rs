use super::{MailContent, ReadContext, text};
use crate::overlay::{Frame, View};

pub const ARCANES: &[&str] = &[];
pub const FIRST_NUMBER: usize = 1;
pub const SAMPLE_NUMBER: usize = 0;
const SAMPLE: &str = "Ceci est une Arcane d'exemple. Les vraies arriveront une à une, le soir.";
const ARCANE_FOOTER: &str = "[l] sommaire  [Echap] fermer";

pub fn title(number: usize) -> String {
    format!("Arcane n° {number}")
}

fn body(number: usize) -> &'static str {
    match number {
        SAMPLE_NUMBER => SAMPLE,
        _ => ARCANES[number - FIRST_NUMBER],
    }
}

pub struct Arcane {
    number: usize,
}

impl Arcane {
    pub fn new(number: usize) -> Self {
        Self { number }
    }
}

impl MailContent for Arcane {
    fn compose(&self, _ctx: &ReadContext) -> View {
        View {
            title: title(self.number),
            lines: vec![text(body(self.number))],
            footer: ARCANE_FOOTER.to_string(),
            frame: Frame::StarryNight,
        }
    }
}
