use crate::story::content::day_and_month;
use chrono::{Datelike, NaiveDate};

const NOTES: [&str; 8] = [
    "Toc toc. Personne\u{202F}? Alors j'ai glissé ton chocolat dans la boîte aux lettres, et ma moufle avec. Elle a l'air bien, là-dedans, au chaud.",
    "Ta boîte aux lettres m'a pincé le doigt. Hop, comme une poignée de main. Elle est très accueillante, ta boîte.",
    "Le jour où je suis parti en tournée, l'atelier du chocolat a fait une grande fête. Avec des guirlandes et tout. Ils sont tellement contents pour moi.",
    "J'ai sonné, sonné, sonné. Et puis j'ai vu que j'appuyais sur le drapeau de ta boîte aux lettres. Il monte et il descend, c'est formidable.",
    "Le Père Noël a rangé toutes les casseroles de l'atelier sur la plus haute étagère. Il sait que j'adore grimper. Je les attraperai à mon retour.",
    "Ta porte s'est refermée sur mon bonnet. Je l'ai laissé là, il garde ta maison en attendant. Il a l'air très fier.",
    "Ton paillasson dit «\u{202F}Bienvenue\u{202F}». Personne ne m'avait jamais écrit ça. Je me suis assis dessus un petit moment, pour en profiter.",
    "Ce mot sent bon\u{202F}? Je l'ai écrit avec un crayon en chocolat. Il en reste un bout dans la boîte, il est pour toi.",
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
