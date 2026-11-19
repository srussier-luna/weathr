use super::recipes::{DIETS, Diet, ROSE_SENDER, diet_labels, diets_from_ids};
use super::{LETTER_FOOTER, MailContent, ReadContext, text};
use crate::overlay::{Frame, Line, View};
use std::collections::BTreeSet;

const DIET_LETTER: &str = "Une idée m'est venue en pétrissant\u{202F}: t'envoyer une de mes recettes tous les dix jours. Avant d'allumer le four, dis-moi ce qui va dans ton assiette et ce qui n'y va pas. Coche ce qui te concerne, puis renvoie-moi ma lettre. Rien de coché\u{202F}? Tu manges de tout, et j'ai carte blanche.";
const CHECKLIST_FOOTER: &str =
    "[1]…[8] cocher  [Entrée] envoyer  [c] suivante  [<] [>]  [Echap] fermer";
const EATS_EVERYTHING: &str = "je mange de tout";
const CHECKBOX_WIDTH: usize = 22;

pub struct DietLetter {
    counter: String,
}

impl DietLetter {
    pub fn new(counter: String) -> Self {
        Self { counter }
    }
}

impl MailContent for DietLetter {
    fn compose(&self, ctx: &ReadContext) -> View {
        let progress = ctx.progress;
        let mut lines = vec![
            text(DIET_LETTER),
            Line::Blank,
            text(format!("— {ROSE_SENDER}")),
            Line::Blank,
        ];
        let footer = if progress.diet_answered_day.is_some() {
            let diets = diets_from_ids(&progress.diets);
            lines.push(text(format!("Ta réponse\u{202F}: {}", answer(&diets))));
            LETTER_FOOTER
        } else {
            lines.extend(diet_checklist(ctx.diet_draft));
            CHECKLIST_FOOTER
        };
        View {
            title: format!("Lettre {}", self.counter),
            lines,
            footer: footer.to_string(),
            frame: Frame::Parchment,
        }
    }
}

fn answer(diets: &BTreeSet<Diet>) -> String {
    if diets.is_empty() {
        EATS_EVERYTHING.to_string()
    } else {
        diet_labels(diets)
    }
}

pub fn diet_checklist(diets: &BTreeSet<Diet>) -> Vec<Line> {
    let boxes: Vec<String> = DIETS
        .iter()
        .enumerate()
        .map(|(index, diet)| {
            let mark = if diets.contains(diet) { '✓' } else { ' ' };
            format!("[{}] {mark} {}", index + 1, diet.label())
        })
        .collect();
    boxes
        .chunks(2)
        .map(|pair| {
            let row: String = pair
                .iter()
                .map(|cell| format!("{cell:<CHECKBOX_WIDTH$}"))
                .collect();
            Line::Preformatted(row.trim_end().to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overlay;
    use crate::progress::Progress;
    use crate::story::StoryClock;
    use chrono::NaiveDate;

    fn read(progress: &Progress, draft: &BTreeSet<Diet>) -> View {
        let clock = StoryClock::new(NaiveDate::from_ymd_opt(2026, 11, 2).unwrap(), 9);
        let context = ReadContext {
            letters: &[],
            progress,
            clock: &clock,
            reading_hour: 12,
            riddle_input: None,
            diet_draft: draft,
        };
        DietLetter::new("3/9".to_string()).compose(&context)
    }

    #[test]
    fn test_unanswered_letter_ends_with_eight_boxes() {
        let draft = BTreeSet::from([Diet::Vegetarian, Diet::NoPork]);
        let view = read(&Progress::default(), &draft);
        let lines = overlay::lay_out(&view.lines, 60);

        assert_eq!(view.title, "Lettre 3/9");
        assert_eq!(view.frame, Frame::Parchment);
        assert_eq!(view.footer, CHECKLIST_FOOTER);
        assert_eq!(
            lines[lines.len() - 4..],
            [
                "[1] ✓ végétarien      [2]   vegan",
                "[3]   sans légume     [4]   sans viande",
                "[5]   sans poisson    [6] ✓ sans porc",
                "[7]   full protéine   [8]   sans fruit",
            ]
        );
        assert!(lines.contains(&"— Rose, la boulangère".to_string()));
    }

    #[test]
    fn test_answered_letter_shows_the_chosen_diets() {
        let mut progress = Progress::default();
        progress.diet_answered_day = Some(10);
        progress.diets = BTreeSet::from(["vegetarian".to_string(), "no_pork".to_string()]);
        let view = read(&progress, &BTreeSet::from([Diet::Vegan]));
        let lines = overlay::lay_out(&view.lines, 60);

        assert_eq!(
            lines.last().unwrap(),
            "Ta réponse\u{202F}: végétarien, sans porc"
        );
        assert!(!lines.iter().any(|line| line.starts_with("[1]")));
        assert_eq!(view.footer, LETTER_FOOTER);

        progress.diets.clear();
        let view = read(&progress, &BTreeSet::new());
        assert_eq!(
            overlay::lay_out(&view.lines, 60).last().unwrap(),
            "Ta réponse\u{202F}: je mange de tout"
        );
    }
}
