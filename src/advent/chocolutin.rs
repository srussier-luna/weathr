pub const SIGNATURE: &str = "— Chocolutin";

pub const INTRODUCTION: &str = "Hop, me voilà\u{202F}! Moi, c'est Chocolutin, de l'atelier du Père Noël. Il m'a choisi spécialement pour t'apporter un chocolat chaque jour jusqu'au 24. Il a même dit que l'atelier serait bien plus calme comme ça. Sûrement parce que je travaille très vite.";

pub const SPEECHES: [&str; 24] = [
    "Tiens, c'est pour toi, tout juste démoulé. J'ai mis mes bottes à l'envers ce matin, mais j'ai trouvé ta maison du premier coup. Pas mal pour un premier jour, non\u{202F}?",
    "Voilà ton chocolat, je l'ai tenu à deux mains tout le chemin. Ce matin, j'ai rangé les mille deux cents moules de l'atelier par taille. Enfin, par couleur, c'était plus simple.",
    "Tiens, il est encore tout brillant. Avant, c'est moi qui tempérais le chocolat à l'atelier. Je me demande qui le fait à ma place, il doit être très fort.",
    "Le tien, je l'ai gardé à l'abri de la neige. Ce matin, j'ai confondu la farine et le sucre glace dans les biscuits des rennes. Ils boudent encore, tu sais.",
    "Tiens, je ne me suis trompé de maison qu'une fois. Avant, je triais aussi les lettres du Père Noël. Et puis il m'a trouvé ce travail de livreur, rien que pour moi.",
    "C'est la Saint-Nicolas, tu le savais\u{202F}? J'ai voulu lui envoyer une carte, mais je l'ai postée dans la cheminée. Ton chocolat, au moins, je te le donne en main propre.",
    "Excuse le retard, Tornade a mangé ma carte de la tournée. Heureusement, je connais le chemin jusqu'à chez toi. Presque.",
    "Tiens, celui-là, je ne l'ai pas fait tomber une seule fois. Le Père Noël a fait changer la serrure de l'atelier de chocolat. Il a sûrement perdu la clé, je lui ai proposé de chercher.",
    "Tu sens cette odeur\u{202F}? C'est mon bonnet, je l'ai trempé dans le chocolat fondu. Il sent bon, mais il colle.",
    "Tiens, garde-le au chaud. Ce matin, j'ai demandé au Père Noël si je pouvais retourner à l'atelier. Il a ri très fort, je crois que c'est bon signe.",
    "Ce matin, j'ai glissé sur le verglas et fait trois tours sur moi-même. Ne t'en fais pas, le chocolat n'a rien eu. Moi non plus, enfin presque.",
    "À l'atelier, ils ont accroché mon portrait près de la porte, avec écrit «\u{202F}Attention\u{202F}» dessous. C'est gentil de penser à moi. Tiens, voilà le tien.",
    "Tiens, et regarde-moi cette neige sur mon bonnet. Fringant a fait trois loopings à l'essai du traîneau, moi j'ai fait le quatrième tout seul. Le Père Noël a dit que j'avais un don.",
    "J'ai proposé d'aider au bureau de poste du Pôle Nord. Ils ont dit que ça allait, merci. Tout le monde est très gentil avec moi, tu as remarqué\u{202F}?",
    "Ce matin, j'ai emballé un cadeau, puis un deuxième. Le troisième, c'était Cupidon. Il a bien voulu en rire, et toi\u{202F}?",
    "Il manquait un grelot au traîneau ce matin, tout l'atelier l'a cherché. Il était dans ma poche. Tiens, je te donne ton chocolat avant de le perdre aussi.",
    "Le Père Noël dit que je suis le plus rapide pour livrer. Il me rendra peut-être ma louche à chocolat l'an prochain. Tu crois que je devrais ralentir\u{202F}?",
    "Hier soir, j'ai fait un chocolat tout seul, en cachette dans ma chambre. La casserole est restée collée au plafond. Ne le dis pas au Père Noël, tu veux bien\u{202F}?",
    "À l'atelier, on tempère à trente et un degrés pile. Moi, on ne me laisse plus approcher du thermomètre. Je ne l'ai cassé qu'une fois, tu sais.",
    "Ce matin, j'ai aidé à charger le traîneau et je suis resté coincé dans la hotte. C'est Comète qui m'en a sorti. Ton chocolat, lui, était bien rangé.",
    "J'ai glissé une clémentine dans un moule à chocolat, pour essayer. Personne n'a trouvé l'idée aussi bonne que moi. Toi, tu aimerais, non\u{202F}?",
    "Je me suis entraîné tout le mois à ne rien renverser. Ce matin, je n'ai renversé qu'un seau, et le Père Noël a souri. Je progresse, tu ne trouves pas\u{202F}?",
    "Dernière répétition\u{202F}: hop, les rennes décollent. Moi, je suis resté accroché au grelot de Rodolphe. Ne le dis à personne, et voilà ton chocolat.",
    "Voilà le dernier, je te l'ai porté comme un trésor. Ce soir, c'est la grande tournée, et l'an prochain, je retourne sûrement à l'atelier de chocolat. Joyeux Noël\u{202F}!",
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
