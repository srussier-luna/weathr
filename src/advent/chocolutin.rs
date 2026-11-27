pub const SIGNATURE: &str = "— Chocolutin";

pub const INTRODUCTION: &str = "Hop, me voilà\u{202F}! Moi, c'est Chocolutin, lutin chocolatier à l'atelier du Père Noël. Il m'envoie chez toi chaque jour jusqu'au 24, avec un chocolat tout juste démoulé. Je ne traîne pas, les rennes n'aiment pas attendre.";

pub const DAY_LINES: [&str; 24] = [
    "Le Père Noël signe chaque calendrier de l'atelier. D'un gros pouce plein de chocolat.",
    "Le savais-tu\u{202F}? Le Père Noël reçoit plus de lettres que tout le village réuni. Il les lit toutes, lunettes sur le nez.",
    "Pourquoi les lutins ne se perdent jamais\u{202F}? Ils ont toujours le Nord en tête.",
    "Ce matin, Tornade a mangé la carte de la tournée. On a tout recopié à la main.",
    "Le traîneau n'a pas de rétroviseur. Le Père Noël dit qu'il ne regarde jamais en arrière.",
    "C'est la Saint-Nicolas. Le Père Noël lui envoie toujours une carte, ils sont un peu cousins.",
    "Que dit un bonhomme de neige à un autre\u{202F}? Ça sent la carotte par ici.",
    "La barbe du Père Noël se brosse cent fois chaque soir. C'est Comète qui compte.",
    "J'ai trempé mon bonnet dans le chocolat fondu. Il sent bon, mais il colle.",
    "Les rennes adorent les clémentines. À l'atelier, on cache les épluchures.",
    "Le Père Noël porte trois pulls sous son manteau. Il refuse d'en parler.",
    "Le savais-tu\u{202F}? Le Père Noël a un bureau de poste rien qu'à lui. Le facteur de là-bas garde ses moufles toute l'année.",
    "Essai du traîneau aujourd'hui. Fringant a fait trois loopings, le Père Noël a perdu son bonnet.",
    "Que fait un lutin qui a froid\u{202F}? Il se met dans un coin, il y fait quatre-vingt-dix degrés.",
    "Le Père Noël connaît le nom de chaque renne. Même ceux des voisins.",
    "On a compté les grelots du traîneau ce matin. Il en manquait un\u{202F}: il était dans ma poche.",
    "Comment appelle-t-on un chat tombé dans la peinture le jour de Noël\u{202F}? Un chat-peint de Noël.",
    "Les carottes laissées sous le sapin, le Père Noël ne les mange jamais. Il les rapporte aux rennes.",
    "Tempérage du jour\u{202F}: trente et un degrés pile. Le chocolat brille comme les bottes du Père Noël.",
    "Le savais-tu\u{202F}? En janvier, le Père Noël fait une sieste de trois jours. Interdit de sonner.",
    "À l'atelier, on n'entend plus que les marteaux et le papier cadeau. Plus que quatre portes.",
    "La hotte du Père Noël a une poche secrète. Pour les bonbons qu'il garde pour la route.",
    "Dernière répétition\u{202F}: hop, les rennes décollent et le Père Noël crie «\u{202F}Plus vite\u{202F}!\u{202F}».",
    "Ce soir, c'est la grande tournée. Laisse la fenêtre entrouverte, le Père Noël passe en coup de vent. Joyeux Noël\u{202F}!",
];

pub fn day_line(day: u32) -> Option<&'static str> {
    DAY_LINES
        .get(usize::try_from(day.checked_sub(1)?).ok()?)
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent::chocolates::CHOCOLATES;
    use std::collections::BTreeSet;

    fn texts() -> impl Iterator<Item = &'static str> {
        DAY_LINES.into_iter().chain([INTRODUCTION])
    }

    #[test]
    fn test_each_day_has_its_own_line() {
        let lines: BTreeSet<_> = DAY_LINES.iter().collect();
        assert_eq!(lines.len(), DAY_LINES.len());
        assert_eq!(DAY_LINES.len(), CHOCOLATES.len());
        assert_eq!(day_line(1), Some(DAY_LINES[0]));
        assert_eq!(day_line(25), None);
        assert_eq!(day_line(0), None);
    }

    #[test]
    fn test_hop_comes_once_at_most_and_never_last() {
        for text in texts() {
            let lower = text.to_lowercase();
            assert!(lower.matches("hop").count() <= 1, "{text}");
            let last = lower
                .trim_end_matches(|ch: char| !ch.is_alphabetic())
                .rsplit(|ch: char| !ch.is_alphabetic())
                .next();
            assert_ne!(last, Some("hop"), "{text}");
        }
    }

    #[test]
    fn test_merry_christmas_waits_for_the_24th() {
        for line in &DAY_LINES[..23] {
            assert!(!line.contains("Joyeux Noël"), "{line}");
        }
        assert!(!INTRODUCTION.contains("Joyeux Noël"));
    }

    #[test]
    fn test_lines_never_name_a_chocolate() {
        for text in texts() {
            for chocolate in &CHOCOLATES {
                assert!(!text.contains(chocolate.name), "{text}");
            }
        }
    }

    #[test]
    fn test_french_punctuation_uses_a_narrow_no_break_space() {
        for text in texts() {
            for mark in ['?', '!', ':', ';'] {
                for (index, _) in text.match_indices(mark) {
                    assert!(text[..index].ends_with('\u{202F}'), "{text}");
                }
            }
        }
    }
}
