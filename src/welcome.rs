use crate::overlay::{self, Frame, Line, View};
use crate::views;

pub const ARRIVAL: &str = "Salut, nouveau voisin. Te voilà. Le village a beaucoup de choses à te raconter. Tiens, c'est pour toi.";
pub const DEPARTURE: &str = "Je viens de te déposer ton courrier. Moi, je file, à demain\u{202F}!";
pub const FOOTER: &str = "[Entrée] suite  [Echap] passer";
pub const NOTE_FRAME: Frame = Frame::Parchment;
const TITLE: &str = "Bienvenue au village";
const SIGNATURE: &str = views::POSTMAN_SIGNATURE;
const UNFOLD_FRAMES: u16 = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Arriving,
    Greeting,
    Unfolding(u16),
    Reading(usize),
    Farewell,
}

impl Step {
    pub fn tick(self, has_arrived: bool) -> Self {
        match self {
            Self::Arriving if has_arrived => Self::Greeting,
            Self::Unfolding(frame) if frame + 1 >= UNFOLD_FRAMES => Self::Reading(0),
            Self::Unfolding(frame) => Self::Unfolding(frame + 1),
            step => step,
        }
    }

    pub fn advance(self) -> Option<Self> {
        match self {
            Self::Arriving => Some(Self::Arriving),
            Self::Greeting => Some(Self::Unfolding(0)),
            Self::Unfolding(_) => Some(Self::Reading(0)),
            Self::Reading(_) => Some(Self::Farewell),
            Self::Farewell => None,
        }
    }

    pub fn scroll(self, delta: isize, last: usize) -> Self {
        match self {
            Self::Reading(first) => Self::Reading(first.saturating_add_signed(delta).min(last)),
            other => other,
        }
    }

    pub fn hides_mail(self) -> bool {
        self != Self::Farewell
    }
}

fn note() -> Vec<Line> {
    let text = |content: &str| Line::Text(content.to_string());
    let key = |content: &str| Line::Preformatted(content.to_string());
    vec![
        text(
            "Règle n°1 du village\u{202F}: on lit son courrier. Règle n°2\u{202F}: on ne demande pas qui habite au n°13.",
        ),
        text(
            "Chaque jour, une lettre t'attend. La plupart viennent des voisins. Pour les autres… on ne sait pas trop.",
        ),
        Line::Blank,
        text("Drapeau rouge levé sur la boîte\u{202F}: du courrier pour toi."),
        key("[c]  lire le courrier"),
        key("[l]  relire les lettres reçues"),
        Line::Blank,
        text(
            "Au fil des jours, tu gagnes des badges (météo, fêtes, secrets…) et des souvenirs du village.",
        ),
        key("[b]  badges"),
        key("[a]  album des souvenirs"),
        Line::Blank,
        text(
            "Un trou de mémoire\u{202F}? [h] montre tout ce que tu peux faire. Le rappel est toujours en haut de l'écran.",
        ),
        key("[q]  quitter"),
        Line::Blank,
        text(
            "Prends ton temps. Le village n'est pas pressé. Lis bien tout. Certains détails comptent plus qu'ils n'en ont l'air\u{202F};)",
        ),
        Line::Blank,
        text(SIGNATURE),
    ]
}

fn note_lines(width: usize) -> Vec<String> {
    overlay::lay_out(&note(), width)
}

pub fn scroll_limit(rows: usize, width: usize) -> usize {
    views::scroll_limit(&note_lines(width), rows)
}

pub fn note_view(first: usize, rows: usize, width: usize) -> View {
    views::scrolled_view(TITLE, note_lines(width), first, rows, FOOTER, NOTE_FRAME)
}

pub fn unfolding_view(frame: u16, rows: usize, width: usize) -> View {
    let mut view = note_view(0, rows, width);
    let shown = view.lines.len() * usize::from(frame) / usize::from(UNFOLD_FRAMES);
    view.lines.truncate(shown);
    view.footer.clear();
    view
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEPS: [Step; 5] = [
        Step::Arriving,
        Step::Greeting,
        Step::Unfolding(3),
        Step::Reading(2),
        Step::Farewell,
    ];

    fn rows(view: &View) -> Vec<&str> {
        view.lines
            .iter()
            .filter_map(|line| match line {
                Line::Preformatted(text) => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn test_entree_walks_every_step_then_ends() {
        let mut step = Step::Arriving.tick(false);
        assert_eq!(step, Step::Arriving);
        assert_eq!(step.advance(), Some(Step::Arriving));

        step = step.tick(true);
        assert_eq!(step, Step::Greeting);
        let mut seen = vec![step];
        while let Some(next) = step.advance() {
            step = next;
            seen.push(step);
        }
        assert_eq!(
            seen,
            [
                Step::Greeting,
                Step::Unfolding(0),
                Step::Reading(0),
                Step::Farewell
            ]
        );
    }

    #[test]
    fn test_note_unfolds_in_under_a_second_then_shows_in_full() {
        let mut step = Step::Unfolding(0);
        let mut previous = 0;
        let mut frames = 0;
        while let Step::Unfolding(frame) = step {
            let shown = unfolding_view(frame, 40, 50).lines.len();
            assert!(shown >= previous);
            previous = shown;
            step = step.tick(true);
            frames += 1;
        }
        assert_eq!(step, Step::Reading(0));
        assert!(frames < 30);
        assert!(previous < note_view(0, 40, 50).lines.len());
    }

    #[test]
    fn test_mail_stays_hidden_until_the_farewell() {
        for step in STEPS {
            assert_eq!(step.hides_mail(), step != Step::Farewell, "{step:?}");
        }
    }

    #[test]
    fn test_only_the_note_scrolls() {
        assert_eq!(Step::Reading(2).scroll(1, 3), Step::Reading(3));
        assert_eq!(Step::Reading(3).scroll(1, 3), Step::Reading(3));
        assert_eq!(Step::Reading(0).scroll(-1, 3), Step::Reading(0));
        assert_eq!(Step::Greeting.scroll(1, 3), Step::Greeting);
    }

    #[test]
    fn test_note_lists_every_key_and_signs_as_the_facteur() {
        let view = note_view(0, 40, 50);
        assert_eq!(view.title, TITLE);
        assert_eq!(view.frame, Frame::Parchment);
        assert_eq!(view.footer, FOOTER);
        let lines = rows(&view);
        for key in [
            "[c]  lire le courrier",
            "[l]  relire les lettres reçues",
            "[b]  badges",
            "[a]  album des souvenirs",
            "[q]  quitter",
        ] {
            assert!(lines.contains(&key), "{key}");
        }
        assert_eq!(lines.last(), Some(&"— Le Facteur"));
    }

    #[test]
    fn test_note_wraps_and_scrolls_in_a_small_terminal() {
        let (rows_shown, width) = (8, 50);
        let top = note_view(0, rows_shown, width);
        assert_eq!(
            top.footer,
            "[↑] [↓] défiler  [Entrée] suite  [Echap] passer"
        );

        let last = scroll_limit(rows_shown, width);
        let lines = note_lines(width);
        assert_eq!(last, lines.len() - rows_shown);
        assert_eq!(
            rows(&note_view(last, rows_shown, width)).last(),
            Some(&SIGNATURE)
        );
        assert!(lines.iter().all(|line| line.chars().count() <= width));
        assert!(
            lines
                .iter()
                .all(|line| !line.starts_with([':', ';', '!', '?', '…']))
        );
    }
}
