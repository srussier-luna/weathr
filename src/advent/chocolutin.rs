pub const SIGNATURE: &str = "— Chocolutin";

pub const INTRODUCTION: &str = "Hop, me voilà\u{202F}! Moi, c'est Chocolutin, de l'atelier du Père Noël. Il m'a choisi spécialement pour t'apporter un chocolat chaque jour jusqu'au 24. Il a même dit que l'atelier serait tout calme sans moi. Il est gentil, il veut que je prenne l'air.";

pub const SPEECHES: [&str; 24] = [
    "Tiens, c'est pour toi, tout juste démoulé. J'ai mis mes bottes à l'envers ce matin, et j'ai trouvé ta maison quand même. C'est une très bonne journée.",
    "Voilà ton chocolat, je l'ai tenu à deux mains tout le chemin. Ce matin, j'ai rangé les mille deux cents moules de l'atelier par couleur. Ça fait un arc-en-ciel, c'est très joli.",
    "Tiens, regarde comme il brille. Avant, c'est moi qui tempérais le chocolat à l'atelier. Quelqu'un le fait à ma place pour que je puisse me promener, c'est gentil.",
    "Le tien, je l'ai gardé à l'abri de la neige. Ce matin, j'ai mis du sucre glace à la place de la farine dans les biscuits des rennes. Ils ont fait une drôle de tête, je crois qu'ils étaient émus.",
    "Tiens, je ne me suis trompé de maison qu'une fois, c'est un record. Avant, je triais les lettres du Père Noël avec mon chocolat chaud à côté. Maintenant, il m'a donné un travail rien que pour moi, dehors, avec plein d'air.",
    "C'est la Saint-Nicolas, tu le savais\u{202F}? Je lui ai posté une carte par la cheminée, elle est partie dans la fumée. Comme ça, elle arrive par le ciel, c'est magique.",
    "Tornade a mangé ma carte de la tournée ce matin. Elle avait faim, la pauvre. Moi, j'ai suivi l'odeur du chocolat et je suis arrivé chez toi.",
    "Tiens, celui-là n'est pas tombé une seule fois. Le Père Noël a mis une deuxième serrure à l'atelier de chocolat. Il protège bien le chocolat, je suis fier de lui.",
    "Tu sens cette odeur\u{202F}? J'ai trempé mon bonnet dans le chocolat fondu. Maintenant, il sent bon toute la journée.",
    "Tiens, garde-le au chaud. Ce matin, j'ai demandé au Père Noël si je pouvais retourner à l'atelier. Il a ri très fort, ça veut sûrement dire oui.",
    "Ce matin, j'ai glissé sur le verglas et j'ai fait trois tours sur moi-même. Comme une toupie, c'était super. Et ton chocolat n'a rien eu.",
    "À l'atelier, ils ont accroché mon portrait près de la porte, avec écrit «\u{202F}Attention\u{202F}» dessous. Tout le monde me regarde en passant. Tiens, voilà le tien.",
    "Tiens, et regarde la neige sur mon bonnet. Fringant a fait trois loopings à l'essai du traîneau, et moi j'ai fait le quatrième en tombant. Le Père Noël a dit qu'il n'avait jamais vu ça.",
    "J'ai proposé d'aider au bureau de poste du Pôle Nord. Ils ont dit «\u{202F}Non merci, surtout pas\u{202F}», tellement ils sont polis. Tout le monde est gentil avec moi, tu as remarqué\u{202F}?",
    "Ce matin, j'ai emballé trois cadeaux. Le troisième, c'était Cupidon, avec un gros nœud rouge. Il était tout beau, tu aurais vu ça\u{202F}!",
    "Tout l'atelier a cherché un grelot du traîneau ce matin. Je l'ai retrouvé dans ma poche, c'est moi le champion. Tiens, ton chocolat, lui, je ne l'ai pas perdu.",
    "Le Père Noël dit que je suis le plus rapide pour livrer. Il me rendra ma louche à chocolat l'an prochain, j'en suis sûr. Tiens, il est encore tout frais.",
    "Hier soir, j'ai fait un chocolat tout seul dans ma chambre. La casserole est restée collée au plafond, comme une lampe. C'est très joli, tu sais.",
    "À l'atelier, on tempère à trente et un degrés pile. Moi, on m'a confié une mission spéciale\u{202F}: ne jamais toucher le thermomètre. Je la réussis très bien, tu sais.",
    "Ce matin, j'ai aidé à charger le traîneau et je me suis endormi dans la hotte. Comète m'a sorti de là, il est tellement fort. Ton chocolat, lui, était bien rangé.",
    "J'ai glissé une clémentine dans un moule à chocolat. Personne n'avait jamais eu cette idée, ils étaient tous bouche bée. Toi aussi, tu vas adorer\u{202F}!",
    "Je me suis entraîné tout le mois à ne rien renverser. Ce matin, je n'ai renversé qu'un seul seau, et le Père Noël a souri. Je suis le plus heureux des lutins, tu sais.",
    "Hop, j'ai cousu un grelot sur mon bonnet ce matin. Maintenant, tout le monde m'entend arriver de très loin, c'est pratique. Tiens, ton chocolat, je l'ai gardé bien au chaud.",
    "Voilà le dernier, je te l'ai porté comme un trésor. Ce soir, c'est la grande tournée, et l'an prochain, je retourne à l'atelier de chocolat, c'est sûr. Joyeux Noël\u{202F}!",
];

pub fn speech(day: u32) -> Option<&'static str> {
    SPEECHES
        .get(usize::try_from(day.checked_sub(1)?).ok()?)
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent::chocolates::CHOCOLATES;
    use std::collections::BTreeSet;

    fn texts() -> impl Iterator<Item = &'static str> {
        SPEECHES
            .into_iter()
            .chain(CHOCOLATES.iter().map(|chocolate| chocolate.wrapper))
            .chain([INTRODUCTION])
    }

    #[test]
    fn test_each_day_has_its_own_speech_and_wrapper() {
        let speeches: BTreeSet<_> = SPEECHES.iter().collect();
        let wrappers: BTreeSet<_> = CHOCOLATES
            .iter()
            .map(|chocolate| chocolate.wrapper)
            .collect();
        assert_eq!(speeches.len(), SPEECHES.len());
        assert_eq!(wrappers.len(), CHOCOLATES.len());
        assert_eq!(SPEECHES.len(), CHOCOLATES.len());
        for (speech, chocolate) in SPEECHES.iter().zip(&CHOCOLATES) {
            assert_ne!(*speech, chocolate.wrapper);
        }
        assert_eq!(speech(1), Some(SPEECHES[0]));
        assert_eq!(speech(25), None);
        assert_eq!(speech(0), None);
    }

    #[test]
    fn test_each_day_he_talks_to_the_player_in_a_few_sentences() {
        for line in SPEECHES {
            let sentences = line.matches(['.', '?', '!']).count();
            assert!((2..=3).contains(&sentences), "{line}");
            let lower = format!(" {} ", line.to_lowercase());
            assert!(
                [
                    " tu ", "toi", " ton ", " ta ", " te ", "t'", "tiens", "tien"
                ]
                .iter()
                .any(|word| lower.contains(word)),
                "{line}"
            );
        }
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
        for line in &SPEECHES[..23] {
            assert!(!line.contains("Joyeux Noël"), "{line}");
        }
        for chocolate in &CHOCOLATES {
            assert!(!chocolate.wrapper.contains("Joyeux Noël"));
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
