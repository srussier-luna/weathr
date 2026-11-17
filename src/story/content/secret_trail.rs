use super::chronicles::Riddle;
use super::{INPUT_FOOTER, text, typed_answer};
use crate::overlay::{Frame, Line, View};

pub const TRAIL_START_DAY: usize = 42;
const ALLUSION_EVERY: usize = 3;
const TAROT_CARD_TITLE: &str = "L'Arcane sans numéro";
const TAROT_CARD_TEXT: &str = "Une carte usée, violette, tiède comme une main. Un voyageur y marche vers le bord d'une falaise, un baluchon sur l'épaule, sans regarder ses pieds. Au dos, d'une écriture tremblée\u{202F}:";
pub const TAROT_RIDDLE: Riddle = Riddle {
    question: "«\u{202F}Je n'ai pas de numéro, pourtant je commence tout. Je marche sans savoir où, et c'est pour ça qu'on me suit. Qui suis-je\u{202F}?\u{202F}»",
    answers: &["le mat", "mat", "le fou", "fou"],
};
const TAROT_RIDDLE_FOOTER: &str = "[e] répondre à l'énigme  [Echap] fermer";
const TAROT_CARD_FOOTER: &str = "[Echap] fermer";
const TAROT_CARD_WARM: &str = "La carte se met à chauffer de plus en plus dans ta main. Quelque chose s'est ouvert, quelque part\u{202F}!";
const TAROT_CARD_COLD: &str = "La carte reste froide.";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NextPassage {
    Tonight(u32),
    Tomorrow(u32),
}

fn evening_line(passage: NextPassage) -> String {
    let (evening, hour) = match passage {
        NextPassage::Tonight(hour) => ("Ce soir", hour),
        NextPassage::Tomorrow(hour) => ("Demain soir", hour),
    };
    format!("{evening}, vers {hour} h, lève les yeux au-dessus de ta boîte aux lettres.")
}

const MYSTERY_RUMORS: [&str; 6] = [
    "On raconte qu'une lueur violette a traversé le ciel au-dessus du lavoir, bien après minuit.",
    "Le boulanger jure avoir vu un vieil homme assis… dans les airs. Il dit qu'il ne boit que de l'eau.",
    "Quelqu'un aurait laissé une carte sur le banc du square. Personne n'a osé la retourner.",
    "Les chats du village fixent tous le même coin de ciel, les nuits sans lune.",
    "Une voix très vieille, très douce, fredonne parfois vers minuit, du côté de chez toi.",
    "Certaines nuits, il manquerait une étoile. Elle ne serait jamais loin.",
];

const ARCANE_RUMORS: [&str; 4] = [
    "Certains soirs, un balai passe au-dessus des toits. Il ne vole jamais au hasard.",
    "Une petite boîte violette est apparue chez quelqu'un. Une seule personne sait l'ouvrir.",
    "Les anciens parlent d'un facteur qui ne livre que le soir. Ils baissent la voix en le disant.",
    "Il y aurait 22 cartes en tout. Qui lit la dernière connaît la fin, dit-on.",
];

const MYSTERY_ALLUSIONS: [&str; 6] = [
    "P.-S.\u{202F}: Tu n'as rien vu passer cette nuit\u{202F}? J'ai cru… non, rien.",
    "P.-S.\u{202F}: Ma grand-mère disait qu'à minuit, le jardin garde ce qu'on y perd.",
    "P.-S.\u{202F}: Drôle de lumière violette hier soir. Sûrement l'orage.",
    "P.-S.\u{202F}: Les cartes trouvées la nuit ne se ramassent pas par hasard, paraît-il.",
    "P.-S.\u{202F}: Si tu entends un froissement de paille au-dessus du toit, ne te retourne pas trop vite.",
    "P.-S.\u{202F}: Quelqu'un rôde après minuit. Pas méchant, je crois. Juste… très ancien.",
];

pub fn second_rumor(reading_day: usize, week: usize, is_unlocked: bool) -> Option<&'static str> {
    if reading_day < TRAIL_START_DAY {
        return None;
    }
    let rumors: &[&str] = if is_unlocked {
        &ARCANE_RUMORS
    } else {
        &MYSTERY_RUMORS
    };
    Some(rumors[week % rumors.len()])
}

pub fn mystery_allusion(letter_day: usize, is_unlocked: bool) -> Option<&'static str> {
    let since_start = letter_day
        .checked_sub(TRAIL_START_DAY)
        .filter(|_| !is_unlocked)?;
    (since_start % ALLUSION_EVERY == 0)
        .then(|| MYSTERY_ALLUSIONS[since_start / ALLUSION_EVERY % MYSTERY_ALLUSIONS.len()])
}

pub fn tarot_card_view(
    riddle_input: Option<&str>,
    next_passage: Option<NextPassage>,
    is_cold: bool,
) -> View {
    let mut lines = vec![
        text(TAROT_CARD_TEXT),
        Line::Blank,
        text(TAROT_RIDDLE.question),
    ];
    let footer = if let Some(passage) = next_passage {
        lines.extend([
            Line::Blank,
            text(TAROT_CARD_WARM),
            text(evening_line(passage)),
        ]);
        TAROT_CARD_FOOTER
    } else if let Some(input) = riddle_input {
        lines.extend([Line::Blank, typed_answer(input)]);
        lines.extend(is_cold.then(|| text(TAROT_CARD_COLD)));
        INPUT_FOOTER
    } else {
        TAROT_RIDDLE_FOOTER
    };
    View {
        title: TAROT_CARD_TITLE.to_string(),
        lines,
        footer: footer.to_string(),
        frame: Frame::StarryNight,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_rumor_starts_on_day_42() {
        assert_eq!(second_rumor(41, 6, false), None);
        assert_eq!(second_rumor(1, 0, true), None);
        assert_eq!(second_rumor(42, 6, false), Some(MYSTERY_RUMORS[0]));
        assert_eq!(second_rumor(43, 7, false), Some(MYSTERY_RUMORS[1]));
    }

    #[test]
    fn test_second_rumor_turns_to_the_arcanes_after_the_unlock() {
        assert_eq!(second_rumor(60, 8, true), Some(ARCANE_RUMORS[0]));
        assert_eq!(second_rumor(60, 9, true), Some(ARCANE_RUMORS[1]));
    }

    #[test]
    fn test_second_rumors_cycle_week_after_week() {
        let mystery: Vec<_> = (0..12)
            .filter_map(|week| second_rumor(50, week, false))
            .collect();
        assert_eq!(mystery[..6], MYSTERY_RUMORS);
        assert_eq!(mystery[6..], MYSTERY_RUMORS);
        let arcane: Vec<_> = (0..8)
            .filter_map(|week| second_rumor(50, week, true))
            .collect();
        assert_eq!(arcane[..4], ARCANE_RUMORS);
        assert_eq!(arcane[4..], ARCANE_RUMORS);
    }

    #[test]
    fn test_one_letter_out_of_three_alludes_from_day_42() {
        let alluding: Vec<usize> = (1..=60)
            .filter(|day| mystery_allusion(*day, false).is_some())
            .collect();
        assert_eq!(alluding, [42, 45, 48, 51, 54, 57, 60]);
    }

    #[test]
    fn test_allusions_cycle_through_the_list() {
        let allusions: Vec<_> = (42..42 + 3 * 7)
            .filter_map(|day| mystery_allusion(day, false))
            .collect();
        assert_eq!(allusions[..6], MYSTERY_ALLUSIONS);
        assert_eq!(allusions[6], MYSTERY_ALLUSIONS[0]);
    }

    #[test]
    fn test_tarot_card_opens_on_its_riddle() {
        let view = tarot_card_view(None, None, false);
        assert_eq!(view.title, "L'Arcane sans numéro");
        assert_eq!(view.frame, Frame::StarryNight);
        assert!(
            matches!(&view.lines[0], Line::Text(text) if text.starts_with("Une carte usée, violette"))
        );
        assert!(
            matches!(&view.lines[2], Line::Text(text) if text.ends_with("Qui suis-je\u{202F}?\u{202F}»"))
        );
        assert_eq!(view.footer, TAROT_RIDDLE_FOOTER);

        let typing = tarot_card_view(Some("le m"), None, false);
        assert!(
            matches!(typing.lines.last(), Some(Line::Text(text)) if text == "Ta réponse : le m_")
        );
        assert_eq!(typing.footer, INPUT_FOOTER);
    }

    fn texts(view: &View) -> Vec<&str> {
        view.lines
            .iter()
            .filter_map(|line| match line {
                Line::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn test_wrong_answer_leaves_the_card_cold_and_the_input_open() {
        let view = tarot_card_view(Some(""), None, true);
        assert_eq!(
            texts(&view)[2..],
            ["Ta réponse : _", "La carte reste froide."]
        );
        assert_eq!(view.footer, INPUT_FOOTER);

        let typing_again = tarot_card_view(Some("l"), None, false);
        assert!(!texts(&typing_again).contains(&TAROT_CARD_COLD));
    }

    #[test]
    fn test_right_answer_warms_the_card_and_names_the_evening_hour() {
        let view = tarot_card_view(None, Some(NextPassage::Tonight(20)), false);
        assert_eq!(
            texts(&view)[2..],
            [
                "La carte se met à chauffer de plus en plus dans ta main. Quelque chose s'est ouvert, quelque part\u{202F}!",
                "Ce soir, vers 20 h, lève les yeux au-dessus de ta boîte aux lettres.",
            ]
        );
        assert_eq!(view.footer, TAROT_CARD_FOOTER);
        assert_eq!(
            texts(&tarot_card_view(
                Some("fou"),
                Some(NextPassage::Tonight(20)),
                false
            )),
            texts(&view)
        );
    }

    #[test]
    fn test_card_solved_after_the_passage_points_to_tomorrow_evening() {
        let view = tarot_card_view(None, Some(NextPassage::Tomorrow(21)), false);
        assert_eq!(
            texts(&view).last(),
            Some(&"Demain soir, vers 21 h, lève les yeux au-dessus de ta boîte aux lettres.")
        );
    }

    #[test]
    fn test_no_allusion_once_unlocked() {
        assert!((1..=90).all(|day| mystery_allusion(day, true).is_none()));
    }
}
