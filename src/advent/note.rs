use crate::story::content::day_and_month;
use chrono::{Datelike, NaiveDate};

const NOTES: [&str; 8] = [
    "Toc toc. Personne\u{202F}? J'ai voulu glisser ton chocolat sous la porte. Il passait pas. Je l'ai mis dans la boîte aux lettres, et ma moufle avec.",
    "Ta boîte aux lettres m'a pincé le doigt. Hop, je l'ai pas laissée gagner. Elle a gardé un bout de mon bonnet, par contre.",
    "À l'atelier du chocolat, ils ont accroché un panneau «\u{202F}Interdit aux lutins pressés\u{202F}». Je me demande bien pour qui.",
    "J'ai sonné, j'ai attendu, j'ai sonné encore. Puis j'ai compris que j'appuyais sur le drapeau de ta boîte aux lettres.",
    "Le Père Noël dit que je suis bien plus utile sur les routes qu'à l'atelier du chocolat. Il a dit ça en rangeant les casseroles. Tu crois qu'il me rappellera après les fêtes\u{202F}?",
    "J'ai glissé sur ton paillasson. Le chocolat n'a rien. Ta porte non plus, c'est moi qui ai tout pris.",
    "Question\u{202F}: pourquoi la porte de l'atelier du chocolat a une deuxième serrure depuis que je suis parti\u{202F}? Sûrement des voleurs.",
    "Ton chocolat est dans la boîte. Le mot aussi. Mon crayon aussi, je crois. Si tu le trouves, garde-le, j'en perds un par jour.",
];

pub fn title(date: NaiveDate) -> String {
    format!("Mot de Chocolutin du {}", day_and_month(date))
}

pub fn text(date: NaiveDate) -> &'static str {
    NOTES[date.day0() as usize % NOTES.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn december(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 12, day).unwrap()
    }

    #[test]
    fn test_note_is_titled_with_its_day() {
        assert_eq!(title(december(1)), "Mot de Chocolutin du 1er décembre");
        assert_eq!(title(december(2)), "Mot de Chocolutin du 2 décembre");
    }

    #[test]
    fn test_notes_change_from_one_day_to_the_next() {
        for day in 1..24 {
            assert_ne!(text(december(day)), text(december(day + 1)));
        }
    }

    #[test]
    fn test_notes_space_french_punctuation() {
        for note in NOTES {
            for mark in ['?', '!', ':', ';'] {
                assert!(!note.contains(&format!(" {mark}")), "{note}");
            }
        }
    }
}
