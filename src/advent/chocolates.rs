pub const DRAWING_WIDTH: usize = 8;

pub struct Chocolate {
    pub name: &'static str,
    pub drawing: [&'static str; 3],
    pub wrapper: &'static str,
}

pub const CHOCOLATES: [Chocolate; 24] = [
    Chocolate {
        name: "Truffe au champagne",
        drawing: ["  .--.  ", " (::::) ", "  `--`  "],
        wrapper: "Le Père Noël reçoit plus de lettres que tout le village réuni. Il les lit toutes, lunettes sur le nez.",
    },
    Chocolate {
        name: "Praliné noisette du Piémont",
        drawing: ["  ____  ", " |oooo| ", " |____| "],
        wrapper: "Que dit un bonhomme de neige à un autre\u{202F}? Ça sent la carotte par ici.",
    },
    Chocolate {
        name: "Ganache framboise",
        drawing: [" .----. ", " | @@ | ", " '----' "],
        wrapper: "Le traîneau n'a pas de rétroviseur. Le Père Noël ne regarde jamais en arrière.",
    },
    Chocolate {
        name: "Rocher aux amandes",
        drawing: ["  .:'.  ", " :'.:'. ", "  ':.'  "],
        wrapper: "Le Père Noël porte trois pulls sous son manteau. Il refuse d'en parler.",
    },
    Chocolate {
        name: "Mendiant aux fruits secs",
        drawing: ["  ____  ", " /o*.o\\ ", " \\____/ "],
        wrapper: "La barbe du Père Noël se brosse cent fois chaque soir. C'est Comète qui compte.",
    },
    Chocolate {
        name: "Palet d'or",
        drawing: ["  _**_  ", " (____) ", "  `--`  "],
        wrapper: "Pourquoi les rennes ne se perdent jamais\u{202F}? Ils ont toujours le Nord en tête.",
    },
    Chocolate {
        name: "Caramel au beurre salé",
        drawing: [" +----+ ", " |~~~~| ", " +----+ "],
        wrapper: "Que fait un lutin qui a froid\u{202F}? Il se met dans un coin, il y fait quatre-vingt-dix degrés.",
    },
    Chocolate {
        name: "Orangette confite",
        drawing: ["    //  ", "   //   ", "  //    "],
        wrapper: "Le Père Noël a un bureau de poste rien qu'à lui. Le facteur de là-bas garde ses moufles toute l'année.",
    },
    Chocolate {
        name: "Griotte à l'eau-de-vie",
        drawing: ["    /   ", "  _/_   ", " (___)  "],
        wrapper: "Comment appelle-t-on un chat tombé dans la peinture le jour de Noël\u{202F}? Un chat-peint de Noël.",
    },
    Chocolate {
        name: "Pavé à la fleur de sel",
        drawing: ["  ____  ", " /___/| ", " |___|/ "],
        wrapper: "Les rennes adorent les clémentines. Le Père Noël en garde toujours au fond de sa hotte.",
    },
    Chocolate {
        name: "Marron glacé",
        drawing: ["  .^.   ", " (   )  ", "  `-`   "],
        wrapper: "Les carottes laissées sous le sapin, le Père Noël ne les mange jamais. Il les rapporte aux rennes.",
    },
    Chocolate {
        name: "Bouchée pistache",
        drawing: ["  ____  ", " /####\\ ", " \\####/ "],
        wrapper: "Quel est le comble pour le Père Noël\u{202F}? Avoir le vertige en traîneau.",
    },
    Chocolate {
        name: "Cœur fondant passion",
        drawing: [" /\\ /\\  ", " \\   /  ", "  \\_/   "],
        wrapper: "En janvier, le Père Noël fait une sieste de trois jours. Interdit de sonner.",
    },
    Chocolate {
        name: "Lingot gianduja",
        drawing: ["   __   ", "  /  \\  ", " /____\\ "],
        wrapper: "Pourquoi le Père Noël arrive toujours à l'heure\u{202F}? Il a une hotte d'avance.",
    },
    Chocolate {
        name: "Nougatine croquante",
        drawing: ["  ____  ", " <____> ", "  ^^^^  "],
        wrapper: "La hotte du Père Noël a une poche secrète, pour les bonbons de la route.",
    },
    Chocolate {
        name: "Florentin aux amandes",
        drawing: [" ,-~~-, ", " (oO o) ", " `-~~-` "],
        wrapper: "Que dit un renne qui arrive en retard\u{202F}? Désolé, j'étais dans les bois.",
    },
    Chocolate {
        name: "Grains de café enrobés",
        drawing: ["  _  _  ", " (|)(|) ", "  `  `  "],
        wrapper: "Le Père Noël connaît le nom de chaque renne. Même ceux des voisins.",
    },
    Chocolate {
        name: "Feuilleté praliné",
        drawing: [" ====== ", " ------ ", " ====== "],
        wrapper: "Quel est le dessert préféré des bonshommes de neige\u{202F}? La bûche glacée.",
    },
    Chocolate {
        name: "Coquille noisette",
        drawing: ["  _,_   ", " /|||\\  ", " \\___/  "],
        wrapper: "Rodolphe a le nez rouge, mais il n'a jamais attrapé froid.",
    },
    Chocolate {
        name: "Bâton menthe poivrée",
        drawing: ["  ||||  ", "  ||||  ", "  ''''  "],
        wrapper: "Le Père Noël signe toutes ses lettres d'un seul trait. Il s'entraîne depuis des siècles.",
    },
    Chocolate {
        name: "Guimauve enrobée",
        drawing: ["  .--.  ", " |    | ", " '----' "],
        wrapper: "Pour faire le tour du monde en une nuit, le Père Noël suit les fuseaux horaires.",
    },
    Chocolate {
        name: "Étoile au miel",
        drawing: ["   /\\   ", " <    > ", "   \\/   "],
        wrapper: "Avant la tournée, le Père Noël boit un chocolat chaud. Avec double crème.",
    },
    Chocolate {
        name: "Cloche à la cannelle",
        drawing: ["  /^^\\  ", " /    \\ ", " '-o--' "],
        wrapper: "Que dit le sapin quand on le décore\u{202F}? J'ai les boules.",
    },
    Chocolate {
        name: "Bûche grand cru",
        drawing: ["  ____  ", " (____@ ", "  ~~~~  "],
        wrapper: "Cette nuit, le Père Noël passe dans toutes les maisons. Laisse-lui un verre de lait.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_every_chocolate_has_its_own_name_and_drawing() {
        let names: BTreeSet<_> = CHOCOLATES.iter().map(|chocolate| chocolate.name).collect();
        let drawings: BTreeSet<_> = CHOCOLATES
            .iter()
            .map(|chocolate| chocolate.drawing)
            .collect();
        assert_eq!(names.len(), CHOCOLATES.len());
        assert_eq!(drawings.len(), CHOCOLATES.len());
    }

    #[test]
    fn test_every_drawing_fills_the_same_box() {
        for chocolate in &CHOCOLATES {
            assert!(!chocolate.name.is_empty());
            for line in chocolate.drawing {
                assert!(line.is_ascii(), "{}", chocolate.name);
                assert_eq!(line.chars().count(), DRAWING_WIDTH, "{}", chocolate.name);
                assert!(
                    line.starts_with(' ') && line.ends_with(' '),
                    "{}",
                    chocolate.name
                );
            }
        }
    }
}
