use super::{LETTER_FOOTER, MailContent, ReadContext, text};
use crate::overlay::{Line, View};

/// A static Lettre: one paragraph and a signature.
pub struct Written {
    title: String,
    body: &'static str,
    sender: &'static str,
}

impl Written {
    pub fn new(title: String, body: &'static str, sender: &'static str) -> Self {
        Self {
            title,
            body,
            sender,
        }
    }
}

impl MailContent for Written {
    fn compose(&self, _ctx: &ReadContext) -> View {
        View {
            title: self.title.clone(),
            lines: vec![
                text(self.body),
                Line::Blank,
                text(format!("— {}", self.sender)),
            ],
            footer: LETTER_FOOTER.to_string(),
        }
    }
}
