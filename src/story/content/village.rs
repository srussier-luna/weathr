use super::chronicles::DRAWINGS;
use super::letters::{choice_for, echo_for};
use super::{LETTER_FOOTER, ReadContext, text};
use crate::overlay::{Frame, Line, View};

const QUESTION_FOOTER: &str = "[1] [2] répondre  [c] suivante  [<] [>]  [Echap] fermer";

/// A village Lettre (from `lettres.txt`), with Léo's drawings, echoes of past answers and its question.
pub struct VillageLetter {
    main: usize,
    counter: String,
}

impl VillageLetter {
    pub fn new(main: usize, counter: String) -> Self {
        Self { main, counter }
    }

    pub fn main(&self) -> usize {
        self.main
    }

    /// Composes the Lettre with `postscript`, if any, just above the signature.
    pub fn compose_with(&self, ctx: &ReadContext, postscript: Option<&str>) -> View {
        let letter = &ctx.letters[self.main];
        let number = self.main + 1;
        let answers = &ctx.progress.answers;
        let mut lines: Vec<Line> = letter
            .body
            .lines()
            .map(|paragraph| {
                if paragraph.trim().is_empty() {
                    Line::Blank
                } else {
                    text(paragraph)
                }
            })
            .collect();

        if let Some((_, drawing)) = DRAWINGS.iter().find(|(letter, _)| *letter == number) {
            lines.push(Line::Blank);
            lines.extend(
                drawing
                    .iter()
                    .map(|row| Line::Preformatted(row.to_string())),
            );
        }
        if let Some(echo) = echo_for(number, answers) {
            lines.extend([Line::Blank, text(echo)]);
        }
        if let Some(postscript) = postscript {
            lines.extend([Line::Blank, text(postscript)]);
        }
        lines.extend([Line::Blank, text(format!("— {}", letter.sender))]);

        let mut footer = LETTER_FOOTER;
        if let Some(choice) = choice_for(number) {
            lines.extend([Line::Blank, text(format!("Question : {}", choice.prompt))]);
            let answer = answers
                .get(choice.id)
                .and_then(|answer| choice.options.get(usize::from(*answer).checked_sub(1)?));
            match answer {
                Some(option) => lines.push(text(format!("Ta réponse : {option}"))),
                None => {
                    lines.push(text(format!(
                        "[1] {}   [2] {}",
                        choice.options[0], choice.options[1]
                    )));
                    footer = QUESTION_FOOTER;
                }
            }
        }

        View {
            title: format!("Lettre {}", self.counter),
            lines,
            footer: footer.to_string(),
            frame: Frame::Parchment,
        }
    }
}
