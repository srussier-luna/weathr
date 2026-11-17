use crate::badges;
use crate::story::punishment::{BARE_VILLAGE_DAYS, Punishment};
use rand::RngExt;

pub const SIGNATURE: &str = "— Tes voisins";

const DISAPPOINTED: [&str; 8] = [
    "Tricher, ça casse le fun. Pour toi, et pour tout le village.",
    "Franchement, on te croyait plus fort que ça.",
    "On en a parlé à la mairie. Verdict à l'unanimité\u{202F}: c'est nul.",
    "Le Facteur passe devant ta boîte sans ralentir. Il trouve ça mérité.",
    "Gagner sans jouer, c'est perdre deux fois.",
    "On attendait mieux de toi. Vraiment.",
    "Tu sais ce qui est encore plus fun que tricher\u{202F}? Jouer.",
    "Les Habitants boudent toujours. Nous aussi, un peu.",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeighboursNote {
    Announcement(Punishment),
    Disappointed(usize),
}

impl NeighboursNote {
    pub fn disappointed(rng: &mut impl rand::Rng) -> Self {
        Self::Disappointed(rng.random_range(0..DISAPPOINTED.len()))
    }

    pub fn text(self) -> String {
        match self {
            Self::Announcement(Punishment::Cheater) => format!(
                "On t'a vu tricher. Au village, ça ne se fait pas. Voici ton badge\u{202F}: {}. Il restera dans ta collection, pour toujours.",
                badges::name(badges::CHEATER)
            ),
            Self::Announcement(Punishment::BareVillage) => format!(
                "Encore une triche\u{202F}? Les Habitants sont partis bouder {BARE_VILLAGE_DAYS} jours, et le grand Arbre en a perdu ses feuilles."
            ),
            Self::Announcement(Punishment::MailBan(days)) => format!(
                "Tu as encore triché. Pendant {days} jours, le Facteur ne s'arrête plus chez toi, les Habitants boudent et le grand Arbre reste tout nu. Ensuite, l'histoire reprendra là où tu l'as laissée."
            ),
            Self::Disappointed(line) => DISAPPOINTED[line % DISAPPOINTED.len()].to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn every_note() -> Vec<NeighboursNote> {
        [
            Punishment::Cheater,
            Punishment::BareVillage,
            Punishment::MailBan(3),
        ]
        .map(NeighboursNote::Announcement)
        .into_iter()
        .chain((0..DISAPPOINTED.len()).map(NeighboursNote::Disappointed))
        .collect()
    }

    #[test]
    fn test_announcements_name_their_punishment() {
        let text = |punishment| NeighboursNote::Announcement(punishment).text();
        assert!(text(Punishment::Cheater).contains("Tricheur"));
        assert!(text(Punishment::BareVillage).contains("7 jours"));
        assert!(text(Punishment::MailBan(6)).contains("6 jours"));
        assert!(text(Punishment::MailBan(9)).contains("9 jours"));
    }

    #[test]
    fn test_disappointed_line_is_drawn_from_the_list() {
        let mut rng = rand::rng();
        for _ in 0..50 {
            let NeighboursNote::Disappointed(line) = NeighboursNote::disappointed(&mut rng) else {
                panic!("not a disappointed note");
            };
            assert!(line < DISAPPOINTED.len());
        }
    }

    #[test]
    fn test_notes_keep_french_punctuation_with_its_word() {
        for note in every_note() {
            let text = note.text();
            for mark in [':', ';', '!', '?'] {
                assert!(!text.contains(&format!(" {mark}")), "{text}");
            }
        }
    }
}
